//! # pythia-options-volsurface
//!
//! Volatility surface no-arbitrage validation for vol desks.
//!
//! ## Lean specification (`Pythia.Finance.Options.VolSurfaceConstraints`)
//!
//! - **Total variance monotone in time** — no calendar spread arb (`total_variance_mono`)
//! - **Butterfly nonneg** — call prices convex in strike (`butterfly_nonneg`)
//! - **Total variance nonneg**: σ²T ≥ 0 (`total_variance_nonneg`)
//! - **SVI minimum nonneg**: a + bσ(1-|ρ|) ≥ 0 (`svi_minimum_nonneg`)
//! - **Lee moment bound**: wing slope ≤ 2 (`lee_moment_bound`)
//! - **Durrleman condition**: local variance ≥ 0 (`durrleman_implies_no_butterfly`)

/// A point on the volatility surface.
#[derive(Debug, Clone, Copy)]
pub struct VolPoint {
    pub strike: f64,
    pub expiry: f64,
    pub implied_vol: f64,
}

impl VolPoint {
    /// Total implied variance: σ²T.
    ///
    /// # Lean: `total_variance_nonneg`
    pub fn total_variance(&self) -> f64 {
        self.implied_vol * self.implied_vol * self.expiry
    }
}

/// SVI (Stochastic Volatility Inspired) parameterization.
#[derive(Debug, Clone, Copy)]
pub struct SviParams {
    pub a: f64,
    pub b: f64,
    pub rho: f64,
    pub m: f64,
    pub sigma: f64,
}

impl SviParams {
    /// SVI total variance: w(k) = a + b*(ρ(k-m) + √((k-m)² + σ²)).
    pub fn total_variance(&self, log_moneyness: f64) -> f64 {
        let km = log_moneyness - self.m;
        self.a + self.b * (self.rho * km + (km * km + self.sigma * self.sigma).sqrt())
    }

    /// Minimum total variance: a + bσ(1 - |ρ|).
    ///
    /// # Lean: `svi_minimum_nonneg`
    pub fn minimum_variance(&self) -> f64 {
        self.a + self.b * self.sigma * (1.0 - self.rho.abs())
    }

    /// Validate SVI parameters for no-arbitrage.
    pub fn is_valid(&self) -> bool {
        self.b >= 0.0
            && self.sigma >= 0.0
            && self.rho.abs() <= 1.0
            && self.minimum_variance() >= 0.0
    }
}

/// Check butterfly constraint: C(K-dK) - 2C(K) + C(K+dK) ≥ 0.
///
/// # Lean: `butterfly_nonneg`
pub fn check_butterfly(c_low: f64, c_mid: f64, c_high: f64) -> bool {
    c_low - 2.0 * c_mid + c_high >= -1e-12
}

/// Check calendar spread constraint: w(T₂) ≥ w(T₁) for T₂ > T₁.
///
/// # Lean: `total_variance_mono`
pub fn check_calendar(total_var_early: f64, total_var_late: f64) -> bool {
    total_var_late >= total_var_early - 1e-12
}

/// Check Lee moment bound: wing slope ≤ 2.
///
/// # Lean: `lee_moment_bound`
pub fn check_lee_bound(wing_slope: f64) -> bool {
    wing_slope <= 2.0 + 1e-12
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn total_variance_nonneg() {
        let p = VolPoint { strike: 100.0, expiry: 0.5, implied_vol: 0.2 };
        assert!(p.total_variance() >= 0.0);
    }

    #[test]
    fn butterfly_valid() {
        assert!(check_butterfly(10.0, 7.0, 5.0));
        assert!(!check_butterfly(10.0, 8.0, 5.0));
    }

    #[test]
    fn calendar_valid() {
        assert!(check_calendar(0.04, 0.08));
        assert!(!check_calendar(0.08, 0.04));
    }

    #[test]
    fn svi_valid_params() {
        let svi = SviParams { a: 0.04, b: 0.1, rho: -0.3, m: 0.0, sigma: 0.2 };
        assert!(svi.is_valid());
        assert!(svi.minimum_variance() >= 0.0);
    }

    #[test]
    fn lee_bound() {
        assert!(check_lee_bound(1.5));
        assert!(!check_lee_bound(2.5));
    }
}
