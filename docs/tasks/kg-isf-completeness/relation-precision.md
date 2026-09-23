# KG-ISF-COMPLETENESS — relation precision

- Part ID: `relation-precision`
- State: `active`

## Post-migration record

- ID: `KG-ISF-COMPLETENESS.1d` · Status: `open` (`2026-09-23`; MEASUREMENT then CODE; opened by
  `.5.xiii.a.5.a`) · Goal: **a phrase is not an actor.** The actor-identity gate (`.1a`) admits prose
  fragments as relation subjects: Wishbone B4, once its names became matchable, publishes 9 relations
  whose actors are `interfaces so`, `similar manner`, `same operation`, `bus cycle`, `DAT_I signal`,
  `cases SLAVE`; ADIv6 publishes `details reads TDO` from *"For details about the Read TDO (RTDO) bit,
  see …"*. This is the class `.1c.ii` measured on eMMC (109 multi-word actors) and deferred as having no
  clean within-document structural gate — **its trigger is met**: it now reaches a rebuildable document.
  Measure corpus-wide which actor strings are neither a declared actor, a role noun the document uses as
  a subject elsewhere, nor a component the document names, and read a sample before choosing a gate.
  Verification: census over all 78 chains; replay over the 27 with a per-relation diff; WIRE-BASED-100
  with both binaries; the Wishbone 41 re-read.

- ID: `KG-ISF-COMPLETENESS.1e` · Status: `open` (`2026-09-23`; MEASUREMENT then CODE; opened by
  `.5.xiii.a.5.a`) · Goal: **a relation's direction comes from the verb that governs the signal, not from
  co-occurrence.** Wishbone publishes *"The MASTER latches [DAT_I()] and negates its [STB_O] signal in
  response to [ACK_I]"* as `MASTER reads STB_O` and `MASTER drives ACK_I` — the opposite of both — and
  `SLAVE drives STB_O` / `SLAVE drives STB_I` / `MASTER reads STB_I` from sentences where the other port
  acts. 7 of Wishbone's 41 relations have the wrong direction. Measure the class on the golds' own
  relation items first (they score direction as `drives`), then over the corpus.
  Verification: per-relation replay diff; WIRE-BASED-100 relation scores with both binaries.
