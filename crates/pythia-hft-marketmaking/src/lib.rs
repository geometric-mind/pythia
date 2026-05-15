//! # pythia-hft-marketmaking
//!
//! Verified market-making primitives for the Avellaneda-Stoikov model.
//!
//! ## Lean specification
//!
//! Every function has a corresponding theorem in
//! `Pythia.Finance.HFT.MarketMaking` (Lean 4):
//!
//! - **Spread profit ≥ 0**: `(ask - bid) * qty ≥ 0` (`spread_profit_nonneg`)
//! - **Inventory risk nonneg**: `q² * σ² ≥ 0` (`inventory_risk_nonneg`)
//! - **Risk monotone in position**: larger |q| → more risk (`inventory_risk_mono`)
//! - **Spread widens with vol**: `k*σ₁ ≤ k*σ₂` when `σ₁ ≤ σ₂` (`spread_widens_with_vol`)
//! - **Inventory skew direction**: long inventory skews quotes down (`inventory_skew_direction`)
//! - **Round trip profitable iff spread > 2*fee** (`round_trip_profitable`)
//! - **Symmetric spread**: `ask - bid = 2 * half_spread` (`symmetric_spread`)

/// A two-sided quote: bid and ask prices.
#[derive(Debug, Clone, Copy)]
pub struct Quote {
    pub bid: f64,
    pub ask: f64,
}

impl Quote {
    /// The spread (ask - bid).
    #[inline(always)]
    pub fn spread(&self) -> f64 {
        self.ask - self.bid
    }

    /// Profit from a completed round trip (buy at bid, sell at ask).
    ///
    /// # Lean theorem: `spread_profit_nonneg`
    /// `0 ≤ (ask - bid) * qty` when `bid ≤ ask` and `0 ≤ qty`
    #[inline(always)]
    pub fn round_trip_profit(&self, qty: f64) -> f64 {
        self.spread() * qty
    }

    /// Net PnL after fees: `(spread - 2*fee) * qty`.
    ///
    /// # Lean theorem: `round_trip_profitable`
    /// Positive iff `spread > 2 * fee`.
    #[inline(always)]
    pub fn net_pnl(&self, qty: f64, fee_per_side: f64) -> f64 {
        (self.spread() - 2.0 * fee_per_side) * qty
    }
}

/// Avellaneda-Stoikov market maker.
///
/// Computes optimal quotes given mid price, volatility, inventory,
/// and risk aversion parameter gamma.
#[derive(Debug, Clone, Copy)]
pub struct AvellanedaStoikov {
    pub gamma: f64,
    pub sigma: f64,
}

impl AvellanedaStoikov {
    pub fn new(gamma: f64, sigma: f64) -> Self {
        assert!(gamma > 0.0, "gamma must be positive");
        assert!(sigma >= 0.0, "sigma must be non-negative");
        Self { gamma, sigma }
    }

    /// Inventory risk: q² * σ².
    ///
    /// # Lean theorem: `inventory_risk_nonneg`
    /// `0 ≤ q² * σ²`
    #[inline(always)]
    pub fn inventory_risk(&self, q: f64) -> f64 {
        q * q * self.sigma * self.sigma
    }

    /// Inventory skew: adjusts mid price based on position.
    ///
    /// # Lean theorem: `inventory_skew_direction`
    /// Long inventory (q > 0) skews mid down.
    #[inline(always)]
    pub fn adjusted_mid(&self, mid: f64, q: f64) -> f64 {
        mid - self.gamma * q
    }

    /// Optimal half-spread (proportional to sigma).
    ///
    /// # Lean theorem: `spread_widens_with_vol`
    /// Higher σ → wider spread.
    #[inline(always)]
    pub fn half_spread(&self, k: f64) -> f64 {
        k * self.sigma
    }

    /// Compute symmetric quotes around adjusted mid.
    ///
    /// # Lean theorem: `symmetric_spread`
    /// `ask - bid = 2 * half_spread`
    #[inline(always)]
    pub fn quote(&self, mid: f64, q: f64, k: f64) -> Quote {
        let adj_mid = self.adjusted_mid(mid, q);
        let hs = self.half_spread(k);
        Quote {
            bid: adj_mid - hs,
            ask: adj_mid + hs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spread_profit_nonneg() {
        let q = Quote { bid: 100.0, ask: 100.05 };
        assert!(q.round_trip_profit(100.0) >= 0.0);
    }

    #[test]
    fn inventory_risk_nonneg() {
        let mm = AvellanedaStoikov::new(0.1, 0.02);
        assert!(mm.inventory_risk(500.0) >= 0.0);
        assert!(mm.inventory_risk(-500.0) >= 0.0);
        assert_eq!(mm.inventory_risk(0.0), 0.0);
    }

    #[test]
    fn inventory_risk_monotone() {
        let mm = AvellanedaStoikov::new(0.1, 0.02);
        assert!(mm.inventory_risk(100.0) <= mm.inventory_risk(200.0));
    }

    #[test]
    fn spread_widens_with_vol() {
        let low_vol = AvellanedaStoikov::new(0.1, 0.01);
        let high_vol = AvellanedaStoikov::new(0.1, 0.05);
        let k = 1.5;
        assert!(low_vol.half_spread(k) <= high_vol.half_spread(k));
    }

    #[test]
    fn long_inventory_skews_down() {
        let mm = AvellanedaStoikov::new(0.1, 0.02);
        let mid = 100.0;
        assert!(mm.adjusted_mid(mid, 500.0) < mid);
    }

    #[test]
    fn round_trip_profitable_iff_spread_gt_2fee() {
        let q = Quote { bid: 100.0, ask: 100.10 };
        assert!(q.net_pnl(100.0, 0.04) > 0.0);
        assert!(q.net_pnl(100.0, 0.06) < 0.0);
    }

    #[test]
    fn symmetric_spread() {
        let mm = AvellanedaStoikov::new(0.1, 0.02);
        let q = mm.quote(100.0, 0.0, 1.5);
        let expected_spread = 2.0 * mm.half_spread(1.5);
        assert!((q.spread() - expected_spread).abs() < 1e-10);
    }
}
