/-
Copyright (c) 2026 Pythia contributors. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.

# Fair Value Estimator Properties

Proves properties of micro-price and fair value estimators used
by market makers for quoting.
-/
import Mathlib
import Pythia.Tactic.Pythia

namespace Pythia.Finance.HFT.FairValueEstimator

/-- Micro-price: volume-weighted mid between bid and ask.
micro = (ask_qty * bid + bid_qty * ask) / (bid_qty + ask_qty) -/
noncomputable def microPrice (bid ask bid_qty ask_qty : ℝ) : ℝ :=
  (ask_qty * bid + bid_qty * ask) / (bid_qty + ask_qty)

/-- **Micro-price between bid and ask.** -/
@[stat_lemma]
theorem microPrice_between {bid ask bid_qty ask_qty : ℝ}
    (h_bid_le : bid ≤ ask) (hbq : 0 < bid_qty) (haq : 0 < ask_qty) :
    bid ≤ microPrice bid ask bid_qty ask_qty ∧
    microPrice bid ask bid_qty ask_qty ≤ ask := by
  unfold microPrice
  have htot : 0 < bid_qty + ask_qty := by linarith
  constructor
  · rw [le_div_iff₀ htot]
    nlinarith
  · rw [div_le_iff₀ htot]
    nlinarith

/-- **Equal sizes gives true mid.** When bid_qty = ask_qty,
micro-price equals arithmetic mid. -/
@[stat_lemma]
theorem microPrice_equal_sizes {bid ask qty : ℝ} (hq : 0 < qty) :
    microPrice bid ask qty qty = (bid + ask) / 2 := by
  unfold microPrice
  field_simp; ring

/-- **Imbalance shifts toward thin side.** When ask_qty > bid_qty
(more selling pressure), micro-price shifts toward bid. -/
@[stat_lemma]
theorem microPrice_shifts_toward_thin {bid ask bid_qty ask_qty : ℝ}
    (h_bid_le : bid ≤ ask) (hbq : 0 < bid_qty) (haq : 0 < ask_qty)
    (h_imbalance : bid_qty < ask_qty)
    (h_spread : bid < ask) :
    microPrice bid ask bid_qty ask_qty < (bid + ask) / 2 := by
  unfold microPrice
  have htot : 0 < bid_qty + ask_qty := by linarith
  rw [div_lt_div_iff₀ htot (by norm_num : (0:ℝ) < 2)]
  nlinarith

/-- **EWMA fair value update.** Exponentially weighted moving
average of trade prices. New estimate = alpha * trade + (1-alpha) * old. -/
noncomputable def ewmaUpdate (alpha trade old : ℝ) : ℝ :=
  alpha * trade + (1 - alpha) * old

/-- **EWMA between old and new.** For alpha in [0,1]. -/
@[stat_lemma]
theorem ewma_between {alpha trade old : ℝ}
    (ha0 : 0 ≤ alpha) (ha1 : alpha ≤ 1) (h : old ≤ trade) :
    old ≤ ewmaUpdate alpha trade old := by
  unfold ewmaUpdate
  linarith [mul_nonneg ha0 (by linarith : 0 ≤ trade - old)]

end Pythia.Finance.HFT.FairValueEstimator
