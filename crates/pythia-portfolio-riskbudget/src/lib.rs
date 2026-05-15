//! # pythia-portfolio-riskbudget
//!
//! Verified Euler risk decomposition for risk parity portfolios.
//!
//! ## Lean specification (`Pythia.Finance.Portfolio.RiskBudgetEuler`)
//!
//! - **Euler sum**: MCR sums to total risk (`euler_sum`)
//! - **Each MCR ≤ total** (`contribution_le_total`)
//! - **Equal risk contribution**: n * (total/n) = total (`equal_risk_contribution`)
//! - **Risk HHI nonneg** (`risk_hhi_nonneg`)

/// Compute risk contributions and validate Euler decomposition.
pub fn euler_check(mcr: &[f64], total: f64, tol: f64) -> bool {
    (mcr.iter().sum::<f64>() - total).abs() < tol
}

/// Each MCR bounded by total (for nonneg contributions).
/// # Lean: `contribution_le_total`
pub fn contribution_bounded(mcr: &[f64], total: f64) -> bool {
    mcr.iter().all(|&m| m <= total + 1e-12)
}

/// Equal risk contribution target: total / n.
/// # Lean: `equal_risk_contribution`
pub fn equal_risk_target(total: f64, n: usize) -> f64 {
    assert!(n > 0);
    total / n as f64
}

/// Risk HHI: Σ share_i². Measures concentration.
/// # Lean: `risk_hhi_nonneg`
pub fn risk_hhi(shares: &[f64]) -> f64 {
    shares.iter().map(|s| s * s).sum()
}

/// Check if portfolio is risk-parity (all MCRs equal within tolerance).
pub fn is_risk_parity(mcr: &[f64], tol: f64) -> bool {
    if mcr.is_empty() { return true; }
    let target = mcr.iter().sum::<f64>() / mcr.len() as f64;
    mcr.iter().all(|&m| (m - target).abs() < tol)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euler_sums() {
        assert!(euler_check(&[3.0, 4.0, 3.0], 10.0, 1e-10));
    }

    #[test]
    fn contribution_bounded_test() {
        assert!(contribution_bounded(&[3.0, 4.0, 3.0], 10.0));
    }

    #[test]
    fn equal_risk_target_test() {
        assert!((equal_risk_target(12.0, 4) - 3.0).abs() < 1e-10);
    }

    #[test]
    fn hhi_nonneg() {
        assert!(risk_hhi(&[0.3, 0.3, 0.4]) >= 0.0);
    }

    #[test]
    fn risk_parity_check() {
        assert!(is_risk_parity(&[5.0, 5.0, 5.0], 0.01));
        assert!(!is_risk_parity(&[5.0, 5.0, 10.0], 0.01));
    }

    #[test]
    fn hhi_minimum_at_equal() {
        let equal_hhi = risk_hhi(&[0.25, 0.25, 0.25, 0.25]);
        let concentrated_hhi = risk_hhi(&[0.7, 0.1, 0.1, 0.1]);
        assert!(equal_hhi <= concentrated_hhi);
    }
}
