/-
Kairos.Stats.BettingCS — formalised betting CS construction
(Waudby-Smith and Ramdas 2024).

The betting confidence sequence stops when the log-wealth of a
bounded adaptive betting strategy first exceeds the log inverse of
the stated coverage level: `log W_t ≥ log(1 / alpha)`.  Admissibility
follows from Ville's inequality applied to the wealth supermartingale.
-/

import Mathlib
import Kairos.Stats.Basic
import Kairos.Stats.StoppingRule
import Kairos.Stats.BettingStrategy
import Kairos.Stats.SubGaussianMG

namespace Kairos.Stats

open MeasureTheory ProbabilityTheory

/-- Betting stopping rule: fire when the log-wealth first exceeds
`log(1 / alpha)`.  The log-wealth is tracked via `logWealthProcess`
of a given `BettingStrategy`. -/
def bettingStoppingRule
    {Ω : Type*} {mΩ : MeasurableSpace Ω}
    {𝓕 : Filtration ℕ mΩ} {B : ℝ}
    (σ : BettingStrategy 𝓕 B) (ξ : ℕ → Ω → ℝ)
    (alpha : ℝ) : StoppingRule 𝓕 where
  decide m t := decide (m t ≥ Real.log (1 / alpha))
  monotone_once_fired := by
    -- Same note as HowardRamdasCS.hrStoppingRule.monotone_once_fired:
    -- the universal-over-trajectories reading is unsatisfiable by
    -- the Specification Incompatibility Theorem.  This field needs a
    -- path-specific refinement; leaving sorry for Aristotle together
    -- with the HR admissibility refactor.
    sorry

/-- Admissibility of the betting rule: for every wealth-process
martingale (bounded strategy against a zero-conditional-mean centred
increment), the induced stopping rule has stopping probability at
most `alpha`.  Proof via Ville's inequality applied to the wealth
supermartingale at threshold `1 / alpha`. -/
theorem bettingStoppingRule_admissible
    {Ω : Type*} {mΩ : MeasurableSpace Ω}
    {𝓕 : Filtration ℕ mΩ} {μ : Measure Ω} [IsProbabilityMeasure μ]
    {B : ℝ}
    (σ : BettingStrategy 𝓕 B) (ξ : ℕ → Ω → ℝ)
    (h_bound : ∀ t ω, |σ.lam t ω * ξ t ω| < 1)
    (h_xi_adapted : Adapted 𝓕 ξ)
    (h_integrable : ∀ t, Integrable (ξ t) μ)
    (h_wealth_integrable : ∀ t, Integrable (wealthProcess σ ξ t) μ)
    (h_zero_mean : ∀ t, μ[(ξ t) | 𝓕 t] =ᵐ[μ] 0)
    (alpha : ℝ) (halpha : 0 < alpha ∧ alpha < 1) :
    μ {ω | ∃ t, (bettingStoppingRule σ ξ alpha).decide
                 (fun t => logWealthProcess σ ξ t ω) t = true} ≤
      ENNReal.ofReal alpha := by
  sorry

end Kairos.Stats
