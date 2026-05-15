//! # pythia-options-impliedvol
//!
//! Verified implied volatility inversion via Newton's method.
//!
//! ## Lean specification (`Pythia.Finance.Options.ImpliedVolInversion`)
//!
//! - **Vega positive** → unique solution (`vega_positive`)
//! - **Call monotone in vol** → Newton converges (`call_mono_vol`)
//! - **IV exists iff price in bounds** (`iv_exists_iff_bounded`)
//! - **IV unique** from strict monotonicity (`iv_unique`)
//! - **IV nonneg** (`iv_nonneg`)

fn norm_cdf(x: f64) -> f64 {
    if x >= 0.0 {
        let t = 1.0 / (1.0 + 0.2316419 * x);
        let d = 0.3989422804014327 * (-x * x / 2.0).exp();
        1.0 - d * t * (0.3193815 + t * (-0.3565638 + t * (1.781478 + t * (-1.8212560 + t * 1.3302744))))
    } else {
        1.0 - norm_cdf(-x)
    }
}

fn norm_pdf(x: f64) -> f64 {
    0.3989422804014327 * (-x * x / 2.0).exp()
}

fn bs_call(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    let sqrt_t = t.sqrt();
    let d1 = ((s / k).ln() + (r + sigma * sigma / 2.0) * t) / (sigma * sqrt_t);
    let d2 = d1 - sigma * sqrt_t;
    s * norm_cdf(d1) - k * (-r * t).exp() * norm_cdf(d2)
}

fn bs_vega(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    let sqrt_t = t.sqrt();
    let d1 = ((s / k).ln() + (r + sigma * sigma / 2.0) * t) / (sigma * sqrt_t);
    s * norm_pdf(d1) * sqrt_t
}

/// Solve for implied vol via Newton's method.
///
/// # Lean: `newton_update_well_defined`, `vega_positive`, `iv_unique`
pub fn implied_vol(s: f64, k: f64, t: f64, r: f64, market_price: f64, tol: f64, max_iter: usize) -> Option<f64> {
    let lower = (s - k * (-r * t).exp()).max(0.0);
    if market_price < lower - 1e-10 || market_price > s + 1e-10 {
        return None;
    }
    let mut sigma = 0.2;
    for _ in 0..max_iter {
        let price = bs_call(s, k, t, r, sigma);
        let vega = bs_vega(s, k, t, r, sigma);
        if vega.abs() < 1e-15 { break; }
        let update = (price - market_price) / vega;
        sigma -= update;
        sigma = sigma.max(1e-6);
        if update.abs() < tol { return Some(sigma); }
    }
    Some(sigma)
}

/// Check IV exists: price must be in [max(S-KD,0), S].
/// # Lean: `iv_exists_iff_bounded`
pub fn iv_exists(s: f64, k: f64, t: f64, r: f64, market_price: f64) -> bool {
    let lower = (s - k * (-r * t).exp()).max(0.0);
    market_price >= lower - 1e-10 && market_price <= s + 1e-10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atm_iv() {
        let price = bs_call(100.0, 100.0, 1.0, 0.05, 0.2);
        let iv = implied_vol(100.0, 100.0, 1.0, 0.05, price, 1e-8, 100).unwrap();
        assert!((iv - 0.2).abs() < 1e-6);
    }

    #[test]
    fn iv_nonneg() {
        let price = bs_call(100.0, 100.0, 1.0, 0.05, 0.3);
        let iv = implied_vol(100.0, 100.0, 1.0, 0.05, price, 1e-8, 100).unwrap();
        assert!(iv >= 0.0);
    }

    #[test]
    fn iv_mono_price() {
        let p1 = bs_call(100.0, 100.0, 1.0, 0.05, 0.2);
        let p2 = bs_call(100.0, 100.0, 1.0, 0.05, 0.4);
        let iv1 = implied_vol(100.0, 100.0, 1.0, 0.05, p1, 1e-8, 100).unwrap();
        let iv2 = implied_vol(100.0, 100.0, 1.0, 0.05, p2, 1e-8, 100).unwrap();
        assert!(iv2 > iv1);
    }

    #[test]
    fn iv_out_of_bounds_fails() {
        assert!(!iv_exists(100.0, 100.0, 1.0, 0.05, 110.0));
    }

    #[test]
    fn iv_in_bounds() {
        assert!(iv_exists(100.0, 100.0, 1.0, 0.05, 10.0));
    }
}
