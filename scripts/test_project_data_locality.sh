#!/usr/bin/env bash
# Focused same-volume and fail-closed tests for scripts/project_data_env.sh.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"

fixture="$TMPDIR/project-data-locality-shell.$$"
case "$fixture" in "$ROOT"/.project-data/tmp/project-data-locality-shell.[0-9]*) ;; *) exit 1 ;; esac
cleanup() { rm -rf "$fixture"; }
trap cleanup EXIT HUP INT TERM

mkdir -p "$fixture/valid" "$fixture/outside"
printf '[workspace]\n' > "$fixture/valid/Cargo.toml"
printf '# Fixture\n' > "$fixture/valid/README.md"

if specforge_activate_project_data "$fixture/missing" >/dev/null 2>&1; then
  printf 'project-data-locality-tests: missing root was accepted\n' >&2
  exit 1
fi

if SPECFORGE_DOCLING_VENV_DIR=. "$ROOT/scripts/bootstrap_docling.sh" >/dev/null 2>&1; then
  printf 'project-data-locality-tests: repository root was accepted as the Docling venv\n' >&2
  exit 1
fi
if SPECFORGE_EVAL_VENV_DIR=. "$ROOT/scripts/bootstrap_eval.sh" >/dev/null 2>&1; then
  printf 'project-data-locality-tests: repository root was accepted as the eval venv\n' >&2
  exit 1
fi

ln -s "$fixture/outside" "$fixture/valid/.cache"
if specforge_activate_project_data "$fixture/valid" >/dev/null 2>&1; then
  printf 'project-data-locality-tests: off-root cache symlink was accepted\n' >&2
  exit 1
fi

specforge_activate_project_data "$ROOT"
repository_device="$(df -P "$ROOT" | awk 'END { print $1 }')"
for path in "$TMPDIR" "$XDG_CACHE_HOME" "$HF_HOME" "$PIP_CACHE_DIR" "$TORCH_HOME" "$MPLCONFIGDIR"; do
  case "$(cd "$path" && pwd -P)" in "$ROOT"/*) ;; *) exit 1 ;; esac
  [[ "$(df -P "$path" | awk 'END { print $1 }')" == "$repository_device" ]]
done

printf 'project-data-locality-tests: 5/5 PASS\n'
