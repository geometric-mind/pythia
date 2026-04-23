# Aristotle target: `wealthProcess_martingale` (Kairos-Stats T4)

## Context

Internal Kairos-Stats Lean 4 library. This theorem gives the martingale property of the betting-wealth process against a centred increment sequence under the null hypothesis. It underlies the betting-family confidence-sequence construction of Waudby-Smith and Ramdas 2024.

## Target

`Kairos/Stats/BettingStrategy.lean:95` has a sorry on:

```lean
theorem wealthProcess_martingale
    {𝓕 : Filtration ℕ mΩ} [IsFiniteMeasure μ] {B : ℝ}
    (σ : BettingStrategy 𝓕 B) (ξ : ℕ → Ω → ℝ)
    (h_bound : ∀ t ω, |σ.lam t ω * ξ t ω| < 1)
    (h_xi_adapted : Adapted 𝓕 ξ)
    (h_integrable : ∀ t, Integrable (ξ t) μ)
    (h_wealth_integrable : ∀ t, Integrable (wealthProcess σ ξ t) μ)
    (h_zero_mean : ∀ t, μ[(ξ t) | 𝓕 t] =ᵐ[μ] 0) :
    Martingale (wealthProcess σ ξ) 𝓕 μ := by
  sorry
```

## Mathematical proof outline

A `Martingale` on a `Filtration ℕ` is:
1. `Adapted 𝓕 (wealthProcess σ ξ)` — follows from `σ.adapted` + `h_xi_adapted` + measurability of pointwise multiplication and addition.
2. `∀ t, Integrable (wealthProcess σ ξ t) μ` — supplied as `h_wealth_integrable`.
3. `∀ s t, s ≤ t → μ[wealthProcess σ ξ t | 𝓕 s] =ᵐ[μ] wealthProcess σ ξ s` — the key one.

For (3), prove the one-step case `μ[wealthProcess σ ξ (t+1) | 𝓕 t] =ᵐ[μ] wealthProcess σ ξ t`:

```
wealthProcess σ ξ (t+1) ω = wealthProcess σ ξ t ω * (1 + σ.lam t ω * ξ t ω)
                          = wealthProcess σ ξ t ω + wealthProcess σ ξ t ω * σ.lam t ω * ξ t ω
```

Take conditional expectation:

```
μ[wealthProcess σ ξ (t+1) | 𝓕 t] =ᵐ[μ] μ[wealthProcess σ ξ t | 𝓕 t] + μ[wealthProcess σ ξ t * σ.lam t * ξ t | 𝓕 t]
                                   =ᵐ[μ] wealthProcess σ ξ t + wealthProcess σ ξ t * σ.lam t * μ[ξ t | 𝓕 t]      (pull-out: W_t · λ_t is 𝓕_t-measurable)
                                   =ᵐ[μ] wealthProcess σ ξ t + 0                                                  (by h_zero_mean)
                                   =ᵐ[μ] wealthProcess σ ξ t
```

Then use induction on `t - s` via the tower property to extend from the one-step case to all `s ≤ t`.

## Relevant Mathlib lemmas (pinned commit `ee3a540`)

- `MeasureTheory.Martingale` — definition in `Probability/Martingale/Basic.lean`.
- `MeasureTheory.condExp_add` — additivity of conditional expectation.
- `MeasureTheory.condExp_mul_of_stronglyMeasurable_left` — pull-out of strongly-measurable factors.
- `MeasureTheory.condExp_of_stronglyMeasurable` — `μ[f | 𝓕_t] =ᵐ[μ] f` when `f` is `𝓕_t`-strongly-measurable.
- `Filter.EventuallyEq.add`, `Filter.EventuallyEq.mul` — ae-equality under operations.
- `MeasureTheory.Filtration.le_natTower` or the tower-property analogue for extending one-step to multi-step.

For adaptedness: `Adapted.mul` (from the strategy × increment × wealth), and we already have `wealthProcess_nonneg` to deduce `0 ≤ wealthProcess σ ξ t ω` in every step.

Related prior art: our `Kairos.Stats.SubGaussianMG.martingale` theorem in `Kairos/Stats/SubGaussianMG.lean` uses a similar `martingale_nat` + `condExp_add` + `condExp_of_stronglyMeasurable` + `filter_upwards` structure. That lemma is axiom-audit clean and provides a template.

## Constraints

- **Only modify** `Kairos/Stats/BettingStrategy.lean`.
- Axiom audit must reduce to `{propext, Classical.choice, Quot.sound}`.
- Do NOT use `native_decide` on non-decidable props.
- Do NOT add new `axiom` declarations.
- You may introduce helper lemmas within the same file.
- Keep the existing proved `wealthProcess_nonneg` and the `@[simp]`-tagged `wealthProcess_zero` / `wealthProcess_succ` intact.
- If you hit a genuine Mathlib gap, leave the sorry with a comment block stating what is missing.

## Expected outcome

`wealthProcess_martingale` closed, axiom-audit clean.  Integrates with our `Kairos.Stats.SubGaussianMG` and enables the betting-family sharpness adversary construction for the deployment-slack paper.
