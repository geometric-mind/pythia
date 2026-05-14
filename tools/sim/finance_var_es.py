"""Value-at-Risk + Expected Shortfall (Normal-case closed form): empirical companion.

Lean side (`Pythia/Finance/ValueAtRisk.lean` + `ExpectedShortfall.lean`):
  * `varNormal μ σ z := -μ + σ · z`
  * `esNormal  μ σ h := -μ + σ · h`
  * Positive homogeneity / translation / ES-dominates-VaR properties.

Run:
    python3 -m tools.sim.finance_var_es

Or via pytest:
    pytest tools/sim/finance_var_es.py
"""
from __future__ import annotations

import math

from tools.sim.harness import (
    Mutation,
    Strategy,
    floats,
    run_harness,
)


def var_normal(mu: float, sigma: float, z: float) -> float:
    return -mu + sigma * z


def es_normal(mu: float, sigma: float, h: float) -> float:
    return -mu + sigma * h


def var_translation_spec(mu: float, sigma: float, z: float, c: float) -> bool:
    """varNormal(μ+c) = varNormal(μ) - c (ADEH cash-invariance)."""
    lhs = var_normal(mu + c, sigma, z)
    rhs = var_normal(mu, sigma, z) - c
    return math.isclose(lhs, rhs, abs_tol=1e-9, rel_tol=1e-9)


def es_dominates_var_spec(mu: float, sigma: float, z: float, h: float) -> bool:
    """For σ ≥ 0 and z ≤ h: VaR(μ, σ, z) ≤ ES(μ, σ, h)."""
    if sigma < 0 or z > h:
        return True  # premise fails
    return var_normal(mu, sigma, z) <= es_normal(mu, sigma, h) + 1e-12


# ============================================================================
# Mutations
# ============================================================================


def _var_translation_wrong_sign(mu: float, sigma: float, z: float, c: float) -> bool:
    """Wrong-sign on translation: claims var(μ+c) = var(μ) + c."""
    lhs = var_normal(mu + c, sigma, z)
    rhs = var_normal(mu, sigma, z) + c  # wrong sign
    return math.isclose(lhs, rhs, abs_tol=1e-9, rel_tol=1e-9)


def _var_translation_drop_c(mu: float, sigma: float, z: float, c: float) -> bool:
    """Drops c on RHS: claims var(μ+c) = var(μ)."""
    lhs = var_normal(mu + c, sigma, z)
    rhs = var_normal(mu, sigma, z)  # missing -c
    return math.isclose(lhs, rhs, abs_tol=1e-9, rel_tol=1e-9)


def _es_dominates_var_reversed(mu: float, sigma: float, z: float, h: float) -> bool:
    """Claims ES ≤ VaR (reversed) — fails when z < h and σ > 0."""
    if sigma < 0 or z > h:
        return True
    return es_normal(mu, sigma, h) <= var_normal(mu, sigma, z) + 1e-12


def _es_dominates_var_drop_sigma(mu: float, sigma: float, z: float, h: float) -> bool:
    """Drops σ from RHS: claims VaR(μ, σ, z) ≤ ES(μ, 0, h) = -μ.
    Fails when σ > 0 and σ·z > 0."""
    if sigma < 0 or z > h:
        return True
    mutant_es = -mu  # σ dropped
    return var_normal(mu, sigma, z) <= mutant_es + 1e-12


# ============================================================================
# Strategies
# ============================================================================


_strategy_translation = Strategy(
    mu=floats(lo=-0.10, hi=0.20),
    sigma=floats(lo=0.01, hi=0.50),
    z=floats(lo=1.0, hi=3.0),
    c=floats(lo=-0.05, hi=0.05),
)


_strategy_dominance = Strategy(
    mu=floats(lo=-0.10, hi=0.20),
    sigma=floats(lo=0.01, hi=0.50),
    z=floats(lo=1.0, hi=2.5),
    h=floats(lo=2.5, hi=4.0),  # h ≥ z guaranteed by range
)


# ============================================================================
# Tests
# ============================================================================


def test_var_translation():
    result = run_harness(
        name="var_translation",
        spec=var_translation_spec,
        strategy=_strategy_translation,
        mutations=(
            Mutation("wrong_sign", _var_translation_wrong_sign, min_failure_rate=0.50),
            Mutation("drop_c", _var_translation_drop_c, min_failure_rate=0.50),
        ),
    )
    assert result.all_passed, result.summarize()


def test_es_dominates_var():
    result = run_harness(
        name="es_dominates_var",
        spec=es_dominates_var_spec,
        strategy=_strategy_dominance,
        mutations=(
            Mutation("reversed", _es_dominates_var_reversed, min_failure_rate=0.30),
            Mutation("drop_sigma", _es_dominates_var_drop_sigma, min_failure_rate=0.30),
        ),
    )
    assert result.all_passed, result.summarize()


if __name__ == "__main__":
    test_var_translation()
    test_es_dominates_var()
    print("var_es: PBT + 4 mutation tests passed.")
