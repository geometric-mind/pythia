/-
Copyright (c) 2026 Pythia contributors. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.

# Transformation Class Specification (ATH-1346)

Lean specification shadow of the TransformationClass enum from
the kairos proposer (src/kairos/sec/closed_loop/_proposer.py).

The proposer's tool_use schema constrains transformation_class to
exactly 3 values: parameterization, structural, impl_swap. This
module proves exhaustiveness and mutual exclusivity.

## References

* kairos proposer: src/kairos/sec/closed_loop/_proposer.py
* ATH-1346: Lean spec shadow for transformation_class
-/
import Mathlib
import Pythia.Tactic.Pythia

namespace Pythia.Security.TransformationClass

/-- The 3 transformation classes the proposer can emit. -/
inductive TransformClass
  | parameterization | structural | impl_swap
  deriving DecidableEq, Repr

/-- Classify a string into a TransformClass (mirrors Python enum). -/
def classifyTransform (s : String) : Option TransformClass :=
  if s == "parameterization" then some .parameterization
  else if s == "structural" then some .structural
  else if s == "impl_swap" then some .impl_swap
  else none

/-- **Exhaustiveness.** Every TransformClass variant has a string
representation that round-trips through classifyTransform. -/
@[stat_lemma]
theorem classifyTransform_parameterization :
    classifyTransform "parameterization" = some .parameterization := by
  native_decide

@[stat_lemma]
theorem classifyTransform_structural :
    classifyTransform "structural" = some .structural := by
  native_decide

@[stat_lemma]
theorem classifyTransform_impl_swap :
    classifyTransform "impl_swap" = some .impl_swap := by
  native_decide

/-- **Precision.** An unrecognized string returns none. -/
@[stat_lemma]
theorem classifyTransform_invalid :
    classifyTransform "invalid" = none := by
  native_decide

/-- **Mutual exclusivity.** No two variants are equal. -/
@[stat_lemma]
theorem variants_distinct :
    TransformClass.parameterization ≠ TransformClass.structural ∧
    TransformClass.parameterization ≠ TransformClass.impl_swap ∧
    TransformClass.structural ≠ TransformClass.impl_swap := by
  exact ⟨by decide, by decide, by decide⟩

/-- **Finite enumeration.** There are exactly 3 variants. -/
@[stat_lemma]
theorem all_variants (t : TransformClass) :
    t = .parameterization ∨ t = .structural ∨ t = .impl_swap := by
  cases t <;> simp

end Pythia.Security.TransformationClass
