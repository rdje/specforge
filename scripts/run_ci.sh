#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT_DIR/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT_DIR"

cd "${ROOT_DIR}"

echo "[specforge-ci] enforcing every registered doctrine via the registry/driver (all tiers)"
./scripts/check_doctrines.sh --all

echo "[specforge-ci] qualifying production genericity with controlled mutations, behavioral-contract faults, and per-rule alpha obligations"
./scripts/check_production_genericity.sh --self-test

echo "[specforge-ci] checking formatting"
cargo fmt --all --check

CI_RUSTFLAGS="-D warnings"
if [[ -n "${RUSTFLAGS:-}" ]]; then
  CI_RUSTFLAGS="${RUSTFLAGS} ${CI_RUSTFLAGS}"
fi
CI_RUSTDOCFLAGS="-D warnings"
if [[ -n "${RUSTDOCFLAGS:-}" ]]; then
  CI_RUSTDOCFLAGS="${RUSTDOCFLAGS} ${CI_RUSTDOCFLAGS}"
fi

echo "[specforge-ci] running clippy with warnings denied"
RUSTFLAGS="${CI_RUSTFLAGS}" cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings

# ── The six tests that need the persisted corpus ────────────────────────────────────────────────
# COMMIT-GATE-SINGLE-RUN.15. `/generated/` is untracked, so on a runner these six replay a normalized
# bundle that does not exist and fail with `authority_unavailable: behavioral source authority is
# unavailable`. They are named ONE BY ONE on purpose: `--skip behavioral_genericity` would also drop
# that module's alpha-transform, comparator and classifier tests, which need no corpus and are real
# coverage a runner CAN collect. A renamed test simply stops being skipped and the suite goes red,
# which is the safe direction; an over-broad filter would go quiet instead, which is not.
CORPUS_DEPENDENT_TESTS=(
  behavioral_genericity::tests::full_alpha_run_emits_five_stage_evidence
  behavioral_genericity::tests::full_reviewed_layout_run_emits_five_stage_evidence
  behavioral_genericity::tests::full_reviewed_paraphrase_run_emits_five_stage_evidence
  behavioral_genericity::tests::full_semantic_negative_run_requires_declared_delta_and_rejects_invariance
  behavioral_genericity::tests::missing_source_attempt_is_unmeasurable_without_creating_output
  behavioral_genericity::tests::retained_authority_rejects_non_scratch_links_and_missing_alpha_report_runs_fresh
)

CI_TEST_FILTER=()
if [ ! -d "${ROOT_DIR}/generated/source_ir" ] || [ -z "$(ls -A "${ROOT_DIR}/generated/source_ir" 2>/dev/null)" ]; then
  if [ -z "${SPECFORGE_CORPUS_ABSENT:-}" ]; then
    echo "[specforge-ci] refused — generated/source_ir is empty or absent and the environment has not" >&2
    echo "[specforge-ci] declared itself corpus-free; set SPECFORGE_CORPUS_ABSENT=1 to run the" >&2
    echo "[specforge-ci] corpus-independent suite and have the rest reported as NOT discharged." >&2
    exit 2
  fi
  echo "[specforge-ci] SKIP: no corpus at generated/source_ir — ${#CORPUS_DEPENDENT_TESTS[@]} behavioral"
  echo "[specforge-ci] SKIP: conformance tests are NOT run here. Nothing was measured for them:"
  CI_TEST_FILTER=(--)
  for t in "${CORPUS_DEPENDENT_TESTS[@]}"; do
    echo "[specforge-ci] SKIP:   ${t}"
    CI_TEST_FILTER+=(--skip "$t")
  done
fi

echo "[specforge-ci] running test suite with Rust warnings denied"
RUSTFLAGS="${CI_RUSTFLAGS}" cargo test --manifest-path Cargo.toml "${CI_TEST_FILTER[@]}"

echo "[specforge-ci] building Rust docs with warnings denied"
RUSTDOCFLAGS="${CI_RUSTDOCFLAGS}" cargo doc --manifest-path Cargo.toml --no-deps

echo "[specforge-ci] building docs"
./scripts/run_docs_ci.sh

echo "[specforge-ci] rechecking project-data residue after all producers"
./scripts/check_project_data_locality.sh
