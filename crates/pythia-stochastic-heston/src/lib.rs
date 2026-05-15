//! # pythia-stochastic-heston
//!
//! Verified Heston stochastic volatility model properties.
//!
//! ## Lean specification (`Pythia.Finance.Stochastic.HestonProperties`)
//!
//! - **Feller condition**: 2κθ ≥ ξ² ensures variance stays positive (`feller_condition`)
//! - **Mean reversion**: v > θ → drift negative, v < θ → drift positive (`mean_reversion_pull_down/up`)
//! - **Equilibrium**: at v = θ drift vanishes (`equilibrium_zero_drift`)
//! - **Vol-of-vol scaling**: higher ξ → more variance-of-variance (`vol_of_vol_scales`)
//! - **Reversion speed**: higher κ → faster mean reversion (`reversion_speed_mono`)

/// Heston model parameters.
#[derive(Debug, Clone, Copy)]
pub struct HestonParams {
    pub kappa: f64,
    pub theta: f64,
    pub xi: f64,
}

impl HestonParams {
    /// Check Feller condition: 2κθ ≥ ξ².
    ///
    /// # Lean: `feller_condition`
    #[inline(always)]
    pub fn feller_satisfied(&self) -> bool {
        2.0 * self.kappa * self.theta >= self.xi * self.xi
    }

    /// Variance drift: κ(θ - v).
    #[inline(always)]
    pub fn drift(&self, v: f64) -> f64 {
        self.kappa * (self.theta - v)
    }

    /// Euler step for variance: v + κ(θ-v)dt + ξ√v dW.
    ///
    /// # Lean: `hestonVarianceStep`
    #[inline(always)]
    pub fn step(&self, v: f64, dt: f64, dw: f64) -> f64 {
        v + self.kappa * (self.theta - v) * dt + self.xi * v.max(0.0).sqrt() * dw
    }

    /// At equilibrium (v = θ), drift is zero.
    ///
    /// # Lean: `equilibrium_zero_drift`
    #[inline(always)]
    pub fn is_equilibrium(&self, v: f64) -> bool {
        (v - self.theta).abs() < 1e-12
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_params() -> HestonParams {
        HestonParams { kappa: 2.0, theta: 0.04, xi: 0.3 }
    }

    #[test]
    fn feller_check() {
        let p = HestonParams { kappa: 2.0, theta: 0.04, xi: 0.3 };
        assert!(p.feller_satisfied()); // 2*2*0.04=0.16 >= 0.09
        let bad = HestonParams { kappa: 0.5, theta: 0.01, xi: 0.5 };
        assert!(!bad.feller_satisfied()); // 2*0.5*0.01=0.01 < 0.25
    }

    #[test]
    fn mean_reversion_down() {
        let p = sample_params();
        assert!(p.drift(0.10) < 0.0); // v > theta
    }

    #[test]
    fn mean_reversion_up() {
        let p = sample_params();
        assert!(p.drift(0.01) > 0.0); // v < theta
    }

    #[test]
    fn equilibrium_zero() {
        let p = sample_params();
        assert!((p.drift(p.theta)).abs() < 1e-15);
    }

    #[test]
    fn step_positive_feller() {
        let p = sample_params();
        let v = 0.04;
        let new_v = p.step(v, 0.01, 0.1);
        assert!(new_v > 0.0);
    }
}
