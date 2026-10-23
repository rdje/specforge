#!/usr/bin/env bash
# SpecForge adapter for the RUSTDOC doctrine (RUSTDOC-AND-XTASK.2).
#
# The API reference is the workspace's rustdoc, so a doc comment rustdoc cannot resolve is a broken
# page. CI ran `cargo doc --workspace --no-deps` under `-D warnings`, but only at push cadence, and one
# unresolved intra-doc link (`[CLK_I]`, quoted from Wishbone) broke it for four weeks without a commit
# noticing (RUSTDOC-AND-XTASK.1). This gate runs the same build at commit cadence.
#
# Self-test first: a throwaway crate whose doc comment links to nothing must fail the same command,
# so the gate's own negative is re-proven on every run (DOCTRINE_ENFORCEMENT.md §3). Offline, no
# dependencies; the scratch crate lives under the repository's project-data root.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"
[ "$#" -eq 0 ] || { printf 'Usage: %s   (this gate takes no arguments)\n' "$0" >&2; exit 2; }
cd "$ROOT"

flags="-D warnings${RUSTDOCFLAGS:+ $RUSTDOCFLAGS}"

work="$(mktemp -d "${TMPDIR:-$ROOT/.project-data/tmp}/rustdoc-selftest.XXXXXX")"
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/src"
printf '[package]\nname = "rustdoc_selftest"\nversion = "0.0.0"\nedition = "2021"\n\n[workspace]\n' > "$work/Cargo.toml"
printf '/// Links to [`missing_item`], which does not exist.\npub fn documented() {}\n' > "$work/src/lib.rs"
if RUSTDOCFLAGS="$flags" CARGO_TARGET_DIR="$work/target" cargo doc --offline --quiet --no-deps \
    --manifest-path "$work/Cargo.toml" > "$work/doc.log" 2>&1; then
  printf 'rustdoc: FAIL — the self-test crate with an unresolved link documented cleanly, so the gate cannot see one\n' >&2
  exit 1
fi
grep -q 'unresolved link' "$work/doc.log" || {
  printf 'rustdoc: FAIL — the self-test crate failed for another reason:\n' >&2
  cat "$work/doc.log" >&2
  exit 1
}

if ! RUSTDOCFLAGS="$flags" cargo doc --offline --locked --quiet --workspace --no-deps > "$work/workspace.log" 2>&1; then
  cat "$work/workspace.log" >&2
  printf 'rustdoc: FAIL — cargo doc --workspace --no-deps reports a rustdoc warning (see above)\n' >&2
  exit 1
fi
printf 'rustdoc: PASS — the self-test crate'"'"'s unresolved link failed as it must, and the workspace documents cleanly\n'
