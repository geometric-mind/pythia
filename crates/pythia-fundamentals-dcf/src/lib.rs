//! # pythia-fundamentals-dcf
//!
//! Verified discounted cash flow valuation.
//!
//! ## Lean specification (`Pythia.Finance.Fundamentals.DCFValuation`)
//!
//! - **PV positive** for positive CF (`pv_pos`)
//! - **PV antitone in rate**: higher r → lower PV (`pv_antitone_rate`)
//! - **PV antitone in time**: later CF → lower PV (`pv_antitone_time`)
//! - **PV at zero rate = CF** (`pv_at_zero_rate`)
//! - **PV additive** (`pv_additive`)

/// Present value of a cash flow: CF * exp(-r*t).
///
/// # Lean: `pvCashFlow`
#[inline(always)]
pub fn pv(cf: f64, rate: f64, time: f64) -> f64 {
    cf * (-rate * time).exp()
}

/// NPV of a stream of cash flows.
pub fn npv(cash_flows: &[(f64, f64)], rate: f64) -> f64 {
    cash_flows.iter().map(|&(cf, t)| pv(cf, rate, t)).sum()
}

/// Find IRR by bisection (rate where NPV = 0).
pub fn irr(cash_flows: &[(f64, f64)], tol: f64, max_iter: usize) -> Option<f64> {
    let mut lo = -0.5;
    let mut hi = 5.0;
    for _ in 0..max_iter {
        let mid = (lo + hi) / 2.0;
        let n = npv(cash_flows, mid);
        if n.abs() < tol { return Some(mid); }
        if n > 0.0 { lo = mid; } else { hi = mid; }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pv_positive() {
        assert!(pv(100.0, 0.05, 5.0) > 0.0);
    }

    #[test]
    fn pv_antitone_rate() {
        assert!(pv(100.0, 0.10, 5.0) <= pv(100.0, 0.05, 5.0));
    }

    #[test]
    fn pv_antitone_time() {
        assert!(pv(100.0, 0.05, 10.0) <= pv(100.0, 0.05, 5.0));
    }

    #[test]
    fn pv_zero_rate() {
        assert!((pv(100.0, 0.0, 5.0) - 100.0).abs() < 1e-10);
    }

    #[test]
    fn pv_additive() {
        let combined = pv(30.0 + 70.0, 0.05, 3.0);
        let separate = pv(30.0, 0.05, 3.0) + pv(70.0, 0.05, 3.0);
        assert!((combined - separate).abs() < 1e-10);
    }

    #[test]
    fn irr_finds_zero() {
        let cfs = vec![(-100.0, 0.0), (110.0, 1.0)];
        let r = irr(&cfs, 1e-6, 100).unwrap();
        assert!(npv(&cfs, r).abs() < 1e-4);
    }
}
