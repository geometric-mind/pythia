//! # pythia-options-parity
//!
//! Put-call parity identities and discounted payoff functions.
//!
//! ## Lean specification (`Pythia.Finance.PutCallParity`)
//!
//! - **Payoff identity**: max(S-K,0) - max(K-S,0) = S - K (`put_call_payoff_identity`)
//! - **Call payoff**: discounted call payoff with exp(-rT) (`callPayoff`)
//! - **Put payoff**: discounted put payoff with exp(-rT) (`putPayoff`)
//! - **Discounted parity**: call - put = (S-K)*exp(-rT) (`put_call_parity_discounted`)

/// Discounted call payoff: max(S - K, 0) * exp(-r * T).
///
/// # Lean: `callPayoff`
pub fn call_payoff(s: f64, k: f64, t: f64, r: f64) -> f64 {
    (s - k).max(0.0) * (-r * t).exp()
}

/// Discounted put payoff: max(K - S, 0) * exp(-r * T).
///
/// # Lean: `putPayoff`
pub fn put_payoff(s: f64, k: f64, t: f64, r: f64) -> f64 {
    (k - s).max(0.0) * (-r * t).exp()
}

/// Check put-call parity: call - put = (S - K) * exp(-r * T).
///
/// Returns true when the identity holds within tolerance `tol`.
///
/// # Lean: `put_call_parity_discounted`
pub fn check_parity(call: f64, put: f64, s: f64, k: f64, t: f64, r: f64, tol: f64) -> bool {
    let lhs = call - put;
    let rhs = (s - k) * (-r * t).exp();
    (lhs - rhs).abs() < tol
}

/// Raw (undiscounted) payoff identity: max(S-K,0) - max(K-S,0) = S - K.
///
/// # Lean: `put_call_payoff_identity`
pub fn payoff_identity(s: f64, k: f64) -> f64 {
    (s - k).max(0.0) - (k - s).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_payoff_itm() {
        let c = call_payoff(110.0, 100.0, 1.0, 0.05);
        let expected = 10.0 * (-0.05_f64).exp();
        assert!((c - expected).abs() < 1e-12);
    }

    #[test]
    fn call_payoff_otm() {
        let c = call_payoff(90.0, 100.0, 1.0, 0.05);
        assert_eq!(c, 0.0);
    }

    #[test]
    fn put_payoff_itm() {
        let p = put_payoff(90.0, 100.0, 1.0, 0.05);
        let expected = 10.0 * (-0.05_f64).exp();
        assert!((p - expected).abs() < 1e-12);
    }

    #[test]
    fn put_payoff_otm() {
        let p = put_payoff(110.0, 100.0, 1.0, 0.05);
        assert_eq!(p, 0.0);
    }

    #[test]
    fn parity_holds_for_payoffs() {
        let s = 105.0;
        let k = 100.0;
        let t = 1.0;
        let r = 0.05;
        let c = call_payoff(s, k, t, r);
        let p = put_payoff(s, k, t, r);
        assert!(check_parity(c, p, s, k, t, r, 1e-12));
    }

    #[test]
    fn payoff_identity_matches() {
        assert!((payoff_identity(110.0, 100.0) - 10.0).abs() < 1e-12);
        assert!((payoff_identity(90.0, 100.0) - (-10.0)).abs() < 1e-12);
        assert!((payoff_identity(100.0, 100.0)).abs() < 1e-12);
    }
}
