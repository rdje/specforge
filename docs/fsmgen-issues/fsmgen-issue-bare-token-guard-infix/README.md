# FSMGen Issue Bundle: bare-token-guard-infix

Title: bare-token-guard-infix
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/bare_guard_repro.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/bare_guard_repro.isf
Failure class: rejected-input
FSMGen commit or capability-manifest producer commit: see observed/command-logs/fsmgen-git-head.txt and observed/json/fsmgen-capability-manifest.json
SPECFORGE version/commit: 2c4819c002f23dbce4058c14afa9ca0f0a5f484f
Expected behavior: FSMGen should accept ISF rules with a bare symbol token as a guard (e.g. (rule name true (sig 1)) or (rule name some_signal (sig 1))). The ISF parser accepts these forms — they are valid ISF syntax. Per Parser.pm _parse_rule, a bare scalar token is consumed as the rule guard. FSMGen should either: (a) accept bare-token guards and emit them as always-active rules with a guard condition wired to that token, or (b) reject them at ISF parse time with a clear ISF-level diagnostic, not lower them to FSM internal form and then reject them as 'infix assignment' in SourceFrontend strict mode.
Observed behavior: FSMGen strict mode rejects bare-token guard rules as 'infix assignment'. The ISF parser (Parser.pm _parse_rule) accepts the bare token and consumes it as the guard, but the lowering produces an FSM internal form like (<true <- (sig_a> 1)) which SourceFrontend.pm strict mode then rejects. The error message references an FSM internal form the user never wrote, making it impossible for downstream tools to understand what went wrong. Non-strict mode accepts the same ISF without error. FSMGen's own test fixture full_featured.isf also fails strict mode with the same infix-assignment error.
First failing command or API call: original
Exit status or exception: see observed/command-logs/original.exit
Is the attached artifact bundle minimized: unknown
Does the minimized/redacted bundle still reproduce: unknown
