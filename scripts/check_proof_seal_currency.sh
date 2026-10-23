#!/usr/bin/env bash
# scripts/check_proof_seal_currency.sh — the PROOF-SEAL-CURRENCY doctrine (SOURCE-IR-REPRODUCIBILITY.16), native
# since RUSTDOC-AND-XTASK.14.e in `cargo xtask check proof-seal-currency` (xtask/src/checks/proof_seal_currency.rs),
# which documents the check: every persisted artifact in the proof-carrying stratum records a seal the current
# build's canonical loader still accepts, asked through the consuming stage's `--dry-run` and never through the
# mutating `specforge validate`; a total census, and a per-stage probe that is total at `semantic` and `intent`
# and one per distinct seal elsewhere, saying so in its own output.
#
# Modes:
#   --check      (default, gate tier) the total census and the per-stage probe. Writes nothing.
#   --total      (CI tier) every in-scope document probed at every non-terminal stage. Writes nothing.
#   --self-test  prove the check's own controls are fail-closed before trusting a PASS.
# Knobs: SPECFORGE_PROOF_SEAL_GENERATED_ROOT=<dir> to point at another corpus root.
#        SPECFORGE_PROOF_SEAL_BIN=<path> to point at another `specforge` binary (skips the build).
#        SPECFORGE_PROOF_SEAL_TOTAL_STAGES=<stages> to replace the default total stages; empty turns them off.
# This shim keeps the doctrine's registered path, its argument loop and usage refusal, `LC_ALL=C`, and the
# project-data activation.
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
    --total)     MODE=total ;;
    --self-test) MODE=self-test ;;
    *) printf 'Usage: %s [--check|--total|--self-test]\n' "$0" >&2; exit 2 ;;
  esac
  shift
done

exec cargo xtask check proof-seal-currency "--$MODE"
