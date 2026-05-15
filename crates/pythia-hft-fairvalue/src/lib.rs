//! # pythia-hft-fairvalue
//!
//! Verified micro-price and EWMA fair value estimators.
//!
//! ## Lean specification (`Pythia.Finance.HFT.FairValueEstimator`)
//!
//! - **Micro-price between bid and ask** (`microPrice_between`)
//! - **Equal sizes → true mid** (`microPrice_equal_sizes`)
//! - **Imbalance shifts toward thin side** (`microPrice_shifts_toward_thin`)
//! - **EWMA between old and new** (`ewma_between`)

/// Compute micro-price: volume-weighted mid.
///
/// # Lean: `microPrice`
/// `micro = (ask_qty * bid + bid_qty * ask) / (bid_qty + ask_qty)`
#[inline(always)]
pub fn micro_price(bid: f64, ask: f64, bid_qty: f64, ask_qty: f64) -> f64 {
    (ask_qty * bid + bid_qty * ask) / (bid_qty + ask_qty)
}

/// EWMA fair value update.
///
/// # Lean: `ewmaUpdate`
/// `new = alpha * trade + (1 - alpha) * old`
#[inline(always)]
pub fn ewma_update(alpha: f64, trade: f64, old: f64) -> f64 {
    alpha * trade + (1.0 - alpha) * old
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn micro_between_bid_ask() {
        let m = micro_price(100.0, 101.0, 50.0, 50.0);
        assert!(100.0 <= m && m <= 101.0);
    }

    #[test]
    fn equal_sizes_true_mid() {
        let m = micro_price(100.0, 102.0, 100.0, 100.0);
        assert!((m - 101.0).abs() < 1e-10);
    }

    #[test]
    fn imbalance_shifts_toward_thin() {
        let mid = (100.0 + 102.0) / 2.0;
        let m = micro_price(100.0, 102.0, 10.0, 90.0);
        assert!(m < mid);
    }

    #[test]
    fn ewma_between_old_and_new() {
        let old = 100.0;
        let trade = 105.0;
        let new_val = ewma_update(0.3, trade, old);
        assert!(old <= new_val && new_val <= trade);
    }
}
