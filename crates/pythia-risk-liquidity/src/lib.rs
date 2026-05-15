//! # pythia-risk-liquidity
//!
//! Verified liquidity risk properties: bid-ask spread cost, market depth,
//! and liquidity-adjusted VaR.
//!
//! ## Lean specification (`Pythia.Finance.Risk.LiquidityRisk`)
//!
//! - **Liquidity cost nonneg**: half_spread * qty >= 0 (`liquidity_cost_nonneg`)
//! - **Liquidity cost monotone in size**: larger trades cost more (`liquidity_cost_mono`)
//! - **LVaR >= VaR**: liquidity-adjusted VaR adds liquidation cost (`lvar_ge_var`)
//! - **Wider spread = higher LVaR** (`lvar_mono_spread`)
//! - **Illiquidity discount**: illiquid asset worth <= mark-to-market (`illiquidity_discount`)

/// Liquidity cost = half_spread * qty.
/// # Lean: `liquidity_cost_nonneg`
#[inline(always)]
pub fn liquidity_cost(half_spread: f64, qty: f64) -> f64 {
    half_spread * qty
}

/// Liquidity-adjusted VaR = VaR + liquidation cost.
/// # Lean: `lvar_ge_var`
#[inline(always)]
pub fn lvar(var: f64, liq_cost: f64) -> f64 {
    var + liq_cost
}

/// Illiquidity-discounted value = mtm - liquidation cost.
/// # Lean: `illiquidity_discount`
#[inline(always)]
pub fn illiquid_value(mtm: f64, liq_cost: f64) -> f64 {
    mtm - liq_cost
}

/// Check that liquidity cost is nonneg given nonneg inputs.
/// # Lean: `liquidity_cost_nonneg`
pub fn check_cost_nonneg(half_spread: f64, qty: f64) -> bool {
    half_spread >= 0.0 && qty >= 0.0 && liquidity_cost(half_spread, qty) >= 0.0
}

/// Check that liquidity cost is monotone in quantity.
/// # Lean: `liquidity_cost_mono`
pub fn check_cost_mono(half_spread: f64, q1: f64, q2: f64) -> bool {
    if half_spread >= 0.0 && q1 <= q2 {
        liquidity_cost(half_spread, q1) <= liquidity_cost(half_spread, q2)
    } else {
        true // precondition not met, vacuously true
    }
}

/// Check LVaR >= VaR given nonneg liquidation cost.
/// # Lean: `lvar_ge_var`
pub fn check_lvar_ge_var(var: f64, liq_cost: f64) -> bool {
    if liq_cost >= 0.0 {
        var <= lvar(var, liq_cost)
    } else {
        true
    }
}

/// Check that wider spread implies higher LVaR.
/// # Lean: `lvar_mono_spread`
pub fn check_lvar_mono_spread(var: f64, qty: f64, s1: f64, s2: f64) -> bool {
    if qty >= 0.0 && s1 <= s2 {
        lvar(var, s1 * qty) <= lvar(var, s2 * qty)
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cost_nonneg_basic() {
        assert!(liquidity_cost(0.01, 100.0) >= 0.0);
        assert!(liquidity_cost(0.0, 50.0) >= 0.0);
    }

    #[test]
    fn cost_mono_basic() {
        let hs = 0.005;
        assert!(liquidity_cost(hs, 10.0) <= liquidity_cost(hs, 20.0));
    }

    #[test]
    fn lvar_ge_var_basic() {
        let var = 1_000_000.0;
        let liq = 50_000.0;
        assert!(var <= lvar(var, liq));
    }

    #[test]
    fn lvar_mono_spread_basic() {
        let var = 500_000.0;
        let qty = 1000.0;
        assert!(lvar(var, 0.01 * qty) <= lvar(var, 0.03 * qty));
    }

    #[test]
    fn illiquidity_discount_basic() {
        let mtm = 1_000_000.0;
        let liq = 25_000.0;
        assert!(illiquid_value(mtm, liq) <= mtm);
    }

    #[test]
    fn zero_spread_zero_cost() {
        assert_eq!(liquidity_cost(0.0, 999.0), 0.0);
    }
}
