#!/usr/bin/env bash
# Reproduce sf-tool-issue-bundle-foreign-git-head from the FSMGen repository root:
#   bash fsmgen-issue-sf-tool-issue-bundle-foreign-git-head/commands.sh
# It needs only git, perl and tar. REPRO_WORKDIR (default: a fresh mktemp directory) holds the scratch
# enclosing repository; nothing is written inside the FSMGen checkout.
set -u

BUNDLE_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
FSMGEN_ROOT=$(pwd)
OUT="$BUNDLE_DIR/observed/command-logs"
mkdir -p "$OUT"
WORK=${REPRO_WORKDIR:-$(mktemp -d)}
rm -rf "$WORK/enclosing"
mkdir -p "$WORK/enclosing/fsmgen"

# 1. The FSMGen revision under test.
git -C "$FSMGEN_ROOT" rev-parse HEAD > "$OUT/fsmgen-actual-head.txt"

# 2. An unrelated git repository with one commit of its own.
git -C "$WORK/enclosing" init -q
git -C "$WORK/enclosing" -c user.email=repro@example.invalid -c user.name=repro \
  commit -q --allow-empty -m "unrelated enclosing repository"
git -C "$WORK/enclosing" rev-parse HEAD > "$OUT/enclosing-repo-head.txt"

# 3. A non-git copy of FSMGen inside it (a release tarball or `git archive` export has this shape).
git -C "$FSMGEN_ROOT" archive HEAD README.md bin perl fsm isf ppif vial plugin \
  | tar -x -C "$WORK/enclosing/fsmgen"

# 4. Capture a bundle from that copy with the documented helper.
cp "$BUNDLE_DIR/sources/fsmgen-input/tiny.isf" "$WORK/enclosing/tiny.isf"
( cd "$WORK/enclosing/fsmgen" \
  && ./bin/fsmgen-issue-bundle --case ../tiny.isf --issue-id helper-head --no-probes \
       -- --strict --check --json ) > "$OUT/helper.stdout" 2> "$OUT/helper.stderr"
echo $? > "$OUT/helper.exit"

# 5. What the helper recorded as FSMGen's revision.
cp "$WORK/enclosing/fsmgen/fsmgen-issue-helper-head/observed/command-logs/fsmgen-git-head.txt" \
  "$OUT/recorded-fsmgen-head.txt"
grep '^FSMGen git HEAD:' "$WORK/enclosing/fsmgen/fsmgen-issue-helper-head/env.txt" \
  > "$OUT/recorded-env-head.txt"

if cmp -s "$OUT/recorded-fsmgen-head.txt" "$OUT/enclosing-repo-head.txt"; then
  echo "REPRODUCED: the bundle records the enclosing repository's commit as FSMGen's revision" \
    | tee "$OUT/verdict.txt"
else
  echo "NOT REPRODUCED: recorded head differs from the enclosing repository's commit" \
    | tee "$OUT/verdict.txt"
fi
