#!/usr/bin/env bash
# scripts/check_chain_currency.sh — the CHAIN-CURRENCY doctrine (CORPUS-CHAIN-CURRENCY.1, ADR 0025 decision 2),
# native since RUSTDOC-AND-XTASK.14.d in `cargo xtask check chain-currency` (xtask/src/checks/chain_currency.rs),
# which documents the oracle: every persisted artifact must be what the current release build reproduces from its
# persisted input, and the retained normalized bundles must equal their declaration exactly.
#
# This shim keeps the doctrine's registered path, its argument loop and usage refusal, `LC_ALL=C`, and the
# project-data activation. Its knobs pass through the environment:
#   SPECFORGE_CHAIN_CURRENCY_GENERATED_ROOT=<dir> to point at another corpus root (the self-test uses it to prove the
#   absent-corpus skip end to end);
#   SPECFORGE_CHAIN_CURRENCY_RETENTION_CONTRACT=<file> to point at another retained-bundle declaration.
# Cost: a cargo freshness build plus the full replay, so CI tier (DOCTRINE_ENFORCEMENT.md §4.7).
set -uo pipefail
export LC_ALL=C

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"

MODE=check
while [ "$#" -gt 0 ]; do
  case "$1" in
    --check)     MODE=check ;;
    --self-test) MODE=self-test ;;
    *) printf 'Usage: %s [--check|--self-test]\n' "$0" >&2; exit 2 ;;
  esac
  shift
done

exec cargo xtask check chain-currency "--$MODE"
