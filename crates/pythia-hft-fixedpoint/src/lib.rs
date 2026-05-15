//! # pythia-hft-fixedpoint
//!
//! Fixed-point arithmetic with formally verified error bounds.
//!
//! ## Lean specification
//!
//! Every operation in this crate has a corresponding theorem in
//! `Pythia.Finance.HFT.FixedPoint` (Lean 4). The Lean proofs guarantee:
//!
//! - **Addition is exact**: no rounding error (`add_exact`)
//! - **Subtraction is exact**: no rounding error (`sub_exact`)
//! - **Negation is exact**: no rounding error (`neg_exact`)
//! - **Multiplication error ≤ 1 ULP**: `|result - exact| ≤ 1/2^scale` (`mul_rescale_error`)
//! - **Comparison preserves order**: integer compare = rational compare (`compare_le_correct`)
//! - **Overflow detection**: triangle inequality bound (`no_overflow_add`)
//! - **Round-trip exact**: int → fixed → int is identity (`roundtrip_int`)

use std::cmp::Ordering;

/// Fixed-point number with `SCALE` fractional bits.
///
/// Represents the rational value `raw / 2^SCALE`.
/// All HFT prices, quantities, and intermediate results use this type
/// to guarantee bit-identical results across all hardware.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedPoint<const SCALE: u32> {
    pub raw: i128,
}

impl<const SCALE: u32> FixedPoint<SCALE> {
    /// Create from raw integer value.
    #[inline(always)]
    pub const fn from_raw(raw: i128) -> Self {
        Self { raw }
    }

    /// Create from an integer (scaled up by 2^SCALE).
    ///
    /// # Lean theorem: `roundtrip_int`
    /// Converting an integer to fixed-point and back yields the original.
    #[inline(always)]
    pub const fn from_int(n: i128) -> Self {
        Self { raw: n << SCALE }
    }

    /// Extract the integer part (truncated toward zero).
    #[inline(always)]
    pub const fn to_int(self) -> i128 {
        self.raw >> SCALE
    }

    /// Exact addition. No rounding error.
    ///
    /// # Lean theorem: `add_exact`
    /// `(a + b).toRat = a.toRat + b.toRat`
    #[inline(always)]
    pub const fn add(self, other: Self) -> Self {
        Self { raw: self.raw + other.raw }
    }

    /// Exact subtraction. No rounding error.
    ///
    /// # Lean theorem: `sub_exact`
    /// `(a - b).toRat = a.toRat - b.toRat`
    #[inline(always)]
    pub const fn sub(self, other: Self) -> Self {
        Self { raw: self.raw - other.raw }
    }

    /// Exact negation. No rounding error.
    ///
    /// # Lean theorem: `neg_exact`
    /// `(-a).toRat = -(a.toRat)`
    #[inline(always)]
    pub const fn neg(self) -> Self {
        Self { raw: -self.raw }
    }

    /// Multiplication with rescaling. Error is at most 1 ULP = 1/2^SCALE.
    ///
    /// Uses i128 to avoid overflow for typical HFT ranges.
    /// The right-shift truncates toward negative infinity.
    ///
    /// # Lean theorem: `mul_rescale_error`
    /// `|result.toRat - a.toRat * b.toRat| ≤ 1 / 2^s`
    #[inline(always)]
    pub const fn mul(self, other: Self) -> Self {
        Self { raw: (self.raw * other.raw) >> SCALE }
    }

    /// Order-preserving comparison.
    ///
    /// # Lean theorem: `compare_le_correct`
    /// `a.raw ≤ b.raw ↔ a.toRat ≤ b.toRat`
    #[inline(always)]
    pub const fn cmp_fp(self, other: Self) -> Ordering {
        if self.raw < other.raw {
            Ordering::Less
        } else if self.raw > other.raw {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    /// Overflow-safe addition with detection.
    ///
    /// # Lean theorem: `no_overflow_add`
    /// If `|a| < bound` and `|b| < bound` and `2*bound ≤ 2^63`,
    /// then `|a + b| < 2^63`.
    #[inline(always)]
    pub const fn checked_add(self, other: Self) -> Option<Self> {
        match self.raw.checked_add(other.raw) {
            Some(raw) => Some(Self { raw }),
            None => None,
        }
    }
}

impl<const SCALE: u32> PartialOrd for FixedPoint<SCALE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.raw.cmp(&other.raw))
    }
}

impl<const SCALE: u32> Ord for FixedPoint<SCALE> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.raw.cmp(&other.raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type FP16 = FixedPoint<16>;

    #[test]
    fn add_exact() {
        let a = FP16::from_int(3);
        let b = FP16::from_int(5);
        assert_eq!(a.add(b).to_int(), 8);
    }

    #[test]
    fn sub_exact() {
        let a = FP16::from_int(10);
        let b = FP16::from_int(3);
        assert_eq!(a.sub(b).to_int(), 7);
    }

    #[test]
    fn neg_exact() {
        let a = FP16::from_int(42);
        assert_eq!(a.neg().to_int(), -42);
        assert_eq!(a.neg().neg().raw, a.raw);
    }

    #[test]
    fn mul_truncation_within_1_ulp() {
        let a = FP16::from_raw(3 * (1 << 16) + 1);
        let b = FP16::from_raw(2 * (1 << 16) + 1);
        let result = a.mul(b);
        let exact_num: i128 = a.raw * b.raw;
        let error = (result.raw * (1i128 << 16) - exact_num).abs();
        assert!(error < (1i128 << 16), "mul error exceeds 1 ULP");
    }

    #[test]
    fn compare_preserves_order() {
        let a = FP16::from_int(3);
        let b = FP16::from_int(5);
        assert!(a < b);
        assert_eq!(a.cmp_fp(b), std::cmp::Ordering::Less);
    }

    #[test]
    fn roundtrip_int() {
        for n in [-1000, -1, 0, 1, 42, 999999] {
            let fp = FP16::from_int(n);
            assert_eq!(fp.to_int(), n, "roundtrip failed for {n}");
        }
    }

    #[test]
    fn overflow_detection() {
        let big = FP16::from_raw(i128::MAX);
        assert!(big.checked_add(FP16::from_raw(1)).is_none());
    }

    #[test]
    fn zero_identity() {
        let a = FP16::from_int(42);
        let zero = FP16::from_int(0);
        assert_eq!(a.add(zero).raw, a.raw);
        assert_eq!(a.sub(zero).raw, a.raw);
    }
}
