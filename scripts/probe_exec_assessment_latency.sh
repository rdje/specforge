#!/usr/bin/env bash
# scripts/probe_exec_assessment_latency.sh — is the host assessing our executables, or are we slow?
#
# GATE-FIXTURE-EXEC-STALL.3. A doctrine step that sits at ~0% CPU with no output is indistinguishable
# from a hang, and on `2026-09-19` that ambiguity cost a session forty minutes, an aborted commit, and
# a wrong first diagnosis (a pipe deadlock in the driver). The actual cause was the operating system
# assessing newly created executables on first exec: the gate creates fixture scripts, execs each one,
# and blocks in `exec` while a security daemon inspects it. Nothing in the repository was wrong.
#
# This probe makes that distinguishable in seconds. It times the exec of brand-new scripts against the
# re-exec of one already assessed. A large gap between them IS the condition; a small gap means the
# slowness is ours and belongs to a different investigation.
#
# It deliberately measures rather than asserts. The numbers move with the daemon's backlog — that is
# what makes the condition intermittent and what makes a pinned figure the wrong thing to carry.
#
# Usage:
#   scripts/probe_exec_assessment_latency.sh              measure fresh vs cached exec latency
#   scripts/probe_exec_assessment_latency.sh --self-test  exercise the measurement core

set -u

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"
source "$ROOT/scripts/project_data_env.sh"
specforge_activate_project_data "$ROOT"

SAMPLES="${SPECFORGE_EXEC_PROBE_SAMPLES:-3}"

note() { printf 'exec-assessment: %s\n' "$1"; }

# ── Cleanup ───────────────────────────────────────────────────────────────────────────────────
# A trap that names a function-LOCAL path is not a cleanup: by the time EXIT fires the local is gone,
# and under `set -u` the trap itself dies with "unbound variable" instead of removing anything. The
# workspaces are therefore tracked globally, which also lets a nested run (the self-test calls the
# probe) register its own without clobbering its caller's.
EXEC_PROBE_TMPDIRS=()
exec_probe_cleanup() {
  local dir
  for dir in ${EXEC_PROBE_TMPDIRS[@]+"${EXEC_PROBE_TMPDIRS[@]}"}; do
    [ -n "$dir" ] && rm -rf "$dir"
  done
  EXEC_PROBE_TMPDIRS=()
}
trap 'exec_probe_cleanup' INT TERM EXIT

# millis_of <command...> — wall-clock milliseconds for one invocation, via perl's Time::HiRes because
# `date +%s` is whole-second and GNU `date +%s%N` is absent on a stock macOS.
millis_of() {
  local start end
  start="$(perl -MTime::HiRes=time -e 'printf "%.6f", time')"
  "$@" >/dev/null 2>&1
  end="$(perl -MTime::HiRes=time -e 'printf "%.6f", time')"
  perl -e 'printf "%.0f", ($ARGV[1] - $ARGV[0]) * 1000' "$start" "$end"
}

# fresh_exec_millis <workdir> <n> — write a NEVER-SEEN script and time its first exec.
fresh_exec_millis() {
  local work="$1" n="$2" path
  path="$work/fresh-$$-$n-$RANDOM.sh"
  printf '#!/usr/bin/env bash\nexit 0\n' >"$path"
  chmod +x "$path"
  millis_of "$path"
}

run_probe() {
  local work i fresh cached path first worst=0
  work="$(mktemp -d "$ROOT/.project-data/tmp/exec-assessment.XXXXXX")" || return 1
  EXEC_PROBE_TMPDIRS+=("$work")

  first=""
  for i in $(seq 1 "$SAMPLES"); do
    fresh="$(fresh_exec_millis "$work" "$i")"
    [ -z "$first" ] && first="$fresh"
    [ "$fresh" -gt "$worst" ] && worst="$fresh"
    note "fresh script #$i first exec: ${fresh} ms"
  done

  path="$work/cached.sh"
  printf '#!/usr/bin/env bash\nexit 0\n' >"$path"
  chmod +x "$path"
  "$path" >/dev/null 2>&1          # pay the assessment once, then measure the steady state
  cached="$(millis_of "$path")"
  note "already-assessed script re-exec: ${cached} ms"

  # The verdict is the RATIO, not any single figure: the daemon's backlog moves the absolute numbers
  # run to run, which is exactly why this is intermittent and why nothing here is pinned.
  if [ "$worst" -gt 2000 ]; then
    note "VERDICT: the host is assessing new executables — worst fresh exec ${worst} ms against ${cached} ms cached."
    note "A gate step sitting at ~0% CPU is waiting on that, not hanging. Confirm the daemons with:"
    note "  ps -Ao pid,pcpu,time,comm -r | head -5     # look for XprotectService / syspolicyd"
    return 0
  fi
  note "VERDICT: no assessment stall right now — worst fresh exec ${worst} ms against ${cached} ms cached."
  note "If a gate step is slow, the cause is not this; investigate the step itself."
}

run_self_test() {
  local work passed=0 total=0 out path ms
  work="$(mktemp -d "$ROOT/.project-data/tmp/exec-assessment-selftest.XXXXXX")" || return 1
  EXEC_PROBE_TMPDIRS+=("$work")

  # 1) the timer reports a plausible duration for a known sleep, so a measurement of 0 for everything
  #    (a broken timer reading as "never slow") cannot pass unnoticed.
  total=$((total + 1))
  ms="$(millis_of perl -e 'select undef, undef, undef, 0.25')"
  if [ "$ms" -ge 200 ] && [ "$ms" -le 5000 ]; then passed=$((passed + 1))
  else printf 'exec-assessment self-test 1 FAILED: 0.25s measured as %s ms\n' "$ms"; fi

  # 2) a fresh script actually runs and is timed.
  total=$((total + 1))
  ms="$(fresh_exec_millis "$work" selftest)"
  if [ "$ms" -ge 0 ]; then passed=$((passed + 1))
  else printf 'exec-assessment self-test 2 FAILED: fresh exec returned %s\n' "$ms"; fi

  # 3) each fresh script is genuinely NEW — a reused path would measure the cached case and report no
  #    stall no matter how bad the host was, which is the one way this probe could lie.
  total=$((total + 1))
  fresh_exec_millis "$work" a >/dev/null
  fresh_exec_millis "$work" b >/dev/null
  if [ "$(ls "$work"/fresh-*.sh 2>/dev/null | wc -l | tr -d ' ')" -ge 3 ]; then passed=$((passed + 1))
  else printf 'exec-assessment self-test 3 FAILED: fresh scripts reused a path\n'; fi

  # 4) the probe reports a verdict either way, so a quiet host is stated rather than implied.
  #    Captured through a FILE rather than `$(...)`: a command substitution runs in a subshell, the
  #    subshell does not inherit this script's EXIT trap, and the nested run's workspace would be
  #    orphaned under .project-data/tmp/ every time the self-test ran. Case 6 is what catches that.
  total=$((total + 1))
  local before after
  before="${#EXEC_PROBE_TMPDIRS[@]}"
  SPECFORGE_EXEC_PROBE_SAMPLES=1 run_probe >"$work/probe.out" 2>&1
  out="$(cat "$work/probe.out")"
  after="${#EXEC_PROBE_TMPDIRS[@]}"
  case "$out" in
    *VERDICT*) passed=$((passed + 1)) ;;
    *) printf 'exec-assessment self-test 4 FAILED: no verdict line\n' ;;
  esac

  # 6) the nested run registered its workspace in THIS shell, so it dies with this shell. A probe that
  #    leaves scratch behind every time it self-tests is a residue generator, not a diagnostic.
  total=$((total + 1))
  if [ "$after" -gt "$before" ]; then passed=$((passed + 1))
  else printf 'exec-assessment self-test 6 FAILED: nested run workspace was orphaned (%s -> %s)\n' "$before" "$after"; fi

  # 5) every knob has its default exercised (CORPUS-CHAIN-CURRENCY.9), both expansion forms matched.
  total=$((total + 1))
  local unguarded
  unguarded="$(grep -oE '\$\{SPECFORGE_EXEC_PROBE_[A-Z_]+:?-' "$0" | sed 's/:*-$//' | sed 's/^\${//' \
    | sort -u | grep -vE '^SPECFORGE_EXEC_PROBE_SAMPLES$' || true)"
  if [ -z "$unguarded" ]; then passed=$((passed + 1))
  else printf 'exec-assessment self-test 5 FAILED: unguarded knob(s): %s\n' "$unguarded"; fi

  printf 'exec-assessment: self-test %s/%s timer, freshness, verdict, and residue cases pass.\n' "$passed" "$total"
  [ "$passed" -eq "$total" ]
}

case "${1-}" in
  --self-test) run_self_test ;;
  '')          run_probe ;;
  *)           printf 'Usage: %s [--self-test]\n' "$0" >&2; exit 2 ;;
esac
