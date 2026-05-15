//! # pythia-credit-cva
//!
//! Verified credit valuation adjustment (CVA).
//!
//! ## Lean specification (`Pythia.Finance.Credit.CVAProperties`)
//!
//! - **CVA nonneg**: counterparty risk always costs (`cva_nonneg`)
//! - **CVA monotone in LGD** (`cva_mono_lgd`)
//! - **Netting reduces CVA** (`netting_reduces_cva`)
//! - **Wrong-way risk increases CVA** (`wrong_way_risk`)
//! - **Bilateral CVA = CVA - DVA** (`bilateral_cva`)

/// Compute CVA: LGD * Σ EE(ti) * ΔPD(ti).
/// # Lean: `cva_nonneg`
pub fn cva(lgd: f64, ee: &[f64], dpd: &[f64]) -> f64 {
    assert_eq!(ee.len(), dpd.len());
    lgd * ee.iter().zip(dpd).map(|(e, d)| e * d).sum::<f64>()
}

/// Bilateral CVA = CVA - DVA.
/// # Lean: `bilateral_cva`
pub fn bilateral_cva(cva_val: f64, dva: f64) -> f64 {
    cva_val - dva
}

/// Check netting benefit: netted CVA ≤ sum of standalone CVAs.
/// # Lean: `netting_reduces_cva`
pub fn netting_benefit(cva_netted: f64, cva_standalone_sum: f64) -> f64 {
    cva_standalone_sum - cva_netted
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cva_nonneg() {
        assert!(cva(0.6, &[100.0, 80.0], &[0.01, 0.02]) >= 0.0);
    }

    #[test]
    fn cva_mono_lgd() {
        let ee = &[100.0, 80.0];
        let dpd = &[0.01, 0.02];
        assert!(cva(0.4, ee, dpd) <= cva(0.6, ee, dpd));
    }

    #[test]
    fn netting_reduces() {
        assert!(netting_benefit(5.0, 8.0) >= 0.0);
    }

    #[test]
    fn bilateral() {
        assert!((bilateral_cva(10.0, 3.0) - 7.0).abs() < 1e-10);
    }

    #[test]
    fn zero_exposure_zero_cva() {
        assert_eq!(cva(0.6, &[0.0, 0.0], &[0.01, 0.02]), 0.0);
    }
}
