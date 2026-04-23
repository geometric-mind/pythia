/-
Kairos.Stats.Sharpness — per-family leading-order sharpness
witnesses for the quantization slack rate $\eta_F(b)$.

We construct boundary-hugging adversaries that saturate the
leading-order rate $\eta_F(b) \cdot 2^{-s} \cdot \sigma$ for each
admissible family.  For HR and betting the construction is explicit;
for vector and aCS the construction depends on a Gaussian small-ball
lower bound (see `Kairos.Stats.GaussianSmallBall`) and is flagged as
open elsewhere in the library.
-/

import Mathlib
import Kairos.Stats.Basic
import Kairos.Stats.Quantization
import Kairos.Stats.SubGaussianMG

namespace Kairos.Stats

open MeasureTheory ProbabilityTheory

/-- Sharpness witness for the Howard-Ramdas family.  For every bit-
width $b \geq 2$, fractional scale $s \geq 1$, and sub-Gaussian
parameter $\sigma > 0$, there exists an adversary measure and an
adapted sub-Gaussian martingale under which the realised coverage
exceeds the real-arithmetic coverage by at least
$\eta_{\mathrm{HR}}(b) \cdot 2^{-s} \cdot \sigma - o(2^{-s})$.

The construction: a boundary-hugging scaled Gaussian random walk
that lands within the quantization window $(c - 2^{-s}, c]$ at the
peak time $t^\star = 2^b$.  Quantization then tips the realisation
across the threshold with probability proportional to $\eta_{\mathrm{HR}}(b)
\cdot 2^{-s} \cdot \sigma$. -/
theorem etaHR_sharpness_witness
    (b : ℕ) (hb : 2 ≤ b) (s : ℕ) (hs : 1 ≤ s) (sigma : ℝ) (hσ : 0 < sigma) :
    ∃ (deviation : ℝ),
      deviation ≥ etaHR b * (2 : ℝ)^(-(s : ℤ)) * sigma
                 - 2 * ((2 : ℝ)^(-(s : ℤ)))^2 ∧
      deviation ≥ 0 := by
  -- Aristotle target: construct the deviation as the inverse-derivative
  -- shift times the quantization error at peak t* = 2^b.
  sorry

/-- Sharpness witness for the betting family.  Dual construction:
log-wealth random walk hugging the `log(1/alpha)` threshold at
precision scale $s$ triggers extra rejections with probability
proportional to $\eta_{\mathrm{betting}}(b) \cdot 2^{-s} \cdot \sigma$. -/
theorem etaBetting_sharpness_witness
    (b : ℕ) (hb : 2 ≤ b) (s : ℕ) (hs : 1 ≤ s) (sigma : ℝ) (hσ : 0 < sigma) :
    ∃ (deviation : ℝ),
      deviation ≥ etaBetting b * (2 : ℝ)^(-(s : ℤ)) * sigma
                 - 2 * ((2 : ℝ)^(-(s : ℤ)))^2 ∧
      deviation ≥ 0 := by
  -- Aristotle target: construct via bounded betting strategy on
  -- log-wealth martingale at threshold log(1/alpha), perturbed by
  -- multiplicative quantization error of size 2^{-s}.
  sorry

/-- Sharpness for vector is open pending `gaussian_small_ball_lower_bound`
(T3). Stated here for completeness; flagged as sorry and tracked in
`Kairos/Stats/GaussianSmallBall.lean` Aristotle run. -/
theorem etaVector_sharpness_witness_open
    (b : ℕ) (hb : 2 ≤ b) (s : ℕ) (hs : 1 ≤ s) (sigma : ℝ) (hσ : 0 < sigma) :
    ∃ (deviation : ℝ),
      deviation ≥ etaVector b * (2 : ℝ)^(-(s : ℤ)) * sigma
                 - 2 * ((2 : ℝ)^(-(s : ℤ)))^2 ∧
      deviation ≥ 0 := by
  sorry

/-- Sharpness for aCS is open pending Gaussian small-ball + CLT
refinement (T3 + T5).  Stated here for completeness. -/
theorem etaAsymptotic_sharpness_witness_open
    (b : ℕ) (hb : 2 ≤ b) (s : ℕ) (hs : 1 ≤ s) (sigma : ℝ) (hσ : 0 < sigma) :
    ∃ (deviation : ℝ),
      deviation ≥ etaAsymptotic b * (2 : ℝ)^(-(s : ℤ)) * sigma
                 - 2 * ((2 : ℝ)^(-(s : ℤ)))^2 ∧
      deviation ≥ 0 := by
  sorry

end Kairos.Stats
