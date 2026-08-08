#!/usr/bin/env bash
# Shared project-data locality initializer. Source this from canonical shell entrypoints.

specforge_activate_project_data() {
  local root="${1:?repository root is required}"
  local canonical_root
  local repository_device
  local project_data_path
  local project_data_device
  local canonical_project_data

  canonical_root="$(cd "$root" && pwd -P)"
  if [[ ! -f "$canonical_root/Cargo.toml" || ! -f "$canonical_root/README.md" ]]; then
    printf 'error: project-data root is not a SpecForge repository: %s\n' "$canonical_root" >&2
    return 1
  fi

  export SPECFORGE_REPO_ROOT="$canonical_root"
  export TMPDIR="$canonical_root/.project-data/tmp"
  export TMP="$TMPDIR"
  export TEMP="$TMPDIR"
  export XDG_CACHE_HOME="$canonical_root/.cache/xdg"
  export HF_HOME="$canonical_root/.cache/huggingface"
  export HUGGINGFACE_HUB_CACHE="$HF_HOME/hub"
  export PIP_CACHE_DIR="$canonical_root/.cache/pip"
  export TORCH_HOME="$canonical_root/.cache/torch"
  export MPLCONFIGDIR="$canonical_root/.cache/matplotlib"

  mkdir -p \
    "$TMPDIR" \
    "$XDG_CACHE_HOME" \
    "$HUGGINGFACE_HUB_CACHE" \
    "$PIP_CACHE_DIR" \
    "$TORCH_HOME" \
    "$MPLCONFIGDIR"

  repository_device="$(df -P "$canonical_root" | awk 'END { print $1 }')"
  for project_data_path in \
    "$TMPDIR" \
    "$XDG_CACHE_HOME" \
    "$HUGGINGFACE_HUB_CACHE" \
    "$PIP_CACHE_DIR" \
    "$TORCH_HOME" \
    "$MPLCONFIGDIR"; do
    canonical_project_data="$(cd "$project_data_path" && pwd -P)"
    case "$canonical_project_data" in
      "$canonical_root"/*) ;;
      *)
        printf 'error: project-data path escapes the repository: %s\n' "$project_data_path" >&2
        return 1
        ;;
    esac
    project_data_device="$(df -P "$canonical_project_data" | awk 'END { print $1 }')"
    if [[ "$project_data_device" != "$repository_device" ]]; then
      printf 'error: project-data path is off the repository filesystem: %s\n' \
        "$project_data_path" >&2
      return 1
    fi
  done
}
