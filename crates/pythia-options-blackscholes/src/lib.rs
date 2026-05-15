//! # pythia-options-blackscholes
//!
//! Black-Scholes call pricing with verified properties.
//!
//! ## Lean specification (`Pythia.Finance.BlackScholesCallClosedForm`)
//!
//! - **BS call formula**: S*Phi(d1) - K*exp(-rT)*Phi(d2) (`bsCall`)
//! - **Zero-time reduction**: at T=0, reduces to S*Phi(d1) - K*Phi(d2) (`bsCall_zero_time`)
//! - **Zero-rate reduction**: at r=0, same reduction (`bsCall_zero_rate`)
//! - **Linearity in S**: shifting S by dS shifts call by dS*Phi(d1) (`bsCall_linear_S`)
//! - **Deep ITM positivity**: with Phi close to 1, call > 0 (`bsCall_strict_pos_under_unit_Phi`)

/// Normal CDF via Abramowitz-Stegun rational approximation.
///
/// Maximum absolute error ~1.5e-7, sufficient for pricing checks.
fn norm_cdf(x: f64) -> f64 {
    if x >= 0.0 {
        let t = 1.0 / (1.0 + 0.2316419 * x);
        let d = 0.3989422804014327 * (-x * x / 2.0).exp();
        1.0 - d * t * (0.3193815 + t * (-0.3565638 + t * (1.781478 + t * (-1.8212560 + t * 1.3302744))))
    } else {
        1.0 - norm_cdf(-x)
    }
}

/// Black-Scholes call price from precomputed Phi(d1), Phi(d2).
///
/// Formula: S * phi_d1 - K * exp(-r * T) * phi_d2
///
/// # Lean: `bsCall`
pub fn bs_call(s: f64, k: f64, t: f64, r: f64, phi_d1: f64, phi_d2: f64) -> f64 {
    s * phi_d1 - k * (-r * t).exp() * phi_d2
}

/// Full Black-Scholes call price computing d1, d2 internally.
///
/// d1 = (ln(S/K) + (r + sigma^2/2)*T) / (sigma * sqrt(T))
/// d2 = d1 - sigma * sqrt(T)
///
/// # Lean: `bsCall` (with d1/d2 definitions expanded)
///
/// # Panics
/// Panics if `t <= 0.0` or `sigma <= 0.0` or `s <= 0.0` or `k <= 0.0`.
pub fn bs_call_full(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    assert!(t > 0.0, "time to expiry must be positive");
    assert!(sigma > 0.0, "volatility must be positive");
    assert!(s > 0.0, "spot must be positive");
    assert!(k > 0.0, "strike must be positive");

    let sqrt_t = t.sqrt();
    let d1 = ((s / k).ln() + (r + sigma * sigma / 2.0) * t) / (sigma * sqrt_t);
    let d2 = d1 - sigma * sqrt_t;
    bs_call(s, k, t, r, norm_cdf(d1), norm_cdf(d2))
}

/// Check that a deep ITM call (S >> K) has positive price.
///
/// # Lean: `bsCall_strict_pos_under_unit_Phi`
pub fn check_deep_itm_positive(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> bool {
    if t <= 0.0 || sigma <= 0.0 || s <= 0.0 || k <= 0.0 {
        return false;
    }
    bs_call_full(s, k, t, r, sigma) > 0.0
}

/// Compute d1 for a given set of parameters (exposed for testing).
///
/// d1 = (ln(S/K) + (r + sigma^2/2)*T) / (sigma * sqrt(T))
pub fn compute_d1(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    let sqrt_t = t.sqrt();
    ((s / k).ln() + (r + sigma * sigma / 2.0) * t) / (sigma * sqrt_t)
}

/// Expose the normal CDF for testing.
pub fn phi(x: f64) -> f64 {
    norm_cdf(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bs_call_basic() {
        // S=100, K=100, T=1, r=0.05, sigma=0.2 => well-known value ~10.45
        let price = bs_call_full(100.0, 100.0, 1.0, 0.05, 0.2);
        assert!(price > 10.0 && price < 11.0, "ATM call ~10.45, got {}", price);
    }

    #[test]
    fn bs_call_deep_itm() {
        // Deep ITM: S=200, K=100
        let price = bs_call_full(200.0, 100.0, 1.0, 0.05, 0.2);
        // Should be close to S - K*exp(-rT) = 200 - 100*exp(-0.05) ~ 104.88
        let intrinsic = 200.0 - 100.0 * (-0.05_f64).exp();
        assert!(price >= intrinsic - 1.0, "deep ITM call should be near intrinsic");
    }

    #[test]
    fn bs_call_deep_otm() {
        // Deep OTM: S=50, K=200
        let price = bs_call_full(50.0, 200.0, 1.0, 0.05, 0.2);
        assert!(price < 1.0, "deep OTM call should be near zero, got {}", price);
    }

    #[test]
    fn bs_call_from_precomputed_phi() {
        // With phi_d1=1, phi_d2=1, call = S - K*exp(-rT) (deep ITM limit)
        let price = bs_call(100.0, 90.0, 1.0, 0.05, 1.0, 1.0);
        let expected = 100.0 - 90.0 * (-0.05_f64).exp();
        assert!((price - expected).abs() < 1e-10);
    }

    #[test]
    fn norm_cdf_symmetry() {
        // Phi(0) = 0.5
        assert!((phi(0.0) - 0.5).abs() < 1e-7);
        // Phi(x) + Phi(-x) = 1
        for &x in &[0.5, 1.0, 2.0, 3.0] {
            assert!((phi(x) + phi(-x) - 1.0).abs() < 1e-7);
        }
    }

    #[test]
    fn deep_itm_positive_check() {
        assert!(check_deep_itm_positive(200.0, 100.0, 1.0, 0.05, 0.2));
    }

    #[test]
    fn check_deep_itm_rejects_bad_inputs() {
        assert!(!check_deep_itm_positive(200.0, 100.0, 0.0, 0.05, 0.2));
        assert!(!check_deep_itm_positive(200.0, 100.0, 1.0, 0.05, 0.0));
    }
}
