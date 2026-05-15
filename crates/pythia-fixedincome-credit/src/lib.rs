//! # Credit Curve Properties
//!
//! Rust implementation of credit term structure properties proven in
//! `Pythia.Finance.FixedIncome.CreditCurve`. Covers hazard rate bootstrapping,
//! survival curve monotonicity, and spread-hazard relationships.

/// Cumulative probability of default at a given time.
/// Monotone in time: more time implies higher default probability.
///
/// Corresponds to Lean theorem `cum_pd_mono`.
pub fn cumulative_pd(hazard_rate: f64, time: f64) -> f64 {
    1.0 - (-hazard_rate * time).exp()
}

/// Survival probability at time t: S(t) = 1 - PD(t) = exp(-h*t).
/// Antitone (decreasing) in time.
///
/// Corresponds to Lean theorem `survival_antitone`.
pub fn survival_probability(hazard_rate: f64, time: f64) -> f64 {
    (-hazard_rate * time).exp()
}

/// Marginal probability of default between t1 and t2.
/// Nonneg when t1 <= t2 and hazard_rate >= 0.
///
/// Corresponds to Lean theorem `marginal_pd_nonneg`.
pub fn marginal_pd(hazard_rate: f64, t1: f64, t2: f64) -> f64 {
    cumulative_pd(hazard_rate, t2) - cumulative_pd(hazard_rate, t1)
}

/// Hazard rate bootstrapped from marginal PD, survival, and time interval.
/// h = dpd / (S * dT). Nonneg when dpd >= 0, S > 0, dT > 0.
///
/// Corresponds to Lean theorem `hazard_from_marginal_nonneg`.
pub fn hazard_from_marginal(dpd: f64, survival: f64, dt: f64) -> f64 {
    assert!(survival > 0.0, "survival must be positive");
    assert!(dt > 0.0, "dt must be positive");
    dpd / (survival * dt)
}

/// Credit spread approximation: spread = h * (1 - R).
/// Monotone in hazard rate for fixed recovery.
///
/// Corresponds to Lean theorem `spread_mono_hazard`.
pub fn credit_spread(hazard_rate: f64, recovery: f64) -> f64 {
    hazard_rate * (1.0 - recovery)
}

/// Risky discount factor: D_risky = D_riskfree * S(t).
/// Always <= riskfree discount when 0 <= S <= 1.
///
/// Corresponds to Lean theorem `risky_discount_le_riskfree`.
pub fn risky_discount(riskfree_discount: f64, survival: f64) -> f64 {
    riskfree_discount * survival
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cumulative_pd_increases_with_time() {
        let h = 0.02;
        let pd1 = cumulative_pd(h, 1.0);
        let pd2 = cumulative_pd(h, 5.0);
        assert!(pd1 <= pd2, "cum_pd_mono: PD must increase with time");
    }

    #[test]
    fn test_survival_decreases_with_time() {
        let h = 0.03;
        let s1 = survival_probability(h, 1.0);
        let s2 = survival_probability(h, 5.0);
        assert!(s2 <= s1, "survival_antitone: survival must decrease");
    }

    #[test]
    fn test_marginal_pd_nonneg() {
        let h = 0.05;
        let mpd = marginal_pd(h, 1.0, 3.0);
        assert!(mpd >= 0.0, "marginal_pd_nonneg: must be >= 0");
    }

    #[test]
    fn test_hazard_from_marginal_nonneg() {
        let result = hazard_from_marginal(0.02, 0.95, 1.0);
        assert!(result >= 0.0, "hazard_from_marginal_nonneg");
    }

    #[test]
    fn test_spread_mono_hazard() {
        let r = 0.4;
        let s1 = credit_spread(0.01, r);
        let s2 = credit_spread(0.03, r);
        assert!(s1 <= s2, "spread_mono_hazard: higher hazard = wider spread");
    }

    #[test]
    fn test_risky_discount_le_riskfree() {
        let d_rf = 0.95;
        let s = 0.90;
        let d_risky = risky_discount(d_rf, s);
        assert!(
            d_risky <= d_rf,
            "risky_discount_le_riskfree: risky <= riskfree"
        );
    }
}
