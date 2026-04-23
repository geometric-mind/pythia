/-
Kairos.Stats.DeploymentDesign — inverse of the quantization-slack
bound.  Given a target coverage deviation δ and a sub-Gaussian
parameter σ, compute the minimal bit-width + fractional scale needed
so the η_F · 2^{-s} · σ slack stays under δ.

Research + regulator-facing formula: "to guarantee coverage within
δ of stated α under family F, deploy at scale s ≥ log₂(η_F(b) · σ
/ δ), given some fixed b."

Target lemmas (Aristotle closures):
- `deploymentScale_bound_HR`: for HR family, the minimal scale s
  is `s ≥ (log₂ b + log b log 2 + log₂(σ/δ))/2`.
- `deploymentScale_bound_Betting`: for betting, vanishing-rate
  version.  Minimal scale decreases as b grows.
- `deploymentBits_bound_HR`: inverse — given s, minimal b.
-/

import Mathlib
import Kairos.Stats.Basic
import Kairos.Stats.Quantization

namespace Kairos.Stats

/-- Target coverage-deviation tolerance.  If the realised coverage
stays within `delta` of the stated `alpha`, the deployment meets the
design spec. -/
structure DeploymentSpec where
  alpha : ℝ
  delta : ℝ
  sigma : ℝ
  alpha_in_range : 0 < alpha ∧ alpha < 1
  delta_pos : 0 < delta
  sigma_pos : 0 < sigma

/-- For the Howard-Ramdas (self-normalized) family: given a deployment
spec, the minimal fractional scale `s` such that the predicted slack
$\eta_\mathrm{HR}(b) \cdot 2^{-s} \cdot \sigma \leq \delta$ is
`s ≥ ⌈(1/2) log₂(b · log 2) + log₂(σ / δ)⌉`.  Stated as an inequality
that Aristotle can close by arithmetic + positivity. -/
theorem deploymentScale_bound_HR
    (ds : DeploymentSpec) (b : ℕ) (hb : 0 < b) (s : ℕ)
    (hs : (Real.sqrt (b * Real.log 2)) * (2 : ℝ)^(-(s : ℤ)) * ds.sigma
          ≤ ds.delta) :
    -- explicit arithmetic consequence: 2^s ≥ (σ/δ) · √(b log 2)
    (2 : ℝ)^(s : ℤ) ≥ (ds.sigma / ds.delta) * Real.sqrt (b * Real.log 2) := by
  sorry

/-- Symmetric bound for the betting family.  Betting is the
vanishing-rate family, so the minimal scale decreases with b. -/
theorem deploymentScale_bound_Betting
    (ds : DeploymentSpec) (b : ℕ) (hb : 0 < b) (s : ℕ)
    (hs : (1 / Real.sqrt ((b : ℝ) * Real.log 2 + 1)) * (2 : ℝ)^(-(s : ℤ))
          * ds.sigma ≤ ds.delta) :
    (2 : ℝ)^(s : ℤ) ≥ (ds.sigma / ds.delta) /
      Real.sqrt ((b : ℝ) * Real.log 2 + 1) := by
  sorry

/-- The minimum bit-width `b` given a fixed fractional scale `s`.  For
the Howard-Ramdas family, `b ≤ (δ · 2^s / σ)^2 / log 2`.  This is the
corollary regulators would apply: "at this embedded-firmware precision
$s$, the maximum horizon supporting $\delta$ coverage deviation is $b$". -/
theorem deploymentBits_bound_HR
    (ds : DeploymentSpec) (s : ℕ) (hs : 0 < s)
    (b : ℕ) (hb_bound : (b : ℝ) * Real.log 2 ≤
      (ds.delta * (2 : ℝ)^(s : ℤ) / ds.sigma) ^ 2) :
    Real.sqrt (b * Real.log 2) * (2 : ℝ)^(-(s : ℤ)) * ds.sigma ≤ ds.delta := by
  sorry

end Kairos.Stats
