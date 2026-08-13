#!/usr/bin/env bash

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"

cd "$ROOT"
exec cargo run --quiet --locked --offline -p specforge-production-graph -- --root "$ROOT"
