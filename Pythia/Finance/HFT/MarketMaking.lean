/-
Copyright (c) 2026 Pythia contributors. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.

# Market Making Invariants

Proves properties of market-making strategies: spread profitability,
inventory risk bounds, and quote update correctness.
-/
import Mathlib
import Pythia.Tactic.Pythia

namespace Pythia.Finance.HFT.MarketMaking

/-- **Spread profit per round trip.** A market maker who buys at bid
and sells at ask earns the spread per share. -/
@[stat_lemma]
theorem spread_profit_nonneg {bid ask qty : ℝ}
    (h_spread : bid ≤ ask) (h_qty : 0 ≤ qty) :
    0 ≤ (ask - bid) * qty :=
  mul_nonneg (by linarith) h_qty

/-- **Inventory risk is quadratic.** The variance of PnL from
holding inventory q at volatility sigma is q^2 * sigma^2. -/
@[stat_lemma]
theorem inventory_risk_nonneg {q sigma : ℝ} :
    0 ≤ q ^ 2 * sigma ^ 2 :=
  mul_nonneg (sq_nonneg q) (sq_nonneg sigma)

/-- **Inventory risk monotone in position.** Larger absolute
position means more risk. -/
@[stat_lemma]
theorem inventory_risk_mono {q₁ q₂ sigma : ℝ}
    (h : q₁ ^ 2 ≤ q₂ ^ 2) :
    q₁ ^ 2 * sigma ^ 2 ≤ q₂ ^ 2 * sigma ^ 2 :=
  mul_le_mul_of_nonneg_right h (sq_nonneg sigma)

/-- **Optimal spread widens with volatility.** The Avellaneda-Stoikov
optimal spread is proportional to sigma. Higher vol = wider quotes. -/
@[stat_lemma]
theorem spread_widens_with_vol {k σ₁ σ₂ : ℝ}
    (hk : 0 ≤ k) (h : σ₁ ≤ σ₂) :
    k * σ₁ ≤ k * σ₂ :=
  mul_le_mul_of_nonneg_left h hk

/-- **Skew from inventory.** A market maker with long inventory
skews quotes down (lower bid and ask) to encourage selling.
The skew is proportional to inventory. -/
@[stat_lemma]
theorem inventory_skew_direction {gamma q mid_adj mid : ℝ}
    (h_gamma : 0 < gamma) (h_long : 0 < q)
    (h_adj : mid_adj = mid - gamma * q) :
    mid_adj < mid := by linarith [mul_pos h_gamma h_long]

/-- **PnL from completed round trip.** Buy at bid, sell at ask,
net PnL = (ask - bid) * qty - 2 * fee * qty. Profitable iff
spread > 2 * fee. -/
@[stat_lemma]
theorem round_trip_profitable {spread fee qty : ℝ}
    (h_qty : 0 < qty) (h_spread : 2 * fee < spread) :
    0 < (spread - 2 * fee) * qty :=
  mul_pos (by linarith) h_qty

/-- **Quote symmetry.** Symmetric quotes around mid:
bid = mid - half_spread, ask = mid + half_spread.
Spread = ask - bid = 2 * half_spread. -/
@[stat_lemma]
theorem symmetric_spread (mid half_spread : ℝ) :
    (mid + half_spread) - (mid - half_spread) = 2 * half_spread := by ring

end Pythia.Finance.HFT.MarketMaking
