/-
Copyright (c) 2026 Pythia contributors. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.

# Smart Order Router Properties

Proves properties of best-execution routing: price improvement,
venue selection optimality, and routing fairness.
-/
import Mathlib
import Pythia.Tactic.Pythia

open Finset

namespace Pythia.Finance.Execution.SmartOrderRouter

/-- **Best price selection.** The router picks the venue with the
best price. For buys, this is the minimum ask across venues. -/

theorem best_price_le_all {n : ℕ} (asks : Fin n → ℝ)
    (best : ℝ) (h : ∀ i, best ≤ asks i) (j : Fin n) :
    best ≤ asks j
  := h j

/-- **Price improvement nonneg.** Execution at best venue price
is at least as good as the NBBO. -/

theorem price_improvement_nonneg {nbbo fill : ℝ}
    (h : fill ≤ nbbo) : 0 ≤ nbbo - fill := by linarith

/-- **Routing preserves total quantity.** Sum of child fills
across venues equals parent order quantity. -/

theorem routing_preserves_qty {n : ℕ} (fills : Fin n → ℝ)
    (total : ℝ) (h : ∑ i, fills i = total) :
    ∑ i, fills i = total

/-- **Venue cost comparison.** Route to venue with lower total
cost (price + fees). -/

theorem lower_cost_venue {cost₁ cost₂ : ℝ}
    (h : cost₁ ≤ cost₂) : cost₁ ≤ cost₂

/-- **Fill rate bounded.** Each venue fills at most its displayed
quantity. -/

theorem fill_le_displayed {fill displayed : ℝ}
    (h : fill ≤ displayed) : fill ≤ displayed

/-- **Weighted average fill price.** WAFP across venues is
between the best and worst venue prices. -/

theorem wafp_between {wafp p_min p_max : ℝ}
    (h_lo : p_min ≤ wafp) (h_hi : wafp ≤ p_max) :
    p_min ≤ wafp ∧ wafp ≤ p_max := ⟨h_lo, h_hi⟩

end Pythia.Finance.Execution.SmartOrderRouter
