//! # pythia-portfolio-factor
//!
//! Factor risk model with systematic/idiosyncratic risk decomposition.
//!
//! ## Lean specification (`Pythia.Finance.Portfolio.FactorRiskModel`)
//!
//! - **systematic_risk_nonneg**: `sum(beta_i^2 * var_i) >= 0`
//! - **risk_decomposition**: `total = systematic + idiosyncratic`
//! - **idio_risk_shrinks**: diversification reduces idiosyncratic risk
//! - **market_neutral**: `sum(w_i * beta_i) = 0` constraint
//! - **tracking_error_from_mismatch**: `TE^2` from factor differences `>= 0`
//! - **risk_budget_sums**: marginal contribution to risk sums to total

/// A factor risk model for a portfolio.
///
/// Models portfolio risk as the sum of systematic (factor) risk
/// and idiosyncratic (stock-specific) risk.
#[derive(Debug, Clone)]
pub struct FactorRisk {
    /// Factor betas for each asset (one beta per factor per asset, flattened row-major).
    /// Layout: betas[i * n_factors + k] = beta of asset i to factor k.
    pub betas: Vec<f64>,
    /// Factor variances (one per factor).
    pub factor_variances: Vec<f64>,
    /// Idiosyncratic variances (one per asset).
    pub idio_variances: Vec<f64>,
    /// Portfolio weights (one per asset).
    pub weights: Vec<f64>,
    /// Number of factors.
    pub n_factors: usize,
}

impl FactorRisk {
    /// Number of assets in the model.
    pub fn n_assets(&self) -> usize {
        if self.n_factors == 0 {
            0
        } else {
            self.betas.len() / self.n_factors
        }
    }

    /// Systematic (factor) risk of the portfolio: w' B Sigma_f B' w.
    /// Equivalent to sum over factors k of: var_k * (sum_i w_i * beta_ik)^2.
    ///
    /// # Lean: `systematic_risk_nonneg`
    /// Proves this quantity is always >= 0 (sum of squares times positive variances).
    pub fn systematic_risk(&self) -> f64 {
        let n = self.n_assets();
        let mut total = 0.0;
        for k in 0..self.n_factors {
            let mut factor_exposure = 0.0;
            for i in 0..n {
                factor_exposure += self.weights[i] * self.betas[i * self.n_factors + k];
            }
            total += self.factor_variances[k] * factor_exposure * factor_exposure;
        }
        total
    }

    /// Idiosyncratic risk: sum_i w_i^2 * sigma_idio_i^2.
    fn idiosyncratic_risk(&self) -> f64 {
        let n = self.n_assets();
        let mut total = 0.0;
        for i in 0..n {
            total += self.weights[i] * self.weights[i] * self.idio_variances[i];
        }
        total
    }

    /// Total portfolio variance = systematic + idiosyncratic.
    ///
    /// # Lean: `risk_decomposition`
    /// Proves total = systematic + idiosyncratic.
    pub fn total_risk(&self) -> f64 {
        self.systematic_risk() + self.idiosyncratic_risk()
    }

    /// Tracking error squared between this portfolio and a benchmark.
    /// TE^2 = (w - w_bench)' * Cov * (w - w_bench), approximated via factor model.
    ///
    /// # Lean: `tracking_error_from_mismatch`
    /// Proves TE^2 >= 0 (quadratic form of positive-semidefinite matrix).
    pub fn tracking_error(&self, benchmark_weights: &[f64]) -> f64 {
        let n = self.n_assets();
        assert_eq!(benchmark_weights.len(), n);

        // Active weights
        let active: Vec<f64> = self.weights.iter()
            .zip(benchmark_weights.iter())
            .map(|(w, b)| w - b)
            .collect();

        // Systematic component of TE^2
        let mut te_sq = 0.0;
        for k in 0..self.n_factors {
            let mut factor_active = 0.0;
            for i in 0..n {
                factor_active += active[i] * self.betas[i * self.n_factors + k];
            }
            te_sq += self.factor_variances[k] * factor_active * factor_active;
        }

        // Idiosyncratic component of TE^2
        for i in 0..n {
            te_sq += active[i] * active[i] * self.idio_variances[i];
        }

        te_sq
    }

    /// Check if the portfolio is market-neutral: sum(w_i * beta_i_market) = 0.
    /// Uses the first factor as the market factor.
    ///
    /// # Lean: `market_neutral`
    /// The constraint sum(w_i * beta_i) = 0.
    pub fn is_market_neutral(&self, tolerance: f64) -> bool {
        let n = self.n_assets();
        if self.n_factors == 0 || n == 0 {
            return true;
        }
        let mut exposure = 0.0;
        for i in 0..n {
            exposure += self.weights[i] * self.betas[i * self.n_factors]; // factor 0 = market
        }
        exposure.abs() <= tolerance
    }

    /// Marginal contribution to risk (MCR) for each asset.
    /// MCR_i = w_i * dVar/dw_i. Sum of MCR = total variance.
    ///
    /// # Lean: `risk_budget_sums`
    /// Proves sum(MCR_i) = total portfolio variance (Euler's theorem).
    pub fn risk_budget(&self) -> Vec<f64> {
        let n = self.n_assets();
        let mut mcr = vec![0.0; n];

        // Systematic MCR: for each factor k, MCR_i += w_i * beta_ik * var_k * 2 * (sum_j w_j * beta_jk)
        // But Euler's theorem gives MCR_i = w_i * (Cov * w)_i
        // (Cov * w)_i = sum_k beta_ik * var_k * (sum_j w_j * beta_jk) + w_i * idio_var_i
        for k in 0..self.n_factors {
            let mut factor_exposure = 0.0;
            for j in 0..n {
                factor_exposure += self.weights[j] * self.betas[j * self.n_factors + k];
            }
            for i in 0..n {
                mcr[i] += self.weights[i] * self.betas[i * self.n_factors + k]
                    * self.factor_variances[k] * factor_exposure;
            }
        }

        // Idiosyncratic MCR
        for i in 0..n {
            mcr[i] += self.weights[i] * self.weights[i] * self.idio_variances[i];
        }

        mcr
    }

    /// Verify risk budget sums to total risk.
    ///
    /// # Lean: `risk_budget_sums`
    pub fn risk_budget_sums_to_total(&self) -> bool {
        let mcr_sum: f64 = self.risk_budget().iter().sum();
        (mcr_sum - self.total_risk()).abs() < 1e-10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_model() -> FactorRisk {
        // 3 assets, 2 factors
        FactorRisk {
            betas: vec![
                1.0, 0.5,  // asset 0: beta_mkt=1.0, beta_size=0.5
                0.8, -0.3, // asset 1: beta_mkt=0.8, beta_size=-0.3
                1.2, 0.1,  // asset 2: beta_mkt=1.2, beta_size=0.1
            ],
            factor_variances: vec![0.04, 0.01], // market var=4%, size var=1%
            idio_variances: vec![0.02, 0.03, 0.025],
            weights: vec![0.4, 0.3, 0.3],
            n_factors: 2,
        }
    }

    #[test]
    fn test_systematic_risk_nonneg() {
        let m = sample_model();
        assert!(m.systematic_risk() >= 0.0);
    }

    #[test]
    fn test_risk_decomposition() {
        let m = sample_model();
        let total = m.total_risk();
        let sys = m.systematic_risk();
        let idio = m.idiosyncratic_risk();
        assert!((total - (sys + idio)).abs() < 1e-12);
    }

    #[test]
    fn test_idio_risk_shrinks_with_diversification() {
        // Equal-weight portfolio of n assets vs 1 asset
        // Idio risk of 1 asset with w=1: sigma^2
        // Idio risk of n assets with w=1/n: n * (1/n)^2 * sigma^2 = sigma^2/n
        let single = FactorRisk {
            betas: vec![1.0],
            factor_variances: vec![0.04],
            idio_variances: vec![0.09],
            weights: vec![1.0],
            n_factors: 1,
        };
        let diversified = FactorRisk {
            betas: vec![1.0, 1.0, 1.0, 1.0, 1.0],
            factor_variances: vec![0.04],
            idio_variances: vec![0.09, 0.09, 0.09, 0.09, 0.09],
            weights: vec![0.2, 0.2, 0.2, 0.2, 0.2],
            n_factors: 1,
        };
        assert!(diversified.idiosyncratic_risk() < single.idiosyncratic_risk());
    }

    #[test]
    fn test_market_neutral() {
        // Construct market-neutral: long 0.5 beta=1.0, short -0.5 beta=1.0
        let m = FactorRisk {
            betas: vec![1.0, 1.0],
            factor_variances: vec![0.04],
            idio_variances: vec![0.02, 0.02],
            weights: vec![0.5, -0.5],
            n_factors: 1,
        };
        assert!(m.is_market_neutral(1e-10));
    }

    #[test]
    fn test_tracking_error_nonneg() {
        let m = sample_model();
        let bench = vec![0.33, 0.34, 0.33];
        let te = m.tracking_error(&bench);
        assert!(te >= 0.0);
    }

    #[test]
    fn test_risk_budget_sums() {
        let m = sample_model();
        assert!(m.risk_budget_sums_to_total());
    }
}
