#!/usr/bin/env bash
# Enforce repository-derived roots for every project-owned temp/cache/artifact seam.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/project_data_env.sh"

fail=0
note() {
  printf 'project-data-locality: %s\n' "$1" >&2
  fail=1
}

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq -- "$text" "$ROOT/$file"; then
    note "$file is missing required contract text: $text"
  fi
}

if ! specforge_activate_project_data "$ROOT"; then
  note "canonical project-data roots failed validation"
fi

if [[ ! -f "$ROOT/.project-data/tmp/.gitkeep" ]]; then
  note ".project-data/tmp must exist in a fresh checkout"
fi

for variable in \
  SPECFORGE_REPO_ROOT TMPDIR TMP TEMP XDG_CACHE_HOME HF_HOME HUGGINGFACE_HUB_CACHE \
  PIP_CACHE_DIR TORCH_HOME MPLCONFIGDIR; do
  require_text ".cargo/config.toml" "$variable = { value ="
  require_text ".cargo/config.toml" "relative = true, force = true"
done

while IFS= read -r script; do
  [[ "$script" == "scripts/project_data_env.sh" ]] && continue
  require_text "$script" 'scripts/project_data_env.sh"'
  require_text "$script" "specforge_activate_project_data"
done < <(cd "$ROOT" && find scripts -maxdepth 1 -type f -name '*.sh' -print | sort)

require_text ".githooks/pre-commit" 'source "$ROOT/scripts/project_data_env.sh"'
require_text ".githooks/pre-commit" 'specforge_activate_project_data "$ROOT"'
require_text "crates/specforge/src/lib.rs" "project_data::prepare()?;"
require_text "crates/specforge/src/project_data.rs" ".tempdir_in(roots.temporary)?"
require_text "crates/specforge/src/project_data.rs" "ensure_same_filesystem"

for seam in \
  crates/specforge/src/commands/doctor.rs \
  crates/specforge/src/commands/enrich.rs \
  crates/specforge/src/commands/llm_text.rs \
  crates/specforge/src/commands/eval_extraction.rs \
  crates/specforge/src/commands/kg_bench.rs \
  crates/specforge/src/ir/source/docling_backend.rs; do
  require_text "$seam" "crate::project_data::tempdir()?"
done

for seam in \
  crates/specforge/src/commands/doctor.rs \
  crates/specforge/src/commands/enrich.rs \
  crates/specforge/src/commands/llm_text.rs \
  crates/specforge/src/commands/recover_register_bits.rs \
  crates/specforge/src/ir/mod.rs \
  crates/specforge/src/ir/source/docling_backend.rs; do
  require_text "$seam" "crate::project_data::configure_command"
done

for lock in requirements/docling-macos-arm64.lock.txt requirements/eval-macos-arm64.lock.txt; do
  if [[ ! -s "$ROOT/$lock" ]]; then
    note "missing non-empty Python environment authority: $lock"
  fi
done

for environment in .venv-docling .venv-eval; do
  [[ -d "$ROOT/$environment" ]] || continue
  prefix="$($ROOT/$environment/bin/python -c 'import sys; print(sys.prefix)' 2>/dev/null)" || {
    note "$environment/bin/python is not executable"
    continue
  }
  if [[ "$prefix" != "$ROOT/$environment" ]]; then
    note "$environment resolves outside the repository: $prefix"
  fi
  stale="$({ rg -n '/[^ ]*/specforge/\.venv' "$ROOT/$environment/bin" \
      "$ROOT/$environment/pyvenv.cfg" 2>/dev/null || true; } | grep -Fv "$ROOT" || true)"
  if [[ -n "$stale" ]]; then
    note "$environment retains a launcher or activation path into another repository copy"
  fi
done

temporary_residue="$(find "$ROOT/.project-data/tmp" -mindepth 1 -maxdepth 1 -type f \
  \( -name '*.fsm' -o -name '*.log' \) -print | sort)"
if [[ -n "$temporary_residue" ]]; then
  note "disposable FSM/log residue remains in .project-data/tmp: $temporary_residue"
fi

if ! "$ROOT/scripts/test_project_data_locality.sh" >/dev/null; then
  note "focused project-data locality cases failed"
fi

if [[ "$fail" -eq 0 ]]; then
  printf 'project-data-locality: PASS — repository-derived roots and producer seams are enforced\n' >&2
else
  printf 'project-data-locality: FAIL\n' >&2
fi
exit "$fail"
