"""Kelly criterion (binary bet, algebraic form): empirical companion.

Lean side (`Pythia/Finance/Kelly.lean`):
  * `kellyFraction p b := (p * (b + 1) - 1) / b`
  * kellyFraction_zero_edge / mono_p / unit_odds

Run:
    python3 -m tools.sim.finance_kelly

Or via pytest:
    pytest tools/sim/finance_kelly.py
"""
from __future__ import annotations

import math

from tools.sim.harness import (
    Mutation,
    Strategy,
    floats,
    run_harness,
)


def kelly_fraction(p: float, b: float) -> float:
    return (p * (b + 1) - 1) / b


def kelly_unit_odds_spec(p: float) -> bool:
    """At b=1: Kelly = 2p - 1."""
    lhs = kelly_fraction(p, 1.0)
    rhs = 2 * p - 1
    return math.isclose(lhs, rhs, abs_tol=1e-9, rel_tol=1e-9)


def kelly_mono_p_spec(b: float, p1: float, p2: float) -> bool:
    """For b > 0 and p1 ≤ p2: Kelly(p1) ≤ Kelly(p2)."""
    if b <= 0 or p1 > p2:
        return True
    return kelly_fraction(p1, b) <= kelly_fraction(p2, b) + 1e-12


# ============================================================================
# Mutations
# ============================================================================


def _unit_odds_drop_minus_one(p: float) -> bool:
    """Wrong: claims Kelly(p, 1) = 2p (drops the -1)."""
    lhs = kelly_fraction(p, 1.0)
    rhs = 2 * p  # missing -1
    return math.isclose(lhs, rhs, abs_tol=1e-9, rel_tol=1e-9)


def _unit_odds_wrong_factor(p: float) -> bool:
    """Wrong: claims Kelly(p, 1) = p - 1 instead of 2p - 1."""
    lhs = kelly_fraction(p, 1.0)
    rhs = p - 1  # wrong factor
    return math.isclose(lhs, rhs, abs_tol=1e-9, rel_tol=1e-9)


def _mono_p_reversed(b: float, p1: float, p2: float) -> bool:
    """Claims Kelly is anti-monotone in p (wrong direction)."""
    if b <= 0 or p1 > p2:
        return True
    return kelly_fraction(p2, b) <= kelly_fraction(p1, b) + 1e-12


def _kelly_wrong_numerator(p: float, b: float) -> bool:
    """Wrong: uses (p·b - 1)/b instead of (p·(b+1) - 1)/b.
    Off by p/b — fails for p ≠ 0."""
    mutant = (p * b - 1) / b
    correct = kelly_fraction(p, b)
    return math.isclose(mutant, correct, abs_tol=1e-9, rel_tol=1e-9)


# ============================================================================
# Strategies
# ============================================================================


_strategy_unit = Strategy(
    p=floats(lo=0.0, hi=1.0),
)


_strategy_mono = Strategy(
    b=floats(lo=0.1, hi=10.0),
    p1=floats(lo=0.0, hi=0.5),
    p2=floats(lo=0.5, hi=1.0),
)


_strategy_general = Strategy(
    p=floats(lo=0.0, hi=1.0),
    b=floats(lo=0.1, hi=10.0),
)


# ============================================================================
# Tests
# ============================================================================


def test_kelly_unit_odds():
    result = run_harness(
        name="kelly_unit_odds",
        spec=kelly_unit_odds_spec,
        strategy=_strategy_unit,
        mutations=(
            Mutation("drop_minus_one", _unit_odds_drop_minus_one, min_failure_rate=0.50),
            Mutation("wrong_factor", _unit_odds_wrong_factor, min_failure_rate=0.30),
        ),
    )
    assert result.all_passed, result.summarize()


def test_kelly_mono_p():
    result = run_harness(
        name="kelly_mono_p",
        spec=kelly_mono_p_spec,
        strategy=_strategy_mono,
        mutations=(
            Mutation("reversed", _mono_p_reversed, min_failure_rate=0.50),
        ),
    )
    assert result.all_passed, result.summarize()


def test_kelly_numerator_form():
    """Closed-form sanity-check via mutation harness against general
    (p, b) range — verifies the (p(b+1) - 1)/b structure is correct
    and not the off-by-p/b mutant."""

    def spec(p: float, b: float) -> bool:
        # The spec just confirms kelly_fraction itself is well-formed.
        # The harness mutation catches the wrong-numerator class.
        return math.isfinite(kelly_fraction(p, b))

    result = run_harness(
        name="kelly_numerator_form",
        spec=spec,
        strategy=_strategy_general,
        mutations=(
            Mutation("wrong_numerator", _kelly_wrong_numerator, min_failure_rate=0.30),
        ),
    )
    # The base spec passes trivially; the mutation must fail. We don't
    # assert all_passed because the trivial spec doesn't catch the
    # mutant — we assert via the mutation-caught list explicitly.
    assert "wrong_numerator" in result.mutations_caught, (
        f"Expected 'wrong_numerator' mutation caught, got "
        f"caught={result.mutations_caught} missed={result.mutations_missed}"
    )


if __name__ == "__main__":
    test_kelly_unit_odds()
    test_kelly_mono_p()
    test_kelly_numerator_form()
    print("kelly: PBT + 4 mutation tests passed.")
