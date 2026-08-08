#!/usr/bin/env bash
# Enforce the repository-owned README landing-page contract on the resulting tree.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"
README="$ROOT/README.md"
POLICY="$ROOT/README_POLICY.md"
ROUTES="$ROOT/doctrine/readme_entrypoint/routed_destinations.tsv"
LINE_CAP=150
BYTE_CAP=5800
ROUTE_RECORD_CAP=40
ROUTE_BYTE_CAP=5000
ROUTE_RECORD_BYTE_CAP=192

within_limits() {
  local lines="$1"
  local bytes="$2"
  [ "$lines" -le "$LINE_CAP" ] && [ "$bytes" -le "$BYTE_CAP" ]
}

route_registry_within_limits() {
  local records="$1"
  local bytes="$2"
  local record_bytes="$3"
  [ "$records" -le "$ROUTE_RECORD_CAP" ] &&
    [ "$bytes" -le "$ROUTE_BYTE_CAP" ] &&
    [ "$record_bytes" -le "$ROUTE_RECORD_BYTE_CAP" ]
}

target_shape_is_allowed() {
  case "$1" in
    https://github.com/rdje/specforge/issues|git:history) return 0 ;;
    /*|../*|*/../*|*\\*) return 1 ;;
    *) return 0 ;;
  esac
}

control_is_known() {
  case "$1" in
    scripts/*.sh|knowledge-map/scripts/*.sh) return 0 ;;
    review_on_behavior_change|review_on_workflow_change|review_on_tooling_change) return 0 ;;
    github_issue_retention_and_query|query_with_git_log) return 0 ;;
    *) return 1 ;;
  esac
}

lifecycle_is_known() {
  case "$1" in
    hot_live|bounded_collection_index|partitioned_collection|bounded_pointer) return 0 ;;
    generated_projection|maintained_reference|maintained_entrypoint|maintained_standard) return 0 ;;
    executable_reference|executable_check|data_registry|external_service|append_only_history) return 0 ;;
    *) return 1 ;;
  esac
}

if [ "${1:-}" = "--self-test" ]; then
  within_limits "$LINE_CAP" "$BYTE_CAP" || {
    printf 'readme-policy self-test: exact ceilings must pass\n' >&2
    exit 1
  }
  if within_limits "$((LINE_CAP + 1))" "$BYTE_CAP"; then
    printf 'readme-policy self-test: line ceiling did not fail closed\n' >&2
    exit 1
  fi
  if within_limits "$LINE_CAP" "$((BYTE_CAP + 1))"; then
    printf 'readme-policy self-test: byte ceiling did not fail closed\n' >&2
    exit 1
  fi
  route_registry_within_limits "$ROUTE_RECORD_CAP" "$ROUTE_BYTE_CAP" "$ROUTE_RECORD_BYTE_CAP" || {
    printf 'readme-policy self-test: exact route-registry ceilings must pass\n' >&2
    exit 1
  }
  if route_registry_within_limits "$((ROUTE_RECORD_CAP + 1))" "$ROUTE_BYTE_CAP" "$ROUTE_RECORD_BYTE_CAP"; then
    printf 'readme-policy self-test: route-record ceiling did not fail closed\n' >&2
    exit 1
  fi
  if route_registry_within_limits "$ROUTE_RECORD_CAP" "$((ROUTE_BYTE_CAP + 1))" "$ROUTE_RECORD_BYTE_CAP"; then
    printf 'readme-policy self-test: route-registry byte ceiling did not fail closed\n' >&2
    exit 1
  fi
  if route_registry_within_limits "$ROUTE_RECORD_CAP" "$ROUTE_BYTE_CAP" "$((ROUTE_RECORD_BYTE_CAP + 1))"; then
    printf 'readme-policy self-test: route-record byte ceiling did not fail closed\n' >&2
    exit 1
  fi
  target_shape_is_allowed 'README.md' || {
    printf 'readme-policy self-test: repository-relative target was rejected\n' >&2
    exit 1
  }
  if target_shape_is_allowed '/tmp/off-volume.md' || target_shape_is_allowed '../escape.md'; then
    printf 'readme-policy self-test: off-repository target did not fail closed\n' >&2
    exit 1
  fi
  if control_is_known 'unreviewed_control'; then
    printf 'readme-policy self-test: unknown pressure control did not fail closed\n' >&2
    exit 1
  fi
  if control_is_known 'transition_debt:LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5'; then
    printf 'readme-policy self-test: retired transition debt did not fail closed\n' >&2
    exit 1
  fi
  if lifecycle_is_known 'unclassified_lifecycle'; then
    printf 'readme-policy self-test: unknown lifecycle did not fail closed\n' >&2
    exit 1
  fi
  if awk -F '\t' 'NR > 1 && $1 == "__self_test_missing_route__" && $2 == "reader_navigation" { found=1 } END { exit !found }' "$ROUTES"; then
    printf 'readme-policy self-test: missing reader route did not fail closed\n' >&2
    exit 1
  fi
  printf 'readme-policy self-test: caps, path locality, controls, and route coverage fail closed\n'
  exit 0
fi

fail=0
problem() {
  printf 'readme-policy: %s\n' "$1" >&2
  fail=1
}

if [ ! -f "$README" ]; then
  problem 'README.md is missing; restore the project landing page.'
else
  lines="$(wc -l < "$README" | tr -d ' ')"
  bytes="$(wc -c < "$README" | tr -d ' ')"
  [ "$lines" -le "$LINE_CAP" ] || problem "README.md has ${lines} lines; the reviewed ceiling is ${LINE_CAP}. Route changing detail through README_POLICY.md."
  [ "$bytes" -le "$BYTE_CAP" ] || problem "README.md has ${bytes} bytes; the reviewed ceiling is ${BYTE_CAP}. Route changing detail through README_POLICY.md."
fi

if [ ! -f "$POLICY" ]; then
  problem 'README_POLICY.md is missing; restore the repository-owned policy.'
else
  grep -Fq 'README-POLICY-LOCAL-ADOPTION:BEGIN' "$POLICY" || problem 'README_POLICY.md lacks its fenced local adoption note.'
  grep -Fq 'Independent ceilings are 150 lines and 5,800' "$POLICY" || problem 'README_POLICY.md and the enforced line/byte ceilings disagree.'
fi

expected_header=$'target\troute_kind\towner\tlifecycle\tpressure_control\tterminal'
if [ ! -f "$ROUTES" ]; then
  problem 'doctrine/readme_entrypoint/routed_destinations.tsv is missing; restore the governed route inventory.'
else
  header="$(sed -n '1p' "$ROUTES")"
  [ "$header" = "$expected_header" ] || problem 'the README route registry header does not match its six-column schema.'
  route_records="$(awk 'END { print (NR > 0 ? NR - 1 : 0) }' "$ROUTES")"
  route_bytes="$(wc -c < "$ROUTES" | tr -d ' ')"
  route_record_bytes="$(LC_ALL=C awk '{ sub(/\r$/, ""); if (length($0) > maximum) maximum=length($0) } END { print maximum + 0 }' "$ROUTES")"
  [ "$route_records" -le "$ROUTE_RECORD_CAP" ] || problem "the README route registry has ${route_records} records; the reviewed ceiling is ${ROUTE_RECORD_CAP}."
  [ "$route_bytes" -le "$ROUTE_BYTE_CAP" ] || problem "the README route registry has ${route_bytes} bytes; the reviewed ceiling is ${ROUTE_BYTE_CAP}."
  [ "$route_record_bytes" -le "$ROUTE_RECORD_BYTE_CAP" ] || problem "the README route registry has a ${route_record_bytes}-byte raw record; the reviewed ceiling is ${ROUTE_RECORD_BYTE_CAP}."

  seen_keys=''
  route_count=0
  while IFS=$'\t' read -r target route_kind owner lifecycle control terminal extra; do
    route_count=$((route_count + 1))
    if [ -z "$target" ] || [ -z "$route_kind" ] || [ -z "$owner" ] || [ -z "$lifecycle" ] || [ -z "$control" ] || [ -z "$terminal" ] || [ -n "${extra:-}" ]; then
      problem "route registry row $((route_count + 1)) is incomplete or has extra columns."
      continue
    fi
    case "$route_kind" in
      reader_navigation|author_overflow) ;;
      *) problem "route registry row $((route_count + 1)) has unknown route kind '${route_kind}'." ;;
    esac
    lifecycle_is_known "$lifecycle" || problem "route '${target}' has unknown lifecycle '${lifecycle}'."
    [ "$terminal" = 'yes' ] || problem "route '${target}' is not a controlled terminal; route chains and cycles are not allowed in this registry."

    key="${route_kind}|${target}"
    if printf '%s' "$seen_keys" | grep -Fqx "$key"; then
      problem "route '${target}' is duplicated for '${route_kind}'."
    else
      seen_keys="${seen_keys}${key}"$'\n'
    fi

    if ! target_shape_is_allowed "$target"; then
      problem "route '${target}' is not repository-relative."
    else
      case "$target" in
        https://github.com/rdje/specforge/issues|git:history) ;;
        *) [ -e "$ROOT/${target%/}" ] || problem "route '${target}' does not resolve inside the repository." ;;
      esac
    fi

    control_is_known "$control" || problem "route '${target}' has unknown pressure control '${control}'."
    case "$control" in
      scripts/*.sh|knowledge-map/scripts/*.sh)
        [ -x "$ROOT/$control" ] || problem "route '${target}' names missing or non-executable control '${control}'."
        ;;
      review_on_behavior_change|review_on_workflow_change|review_on_tooling_change|github_issue_retention_and_query|query_with_git_log) ;;
    esac
  done < <(sed '1d' "$ROUTES")
  [ "$route_count" -gt 0 ] || problem 'the README route registry has no route records.'

  if [ -f "$README" ]; then
    while IFS= read -r link; do
      target="${link#](}"
      target="${target%)}"
      target="${target%%#*}"
      [ -n "$target" ] || continue
      awk -F '\t' -v wanted="$target" 'NR > 1 && $1 == wanted && $2 == "reader_navigation" { found=1 } END { exit !found }' "$ROUTES" || problem "README reader route '${target}' is not registered as reader_navigation."
    done < <(grep -oE '\]\([^)]+\)' "$README" || true)
  fi

  for target in docs/book/src/SUMMARY.md ROADMAP.md docs/tasks/ docs/decisions/ git:history docs/book/src/reference/troubleshooting.md TOOLBOX.md; do
    awk -F '\t' -v wanted="$target" 'NR > 1 && $1 == wanted && $2 == "author_overflow" { found=1 } END { exit !found }' "$ROUTES" || problem "author-overflow route '${target}' is missing from the registry."
  done
fi

if [ "$fail" -ne 0 ]; then
  printf 'readme-policy: FAILED — repair README.md, README_POLICY.md, or the registered route/control instead of widening a ceiling.\n' >&2
  exit 1
fi

printf 'readme-policy: README.md and all registered entrypoint routes satisfy the local contract.\n'
