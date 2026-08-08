#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT_DIR/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT_DIR"

VENV_RELATIVE="${SPECFORGE_EVAL_VENV_DIR:-.venv-eval}"
LOCK_RELATIVE="${SPECFORGE_EVAL_REQUIREMENTS:-requirements/eval-macos-arm64.lock.txt}"
BOOTSTRAP_PYTHON="${SPECFORGE_EVAL_BOOTSTRAP_PYTHON:-}"

if [[ ! "$VENV_RELATIVE" =~ ^[A-Za-z0-9._-]+$ || "$VENV_RELATIVE" == "." || "$VENV_RELATIVE" == ".." ]]; then
  printf 'error: eval venv must be one safe repository-root child directory\n' >&2
  exit 1
fi
case "$LOCK_RELATIVE" in /*|../*|*/../*|*/..) printf 'error: eval requirements must be repository-relative\n' >&2; exit 1 ;; esac

VENV_DIR="$ROOT_DIR/$VENV_RELATIVE"
LOCK_PATH="$ROOT_DIR/$LOCK_RELATIVE"
BACKUP_DIR="$ROOT_DIR/.project-data/tmp/eval-venv-backup.$$"

pick_python() {
  if [[ -n "$BOOTSTRAP_PYTHON" ]]; then
    command -v "$BOOTSTRAP_PYTHON"
    return
  fi
  local candidate
  for candidate in python3.14 python3.13 python3.12 python3 python; do
    if command -v "$candidate" >/dev/null 2>&1; then
      command -v "$candidate"
      return
    fi
  done
  printf 'error: no usable eval bootstrap Python found\n' >&2
  return 1
}

PYTHON_BIN="$(pick_python)"
[[ -s "$LOCK_PATH" ]] || { printf 'error: missing requirements lock: %s\n' "$LOCK_RELATIVE" >&2; exit 1; }

restore_previous_environment() {
  rm -rf -- "$VENV_DIR"
  if [[ -d "$BACKUP_DIR" ]]; then
    mv "$BACKUP_DIR" "$VENV_DIR"
  fi
}
interrupted() {
  restore_previous_environment
  exit 130
}
trap interrupted HUP INT TERM

if [[ -e "$VENV_DIR" ]]; then
  mv "$VENV_DIR" "$BACKUP_DIR"
fi

printf '[specforge-eval] root: %s\n' "$ROOT_DIR"
printf '[specforge-eval] bootstrap_python: %s\n' "$PYTHON_BIN"
printf '[specforge-eval] venv: %s\n' "$VENV_RELATIVE"
printf '[specforge-eval] requirements: %s\n' "$LOCK_RELATIVE"

build_environment() {
  "$PYTHON_BIN" -m venv "$VENV_DIR" || return
  "$VENV_DIR/bin/python" -m pip install --disable-pip-version-check pip==26.1.2 || return
  "$VENV_DIR/bin/python" -m pip install --disable-pip-version-check -r "$LOCK_PATH" || return
  diff -u <(grep -Ev '^(#|$)' "$LOCK_PATH") <("$VENV_DIR/bin/python" -m pip freeze --all) || return
  "$VENV_DIR/bin/python" -c 'import cryptography, pdfplumber' || return
  first_line="$(head -n 1 "$VENV_DIR/bin/pip")" || return
  [[ "$first_line" == "#!$VENV_DIR/bin/python" ]]
}

if build_environment; then
  :
else
  status=$?
  printf 'error: eval rebuild failed; restoring the previous environment\n' >&2
  restore_previous_environment
  trap - HUP INT TERM
  exit "$status"
fi

rm -rf -- "$BACKUP_DIR"
trap - HUP INT TERM
printf '[specforge-eval] bootstrap complete\n'
