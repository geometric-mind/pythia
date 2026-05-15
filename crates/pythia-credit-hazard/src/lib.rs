//! # pythia-credit-hazard
//!
//! Verified hazard rate (default intensity) model.
//!
//! ## Lean specification (`Pythia.Finance.Credit.HazardRate`)
//!
//! - **Survival positive**: S(T) = exp(-hT) > 0 (`survivalProb_pos`)
//! - **Survival at zero = 1** (`survivalProb_at_zero`)
//! - **Survival antitone in time** (`survivalProb_antitone`)
//! - **Survival ≤ 1** (`survivalProb_le_one`)
//! - **Default prob in [0,1]** (`defaultProb_nonneg`, `defaultProb_le_one`)
//! - **Higher hazard → higher default** (`defaultProb_mono_hazard`)

/// Constant hazard rate model.
#[derive(Debug, Clone, Copy)]
pub struct HazardModel {
    pub hazard_rate: f64,
}

impl HazardModel {
    pub fn new(hazard_rate: f64) -> Self {
        assert!(hazard_rate >= 0.0);
        Self { hazard_rate }
    }

    /// Survival probability: exp(-h*T).
    ///
    /// # Lean: `survivalProb_pos`, `survivalProb_at_zero`, `survivalProb_antitone`, `survivalProb_le_one`
    #[inline(always)]
    pub fn survival(&self, t: f64) -> f64 {
        (-self.hazard_rate * t).exp()
    }

    /// Default probability: 1 - exp(-h*T).
    ///
    /// # Lean: `defaultProb_nonneg`, `defaultProb_le_one`
    #[inline(always)]
    pub fn default_prob(&self, t: f64) -> f64 {
        1.0 - self.survival(t)
    }

    /// CDS spread approximation: h*(1-R).
    ///
    /// # Lean: `cds_spread_nonneg`
    #[inline(always)]
    pub fn cds_spread(&self, recovery: f64) -> f64 {
        self.hazard_rate * (1.0 - recovery)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn survival_positive() {
        let m = HazardModel::new(0.05);
        assert!(m.survival(10.0) > 0.0);
    }

    #[test]
    fn survival_at_zero_is_one() {
        let m = HazardModel::new(0.05);
        assert!((m.survival(0.0) - 1.0).abs() < 1e-15);
    }

    #[test]
    fn survival_antitone() {
        let m = HazardModel::new(0.05);
        assert!(m.survival(5.0) >= m.survival(10.0));
    }

    #[test]
    fn survival_at_most_one() {
        let m = HazardModel::new(0.05);
        assert!(m.survival(1.0) <= 1.0);
    }

    #[test]
    fn default_in_01() {
        let m = HazardModel::new(0.05);
        let p = m.default_prob(5.0);
        assert!(p >= 0.0 && p <= 1.0);
    }

    #[test]
    fn higher_hazard_more_default() {
        let low = HazardModel::new(0.01);
        let high = HazardModel::new(0.10);
        assert!(low.default_prob(5.0) <= high.default_prob(5.0));
    }
}
