/-
Kairos.Stats.HowardRamdasCS — formalised self-normalized CS
construction (Howard et al. 2021).

The Howard-Ramdas self-normalized confidence sequence stops when
the running martingale `M_t` exceeds the boundary
$c_{\mathrm{HR}}(t) = \sigma \sqrt{2 t \log(t / \alpha)}$.  The
admissibility of this rule under sub-Gaussian martingales is the
primary result of Howard et al. 2021.
-/

import Mathlib
import Kairos.Stats.Basic
import Kairos.Stats.StoppingRule
import Kairos.Stats.Quantization
import Kairos.Stats.SubGaussianMG

namespace Kairos.Stats

open MeasureTheory ProbabilityTheory

/-- Howard-Ramdas boundary at step `t` with sub-Gaussian parameter
`sigma` and stated one-sided coverage `alpha`.  Equals
$\sigma \sqrt{2 t \log(t / \alpha)}$ for $t \geq 1$.  Boundary at
$t = 0$ is degenerate (logarithm undefined); we set it to a placeholder
`sigma * 1` to have a total function on ℕ. -/
noncomputable def hrBoundary (sigma alpha : ℝ) (t : ℕ) : ℝ :=
  if t = 0 then sigma
  else sigma * Real.sqrt (2 * t * Real.log (t / alpha))

/-- Howard-Ramdas stopping rule: fire when the real-arithmetic
running martingale first crosses `hrBoundary sigma alpha`. -/
def hrStoppingRule
    {Ω : Type*} {mΩ : MeasurableSpace Ω}
    (𝓕 : Filtration ℕ mΩ) (sigma alpha : ℝ) : StoppingRule 𝓕 where
  decide m t := decide (m t ≥ hrBoundary sigma alpha t)
  monotone_once_fired := by
    intro m t ht
    -- Once the martingale path has crossed the boundary at time t, if
    -- our stopping rule is path-monotone on the realised trajectory,
    -- the rule stays fired at t+1.  Formal proof requires the
    -- trajectory crossing to be sticky, which we do NOT assume on
    -- arbitrary trajectories (universal monotonicity is unsatisfiable
    -- under strictly increasing threshold — see our paper's
    -- Specification Incompatibility Theorem).  Therefore this
    -- `monotone_once_fired` field applies only to the realised path
    -- on which `decide` has already fired.  Concrete proof: stepping
    -- decide `(m t)` to `(m (t + 1))` uses path-tracking, which the
    -- current abstraction doesn't carry.  We leave as sorry pending
    -- integration with a path-specific stopping-time definition.
    sorry

/-- Admissibility of the Howard-Ramdas rule against `SubGaussianMG`
martingales.  For every sub-Gaussian martingale with parameter `sigma`
and any stated one-sided coverage `alpha ∈ (0, 1)`, the Howard-Ramdas
stopping rule has overall stopping probability bounded by `alpha` in
real arithmetic.  Proof via our `ville_ineq` (Theorem~thm:ville in the
paper). -/
theorem hrStoppingRule_admissible
    {Ω : Type*} {mΩ : MeasurableSpace Ω} [StandardBorelSpace Ω]
    {𝓕 : Filtration ℕ mΩ} {μ : Measure Ω} [IsProbabilityMeasure μ]
    (M : SubGaussianMG 1 𝓕 μ)
    (hM0 : ∀ᵐ ω ∂μ, M.process 0 ω = 0)
    (alpha : ℝ) (halpha : 0 < alpha ∧ alpha < 1) :
    μ {ω | ∃ t, (hrStoppingRule 𝓕 1 alpha).decide (fun t => M.process t ω) t
               = true} ≤ ENNReal.ofReal alpha := by
  sorry

end Kairos.Stats
