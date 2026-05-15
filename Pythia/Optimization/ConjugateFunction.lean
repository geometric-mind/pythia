/-
Copyright (c) 2026 Pythia contributors. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.

# Fenchel Conjugate of a Scalar Quadratic

The Fenchel conjugate (convex conjugate) of a function f : ℝ → ℝ is

    f*(y) = sup_x { y * x - f(x) }

This file formalizes the conjugate for the concrete family of scalar
quadratics `f(x) = (a/2) * x^2` and proves the associated algebraic
identities, culminating in the Fenchel-Young inequality.

## Definitions

* `quadratic a x`            : `a / 2 * x ^ 2`
* `quadraticConjugate a y`   : `y ^ 2 / (2 * a)`

## Main results

* `quadratic_nonneg`               : `a >= 0 → quadratic a x >= 0`
* `quadratic_zero_at_zero`         : `quadratic a 0 = 0`
* `quadratic_symmetric`            : `quadratic a x = quadratic a (-x)`
* `quadraticConjugate_nonneg`      : `a > 0 → quadraticConjugate a y >= 0`
* `fenchel_young_quadratic`        : `a > 0 → x * y ≤ quadratic a x + quadraticConjugate a y`
* `quadraticConjugate_zero_at_zero`: `quadraticConjugate a 0 = 0`

## Why this lemma

The Fenchel-Young inequality is the algebraic heart of duality in
convex analysis. It states that for any x, y and any convex f,

    f(x) + f*(y) >= x * y

with equality iff y is a subgradient of f at x. For the quadratic
f(x) = (a/2)*x^2, the conjugate is f*(y) = y^2/(2*a), and equality
holds iff y = a*x (i.e., x is the unconstrained minimizer of
y*x - f(x)). Surfacing this algebraic certificate in Pythia gives
the `pythia` tactic cascade a concrete closure target for
optimization duality goals that involve quadratic regularizers.

## References

* Rockafellar, R. T. "Convex Analysis." Princeton University Press
  (1970), Section 12.
-/
import Mathlib
import Pythia.Tactic.Pythia

namespace Pythia.Optimization

/-! ### Definitions -/

/-- The scalar quadratic function `f(x) = (a/2) * x^2`. -/
noncomputable def quadratic (a : ℝ) (x : ℝ) : ℝ := a / 2 * x ^ 2

/-- The Fenchel conjugate of `quadratic a`, namely `f*(y) = y^2 / (2*a)`.
Valid (as a tight sup) when `a > 0`. -/
noncomputable def quadraticConjugate (a : ℝ) (y : ℝ) : ℝ := y ^ 2 / (2 * a)

/-! ### Lemma 1: nonnegativity of the quadratic -/

/-- **Nonnegativity of the quadratic.**
When `a >= 0`, the quadratic `(a/2) * x^2` is nonneg for all `x`.

Proof: `a / 2 >= 0` by `div_nonneg`, and `x^2 >= 0` by `sq_nonneg`;
multiply by `mul_nonneg`. -/
@[stat_lemma]
theorem quadratic_nonneg {a x : ℝ} (ha : 0 ≤ a) :
    0 ≤ quadratic a x := by
  unfold quadratic
  apply mul_nonneg
  · exact div_nonneg ha (by norm_num)
  · exact sq_nonneg x

/-! ### Lemma 2: quadratic vanishes at zero -/

/-- **Zero at the origin.** `quadratic a 0 = 0` for any `a`. -/
@[stat_lemma]
theorem quadratic_zero_at_zero (a : ℝ) :
    quadratic a 0 = 0 := by
  unfold quadratic
  ring

/-! ### Lemma 3: symmetry -/

/-- **Symmetry.** The quadratic is an even function: `quadratic a x = quadratic a (-x)`.

Proof: `(-x)^2 = x^2` by ring. -/
@[stat_lemma]
theorem quadratic_symmetric (a x : ℝ) :
    quadratic a x = quadratic a (-x) := by
  unfold quadratic
  ring

/-! ### Lemma 4: nonnegativity of the conjugate -/

/-- **Nonnegativity of the conjugate.**
When `a > 0`, `quadraticConjugate a y = y^2 / (2*a) >= 0`.

Proof: `y^2 >= 0` by `sq_nonneg`; `2 * a > 0` by `mul_pos`; combine
with `div_nonneg`. -/
@[stat_lemma]
theorem quadraticConjugate_nonneg {a y : ℝ} (ha : 0 < a) :
    0 ≤ quadraticConjugate a y := by
  unfold quadraticConjugate
  apply div_nonneg (sq_nonneg y)
  exact le_of_lt (mul_pos (by norm_num) ha)

/-! ### Lemma 5: Fenchel-Young inequality -/

/-- **Fenchel-Young inequality for the scalar quadratic.**
For `a > 0` and any `x y : ℝ`,

    x * y ≤ quadratic a x + quadraticConjugate a y

i.e.,   x * y ≤ (a/2) * x^2 + y^2 / (2*a).

Proof: The gap equals `(a * x - y)^2 / (2 * a)`, which is nonneg.
Concretely, establish the gap identity by `field_simp` + `ring`,
then conclude by `div_nonneg` + `sq_nonneg`. -/
@[stat_lemma]
theorem fenchel_young_quadratic {a x y : ℝ} (ha : 0 < a) :
    x * y ≤ quadratic a x + quadraticConjugate a y := by
  unfold quadratic quadraticConjugate
  have ha2 : (0 : ℝ) < 2 * a := by linarith
  suffices h : 0 ≤ a / 2 * x ^ 2 + y ^ 2 / (2 * a) - x * y by linarith
  have key : a / 2 * x ^ 2 + y ^ 2 / (2 * a) - x * y =
      (a * x - y) ^ 2 / (2 * a) := by
    field_simp [ha.ne', two_ne_zero]
    ring
  rw [key]
  exact div_nonneg (sq_nonneg _) (le_of_lt ha2)

/-! ### Lemma 6: conjugate vanishes at zero -/

/-- **Conjugate zero at zero.** `quadraticConjugate a 0 = 0` for any `a`. -/
@[stat_lemma]
theorem quadraticConjugate_zero_at_zero (a : ℝ) :
    quadraticConjugate a 0 = 0 := by
  unfold quadraticConjugate
  ring

end Pythia.Optimization
