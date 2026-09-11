# Issue #520 proof-gap reconciliation

## Gap Analysis Summary

This report reconciles `T520-TEST-002` through `T520-TEST-006` against the
later issue #517 / PR #752 accounting artifacts retained by exact candidate
`c24f8fa65ce445b03ce6cd69007307291d78b60c`. The original five historical
exception groups are all accounted and have no unowned rows, but accounting is
not release proof. The exact-candidate result is:

| Finding | Historical rows | Exact-candidate classification | Current rows requiring proof or review |
| --- | ---: | --- | ---: |
| `T520-TEST-002` retained predecessors | 227 | accounting-resolved but product-unproved | 198 |
| `T520-TEST-003` exact-head review | 7 | partially resolved; still current for the four V3-F rows | 4 |
| `T520-TEST-004` current semantic criteria | 11 | accounting-resolved but product-unproved | 1 |
| `T520-TEST-005` live/spec synchronization | 4 | stale/fully resolved | 0 |
| `T520-TEST-006` shared-path resolution | 3 | stale/fully resolved | 0 |

The historical quality gate remains immutable: 393 inventoried lanes, 366
required lanes, 121 pass, 245 non-proving, five grouped exceptions, decision
`blocked`, and `downstream_unlock: false`. The later reconciliation maps all
245 historical non-proving rows exactly once, but expressly does not recalculate
them into release passes or authorize release.

## Scope

- Expected baseline: the five blocker groups recorded in
  `docs/milestones/v0.92.1/evidence/release/tail-01/blockers.json` and restated
  as `T520-TEST-002` through `T520-TEST-006`.
- Observed evidence: the candidate-retained issue #517 reconciliation packet,
  including `census.json`, `exception-dispositions.json`,
  `current-exceptions.json`, `corrections.json`,
  `retained-corporate-runtime.json`, `retained-v3.json`,
  `release-stage-mapping.json`, `accounting-closeout.md`, and
  `validation-final.json`.
- Candidate binding: every cited artifact was read from detached checkout
  `c24f8fa65ce445b03ce6cd69007307291d78b60c`. The five original groups are
  historical assessments of candidate `bf159eb416950dfa3399933829726a7b7e71f897`;
  no later artifact silently changes that identity.
- Exclusions: no live cloud run, product mutation, GitHub mutation, release
  approval, or attempt to manufacture missing proof.

## Findings

### `T520-TEST-002` — accounting-resolved but product-unproved

The statement that all 227 retained-predecessor rows still have no proof is no
longer exact. PR #752 maps every row to a successor, evidence boundary, or
release stage, but leaves 198 rows without full execution/release proof:

| Source slice | Total | Proven | Accepted amendment | Not applicable at TAIL-01 | Current gate obligation | Non-proving | Source-supported, not execution proof |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Corporate/Runtime retained rows | 60 | 10 | 8 | 0 | 0 | 42 | 0 |
| V3 retained rows | 155 | 0 | 0 | 3 | 0 | 101 | 51 |
| Later release-stage rows | 12 | 0 | 0 | 8 | 4 | 0 | 0 |
| **Total** | **227** | **10** | **8** | **11** | **4** | **143** | **51** |

Accordingly, 143 explicit `non_proving` rows, 51 source-supported rows lacking
execution proof, and four current TAIL-01 obligations remain unproved: 198
rows. Ten rows have bounded proof; eight have accepted scope amendments that do
not claim the deferred work ran; eleven remain obligations at a later release
stage. `exception-dispositions.json` calls the group `successor_accounted` and
explicitly distinguishes direct execution sufficiency from accounting. The
current WBS clarification likewise says an owned exception does not waive a
required proving lane.

This remains a release-level proof gap, but remediation must target the 198
current unproved rows rather than repeat the stale 227-row claim.

### `T520-TEST-003` — partially resolved; four V3-F rows are still current

The reconciliation classifies all seven historical rows as
`review_freshness_resolved`:

- three CORP-A rows (`CORP-A-ac-1` through `CORP-A-ac-3`) received a bounded
  review of the issue #482 public-register delta; the exact-candidate blobs for
  the reviewed denominator and validators remain unchanged;
- four V3-F rows (`V3-F-ac-1` through `V3-F-ac-4`) received a bounded #505
  post-review delta review for the historical candidate.

The three CORP-A rows are stale/fully resolved at the exact candidate. The four
V3-F rows are not: the reconciliation binds
`csdlc-v3/tests/terminal_cleanup_cutover_commands.rs` to blob
`c5d67168df2c923eb77c592b7545386ad0a39234`, while the exact candidate contains
blob `556cb4957407772abfbe135f3e320f1936011c3e`. The later delta changes the
fixture registry version from `1.0.3` to `1.0.4`. That one-line change is also
inside the exact-candidate C-SDLC suite now reported failing by
`T520-TEST-001`, and the PR #752 review contract is explicitly docs-accounting
only. Therefore the historical seven-row finding is too broad, but exact-head
review/proof remains current for four V3-F rows.

### `T520-TEST-004` — accounting-resolved but one product proof gap remains

All eleven rows have an exact disposition. Their present split is:

- six proven: `DRT-B-ac-4`, `GCP-B-ac-4`, and `OBS-A-ac-1` through
  `OBS-A-ac-4`;
- four accepted amendments: `CORP-B-ac-1` through `CORP-B-ac-3` and
  `OBS-B-ac-2`;
- one still non-proving: `GCP-B-ac-1`.

The proof-artifact blobs named for these corrections all match the exact
candidate. `GCP-B-ac-1` remains current because #740 proves private posture,
versioning, and recovery, but the required audit/log posture has neither audit
configuration nor log readback. The reconciliation explicitly records the
remaining obligation and says reauthentication prevented a current read-only
audit check. The original eleven-row claim is stale; the current product-proof
finding is one row.

### `T520-TEST-005` — stale/fully resolved

The four WP-01, GCP-E, HOT-01, and OBS-B live/spec rows are explicitly marked
`synchronized`. `current-exceptions.json` retains the captured live authority
and required synchronized fields, and the exact-candidate reconciliation
validator accepts those canonical YAML fields. No remaining synchronization
row is recorded. This historical four-row finding should not be carried into
the current findings register.

### `T520-TEST-006` — stale/fully resolved

The three shared surfaces received independent bounded final-content review:

- `infra/aws/account-foundation`
- `infra/aws/runtime`
- `csdlc-v3/src/lib.rs`

Their exact-candidate Git objects are respectively
`749d8e94fe9763020721ccd3ecfe647f722cc43d`,
`285f591dca38cc701f71c21c77281f682b151a4d`, and
`17c7660320811196371aa3acb813d8c76b6862a0`, exactly matching the objects
accepted by `current-exceptions.json`. The reconciliation does not fabricate
historical owner sign-off; it replaces that missing provenance with an
independent exact-content review and records no requirement loss. Because the
reviewed bytes remain the candidate bytes, this proof-debt finding is fully
resolved for the exact candidate.

## Gap Buckets

- `release_blockers`: `T520-TEST-002` as corrected to 198 unproved retained
  rows; `T520-TEST-003` for four V3-F rows, considered together with the
  independently observed failing C-SDLC suite.
- `durable_proof_gaps`: `T520-TEST-004` narrowed to `GCP-B-ac-1`.
- `routed_work`: accepted amendments and later-stage obligations remain with
  their recorded owners; they are not relabeled executed.
- `stale_release_readiness_docs`: any current packet still reporting all 227,
  all seven, all eleven, all four, or all three rows as unresolved without the
  reconciliation above.
- `non_blocking_quality_concerns`: none added by this bounded pass.

## Validation Performed

- Recomputed `census.json` with the retained summarizer: all 245 historical
  non-proving rows mapped, zero unowned rows/groups, classification counts
  `12 amendment / 4 current obligation / 144 non-proving / 11 not-applicable /
  16 proven / 7 review-freshness-resolved / 51 source-supported-not-execution`.
  The recomputed file was byte-identical to the candidate.
- Ran the reconciliation validator: 393-row inventory preserved; eight initial
  corrections accepted structurally; release authorization remained false.
- Compared every proof-artifact blob named by the current-criterion corrections
  with exact candidate `c24f8fa...`; no mismatch was found.
- Compared all three shared-path Git objects with the reconciled candidate
  objects; no mismatch was found.
- Compared the #505 post-review scope with the exact candidate and found the
  single test-fixture blob drift described under `T520-TEST-003`.

## Missing Evidence and Uncertainty

- No new quality-gate recomputation promotes the later exact candidate from the
  immutable historical `blocked` decision. Later terminal receipts and issue
  closure establish lifecycle/accounting truth, not criterion execution.
- The 198-row retained-proof count is a reconciliation classification, not a
  claim that 198 implementations are defective. Many rows may have reusable
  evidence, but candidate-bound execution proof is not retained here.
- This pass did not reproduce paid cloud, GPU, distributed soak, browser, or
  provider operations.
- The four V3-F rows require a current review/proof disposition; this report
  does not decide whether the one-line registry fixture update is the only
  cause of the separately reported ten test failures.

## Recommended Follow-up

1. Replace `T520-TEST-002` with the exact 198-row product-proof deficit and
   preserve the 29 proven/amended/later-stage dispositions.
2. Keep `T520-TEST-003` only for the four V3-F rows; bind a fresh review and a
   passing exact-candidate suite to the current test blob.
3. Replace `T520-TEST-004` with the single `GCP-B-ac-1` audit/log proof gap.
4. Drop `T520-TEST-005` and `T520-TEST-006` from current actionable findings,
   retaining their reconciliation as audit history.

## Artifact Routing

This report is a specialist input to the issue #520 synthesis. It neither
updates the historical quality gate nor creates or authorizes remediation.

## Stop Boundary

- Fixed gaps: no.
- Created issues: no.
- Created pull requests: no.
- Approved closeout: no.
- Approved release: no.
- Product or GitHub state mutated: no.
- The only write is this requested issue #520 specialist report.
