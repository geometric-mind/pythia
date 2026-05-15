//! # pythia-stochastic-gbm
//!
//! Verified GBM (geometric Brownian motion) path properties.
//!
//! ## Lean specification (`Pythia.Finance.Stochastic.GBMProperties`)
//!
//! - **GBM positive**: S0 > 0 ⟹ S_T > 0 (`gbmTerminal_pos`)
//! - **Log return decomposition**: log(S_T/S0) = (μ-σ²/2)T + σW_T (`log_return_decompose`)
//! - **Drift monotonicity**: higher μ → higher terminal (`gbmTerminal_mono_drift`)
//! - **Vol drag nonneg**: σ²/2 ≥ 0 (`vol_drag_nonneg`)

/// GBM terminal value: S_T = S0 * exp((μ - σ²/2)*T + σ*W_T).
///
/// # Lean: `gbmTerminal`
#[inline(always)]
pub fn gbm_terminal(s0: f64, mu: f64, sigma: f64, t: f64, w_t: f64) -> f64 {
    s0 * ((mu - sigma * sigma / 2.0) * t + sigma * w_t).exp()
}

/// Log return: log(S_T / S0).
///
/// # Lean: `log_return_decompose`
#[inline(always)]
pub fn log_return(s0: f64, mu: f64, sigma: f64, t: f64, w_t: f64) -> f64 {
    (mu - sigma * sigma / 2.0) * t + sigma * w_t
}

/// Volatility drag: σ²/2.
///
/// # Lean: `vol_drag_nonneg`
#[inline(always)]
pub fn vol_drag(sigma: f64) -> f64 {
    sigma * sigma / 2.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gbm_positive() {
        let s = gbm_terminal(100.0, 0.05, 0.2, 1.0, -2.0);
        assert!(s > 0.0);
    }

    #[test]
    fn gbm_at_zero() {
        let s = gbm_terminal(100.0, 0.05, 0.2, 0.0, 0.0);
        assert!((s - 100.0).abs() < 1e-10);
    }

    #[test]
    fn log_return_matches() {
        let s0 = 100.0;
        let s_t = gbm_terminal(s0, 0.05, 0.2, 1.0, 0.5);
        let lr_from_price = (s_t / s0).ln();
        let lr_formula = log_return(s0, 0.05, 0.2, 1.0, 0.5);
        assert!((lr_from_price - lr_formula).abs() < 1e-10);
    }

    #[test]
    fn higher_drift_higher_terminal() {
        let s_low = gbm_terminal(100.0, 0.03, 0.2, 1.0, 0.0);
        let s_high = gbm_terminal(100.0, 0.10, 0.2, 1.0, 0.0);
        assert!(s_high >= s_low);
    }

    #[test]
    fn vol_drag_nonneg() {
        assert!(vol_drag(0.3) >= 0.0);
        assert!(vol_drag(-0.3) >= 0.0);
    }
}
