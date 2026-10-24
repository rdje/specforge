#!/usr/bin/env bash
# scripts/check_proof_seal_total.sh — the CI-tier registry entry for the TOTAL proof-seal probe, native since
# RUSTDOC-AND-XTASK.14.f in `cargo xtask check proof-seal-total` (xtask/src/checks/proof_seal_currency.rs).
#
# SIGNAL-DECLARATION-ROW-DROP.1c. `PROOF-SEAL-CURRENCY` owns both the census and the probe; this entry exists only
# because the doctrine driver's registry names ONE executable per row and passes it no arguments
# (`scripts/check_doctrines.sh`, the `-x "$ROOT/$script"` contract). So the two tiers of the same check need two
# entries, and this is the second one.
#
# WHY THERE ARE TWO TIERS, measured. The gate-tier mode probes one document per distinct seal at the extraction-
# replaying stages. That is a sample, and the counterexample is in this repository's history: at `48def695` all 27
# evidence artifacts carried ONE seal, so one probe ran and the gate reported green while the canonical loader
# refused 4 of the 27 — every wire-bearing document — for three commits. The seal is a digest over the ruleset and
# ignores artifact content; the loader also verifies a per-document replay topology that does not. Only a
# per-document probe can see that class, and a per-document probe over this corpus is minutes rather than seconds,
# so it is enforced here instead of on every commit.
#
# All behaviour, knobs, skip semantics, and self-tests live in the owning check — including `--self-test`, which
# this entry forwards, so a `--self-test` sweep over the registry reaches the same controls (their declared total
# is the owning check's, not a number carried here) rather than a second, weaker copy. This shim keeps the
# registered path, its first-argument dispatch and usage refusal, and the project-data activation: the locality
# contract is a property of every governed entrypoint, not only of the one that does the work.
set -uo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"

case "${1:-}" in
  --self-test) exec cargo xtask check proof-seal-total --self-test ;;
  ''|--check|--total) exec cargo xtask check proof-seal-total --total ;;
  *) printf 'Usage: %s [--check|--total|--self-test]  (all modes run the TOTAL probe)\n' "$0" >&2; exit 2 ;;
esac
