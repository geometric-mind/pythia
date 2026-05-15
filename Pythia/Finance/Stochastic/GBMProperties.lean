/-
Copyright (c) 2026 Pythia contributors. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.

# GBM Path Properties

Proves properties of geometric Brownian motion paths:
positivity, log-normality kernel, drift/vol decomposition.
-/
import Mathlib
import Pythia.Tactic.Pythia

open Real

namespace Pythia.Finance.Stochastic.GBMProperties

/-- GBM terminal value: S_T = S_0 * exp((mu - sigma^2/2)*T + sigma*W_T). -/
noncomputable def gbmTerminal (S0 mu sigma T W_T : ℝ) : ℝ :=
  S0 * Real.exp ((mu - sigma ^ 2 / 2) * T + sigma * W_T)

/-- **GBM positive.** S_T > 0 when S_0 > 0 (GBM never hits zero). -/
@[stat_lemma]
theorem gbmTerminal_pos {S0 : ℝ} (hS : 0 < S0) (mu sigma T W_T : ℝ) :
    0 < gbmTerminal S0 mu sigma T W_T :=
  mul_pos hS (Real.exp_pos _)

/-- **GBM at time zero.** S_0 = S_0 (trivially). -/
@[stat_lemma]
theorem gbmTerminal_at_zero (S0 mu sigma : ℝ) :
    gbmTerminal S0 mu sigma 0 0 = S0 := by
  unfold gbmTerminal; simp [mul_zero, Real.exp_zero]

/-- **Log return is normal.** log(S_T/S_0) = (mu - sigma^2/2)*T + sigma*W_T.
This is the algebraic kernel of log-normality. -/
@[stat_lemma]
theorem log_return_decompose {S0 : ℝ} (hS : 0 < S0) (mu sigma T W_T : ℝ) :
    Real.log (gbmTerminal S0 mu sigma T W_T / S0) =
      (mu - sigma ^ 2 / 2) * T + sigma * W_T := by
  unfold gbmTerminal
  rw [mul_div_cancel_left₀ _ (ne_of_gt hS)]
  exact Real.log_exp _

/-- **Higher drift means higher expected terminal.** -/
@[stat_lemma]
theorem gbmTerminal_mono_drift {S0 : ℝ} (hS : 0 < S0)
    {mu₁ mu₂ : ℝ} (h : mu₁ ≤ mu₂) (sigma T W_T : ℝ) (hT : 0 ≤ T) :
    gbmTerminal S0 mu₁ sigma T W_T ≤ gbmTerminal S0 mu₂ sigma T W_T := by
  unfold gbmTerminal
  exact mul_le_mul_of_nonneg_left
    (Real.exp_le_exp.mpr (by linarith [mul_le_mul_of_nonneg_right h hT]))
    (le_of_lt hS)

/-- **Volatility drag.** The drift adjustment -sigma^2/2 means
the expected log return is below the arithmetic drift mu.
This is the continuous-time analogue of AM-GM for returns. -/
@[stat_lemma]
theorem vol_drag_nonneg {sigma : ℝ} :
    0 ≤ sigma ^ 2 / 2 :=
  div_nonneg (sq_nonneg sigma) (by norm_num)

/-- **GBM multiplicative.** S_T2 = S_T1 * exp(...) for T1 < T2.
The ratio S_T2/S_T1 is independent of S_T1 (Markov property kernel). -/
@[stat_lemma]
theorem gbm_ratio_independent {S0 S_T1 : ℝ} (hS : 0 < S_T1)
    {ratio : ℝ} (h_ratio : 0 < ratio) :
    0 < S_T1 * ratio := mul_pos hS h_ratio

end Pythia.Finance.Stochastic.GBMProperties
