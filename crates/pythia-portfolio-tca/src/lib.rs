//! # Transaction Cost Analysis (TCA)
//!
//! Rust implementation of TCA metrics proven in
//! `Pythia.Finance.Portfolio.TransactionCostAnalysis`. Covers implementation
//! shortfall, IS decomposition, market impact, and VWAP benchmarking.

/// Implementation shortfall for a buy order.
/// IS = execution_price - decision_price. Nonneg for adverse execution.
///
/// Corresponds to Lean theorem `implementation_shortfall_nonneg`.
pub fn implementation_shortfall(decision_price: f64, execution_price: f64) -> f64 {
    execution_price - decision_price
}

/// IS decomposition into delay cost, market impact, and timing.
/// Returns (delay, impact, timing, total) where total = delay + impact + timing.
///
/// Corresponds to Lean theorem `is_decomposition`.
pub struct IsDecomposition {
    pub delay: f64,
    pub impact: f64,
    pub timing: f64,
}

impl IsDecomposition {
    pub fn new(delay: f64, impact: f64, timing: f64) -> Self {
        Self { delay, impact, timing }
    }

    /// Total implementation shortfall from decomposition.
    pub fn total(&self) -> f64 {
        self.delay + self.impact + self.timing
    }
}

/// Market impact cost as a function of impact coefficient and order size.
/// Impact grows with order size (monotone in Q for eta >= 0).
///
/// Corresponds to Lean theorem `impact_grows_with_size`.
pub fn market_impact(eta: f64, quantity: f64) -> f64 {
    eta * quantity
}

/// Relative TCA: IS as a fraction of trade value.
/// Nonneg when IS >= 0 and trade_value > 0.
///
/// Corresponds to Lean theorem `relative_tca_nonneg`.
pub fn relative_tca(is_cost: f64, trade_value: f64) -> f64 {
    assert!(trade_value > 0.0, "trade_value must be positive");
    is_cost / trade_value
}

/// VWAP slippage: execution price minus VWAP benchmark.
/// Can be positive (overpaid) or negative (underpaid).
///
/// Corresponds to Lean theorem `vwap_slippage_decompose`.
pub fn vwap_slippage(execution_price: f64, vwap: f64) -> f64 {
    execution_price - vwap
}

/// Total trading cost: commission + spread + impact + timing.
/// Nonneg when all components are nonneg.
///
/// Corresponds to Lean theorem `total_cost_nonneg`.
pub fn total_trading_cost(commission: f64, spread: f64, impact: f64, timing: f64) -> f64 {
    commission + spread + impact + timing
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implementation_shortfall_nonneg() {
        let is = implementation_shortfall(100.0, 100.5);
        assert!(is >= 0.0, "implementation_shortfall_nonneg: adverse exec");
    }

    #[test]
    fn test_is_decomposition_sums() {
        let decomp = IsDecomposition::new(0.10, 0.25, 0.05);
        let total = decomp.total();
        let expected = 0.10 + 0.25 + 0.05;
        assert!(
            (total - expected).abs() < 1e-12,
            "is_decomposition: total must equal sum of parts"
        );
    }

    #[test]
    fn test_impact_grows_with_size() {
        let eta = 0.001;
        let i1 = market_impact(eta, 1000.0);
        let i2 = market_impact(eta, 5000.0);
        assert!(i1 <= i2, "impact_grows_with_size: larger order = more impact");
    }

    #[test]
    fn test_relative_tca_nonneg() {
        let rel = relative_tca(0.5, 10000.0);
        assert!(rel >= 0.0, "relative_tca_nonneg");
    }

    #[test]
    fn test_vwap_slippage_can_be_negative() {
        let slip = vwap_slippage(99.5, 100.0);
        assert!(slip < 0.0, "vwap can be negative (favorable execution)");
    }

    #[test]
    fn test_total_cost_nonneg() {
        let tc = total_trading_cost(5.0, 0.02, 0.15, 0.03);
        assert!(tc >= 0.0, "total_cost_nonneg: all components nonneg");
    }
}
