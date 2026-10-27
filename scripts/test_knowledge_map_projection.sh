#!/usr/bin/env bash
# Repository-local integration tests for the portable Knowledge Map projection-set bundle. Native implementation:
# xtask/src/checks/knowledge_map_projection.rs (RUSTDOC-AND-XTASK.15.f); the kit commands stay external.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"
cd "$ROOT"
exec cargo xtask check knowledge-map-projection-tests "$@"
