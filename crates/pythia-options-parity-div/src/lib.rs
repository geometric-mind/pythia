//! # pythia-options-parity-div
//!
//! Put-call parity for dividend-paying underlying assets.
//!
//! ## Lean specification (`Pythia.Finance.PutCallParityDividend`)
//!
//! - **Dividend-adjusted call payoff**: max(S*exp(-qT) - K*exp(-rT), 0) (`callPayoffDiv`)
//! - **Dividend-adjusted put payoff**: max(K*exp(-rT) - S*exp(-qT), 0) (`putPayoffDiv`)
//! - **Parity with dividends**: C - P = S*exp(-qT) - K*exp(-rT) (`put_call_parity_dividend`)
//!
//! Setting q = 0 recovers the standard no-dividend parity.

/// Dividend-adjusted European call payoff:
/// max(S * exp(-q*T) - K * exp(-r*T), 0).
///
/// # Lean: `callPayoffDiv`
pub fn call_payoff_div(s: f64, k: f64, t: f64, r: f64, q: f64) -> f64 {
    let s_adj = s * (-q * t).exp();
    let k_disc = k * (-r * t).exp();
    (s_adj - k_disc).max(0.0)
}

/// Dividend-adjusted European put payoff:
/// max(K * exp(-r*T) - S * exp(-q*T), 0).
///
/// # Lean: `putPayoffDiv`
pub fn put_payoff_div(s: f64, k: f64, t: f64, r: f64, q: f64) -> f64 {
    let s_adj = s * (-q * t).exp();
    let k_disc = k * (-r * t).exp();
    (k_disc - s_adj).max(0.0)
}

/// Dividend-adjusted forward: S * exp(-q*T) - K * exp(-r*T).
///
/// This is the parity RHS.
pub fn dividend_forward(s: f64, k: f64, t: f64, r: f64, q: f64) -> f64 {
    s * (-q * t).exp() - k * (-r * t).exp()
}

/// Check put-call parity with dividends:
/// call - put = S*exp(-qT) - K*exp(-rT).
///
/// Returns true when the identity holds within tolerance `tol`.
///
/// # Lean: `put_call_parity_dividend`
pub fn check_parity_div(
    call: f64,
    put: f64,
    s: f64,
    k: f64,
    t: f64,
    r: f64,
    q: f64,
    tol: f64,
) -> bool {
    let lhs = call - put;
    let rhs = dividend_forward(s, k, t, r, q);
    (lhs - rhs).abs() < tol
}

/// Check that setting q = 0 recovers standard parity:
/// call - put = S - K*exp(-rT).
///
/// This is a specialization of `put_call_parity_dividend` with q=0.
pub fn check_parity_no_div(
    call: f64,
    put: f64,
    s: f64,
    k: f64,
    t: f64,
    r: f64,
    tol: f64,
) -> bool {
    check_parity_div(call, put, s, k, t, r, 0.0, tol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_payoff_div_itm() {
        // S=110, K=100, r=5%, q=2%, T=1
        let c = call_payoff_div(110.0, 100.0, 1.0, 0.05, 0.02);
        let s_adj = 110.0 * (-0.02_f64).exp();
        let k_disc = 100.0 * (-0.05_f64).exp();
        let expected = (s_adj - k_disc).max(0.0);
        assert!((c - expected).abs() < 1e-12);
    }

    #[test]
    fn call_payoff_div_otm() {
        // S=80, K=100, r=5%, q=2%, T=1 => s_adj ~ 78.4, k_disc ~ 95.1 => OTM
        let c = call_payoff_div(80.0, 100.0, 1.0, 0.05, 0.02);
        assert_eq!(c, 0.0);
    }

    #[test]
    fn put_payoff_div_itm() {
        let p = put_payoff_div(80.0, 100.0, 1.0, 0.05, 0.02);
        let s_adj = 80.0 * (-0.02_f64).exp();
        let k_disc = 100.0 * (-0.05_f64).exp();
        let expected = (k_disc - s_adj).max(0.0);
        assert!((p - expected).abs() < 1e-12);
    }

    #[test]
    fn parity_with_dividends() {
        let s = 110.0;
        let k = 100.0;
        let t = 1.0;
        let r = 0.05;
        let q = 0.03;
        let c = call_payoff_div(s, k, t, r, q);
        let p = put_payoff_div(s, k, t, r, q);
        assert!(check_parity_div(c, p, s, k, t, r, q, 1e-12));
    }

    #[test]
    fn parity_zero_dividend_recovers_standard() {
        let s = 105.0;
        let k = 100.0;
        let t = 1.0;
        let r = 0.05;
        let c = call_payoff_div(s, k, t, r, 0.0);
        let p = put_payoff_div(s, k, t, r, 0.0);
        // C - P should equal S - K*exp(-rT) (standard parity)
        let diff = c - p;
        let rhs = s - k * (-r * t).exp();
        assert!((diff - rhs).abs() < 1e-12);
    }

    #[test]
    fn dividend_forward_at_zero_time() {
        // At T=0, dividend_forward = S - K regardless of r, q
        let fwd = dividend_forward(110.0, 100.0, 0.0, 0.05, 0.03);
        assert!((fwd - 10.0).abs() < 1e-12);
    }

    #[test]
    fn payoffs_nonneg() {
        let c = call_payoff_div(50.0, 200.0, 2.0, 0.1, 0.05);
        let p = put_payoff_div(200.0, 50.0, 2.0, 0.1, 0.05);
        assert!(c >= 0.0);
        assert!(p >= 0.0);
    }
}
