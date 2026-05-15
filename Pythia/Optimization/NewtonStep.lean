/-
Copyright (c) 2026 Pythia contributors. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.

# Newton Step: Algebraic Kernel (scalar case)

For a twice-differentiable function `f : ℝ → ℝ`, the Newton step from
a point `x` is

    x_new = x - f'(x) / f''(x)

The quadratic convergence analysis of Newton's method turns on two
quantities: the Newton decrement

    lambda^2 = (f'(x))^2 / f''(x)

and the sufficient decrease condition for self-concordant functions.

This file works entirely with scalar (ℝ → ℝ) functions and treats
`f'(x)` and `f''(x)` as given parameters, avoiding any
differentiability or topology machinery.

## Definitions

* `newtonStep`      : `x - fprime / fprimeprime`
* `newtonDecrement` : `fprime^2 / fprimeprime`

## Main results

* `newtonStep_at_stationary`           : at a stationary point, the step is idle
* `newtonDecrement_nonneg`             : decrement is nonneg when `f'' > 0`
* `newtonDecrement_zero_iff_stationary`: decrement is zero iff `f' = 0` (when `f'' > 0`)
* `newtonStep_fixed_point`             : step is a fixed point iff `f'/f'' = 0`
* `newtonDecrement_scale`              : decrement is scale-invariant

## References

* Boyd, S. and Vandenberghe, L. "Convex Optimization." Cambridge
  University Press (2004), Section 9.5.
-/
import Mathlib
import Pythia.Tactic.Pythia

-- `@[stat_lemma]` on iff-conclusions triggers an aesop advisory; the
-- lemmas are still correct and usable. Suppress the noise here.
set_option aesop.warn.applyIff false

namespace Pythia.Optimization

/-! ### Definitions -/

/-- The Newton step from `x` with first derivative `fprime` and second
derivative `fprimeprime`:

    newtonStep x f' f'' = x - f'(x) / f''(x)

Marked `noncomputable` because it uses real division. -/
noncomputable def newtonStep (x fprime fprimeprime : ℝ) : ℝ :=
  x - fprime / fprimeprime

/-- The Newton decrement squared:

    lambda^2 = (f'(x))^2 / f''(x)

This is the quantity that governs quadratic convergence of Newton's
method on self-concordant functions. Marked `noncomputable` because
it uses real division. -/
noncomputable def newtonDecrement (fprime fprimeprime : ℝ) : ℝ :=
  fprime ^ 2 / fprimeprime

/-! ### Lemma 1: stationary point is a fixed point of the step -/

/-- **Stationary point is idle.**
When the gradient is zero, the Newton step returns the current point.
The correction term `0 / fpp` evaluates to zero regardless of `fpp`. -/
@[stat_lemma]
theorem newtonStep_at_stationary (x fpp : ℝ) :
    newtonStep x 0 fpp = x := by
  unfold newtonStep
  simp

/-! ### Lemma 2: Newton decrement is nonneg under positive curvature -/

/-- **Nonneg Newton decrement.**
When `fprimeprime > 0`, the Newton decrement is nonneg.
Proof: the numerator `fprime^2 >= 0` by `sq_nonneg`, the denominator
`fprimeprime > 0`, so `div_nonneg` closes the goal. -/
@[stat_lemma]
theorem newtonDecrement_nonneg {fprime fprimeprime : ℝ}
    (hfpp : 0 < fprimeprime) :
    0 ≤ newtonDecrement fprime fprimeprime := by
  unfold newtonDecrement
  exact div_nonneg (sq_nonneg _) (le_of_lt hfpp)

/-! ### Lemma 3: zero decrement iff stationary -/

/-- **Decrement zero iff stationary.**
Under positive curvature (`fprimeprime > 0`), the Newton decrement is
zero if and only if the gradient is zero.

Proof: `div_eq_zero_iff` rewrites `fprime^2 / fprimeprime = 0` to
`fprime^2 = 0 ∨ fprimeprime = 0`. The second disjunct is ruled out by
`hfpp.ne'`, and `sq_eq_zero_iff` converts `fprime^2 = 0` to
`fprime = 0`. -/
@[stat_lemma]
theorem newtonDecrement_zero_iff_stationary {fprime fprimeprime : ℝ}
    (hfpp : 0 < fprimeprime) :
    newtonDecrement fprime fprimeprime = 0 ↔ fprime = 0 := by
  unfold newtonDecrement
  rw [div_eq_zero_iff]
  constructor
  · rintro (h | h)
    · exact sq_eq_zero_iff.mp h
    · exact absurd h hfpp.ne'
  · intro h
    left
    rw [h]
    simp

/-! ### Lemma 4: fixed-point characterisation -/

/-- **Fixed-point characterisation.**
The Newton step equals `x` if and only if the correction term
`fprime / fpp` is zero.

Proof: `x - fprime/fpp = x` rewrites to `fprime/fpp = 0` via
`sub_eq_self`. -/
@[stat_lemma]
theorem newtonStep_fixed_point {x fprime fpp : ℝ} :
    newtonStep x fprime fpp = x ↔ fprime / fpp = 0 := by
  unfold newtonStep
  constructor
  · intro h
    linarith
  · intro h
    linarith

/-! ### Lemma 5: scale invariance of the Newton decrement -/

/-- **Scale invariance.**
The Newton decrement is invariant under simultaneous scaling of the
gradient by `c` and the Hessian by `c^2`:

    newtonDecrement (c * fprime) (c^2 * fprimeprime) = newtonDecrement fprime fprimeprime

This reflects the homogeneity of the Newton decrement: it is a
dimensionless quantity.

Proof: `field_simp` clears denominators (using `hc` and `hfpp`), then
`ring` closes the resulting polynomial identity. -/
@[stat_lemma]
theorem newtonDecrement_scale {fprime fprimeprime c : ℝ}
    (hc : c ≠ 0) (hfpp : fprimeprime ≠ 0) :
    newtonDecrement (c * fprime) (c ^ 2 * fprimeprime) =
      newtonDecrement fprime fprimeprime := by
  unfold newtonDecrement
  have hc2 : c ^ 2 ≠ 0 := pow_ne_zero _ hc
  field_simp [mul_ne_zero hc2 hfpp, hfpp]

end Pythia.Optimization
