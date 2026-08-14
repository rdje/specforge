# Behavioral held-out initial run — fail-closed diagnostic

- Date: `2026-08-14`
- Owner: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii`
- Selection boundary: frozen 17-row prospective partition from the 24-row behavioral population
- Production revision: `5dd1302a7fbb21519d9b428f177f7a681c9639df`
- Initial machine evidence: commit `32f044d82bff35e546b16cc0ff78c4fc07915816`,
  `doctrine/production_genericity/behavioral_holdout_evidence.json`, SHA-256
  `4a05ec3dd7112897679088f9220af785c70caf354441c3342369c6cdc17c6d68`

## Result

All 51 declared relation/document attempts executed. The first aggregate is a diagnostic checkpoint, not a
qualification result:

| Relation | Pass | Fail | Unmeasurable | Invalid | Descriptive 95% Wilson interval |
| --- | ---: | ---: | ---: | ---: | ---: |
| unchanged PDF replay | 17 | 0 | 0 | 0 | `0.815682–1.000000` |
| adversarial document identity | 0 | 17 | 0 | 0 | `0.000000–0.184318` |
| normalized-text symbol alpha | 0 | 15 | 1 | 1 | `0.000000–0.203883` |

The interval denominator is pass plus fail; invalid and unmeasurable attempts are published separately. These
intervals describe this frozen, non-random population only. It contains five vendors, 11 document families,
three layouts, and four represented categories. The absent prospective `cpu-isa` and `register-ip` categories
remain explicit zero-denominator unmeasurable strata.

## Root cause

Every adversarial-identity failure is confined to the same two SourceIR values:
`/source/stable_artifact_stem` and
`/proof_context/field_premises/source/stable_artifact_stem`. The value is derived from the input filename, so it
belongs to the contract's declared document/artifact identity allowance. The comparator normalized document key
and filename but omitted this separately stored derivative. The one-page calibration did not expose the defect
because its baseline stem already matched its normalized document key.

The alpha failures do not yet demonstrate production coupling. The conformance transform built its authority by
combining typed signal declarations with every schema actor, state, transition, member, base, and module value.
It then replaced all matching source tokens globally. Real held-out documents expose role-bearing English such as
`device`, `controller`, `source`, `target`, `receiver`, and `host`; renaming those words changes meaning and can
remove semantic actors. The comparison correctly reports the resulting deltas, but the transform is not an
intent-preserving alpha relation. The I2C document additionally reaches a non-bijective derived-id comparison
with this polluted catalog. Its honest opaque catalog is the six explicitly declared bus signals `SCL`, `SDA`,
`USCL`, `USDA`, `SCLH`, and `SDAH`. The OpenCAPI AFU row is independently `vacuous_baseline`.

## Evidence integrity

The report contains exactly 17 documents, 51 attempts, and 90 relation/stratum rows. It accounts for 999,551
baseline proof claims, 993,116 transformed proof claims, 23,825,688 compared leaf values, and all 117 declared
and observed deltas. It contains no absolute path. The executable contract checker reconstructs split identity,
novelty, attempt coverage, aggregate counts, stratum denominators, dispositions, and Wilson limits; its 14/14
mutation suite rejects attempt omission, calibration/holdout overlap, state laundering, interval drift, absolute
paths, tool drift, and the original frozen-contract faults.

Held-out labels, results, intervals, and thresholds did not enter `specforge-core`. The defects are in the
conformance oracle and are retained as failed evidence rather than converted into favorable outcomes.

## Tracked remediation

The same active leaf owns the repair; there is no task-tree pivot:

1. normalize only the declared identity-derived stable artifact stem and retain a semantic-mutation rejection;
2. derive alpha authority exclusively from typed opaque declarations;
3. classify an absent eligible symbol surface as its own unmeasurable disposition;
4. reuse retained repository-local raw pairs where that preserves execution identity, rerunning only what the
   corrected transform changes;
5. regenerate and independently validate the aggregate before any held-out qualification claim.

The initial evidence remains durable in Git history so the repair cannot erase or silently reclassify this
checkpoint. The [corrected qualification](behavioral-held-out-qualification.md) records the subsequent
34-pass / one-fail / 16-unmeasurable / zero-invalid result and the real I2C production coupling it exposed.
