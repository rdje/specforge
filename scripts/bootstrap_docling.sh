#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
VENV_DIR="${SPECFORGE_DOCLING_VENV_DIR:-$ROOT_DIR/.venv-docling}"
DOCLING_REQUIREMENT="${SPECFORGE_DOCLING_REQUIREMENT:-docling==2.84.0}"
BOOTSTRAP_PYTHON="${SPECFORGE_DOCLING_BOOTSTRAP_PYTHON:-}"

pick_python() {
  if [[ -n "$BOOTSTRAP_PYTHON" ]]; then
    if command -v "$BOOTSTRAP_PYTHON" >/dev/null 2>&1; then
      printf '%s\n' "$BOOTSTRAP_PYTHON"
      return 0
    fi
    printf 'error: SPECFORGE_DOCLING_BOOTSTRAP_PYTHON=%s is not executable on PATH\n' "$BOOTSTRAP_PYTHON" >&2
    return 1
  fi

  local candidate
  for candidate in python3.11 python3.12 python3.10 python3 python; do
    if command -v "$candidate" >/dev/null 2>&1; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done

  printf 'error: no usable Python bootstrap interpreter found (tried python3.11, python3.12, python3.10, python3, python)\n' >&2
  return 1
}

PYTHON_BIN="$(pick_python)"

echo "[specforge-docling] root: $ROOT_DIR"
echo "[specforge-docling] bootstrap_python: $PYTHON_BIN"
echo "[specforge-docling] venv: $VENV_DIR"
echo "[specforge-docling] requirement: $DOCLING_REQUIREMENT"

"$PYTHON_BIN" -m venv "$VENV_DIR"
"$VENV_DIR/bin/python" -m pip install --upgrade pip setuptools wheel
"$VENV_DIR/bin/python" -m pip install "$DOCLING_REQUIREMENT"

"$VENV_DIR/bin/python" - <<'PY'
import sys
from importlib import metadata
import docling

print(f"[specforge-docling] ready_python: {sys.executable}")
print(f"[specforge-docling] docling_version: {metadata.version('docling')}")
PY

echo "[specforge-docling] bootstrap complete"
echo "[specforge-docling] specforge will auto-discover $VENV_DIR/bin/python when run from this repository"
echo "[specforge-docling] optional explicit override: export SPECFORGE_DOCLING_PYTHON=\"$VENV_DIR/bin/python\""
