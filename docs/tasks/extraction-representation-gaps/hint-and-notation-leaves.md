# EXTRACTION-REPRESENTATION-GAPS — hint and notation leaves

- Part ID: `hint-and-notation-leaves`
- State: `active`

Open ERG leaves continue here, after `register-binding-leaves` neared its rollover (`2026-10-20`): the roles a prose
hint gives a signal, and the sentences that define the document's own notation. `.50` and `.51` moved here whole.

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.50`
  Status: `open` (`2026-10-20`; CODE; found by `.3.d.iii.b`)
  Goal: **a prose hint gives a signal the role its sentence gives it.** The prose hint reader
  (`synthesize_signal_semantic_hints_from_prose`, source facts only) tags a signal from the words near it. When
  `.3.d.iii.b` returned 534 misfiled timing statements to their own classes it read them for the first time, and of
  its 9 new hints 6 give a role the sentence does not: TMC's `ATREADYM`/`ATREADYS` -> `handshake_valid_like` from *If
  there is valid data, that is, ATVALID HIGH, the data was accepted this cycle* (the valid signal is another named
  one), AHB's `HREADY` -> `handshake_valid_like` from *provide valid data in the final cycle of the transfer, as
  indicated by HREADY HIGH*, Wishbone's `RTY_I`/`RTY_O` -> `handshake_ready_like` from *indicates that the interface is
  not ready to accept or send data* (a negated readiness: a retry), and Wishbone's `DAT_O` (already tagged from another
  sentence) from *presents valid data on its [DAT_O()] lines* (the data the validity qualifies). **Reproduce:**
  `signal_semantic_hints` of those three chains, `statement_1727`/`1751`, `statement_0733`, `statement_0398`/`0412`,
  `statement_1232`. **Impact:** interfaces' signal records carry a handshake role the document does not give (TMC's
  table tags were already wrong and are now `cross_modality`); no transaction, temporal record or `.isf` text moves
  today. Acceptance: tag a signal valid- or ready-like only when the sentence makes that signal the bearer of the
  validity or readiness (not the data it qualifies, not another named signal, not a negated readiness), and measure
  every chain's hints and interfaces.

- ID: `EXTRACTION-REPRESENTATION-GAPS.51`
  Status: `open` (`2026-10-20`; CODE; found by `.3.d.iii.d`)
  Goal: **a sentence defining the document's own notation is apparatus.** ADIv6's appendix E3 (*Pseudocode
  Definition*) explains the pseudocode language the document uses, and its sentences are read as protocol content:
  `.3.d.iii.d` read *The block of lines consisting of elsif and its indented statements is optional* as an optional
  feature (`option_0006`), and *This statement indicates a special case that replaces the behavior defined by the
  current pseudocode* is classed `normative_statement` five times and `derived_rule` once. **Reproduce:** statements
  on the 31 chains not classed `apparatus` that name *pseudocode*, *elsif*, *indented statements* or *indentation*:
  55, 46 of them ADIv6's (39 `source_fact`, 6 `normative_statement`, 1 `derived_rule`), AXI 6, ACE 2, CHI C2C 1.
  **Impact:** a notation rule becomes a requirement or an option the hardware is held to, and an obligation reader
  can take it. Acceptance: tell a section that defines the document's notation (pseudocode, typographic or
  numbering conventions) from one that uses it, by its heading and its own words rather than a document's name, file
  its sentences as apparatus, keep a pseudocode function that describes hardware behavior (*The pseudocode function
  EnterDormantState describes the function of the JTAG-to-DS sequence detector*) where it is, and measure every chain.
