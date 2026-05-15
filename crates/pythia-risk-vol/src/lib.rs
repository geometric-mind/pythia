//! # pythia-risk-vol
//!
//! Verified EWMA and GARCH volatility forecasting.
//!
//! ## Lean specification (`Pythia.Finance.Risk.VolForecasting`)
//!
//! - **EWMA nonneg** (`ewma_nonneg`)
//! - **EWMA convex combination** (`ewma_between`)
//! - **EWMA persistence monotone** (`ewma_persistence`)
//! - **GARCH unconditional positive** (`garch_unconditional_pos`)
//! - **GARCH stationarity**: α+β < 1 (`garch_stationarity_condition`)

/// EWMA variance: λσ² + (1-λ)r².
/// # Lean: `ewmaVariance`
#[inline(always)]
pub fn ewma(lambda: f64, sigma_sq: f64, r_sq: f64) -> f64 {
    lambda * sigma_sq + (1.0 - lambda) * r_sq
}

/// GARCH(1,1) unconditional variance: ω / (1 - α - β).
/// # Lean: `garch_unconditional_pos`
pub fn garch_unconditional(omega: f64, alpha: f64, beta: f64) -> f64 {
    assert!(alpha + beta < 1.0, "stationarity requires α+β < 1");
    omega / (1.0 - alpha - beta)
}

/// Check GARCH stationarity: α+β < 1.
/// # Lean: `garch_stationarity_condition`
pub fn is_stationary(alpha: f64, beta: f64) -> bool {
    alpha + beta < 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ewma_nonneg() {
        assert!(ewma(0.94, 0.01, 0.02) >= 0.0);
    }

    #[test]
    fn ewma_convex() {
        let s = 0.01; let r = 0.04;
        let e = ewma(0.94, s, r);
        assert!(e >= s && e <= r);
    }

    #[test]
    fn ewma_persistence_mono() {
        let s = 0.04; let r = 0.01; // s > r case
        assert!(ewma(0.9, s, r) <= ewma(0.95, s, r) + 1e-10);
    }

    #[test]
    fn garch_unconditional_pos() {
        assert!(garch_unconditional(0.00001, 0.05, 0.90) > 0.0);
    }

    #[test]
    fn garch_stationary() {
        assert!(is_stationary(0.05, 0.90));
        assert!(!is_stationary(0.1, 0.95));
    }
}
