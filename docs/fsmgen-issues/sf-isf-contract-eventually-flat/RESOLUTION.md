# RESOLUTION — sf-isf-contract-eventually-flat (F1)

Status: **RESOLVED upstream** (2026-05-18).

FSMGEN fixed the flat `(contract n (eventually signal within N))` strict
rejection in commit `610cb26e ISF-SPECFORGE-REPORTED-STAGE-CONTRACT-BUGS.1:
accept flat eventual contracts` (tracked under FSMGEN's own
`ISF-SPECFORGE-REPORTED-STAGE-CONTRACT-BUGS` tree, `a60cc1ab`). The related
"strict reject exits 255 with no JSON despite `--json`" surface was fixed in
`9bfb9a20 …STAGE-CONTRACT-BUGS.3: emit ISF check JSON failures`.

SPECFORGE bumped the `subs/fsmgen` pin `effe591d → 9bfb9a20`
(`FSMGEN-SUBMODULE-BUMP.1`) and **empirically verified** on the new binary:
running this bundle's `sources/fsmgen-input/f1-contract-eventually-flat.isf`
through `./bin/fsmgen --strict --check --json` now yields `success: true`
with `diagnostic_count: 0` and a populated JSON document on stdout (the
original capture under `observed/` is exit 255 / empty stdout on the old
`effe591d` binary, kept intact as the historical reproduction).

No SPECFORGE code change is required: SPECFORGE already emits the nested
`(eventually s (within N))` form, which remained strict-valid throughout.
This bundle is retained as the archived reproduction; the `observed/`
capture is intentionally NOT rewritten.
