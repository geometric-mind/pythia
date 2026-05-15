/-
Copyright (c) 2026 Pythia contributors. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.

# Duality Gap and Complementary Slackness

Formalizes the scalar duality gap for convex optimization programs
and its fundamental algebraic properties.

The duality gap between a primal objective value and a dual bound is:

    gap(primal, dual) = primal - dual

Strong duality asserts that gap = 0 at optimality; weak duality asserts
gap >= 0 whenever the primal value is a feasible objective and the dual
value is a lower bound via the dual problem.

## Definition

* `dualityGap primal dual` : `primal - dual`

## Main results

* `dualityGap_nonneg`         : weak duality — `primal >= dual → gap >= 0`
* `dualityGap_zero_iff`       : strong duality — `gap = 0 ↔ primal = dual`
* `dualityGap_mono_primal`    : gap is monotone (non-decreasing) in the primal value
* `dualityGap_antitone_dual`  : gap is antitone (non-increasing) in the dual value
* `dualityGap_add`            : additivity of the gap over independent primal/dual pairs
* `dualityGap_scale`          : positive homogeneity: `gap(c*p, c*d) = c * gap(p, d)` for `c >= 0`
* `dualityGap_triangle`       : triangle-style decomposition through an intermediate bound

## References

* Boyd, S. and Vandenberghe, L. "Convex Optimization." Cambridge University
  Press (2004), Chapter 5.
-/
import Mathlib
import Pythia.Tactic.Pythia

namespace Pythia.Optimization

/-! ### Definition: duality gap -/

/-- The duality gap between a primal objective value and a dual bound.

    dualityGap primal dual = primal - dual

At optimality under strong duality the gap is zero; weak duality
guarantees the gap is nonneg whenever `primal >= dual`. -/
def dualityGap (primal dual : ℝ) : ℝ := primal - dual

/-! ### Lemma 1: weak duality — gap is nonneg -/

/-- **Weak duality.**
When `primal >= dual` (i.e., the dual bound is a valid lower bound on
the primal objective), the duality gap is nonneg.

Proof: `dualityGap p d = p - d`, and `p - d >= 0` follows from
`sub_nonneg.mpr`. -/
@[stat_lemma]
theorem dualityGap_nonneg {primal dual : ℝ} (h : dual ≤ primal) :
    0 ≤ dualityGap primal dual := by
  unfold dualityGap
  exact sub_nonneg.mpr h

/-! ### Lemma 2: strong duality — gap = 0 iff primal = dual -/

/-- **Strong duality characterization.**
The duality gap is zero if and only if the primal and dual values coincide.
This is the algebraic certificate of strong duality.

Proof: `p - d = 0 ↔ p = d` by `sub_eq_zero`. -/
@[stat_lemma]
theorem dualityGap_zero_iff {primal dual : ℝ} :
    dualityGap primal dual = 0 ↔ primal = dual := by
  unfold dualityGap
  exact sub_eq_zero

/-! ### Lemma 3: monotonicity in the primal -/

/-- **Monotonicity in the primal.**
If `p1 <= p2`, then `dualityGap p1 d <= dualityGap p2 d`.
A larger primal objective produces a larger duality gap against the
same dual bound.

Proof: `p1 - d <= p2 - d` by `sub_le_sub_right`. -/
@[stat_lemma]
theorem dualityGap_mono_primal {p1 p2 d : ℝ} (h : p1 ≤ p2) :
    dualityGap p1 d ≤ dualityGap p2 d := by
  unfold dualityGap
  exact sub_le_sub_right h d

/-! ### Lemma 4: antitone in the dual -/

/-- **Antitone in the dual.**
If `d1 <= d2`, then `dualityGap p d2 <= dualityGap p d1`.
A stronger (larger) dual bound shrinks the duality gap.

Proof: `p - d2 <= p - d1` by `sub_le_sub_left`. -/
@[stat_lemma]
theorem dualityGap_antitone_dual {p d1 d2 : ℝ} (h : d1 ≤ d2) :
    dualityGap p d2 ≤ dualityGap p d1 := by
  unfold dualityGap
  exact sub_le_sub_left h p

/-! ### Lemma 5: additivity -/

/-- **Additivity.**
The duality gap is additive over independent primal-dual pairs:

    dualityGap (p1 + p2) (d1 + d2) = dualityGap p1 d1 + dualityGap p2 d2.

This means two independent optimization problems can have their gaps
summed without cross-terms. Proof: ring. -/
@[stat_lemma]
theorem dualityGap_add (p1 p2 d1 d2 : ℝ) :
    dualityGap (p1 + p2) (d1 + d2) = dualityGap p1 d1 + dualityGap p2 d2 := by
  unfold dualityGap
  ring

/-! ### Lemma 6: positive homogeneity (scaling) -/

/-- **Positive homogeneity.**
For any `c >= 0`, scaling both the primal and the dual by `c` scales
the duality gap by `c`:

    dualityGap (c * p) (c * d) = c * dualityGap p d.

This reflects the fact that the gap is a linear functional in the
primal and dual values. Proof: ring. -/
@[stat_lemma]
theorem dualityGap_scale {c : ℝ} (_hc : 0 ≤ c) (p d : ℝ) :
    dualityGap (c * p) (c * d) = c * dualityGap p d := by
  unfold dualityGap
  ring

/-! ### Lemma 7: triangle decomposition through an intermediate value -/

/-- **Triangle decomposition.**
For any intermediate value `m`, the duality gap from `p` to `d` is
bounded above by the sum of the gap from `p` to `m` and the gap from
`m` to `d`:

    dualityGap p d <= dualityGap p m + dualityGap m d.

In fact equality holds identically (since `(p - d) = (p - m) + (m - d)`),
so the bound is tight. The inequality form is the natural statement for
use in cascade proofs. Proof: ring (the difference is zero). -/
@[stat_lemma]
theorem dualityGap_triangle (p d m : ℝ) :
    dualityGap p d ≤ dualityGap p m + dualityGap m d := by
  unfold dualityGap
  linarith

end Pythia.Optimization
