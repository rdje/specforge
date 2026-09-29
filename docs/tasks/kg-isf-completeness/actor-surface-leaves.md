# KG-ISF-COMPLETENESS — actor surface leaves

- Part ID: `actor-surface-leaves`
- State: `active`

New leaves of the actor surface (which phrase is an agent, and what a relation's actor is) continue here, after
`relation-precision` reached its size warning (`.1g`, `2026-09-29`).

## Post-migration records

- ID: `KG-ISF-COMPLETENESS.1g` · Status: `open` (`2026-09-29`; MEASUREMENT then CODE; opened by
  `EXTRACTION-REPRESENTATION-GAPS.3.c.ii`) · Goal: **a register, a field or a bit is not an actor.** ADIv6's *This bit
  controls the CSYSPWRUPREQ signal* makes `bit` drive the wire once that wire is known. Across the 78 persisted chains
  105 relations name an actor whose head noun is a storage or value word: `register` 18, `field` 12, `CSW register` 8,
  `maximum number` 6, `bit` 6, `signal` 5, `register bits` 4, and more. The `.1a` gate rejects a phrase by its leading
  token, never by its head. Measure the class by the head noun structurally (a noun of the register model, a value
  noun), read a sample of each, and decide whether *X register controls S* states something the IR should keep (a
  software-visible control, not a drive) before gating. Verification: per-relation replay diff over the 31; the
  persisted-statement replay over all 78.
