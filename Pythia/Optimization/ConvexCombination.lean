/-
Copyright (c) 2026 Pythia contributors. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.

# Convex Combination (algebraic identities)

A convex combination of two points `x` and `y` with parameter `t ∈ [0,1]` is

    convexComb(t, x, y) = t * x + (1 - t) * y

This is the algebraic kernel of convex sets, convex functions, and virtually
all optimization algorithms: gradient descent updates, proximal steps, and
Frank-Wolfe steps are all convex combinations.

## Main results

* `convexComb`                : `t * x + (1 - t) * y`
* `convexComb_at_zero`        : `convexComb 0 x y = y`
* `convexComb_at_one`         : `convexComb 1 x y = x`
* `convexComb_at_half`        : `convexComb (1/2) x y = (x + y) / 2`
* `convexComb_self`           : `convexComb t x x = x`
* `convexComb_comm`           : `convexComb t x y = convexComb (1-t) y x`
* `convexComb_between`        : for `0 ≤ t ≤ 1` and `x ≤ y`, `x ≤ convexComb t x y ≤ y`
* `convexComb_linear`         : `convexComb t x y = y + t * (x - y)`

## References

* Rockafellar, R. T. "Convex Analysis." Princeton (1970), Section 1.
-/
import Mathlib
import Pythia.Tactic.Pythia

namespace Pythia.Optimization

/-- Convex combination of `x` and `y` with parameter `t`. When `t ∈ [0,1]`
this interpolates between `y` (at `t = 0`) and `x` (at `t = 1`). -/
noncomputable def convexComb (t x y : ℝ) : ℝ := t * x + (1 - t) * y

/-- **Endpoint at zero.** At `t = 0` the convex combination collapses to `y`. -/
@[stat_lemma]
theorem convexComb_at_zero {x y : ℝ} : convexComb 0 x y = y := by
  unfold convexComb; ring

/-- **Endpoint at one.** At `t = 1` the convex combination collapses to `x`. -/
@[stat_lemma]
theorem convexComb_at_one {x y : ℝ} : convexComb 1 x y = x := by
  unfold convexComb; ring

/-- **Midpoint.** At `t = 1/2` the convex combination is the arithmetic mean. -/
@[stat_lemma]
theorem convexComb_at_half {x y : ℝ} : convexComb (1 / 2) x y = (x + y) / 2 := by
  unfold convexComb; ring

/-- **Self-combination.** Combining a point with itself yields the same point
regardless of the parameter. -/
@[stat_lemma]
theorem convexComb_self {t x : ℝ} : convexComb t x x = x := by
  unfold convexComb; ring

/-- **Commutativity.** Swapping the two points is the same as replacing `t`
with `1 - t`. -/
@[stat_lemma]
theorem convexComb_comm {t x y : ℝ} : convexComb t x y = convexComb (1 - t) y x := by
  unfold convexComb; ring

/-- **Betweenness.** For `t ∈ [0, 1]` and `x ≤ y`, the convex combination
lies between the two endpoints: `x ≤ convexComb t x y ≤ y`.

Proof sketch for the lower bound:
  `convexComb t x y = t*x + (1-t)*y ≥ t*x + (1-t)*x = x`
using the hypothesis `x ≤ y` scaled by the nonneg weight `1 - t`.
The upper bound follows symmetrically using `convexComb_comm`. -/
@[stat_lemma]
theorem convexComb_between {t x y : ℝ}
    (ht0 : 0 ≤ t) (ht1 : t ≤ 1) (hxy : x ≤ y) :
    x ≤ convexComb t x y ∧ convexComb t x y ≤ y := by
  unfold convexComb
  constructor
  · -- Lower bound: t*x + (1-t)*y ≥ x
    -- Equivalently: (1-t)*y ≥ (1-t)*x, i.e. (1-t)*(y-x) ≥ 0.
    have h1t : 0 ≤ 1 - t := by linarith
    have hyx : 0 ≤ y - x := by linarith
    nlinarith [mul_nonneg h1t hyx]
  · -- Upper bound: t*x + (1-t)*y ≤ y
    -- Equivalently: t*x ≤ t*y, i.e. t*(y-x) ≥ 0.
    have hyx : 0 ≤ y - x := by linarith
    nlinarith [mul_nonneg ht0 hyx]

/-- **Linear reparametrization.** The convex combination can be written as
`y` plus a fraction `t` of the directed displacement from `y` to `x`. -/
@[stat_lemma]
theorem convexComb_linear {t x y : ℝ} : convexComb t x y = y + t * (x - y) := by
  unfold convexComb; ring

end Pythia.Optimization
