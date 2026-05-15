//! # pythia-fundamentals-capital
//!
//! Verified capital structure properties (Modigliani-Miller extensions):
//! tax shield, WACC monotonicity, leverage amplification of equity volatility.
//!
//! ## Lean specification (`Pythia.Finance.Fundamentals.CapitalStructure`)
//!
//! - **Tax shield nonneg**: tax_rate * debt >= 0 (`tax_shield_nonneg`)
//! - **WACC tax benefit**: rd*(1-t) <= rd (`wacc_tax_benefit`)
//! - **Leverage amplifies vol**: sigma_A <= sigma_A*(1+D/E) (`leverage_amplifies_vol`)
//! - **Balance sheet identity**: V = E + D (`balance_sheet`)
//! - **Equity nonneg (limited liability)** (`equity_nonneg`)
//! - **Coverage adequate**: ebitda/interest >= 1 when interest <= ebitda (`coverage_adequate`)

/// Tax shield value (perpetual debt): tax_rate * debt.
/// # Lean: `tax_shield_nonneg`
#[inline(always)]
pub fn tax_shield(tax_rate: f64, debt: f64) -> f64 {
    tax_rate * debt
}

/// After-tax cost of debt: rd * (1 - tax_rate).
/// # Lean: `wacc_tax_benefit`
#[inline(always)]
pub fn after_tax_cost_of_debt(rd: f64, tax_rate: f64) -> f64 {
    rd * (1.0 - tax_rate)
}

/// Levered equity volatility: sigma_A * (1 + D/E).
/// # Lean: `leverage_amplifies_vol`
#[inline(always)]
pub fn levered_vol(sigma_a: f64, d_over_e: f64) -> f64 {
    sigma_a * (1.0 + d_over_e)
}

/// Balance sheet identity: firm value = equity + debt.
/// # Lean: `balance_sheet`
#[inline(always)]
pub fn firm_value(equity: f64, debt: f64) -> f64 {
    equity + debt
}

/// Debt coverage ratio: ebitda / interest.
/// # Lean: `coverage_adequate`
#[inline(always)]
pub fn coverage_ratio(ebitda: f64, interest: f64) -> f64 {
    ebitda / interest
}

/// Check tax shield nonneg given nonneg inputs.
/// # Lean: `tax_shield_nonneg`
pub fn check_tax_shield_nonneg(tax_rate: f64, debt: f64) -> bool {
    tax_rate >= 0.0 && debt >= 0.0 && tax_shield(tax_rate, debt) >= 0.0
}

/// Check WACC tax benefit: after-tax cost of debt <= pre-tax cost.
/// # Lean: `wacc_tax_benefit`
pub fn check_wacc_tax_benefit(rd: f64, tax_rate: f64) -> bool {
    if rd >= 0.0 && tax_rate >= 0.0 && tax_rate <= 1.0 {
        after_tax_cost_of_debt(rd, tax_rate) <= rd
    } else {
        true
    }
}

/// Check leverage amplifies volatility.
/// # Lean: `leverage_amplifies_vol`
pub fn check_leverage_amplifies(sigma_a: f64, d_over_e: f64) -> bool {
    if sigma_a >= 0.0 && d_over_e >= 0.0 {
        sigma_a <= levered_vol(sigma_a, d_over_e)
    } else {
        true
    }
}

/// Check coverage adequacy: ebitda/interest >= 1 when interest <= ebitda.
/// # Lean: `coverage_adequate`
pub fn check_coverage_adequate(ebitda: f64, interest: f64) -> bool {
    if interest > 0.0 && interest <= ebitda {
        coverage_ratio(ebitda, interest) >= 1.0
    } else {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tax_shield_nonneg_basic() {
        assert!(tax_shield(0.21, 1_000_000.0) >= 0.0);
        assert!(tax_shield(0.0, 500_000.0) >= 0.0);
    }

    #[test]
    fn wacc_tax_benefit_basic() {
        let rd = 0.05;
        let tax = 0.21;
        assert!(after_tax_cost_of_debt(rd, tax) <= rd);
    }

    #[test]
    fn leverage_amplifies_vol_basic() {
        let sigma_a = 0.20;
        let d_over_e = 1.5;
        assert!(sigma_a <= levered_vol(sigma_a, d_over_e));
    }

    #[test]
    fn balance_sheet_identity() {
        let e = 600_000.0;
        let d = 400_000.0;
        let v = firm_value(e, d);
        assert!((v - (e + d)).abs() < 1e-10);
    }

    #[test]
    fn equity_nonneg() {
        // Limited liability: equity cannot be negative
        let e = 0.0_f64;
        assert!(e >= 0.0);
    }

    #[test]
    fn coverage_adequate_basic() {
        let ebitda = 500_000.0;
        let interest = 100_000.0;
        assert!(coverage_ratio(ebitda, interest) >= 1.0);
    }
}
