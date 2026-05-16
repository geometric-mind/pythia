//! # pythia-credit-recovery
//!
//! Verified recovery rate and loss-given-default.
//!
//! ## Lean specification (`Pythia.Finance.Credit.RecoveryRate`)
//!
//! - **Recovery in [0,1]** (`recovery_bounded`)
//! - **LGD = 1-R, in [0,1]** (`lgd_complement`)
//! - **LGD antitone in recovery** (`lgd_antitone_recovery`)
//! - **Expected loss = PD*LGD, nonneg** (`expected_loss_nonneg`)
//! - **Expected loss ≤ PD** (`expected_loss_le_pd`)

/// LGD = 1 - R.
/// # Lean: `lgd_complement`
pub fn lgd(recovery: f64) -> f64 { 1.0 - recovery }

/// Expected loss = PD * LGD.
/// # Lean: `expected_loss_nonneg`
pub fn expected_loss(pd: f64, recovery: f64) -> f64 { pd * lgd(recovery) }

/// Check recovery in [0,1].
/// # Lean: `recovery_bounded`
pub fn is_valid_recovery(r: f64) -> bool { r >= 0.0 && r <= 1.0 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lgd_in_01() {
        let l = lgd(0.4);
        assert!(l >= 0.0 && l <= 1.0);
    }

    #[test]
    fn lgd_antitone() {
        assert!(lgd(0.6) <= lgd(0.4));
    }

    #[test]
    fn el_nonneg() {
        assert!(expected_loss(0.02, 0.4) >= 0.0);
    }

    #[test]
    fn el_le_pd() {
        let pd = 0.05;
        assert!(expected_loss(pd, 0.4) <= pd);
    }

    #[test]
    fn valid_recovery() {
        assert!(is_valid_recovery(0.4));
        assert!(!is_valid_recovery(1.5));
    }
}
