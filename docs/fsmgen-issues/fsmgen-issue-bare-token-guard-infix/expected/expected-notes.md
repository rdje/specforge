# Expected Behavior: Bare Token Guards in ISF Rules

## What SPECFORGE expected

When SPECFORGE emits an ISF rule with a bare symbol token as the guard:

```isf
(rule unconditional_rule true
  (sig_a 1))
```

FSMGen should either:

1. **Accept it** — The ISF parser (`Parser.pm _parse_rule` line 1601) explicitly
   consumes a bare scalar token as the rule guard. This is valid ISF syntax.
   The guard `true` should be treated as "always active" (unconditional rule).
   Similarly, a signal name like `start` should be treated as "when signal is
   asserted."

   The lowering should produce a valid FSM internal form, not an infix
   assignment that strict mode later rejects.

2. **Or reject it cleanly at ISF parse time** — If bare-token guards are not
   actually supported semantics, the ISF parser should reject them with an
   ISF-level diagnostic like "bare token guard 'true' is not a supported guard
   form; use S-expression guards (e.g. (== signal 1) or (& a b))".

## What actually happens (the bug)

The ISF parser accepts the bare token. The lowering produces an FSM internal
form like `(<true <- (sig_a> 1))`. SourceFrontend strict mode then rejects this
as "infix assignment." The diagnostic references an FSM internal form the user
never wrote, making the error indecipherable to downstream tools.

## Evidence that this is an FSMGen bug

- FSMGen's own test fixture `isf/full_featured.isf` also fails strict mode with
  the same infix-assignment error: `<ready <- (valid> 1))`
- Non-strict mode accepts the same ISF without error
- This creates a situation where `--check` passes but `--strict --check` fails
  on standard ISF syntax that the parser itself accepts

## Desired resolution

The FSM internal lowering should handle bare-token guards by emitting proper
S-expression assignment forms that pass strict mode, or the ISF parser should
reject bare-token guards with a clear ISF-level diagnostic.
