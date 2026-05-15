//! # pythia-stochastic-mc
//!
//! Verified Monte Carlo pricing bounds.
//!
//! ## Lean specification (`Pythia.Finance.Stochastic.MonteCarloBounds`)
//!
//! - **SE nonneg** and **SE antitone in n** (`mc_se_nonneg`, `mc_se_antitone`)
//! - **CI width nonneg** (`ci_width_nonneg`)
//! - **Variance reduction improves SE** (`variance_reduction`)
//! - **Antithetic reduces variance** (`antithetic_reduces`)
//! - **Quadruple samples halves SE** (`quadruple_halves_se`)

/// MC standard error: σ / √n.
/// # Lean: `mc_se_nonneg`, `mc_se_antitone`
#[inline(always)]
pub fn mc_se(sigma: f64, n: usize) -> f64 {
    assert!(n > 0);
    sigma / (n as f64).sqrt()
}

/// CI width: 2 * z * SE.
/// # Lean: `ci_width_nonneg`
#[inline(always)]
pub fn ci_width(z: f64, se: f64) -> f64 {
    2.0 * z * se
}

/// Samples needed for target SE: (σ / target_se)².
pub fn samples_for_se(sigma: f64, target_se: f64) -> usize {
    assert!(target_se > 0.0);
    ((sigma / target_se).powi(2)).ceil() as usize
}

/// SE after variance reduction.
/// # Lean: `variance_reduction`
pub fn mc_se_reduced(sigma_reduced: f64, n: usize) -> f64 {
    mc_se(sigma_reduced, n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn se_nonneg() { assert!(mc_se(0.2, 1000) >= 0.0); }

    #[test]
    fn se_decreases_with_n() {
        assert!(mc_se(0.2, 10000) < mc_se(0.2, 1000));
    }

    #[test]
    fn ci_width_nonneg() { assert!(ci_width(1.96, 0.01) >= 0.0); }

    #[test]
    fn quadruple_halves() {
        let se1 = mc_se(0.2, 1000);
        let se4 = mc_se(0.2, 4000);
        assert!((se4 - se1 / 2.0).abs() < 1e-10);
    }

    #[test]
    fn variance_reduction() {
        assert!(mc_se_reduced(0.1, 1000) < mc_se(0.2, 1000));
    }

    #[test]
    fn samples_for_target() {
        let n = samples_for_se(0.2, 0.01);
        assert!(mc_se(0.2, n) <= 0.01 + 1e-6);
    }
}
