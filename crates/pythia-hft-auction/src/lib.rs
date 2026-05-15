//! # pythia-hft-auction
//!
//! Verified opening/closing auction mechanism.
//!
//! ## Lean specification (`Pythia.Finance.HFT.AuctionMechanism`)
//!
//! - **Clearing price bounded**: between best bid and ask (`clearing_price_bounded`)
//! - **Buyer/seller surplus nonneg** (`buyer_surplus_nonneg`, `seller_surplus_nonneg`)
//! - **Uniform price fairness** (`uniform_price`)

/// An auction order (limit price + quantity).
#[derive(Debug, Clone, Copy)]
pub struct AuctionOrder {
    pub limit_price: f64,
    pub qty: f64,
    pub is_buy: bool,
}

/// Auction result.
#[derive(Debug, Clone, Copy)]
pub struct AuctionResult {
    pub clearing_price: f64,
    pub matched_volume: f64,
}

impl AuctionResult {
    /// Buyer surplus: limit - clearing price.
    ///
    /// # Lean: `buyer_surplus_nonneg`
    pub fn buyer_surplus(&self, limit_price: f64) -> f64 {
        limit_price - self.clearing_price
    }

    /// Seller surplus: clearing price - limit.
    ///
    /// # Lean: `seller_surplus_nonneg`
    pub fn seller_surplus(&self, limit_price: f64) -> f64 {
        self.clearing_price - limit_price
    }

    /// Check clearing price is between bid and ask.
    ///
    /// # Lean: `clearing_price_bounded`
    pub fn price_bounded(&self, best_bid: f64, best_ask: f64) -> bool {
        best_bid <= self.clearing_price && self.clearing_price <= best_ask
    }
}

/// Run a simple uniform-price auction.
pub fn run_auction(orders: &[AuctionOrder]) -> Option<AuctionResult> {
    let mut bids: Vec<f64> = orders.iter().filter(|o| o.is_buy).map(|o| o.limit_price).collect();
    let mut asks: Vec<f64> = orders.iter().filter(|o| !o.is_buy).map(|o| o.limit_price).collect();
    bids.sort_by(|a, b| b.partial_cmp(a).unwrap());
    asks.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if bids.is_empty() || asks.is_empty() || bids[0] < asks[0] {
        return None;
    }
    let clearing = (bids[0] + asks[0]) / 2.0;
    let buy_vol: f64 = orders.iter().filter(|o| o.is_buy && o.limit_price >= clearing).map(|o| o.qty).sum();
    let sell_vol: f64 = orders.iter().filter(|o| !o.is_buy && o.limit_price <= clearing).map(|o| o.qty).sum();
    Some(AuctionResult {
        clearing_price: clearing,
        matched_volume: buy_vol.min(sell_vol),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_auction() {
        let orders = vec![
            AuctionOrder { limit_price: 101.0, qty: 100.0, is_buy: true },
            AuctionOrder { limit_price: 99.0, qty: 100.0, is_buy: false },
        ];
        let result = run_auction(&orders).unwrap();
        assert!(result.price_bounded(99.0, 101.0));
        assert!(result.buyer_surplus(101.0) >= 0.0);
        assert!(result.seller_surplus(99.0) >= 0.0);
    }

    #[test]
    fn no_cross_no_auction() {
        let orders = vec![
            AuctionOrder { limit_price: 99.0, qty: 100.0, is_buy: true },
            AuctionOrder { limit_price: 101.0, qty: 100.0, is_buy: false },
        ];
        assert!(run_auction(&orders).is_none());
    }
}
