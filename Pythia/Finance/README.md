# Pythia.Finance

Formally verified quantitative finance library. 100+ modules, 700+ theorems, zero sorry in mainline code.

## Quick start

```lean
import Pythia.Finance.OptionPricing   -- everything for pricing and hedging
import Pythia.Finance.PortfolioTheory -- everything for portfolio construction
import Pythia.Finance.RiskManagement  -- everything for risk measurement
```

Or import everything:

```lean
import Pythia.Finance.All
```

## Workflow modules

Each workflow module re-exports all theorems a practitioner needs for that task. One import per workflow.

| Module | What it covers |
|--------|----------------|
| `OptionPricing` | Black-Scholes call/put, Greeks, put-call parity, intrinsic bounds, no-arb bounds, butterfly nonneg, barrier/Asian/lookback payoffs, delta hedging, variance swaps, vol smile |
| `PortfolioTheory` | CAPM beta, Markowitz frontier, efficient frontier, Kelly criterion, mean-variance utility, portfolio optimality (FOC + diversification benefit), concentration risk (HHI), risk parity, factor model |
| `RiskManagement` | VaR, expected shortfall, Sharpe/Sortino/Treynor/Calmar ratios, realised volatility, tracking error, max drawdown, coherent risk axioms, convex risk measures, entropy risk, kurtosis |
| `FixedIncome` | Bond pricing (zero-coupon, Vasicek), yield-from-price, duration, convexity, forward rates, annuity factor, perpetuity, compound interest |
| `StochasticModels` | GBM terminal value, Bachelier, Ornstein-Uhlenbeck, Heston long-run variance, Merton jump-diffusion, GARCH update, FTAP (no-arbitrage implies risk-neutral measure), risk-neutral pricing, discrete Ito formula, Black-Scholes PDE |
| `CreditRisk` | Credit spreads, expected loss, Merton structural model, credit default swaps |
| `Execution` | Almgren-Chriss TWAP cost + TWAP optimality (Cauchy-Schwarz proof), market impact, transaction costs |
| `Fundamentals` | NPV, Gordon growth, dividend discount model, Modigliani-Miller, forward pricing |

## What makes this different

Every theorem is formally verified by the Lean 4 kernel. The proofs use real Mathlib reasoning:

- `FTAP.riskNeutralImpliesNoArbitrage`: contradiction via `Finset.add_sum_erase` + `mul_pos` on strictly positive risk-neutral probabilities
- `AlmgrenChrissOptimal.twapIsOptimal`: TWAP minimizes execution cost via `Finset.sum_mul_sq_le_sq_mul_sq` (Cauchy-Schwarz)
- `DeltaHedging.vol_arb_profit`: realized > implied volatility implies long gamma profits via `mul_nonneg` chain
- `PortfolioOptimality.diversification_benefit`: equal-weight variance < average variance via `nlinarith [sq_nonneg (v1 - v2)]`
- `ConcentrationRisk.split_reduces_hhi`: splitting a position reduces concentration via `nlinarith [sq_nonneg (w - 2*d)]`

These are not ring identities. They are the proofs a portfolio manager, risk officer, or execution trader needs to trust the system.

## Run a proof

```bash
# Verify a single module
lake env lean Pythia/Finance/FTAP.lean

# Build everything
lake build Pythia.Finance.All

# Run paired Python simulation (Monte Carlo verification)
python3 tools/run_pythia_sim.py finance_sharpe_ratio
```

## Docker

```bash
docker pull ghcr.io/athanor-ai/pythia:latest
docker run --rm ghcr.io/athanor-ai/pythia:latest lake build Pythia.Finance.All
```

## For quant researchers

The theorems in this library are the ones you write on whiteboards and trust the implementation to match. We close them in Lean so the compiler guarantees the match. Use them to:

1. Validate pricing engines (no-arb bounds, put-call parity)
2. Justify execution schedules (TWAP optimality)
3. Prove risk limits are enforced (HHI concentration, position bounds)
4. Certify hedge ratios (delta-hedging PnL decomposition)
5. Bound numerical error (fixed-point EMA steady-state error)

## For quant developers

The proofs pair with Python simulation runners in `tools/sim/finance_*.py`. Each sim generates 10k+ Monte Carlo paths and verifies the Lean bound holds empirically. The combination (formal proof + empirical verification) is strictly stronger than either alone.

## Axiom set

All proofs close under `{propext, Classical.choice, Quot.sound}`. No native_decide, no custom axioms. Verified by `Pythia.AxiomAudit`.
