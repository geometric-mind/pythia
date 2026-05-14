#!/usr/bin/env bash
# tools/pre-push.sh — local pre-push gate.
#
# Per Aidan 2026-04-25 mandate: never push anything (not even into a
# branch) if any Lean doesn't compile. This script enforces that
# locally before `git push`.
#
# Install as a git pre-push hook:
#
#     ln -s ../../tools/pre-push.sh .git/hooks/pre-push
#
# Or run manually before any push:
#
#     bash tools/pre-push.sh && git push
#
# Exits 0 only if `lake build` succeeds. Exits non-zero (blocking the
# push) on any build failure.

set -e

cd "$(git rev-parse --show-toplevel)"

echo "🔒 pre-push: running lake build (Aidan 2026-04-25 mainline-protection mandate)..."
lake build
echo "✅ lake build clean"

# README-stats-freshness gate (ATH-1300 structural lift, 5-incident receipt:
# fb8376f / bfc8921 / 47248da / cd271be / 4a2c366). Refresh README stats
# whenever any Pythia/**/*.lean change is in the push range; refuse the push
# if doing so produces a non-empty diff (i.e., stats were stale).
PUSH_RANGE="@{upstream}..HEAD"
if ! git rev-parse --verify --quiet "$PUSH_RANGE" >/dev/null; then
  PUSH_RANGE="HEAD~1..HEAD"
fi
PYTHIA_TOUCHED=$(git diff --name-only "$PUSH_RANGE" -- 'Pythia/**/*.lean' 2>/dev/null || true)
if [ -n "$PYTHIA_TOUCHED" ]; then
  echo "🔒 pre-push: Pythia/*.lean change detected — refreshing README stats..."
  python3 tools/refresh_readme_stats.py >/dev/null
  if ! git diff --quiet README.md; then
    echo "❌ pre-push: README stats are stale (refresh_readme_stats.py produced a diff)."
    echo "   Commit the refreshed README.md and re-push:"
    echo "     git add README.md && git commit -m 'chore(README): refresh stats' && git push"
    echo ""
    echo "   Diff preview:"
    git --no-pager diff README.md | head -20
    exit 1
  fi
  echo "✅ README stats fresh"
fi
echo "✅ all pre-push gates clean — push permitted"
