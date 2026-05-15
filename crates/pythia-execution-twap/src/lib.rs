//! # pythia-execution-twap
//!
//! TWAP execution scheduling with participation-rate bounds and adaptive slicing.
//!
//! ## Lean specification (`Pythia.Finance.Execution.TWAPSchedule`)
//!
//! - **participation_rate_bounded**: `Q/(T*V) <= limit`
//! - **longer_horizon_lower_rate**: longer T implies lower participation rate
//! - **schedule_completes**: `n` slices of `Q/n = Q`
//! - **shortfall_nonneg**: incomplete execution shortfall >= 0
//! - **adaptive_slice**: `remaining / time_remaining >= 0`

/// A TWAP (Time-Weighted Average Price) execution schedule.
///
/// Divides a parent order Q into n equal slices over horizon T,
/// respecting a maximum participation-rate limit relative to market volume V.
#[derive(Debug, Clone)]
pub struct TWAPSchedule {
    /// Total order quantity.
    pub total_qty: f64,
    /// Number of time slices.
    pub n_slices: usize,
    /// Horizon in time units.
    pub horizon: f64,
    /// Expected market volume over the horizon.
    pub market_volume: f64,
    /// Maximum allowed participation rate.
    pub rate_limit: f64,
}

impl TWAPSchedule {
    /// Create a new TWAP schedule.
    ///
    /// # Panics
    /// Panics if `n_slices == 0`, `horizon <= 0`, or `market_volume <= 0`.
    pub fn new(total_qty: f64, n_slices: usize, horizon: f64, market_volume: f64, rate_limit: f64) -> Self {
        assert!(n_slices > 0, "n_slices must be positive");
        assert!(horizon > 0.0, "horizon must be positive");
        assert!(market_volume > 0.0, "market_volume must be positive");
        assert!(rate_limit > 0.0, "rate_limit must be positive");
        Self { total_qty, n_slices, horizon, market_volume, rate_limit }
    }

    /// Participation rate: Q / (T * V).
    ///
    /// # Lean: `participation_rate_bounded`
    /// The theorem states Q/(T*V) <= limit.
    pub fn participation_rate(&self) -> f64 {
        self.total_qty.abs() / (self.horizon * self.market_volume)
    }

    /// Whether the participation rate is within the configured limit.
    ///
    /// # Lean: `participation_rate_bounded`
    pub fn is_within_limit(&self) -> bool {
        self.participation_rate() <= self.rate_limit + 1e-15
    }

    /// Compute the remaining slice quantity given already-executed quantity
    /// and remaining time slices.
    ///
    /// # Lean: `adaptive_slice`
    /// The theorem proves remaining / time_remaining >= 0 when both are non-negative.
    pub fn remaining_slice(&self, executed_qty: f64, slices_done: usize) -> f64 {
        let remaining_qty = self.total_qty - executed_qty;
        let slices_left = self.n_slices.saturating_sub(slices_done);
        if slices_left == 0 {
            0.0
        } else {
            remaining_qty / slices_left as f64
        }
    }

    /// Implementation shortfall for incomplete execution.
    /// Shortfall = target_qty - executed_qty (clamped to non-negative).
    ///
    /// # Lean: `shortfall_nonneg`
    /// Proves that shortfall of incomplete execution is >= 0.
    pub fn shortfall(&self, executed_qty: f64) -> f64 {
        (self.total_qty - executed_qty).max(0.0)
    }

    /// Participation rate for a given horizon, holding Q and V fixed.
    /// Used to demonstrate `longer_horizon_lower_rate`.
    ///
    /// # Lean: `longer_horizon_lower_rate`
    /// Longer T implies Q/(T*V) is smaller.
    pub fn rate_at_horizon(&self, t: f64) -> f64 {
        assert!(t > 0.0);
        self.total_qty.abs() / (t * self.market_volume)
    }

    /// Verify that schedule completes: n slices of Q/n sums to Q.
    ///
    /// # Lean: `schedule_completes`
    pub fn schedule_sum(&self) -> f64 {
        let per_slice = self.total_qty / self.n_slices as f64;
        per_slice * self.n_slices as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_participation_rate_bounded() {
        // Q=100, T=10, V=200 => rate = 100/(10*200) = 0.05
        let s = TWAPSchedule::new(100.0, 10, 10.0, 200.0, 0.10);
        assert!((s.participation_rate() - 0.05).abs() < 1e-10);
        assert!(s.is_within_limit());
    }

    #[test]
    fn test_longer_horizon_lower_rate() {
        let s = TWAPSchedule::new(100.0, 10, 10.0, 200.0, 0.10);
        let rate_short = s.rate_at_horizon(10.0);
        let rate_long = s.rate_at_horizon(20.0);
        assert!(rate_long < rate_short);
    }

    #[test]
    fn test_schedule_completes() {
        let s = TWAPSchedule::new(1000.0, 7, 7.0, 500.0, 0.50);
        assert!((s.schedule_sum() - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_shortfall_nonneg() {
        let s = TWAPSchedule::new(1000.0, 10, 10.0, 5000.0, 0.10);
        // Partial fill
        assert!(s.shortfall(600.0) >= 0.0);
        assert!((s.shortfall(600.0) - 400.0).abs() < 1e-10);
        // Over-fill clamped
        assert_eq!(s.shortfall(1200.0), 0.0);
    }

    #[test]
    fn test_adaptive_slice() {
        let s = TWAPSchedule::new(1000.0, 10, 10.0, 5000.0, 0.10);
        // After 3 slices executed 300
        let slice = s.remaining_slice(300.0, 3);
        // remaining = 700, slices_left = 7 => 100 per slice
        assert!((slice - 100.0).abs() < 1e-10);
        assert!(slice >= 0.0);
    }
}
