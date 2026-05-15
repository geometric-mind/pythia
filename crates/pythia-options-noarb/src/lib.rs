//! # pythia-options-noarb
//!
//! No-arbitrage price bounds for European options.
//!
//! ## Lean specification (`Pythia.Finance.NoArbitrageBounds`)
//!
//! - **Call lower bound**: C >= max(S - K*exp(-rT), 0) (`call_lower_bound`)
//! - **Call upper bound**: C <= S (`call_upper_bound`)
//! - **Put from parity**: P = C - S + K_disc (`put_from_parity`)
//! - **Call spread nonneg**: 0 <= C(K1) - C(K2) for K1 < K2 (`call_spread_nonneg`)
//! - **Call spread bounded**: C(K1) - C(K2) <= K2_disc - K1_disc (`call_spread_bounded`)
//! - **Butterfly nonneg**: C1 - 2*C2 + C3 >= 0 (`butterfly_nonneg`)
//! - **Call convex in strike**: C(K2) <= lam*C(K1) + (1-lam)*C(K3) (`call_convex_in_strike`)
//! - **Calendar spread nonneg**: C_long - C_short >= 0 (`calendar_spread_nonneg`)
//! - **Law of one price**: price1 = price2 when each <= the other (`law_of_one_price`)

/// Compute the discounted strike: K * exp(-r * T).
pub fn discount(k: f64, r: f64, t: f64) -> f64 {
    k * (-r * t).exp()
}

/// Call lower bound: max(S - K*exp(-rT), 0).
///
/// # Lean: `call_lower_bound`
pub fn call_lower_bound(s: f64, k: f64, r: f64, t: f64) -> f64 {
    (s - discount(k, r, t)).max(0.0)
}

/// Check that a call price respects the lower bound: C >= max(S - K_disc, 0).
///
/// # Lean: `call_lower_bound`
pub fn check_call_lower_bound(c: f64, s: f64, k: f64, r: f64, t: f64, tol: f64) -> bool {
    c >= call_lower_bound(s, k, r, t) - tol
}

/// Check that a call price respects the upper bound: C <= S.
///
/// # Lean: `call_upper_bound`
pub fn check_call_upper_bound(c: f64, s: f64, tol: f64) -> bool {
    c <= s + tol
}

/// Derive the put price from put-call parity: P = C - S + K_disc.
///
/// # Lean: `put_from_parity`
pub fn put_from_parity(c: f64, s: f64, k: f64, r: f64, t: f64) -> f64 {
    c - s + discount(k, r, t)
}

/// Check call spread nonneg: C(K1) - C(K2) >= 0 for K1 < K2.
///
/// # Lean: `call_spread_nonneg`
pub fn check_call_spread_nonneg(c1: f64, c2: f64, tol: f64) -> bool {
    c1 - c2 >= -tol
}

/// Check call spread bounded: C(K1) - C(K2) <= (K2_disc - K1_disc).
///
/// # Lean: `call_spread_bounded`
pub fn check_call_spread_bounded(
    c1: f64,
    c2: f64,
    k1: f64,
    k2: f64,
    r: f64,
    t: f64,
    tol: f64,
) -> bool {
    let spread = c1 - c2;
    let bound = discount(k2, r, t) - discount(k1, r, t);
    spread <= bound + tol
}

/// Check butterfly nonneg: C1 - 2*C2 + C3 >= 0.
///
/// # Lean: `butterfly_nonneg`
pub fn check_butterfly_nonneg(c1: f64, c2: f64, c3: f64, tol: f64) -> bool {
    c1 - 2.0 * c2 + c3 >= -tol
}

/// Check call convexity in strike:
/// C(K2) <= lam * C(K1) + (1 - lam) * C(K3), where 0 <= lam <= 1.
///
/// # Lean: `call_convex_in_strike`
pub fn check_call_convex(c1: f64, c2: f64, c3: f64, lam: f64, tol: f64) -> bool {
    c2 <= lam * c1 + (1.0 - lam) * c3 + tol
}

/// Check calendar spread nonneg: C_long - C_short >= 0.
///
/// # Lean: `calendar_spread_nonneg`
pub fn check_calendar_spread_nonneg(c_long: f64, c_short: f64, tol: f64) -> bool {
    c_long - c_short >= -tol
}

/// Law of one price: two prices are equal if each is <= the other.
///
/// # Lean: `law_of_one_price`
pub fn check_law_of_one_price(p1: f64, p2: f64, tol: f64) -> bool {
    (p1 - p2).abs() < tol
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_lower_bound_itm() {
        // S=110, K=100, r=5%, T=1 => K_disc ~ 95.12, bound ~ 14.88
        let bound = call_lower_bound(110.0, 100.0, 0.05, 1.0);
        assert!(bound > 14.0);
        assert!(bound < 15.0);
    }

    #[test]
    fn call_lower_bound_otm() {
        // S=90, K=100, r=5%, T=1 => K_disc ~ 95.12, S - K_disc < 0 => bound = 0
        let bound = call_lower_bound(90.0, 100.0, 0.05, 1.0);
        assert_eq!(bound, 0.0);
    }

    #[test]
    fn put_from_parity_basic() {
        let c = 15.0;
        let s = 110.0;
        let k = 100.0;
        let r = 0.05;
        let t = 1.0;
        let p = put_from_parity(c, s, k, r, t);
        // P = C - S + K_disc = 15 - 110 + 95.12... ~ 0.12
        let k_disc = discount(k, r, t);
        assert!((p - (c - s + k_disc)).abs() < 1e-12);
    }

    #[test]
    fn butterfly_nonneg_with_convex_prices() {
        // Convex prices: C(90)=25, C(100)=18, C(110)=12
        // butterfly = 25 - 36 + 12 = 1 >= 0
        assert!(check_butterfly_nonneg(25.0, 18.0, 12.0, 1e-12));
    }

    #[test]
    fn butterfly_fails_on_concave_prices() {
        // Concave prices (arb!): C(90)=20, C(100)=18, C(110)=10
        // butterfly = 20 - 36 + 10 = -6 < 0
        assert!(!check_butterfly_nonneg(20.0, 18.0, 10.0, 1e-12));
    }

    #[test]
    fn law_of_one_price_equal() {
        assert!(check_law_of_one_price(100.0, 100.0, 1e-12));
    }

    #[test]
    fn law_of_one_price_unequal() {
        assert!(!check_law_of_one_price(100.0, 101.0, 0.5));
    }
}
