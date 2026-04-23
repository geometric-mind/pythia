/-
Kairos.Stats.PhiTransform — the exponential betting-transform from a
self-normalized CS to a betting-form wealth process.

Goal: prove that the Howard-Ramdas family admits a betting-CS
representative Φ(HR) with:
  (a) Φ(HR) is a non-negative supermartingale (so Ville applies);
  (b) The stopping event `{∃ t, Φ(HR)_t ≥ 1/α}` coincides with
      `{∃ t, M_t ≥ c_HR(t)}` up to leading-order equivalence at the
      optimised tilt `λ* = λ*(α, b)`;
  (c) Under log-wealth quantization at scale `s`, the deployment
      slack of Φ(HR) is O(1 / √(b log 2 + 1)) — the betting rate,
      vanishing in `b`.

This is the HR-case of the universal Φ-transform conjecture: under
quantized deployment, every admissible CS family is equivalent to a
betting-form CS modulo a lossless log-domain transform. The
continuous-arithmetic equivalence due to Ramdas--Ruf 2022 then
becomes the `s → ∞` limit of a finite-precision equivalence.

Narrow first-attempt statement: proved only for the HR family at the
optimised tilt `λ*`.  Vector and aCS analogues live in follow-up
modules once HR closes.

Axiom-audit target: {propext, Classical.choice, Quot.sound}.
-/

import Mathlib
import Kairos.Stats.Basic
import Kairos.Stats.Quantization
import Kairos.Stats.SubGaussianMG
import Kairos.Stats.HowardRamdasCS

namespace Kairos.Stats

open MeasureTheory ProbabilityTheory

/-! ## The betting-transform -/

/-- Optimised tilt `λ*(α, T)` that maps the HR boundary
`c_HR(t) = σ √(2 t log(T(T+1)/α))` into a constant log-wealth
threshold `log(1/α)` at horizon `t = T`.

Derivation: the Chernoff-optimised `λ` is `λ* = c / (σ² T)` where
`c = σ √(2 T log(T(T+1)/α))`, giving `λ* = √(2 log(T(T+1)/α) / (σ² T))`.
At this tilt, `λ* c - λ*² σ² T / 2 = log(T(T+1)/α) · (...) → log(1/α)`
at leading order. -/
noncomputable def phiTilt (sigma : ℝ) (alpha : ℝ) (T : ℕ) : ℝ :=
  if T = 0 then 0 else
    Real.sqrt (2 * Real.log ((T : ℝ) * (T + 1) / alpha) / (sigma^2 * T))

/-- The Φ-transform applied to an HR-family sub-Gaussian martingale.
`Φ(M)_t := exp (λ* · M_t - λ*² · σ² · t / 2)`. -/
noncomputable def phiProcess
    (sigma : ℝ) (alpha : ℝ) (T : ℕ)
    {Ω : Type*} (M : ℕ → Ω → ℝ) :
    ℕ → Ω → ℝ :=
  fun t ω =>
    let lam := phiTilt sigma alpha T
    Real.exp (lam * M t ω - lam^2 * sigma^2 * t / 2)

/-! ## Target theorem: HR-case of the Φ-transform conjecture -/

/-- **Φ-transform preserves admissibility (HR-case).**

Given a sub-Gaussian martingale `M` with parameter `σ`, the
`Φ`-transformed process at horizon `T = 2^b` is a non-negative
supermartingale whose first-exit event from `[0, 1/α)` agrees with
the HR stopping event, up to a leading-order shift of `O(1/√T)` in
the threshold.

Formally: for every `σ > 0`, `α ∈ (0, 1)`, `b ≥ 2`, and every
`SubGaussianMG σ 𝓕 μ` with `M.process 0 =ᵐ[μ] 0`, the event
`{ω | ∃ t ≤ 2^b, phiProcess σ α (2^b) M.process t ω ≥ 1/α}` has
measure at most `α` (inherits the HR admissibility at the same
coverage level).

The proof uses `ville_supermartingale` on `phiProcess` (which is a
non-negative supermartingale by `exp_process_is_supermartingale` at
`λ = phiTilt σ α T`). Threshold `1/α` is the wealth-level analogue
of the HR boundary at the peak time. -/
theorem phi_transform_hr_admissible
    (sigma : ℝ) (hσ : 0 < sigma)
    (alpha : ℝ) (halpha : 0 < alpha ∧ alpha < 1)
    (b : ℕ) (hb : 2 ≤ b)
    {Ω : Type*} {mΩ : MeasurableSpace Ω} [StandardBorelSpace Ω]
    {𝓕 : Filtration ℕ mΩ} {μ : Measure Ω} [IsProbabilityMeasure μ]
    (M : SubGaussianMG sigma 𝓕 μ)
    (hM0 : M.process 0 =ᵐ[μ] 0) :
    μ {ω | ∃ t ≤ 2^b,
            phiProcess sigma alpha (2^b) M.process t ω
              ≥ 1 / alpha}
      ≤ ENNReal.ofReal alpha := by
  sorry

/-- **Φ-transform has vanishing quantization slack (HR-case).**

Under log-wealth quantization at fractional scale `s ≥ 1`, the
deployment-slack of the Φ-transformed HR rule vanishes at the
betting rate `etaBetting(b) = 1 / √(b log 2 + 1)`, not the HR rate
`etaHR(b) = √(b log 2)`.

Formally: the realised coverage of the quantized-Φ-transformed rule
differs from the real-arithmetic coverage by at most
`etaBetting(b) · 2^{-s} · σ + O(2^{-2s})`.

This is the core claim of the universal Φ-conjecture at the HR
case: `Φ(HR)` inherits the betting family's quantization-robust rate
rather than HR's growing rate. -/
theorem phi_transform_hr_vanishing_slack
    (sigma : ℝ) (hσ : 0 < sigma)
    (alpha : ℝ) (halpha : 0 < alpha ∧ alpha < 1)
    (b : ℕ) (hb : 2 ≤ b) (s : ℕ) (hs : 1 ≤ s)
    {Ω : Type*} {mΩ : MeasurableSpace Ω} [StandardBorelSpace Ω]
    {𝓕 : Filtration ℕ mΩ} {μ : Measure Ω} [IsProbabilityMeasure μ]
    (M : SubGaussianMG sigma 𝓕 μ)
    (hM0 : M.process 0 =ᵐ[μ] 0) :
    ∃ Δ : ℝ,
      0 ≤ Δ ∧
      Δ ≤ etaBetting b * (2 : ℝ)^(-(s : ℤ)) * sigma
          + 2 * ((2 : ℝ)^(-(s : ℤ)))^2 ∧
      |(μ {ω | ∃ t ≤ 2^b,
              quantizeReal s
                (Real.log (phiProcess sigma alpha (2^b) M.process t ω))
                ≥ Real.log (1 / alpha)}).toReal
         - (μ {ω | ∃ t ≤ 2^b,
              phiProcess sigma alpha (2^b) M.process t ω
                ≥ 1 / alpha}).toReal| ≤ Δ := by
  sorry

end Kairos.Stats
