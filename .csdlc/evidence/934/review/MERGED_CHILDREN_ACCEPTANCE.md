# Sprint 8 #934 — merged-child acceptance review preparation

Status: **partial sprint review; three merged children reviewed, #910 pending**.
This packet is an acceptance-review handoff, not umbrella closure, release
approval or a finding that Observatory deployment has succeeded.

## Findings first

No new actionable acceptance gap was identified in the bounded merged #720,
#908 and #909 surfaces inspected below. #910 remains open and must supply its
actual deployed posture, uploaded-content hashes/headers, browser HTTPS/WSS,
authentication, rollback proof and final independent review before Sprint 8 can
receive a complete acceptance recommendation. Prepared infrastructure, a saved
create plan and mocked browser tests do not satisfy that deployment obligation.

The following truth boundaries remain material:

- #720 proves live-only product routing and failure behavior. Its local Chrome
  regression intercepts network traffic and replaces WebSocket; it is not
  public-site connectivity/TLS/authentication proof.
- #908 is a timestamped scoped census. All ownership remains frozen-unknown;
  hashed identifiers and purpose hints do not establish application ownership.
  Missing historical identifiers do not prove deletion. Its 24-hour freshness
  validator does not make this historical packet permanently current.
- #909 is a completed planning deliverable with actual plans. Its platform state
  is an explicitly authorized reconstructed private candidate, not recovered
  authoritative final state or approved adoption. No cloud apply, remote state
  upload or backend migration was performed. `planning_complete: true` and
  `apply_ready: false` preserve that distinction.
- Historical validation files that say CI pending describe their prepublication
  capture. Live merged PR observations and durable native terminal receipts
  supersede them for integration truth. They must not trigger postmerge tracked
  card edits or a second closeout PR.

## Exact delivery and closeout

| Child | Reviewed PR head | Merge commit | Live observation |
| --- | --- | --- | --- |
| #720 / [PR #939](https://github.com/agent-logic/agent-design-language/pull/939) | `9b469bdf5efb9d4786b612897696c78d0b9fb7bb` | `9c583cec78d396527798e592082595f316c67ce2` | Issue closed; PR merged 2026-09-12 01:22:09 UTC |
| #908 / [PR #940](https://github.com/agent-logic/agent-design-language/pull/940) | `8108b9b42260c0ccc2470ff4327796370d366349` | `2c09eca135808fb6385a6613f7317088b01ffeab` | Issue closed; PR merged 2026-09-12 00:29:57 UTC |
| #909 / [PR #949](https://github.com/agent-logic/agent-design-language/pull/949) | `fd734c660b8a687fcc93aa6867f3ef801923e503` | `52027fe7b6a5b84379f338378aaed4e658cfa7ea` | Issue closed; PR merged 2026-09-12 01:49:17 UTC |

Owned implementation/evidence paths were compared between each exact reviewed
head and its merge commit: no differences. Native issue-specific terminal
receipts have disposition `closed_out` and matching PR/head identity for all
three. Durable receipt locations and state digests are in `observations.json`.
#934 and #910 were observed open; no whole-sprint closeout is claimed.

## #720 acceptance map — actual live-only implementation

Source acceptance: `.csdlc/issues/720/cards/sip.values.json`, captured issue body.
Immutable merged implementation:
[app.js](https://github.com/agent-logic/agent-design-language/blob/9c583cec78d396527798e592082595f316c67ce2/demos/html-observatory/app.js),
[index.html](https://github.com/agent-logic/agent-design-language/blob/9c583cec78d396527798e592082595f316c67ce2/demos/html-observatory/index.html).

| AC | Implementation/proof inspected | Assessment |
| --- | --- | --- |
| 1. No Published/Retained mode controls | Index mode controls; `live_only.test.mjs` rejects historical control selectors; browser test attempts `?mode=retained` and asserts live selection | Satisfied for live product controls |
| 2. Startup/navigation cannot substitute historical API telemetry | `renderLiveError` preserves last live snapshot on failed live fallback; startup uses `FALLBACK_PACKET`, not a fetched historical packet; browser request ledger rejects historical API/packet reads during startup, outage and navigation | Satisfied; historical integration/report evidence remains separately labelled |
| 3. No obsolete retained three-second timer | Retained refresh function/timer paths removed. Browser interval ledger rejects retained timer and any 3000ms timer in selected v3 product path. Legacy live polling is not relabelled retained behavior | Satisfied for selected live path |
| 4. Remove unused status assignments | Diff removes published/live orphan constants; recorded source reconciliation notes first obsolete assignment/function already absent in baseline | Satisfied without inventing deletion of nonexistent source |
| 5. Focused functional/no-retained tests | Two targeted node tests plus local browser live startup, offline startup, disconnect preserving count seven, stale banner, navigation and no historical requests/timers; generation guard prevents stale failed fallback from overwriting a newer connection | Satisfied locally; not deployed proof |
| 6. Historical/live docs distinction | Merged README distinguishes immutable evidence from live product; historical docs left intact | Satisfied |

Test source inspected:
[live_only.test.mjs](https://github.com/agent-logic/agent-design-language/blob/9c583cec78d396527798e592082595f316c67ce2/demos/html-observatory/tests/live_only.test.mjs),
[live_only.browser.mjs](https://github.com/agent-logic/agent-design-language/blob/9c583cec78d396527798e592082595f316c67ce2/demos/html-observatory/tests/live_only.browser.mjs).
The browser test uses meaningful failure transitions and observable DOM/request
assertions rather than only checking strings. The string assertions remain
supplementary. Recorded validation: 24 node tests, focused shell contracts,
headless Chrome regression and integrated shell proof passed. This review did
not rerun those suites; it inspected their implementation and retained results,
plus prior independent review in this sprint session. No new broad suite was
needed for this unchanged merged source.

## #908 acceptance map — actual AWS collector and delta

Immutable source:
[inventory.py](https://github.com/agent-logic/agent-design-language/blob/2c09eca135808fb6385a6613f7317088b01ffeab/.csdlc/evidence/908/inventory.py),
[test_inventory.py](https://github.com/agent-logic/agent-design-language/blob/2c09eca135808fb6385a6613f7317088b01ffeab/.csdlc/evidence/908/test_inventory.py),
[maintenance](https://github.com/agent-logic/agent-design-language/blob/2c09eca135808fb6385a6613f7317088b01ffeab/.csdlc/evidence/908/MAINTENANCE.md).

| AC | Implementation/proof inspected | Assessment |
| --- | --- | --- |
| 1. Verified business identity, no personal/default or public account ID | Collector strips ambient AWS overrides, fixes `agent-logic-admin`, checks live STS equality to approved #484 before resource queries; public packet retains boolean identity proof | Satisfied for captured run; no raw identity copied into this review |
| 2. Current scoped census/delta including SCR/model staleness | Dynamic enabled-region discovery; four global plus nine per-region families; S3 location/tag/object metadata. Packet has 157 surfaces, 17 enabled regions, 13 buckets; zero census failures and explicit tag outcomes | Satisfied within declared scope; not all AWS services, versions or regions |
| 3. Preserve ownership/failure/absence distinctions | `make_delta` has explicit read-failed/not-surveyed branches and `deletion_proven: false`; resources default frozen-unknown; dates/purpose hints do not authorize deletion | Satisfied; no ownership upgrade inferred |
| 4. Maintenance and meaningful negatives | Weekly/after-change cadence, new issue-bound capture, 24-hour evidence freshness, 10,000-object bound and partial marker, independent review requirement. Tests mutate wrong identity, missing surface/bucket, old time, raw/numeric IDs, delta and failure status; reject mutation API | Satisfied; 13 focused tests recorded passed |
| 5. Independent actual-readback review/no mutation | Prior exact-head independent review checked actual sanitized census/delta, baseline preservation, privacy and metadata outcomes; no cloud mutation route exercised | Satisfied for this census, not future inventory freshness |

The implementation's API allowlist is checked before subprocess dispatch;
responses remain in memory and only selected hashed references/counts/times
are persisted. CloudFront collection requires actual Items rather than treating
malformed missing collections as empty. Validator recomputes the full delta,
requires unique complete surface denominator, compares source and baseline
hashes, checks freshness and rejects unsafe string/numeric identifiers.

Capture completed `2026-09-12T00:21:06.735126+00:00`. This review reverified all
146 historical baseline file hashes without cloud calls. Five `NoSuchTagSet`
metadata outcomes are not bucket absence; all 13 object listings were complete
within the bound in the accepted packet. Old/new missing-reference differences
remain observations rather than creation/deletion authority. Recorded tests were
not rerun solely to repeat previously accepted proof.

## #909 acceptance map — actual three-package plan and custody

Immutable packet:
[MOVE_IN_PACKET](https://github.com/agent-logic/agent-design-language/blob/52027fe7b6a5b84379f338378aaed4e658cfa7ea/.csdlc/evidence/909/MOVE_IN_PACKET.md),
[application checklist](https://github.com/agent-logic/agent-design-language/blob/52027fe7b6a5b84379f338378aaed4e658cfa7ea/.csdlc/evidence/909/APPLICATION_CHECKLIST.md),
[recovery receipt](https://github.com/agent-logic/agent-design-language/blob/52027fe7b6a5b84379f338378aaed4e658cfa7ea/.csdlc/evidence/909/local-recovery-receipt.json).

| AC | Implementation/proof inspected | Assessment |
| --- | --- | --- |
| 1. Company hierarchy/billing/ownership boundaries | Explicit company account/host, folder/organization/billing readbacks; 20 effective org policies, 113 quotas per project, selected foundation/bucket/IAM and preserved POC boundary | Satisfied for captured metadata; no personal context |
| 2. Reuse reviewed Terraform and actual plans | 18 unchanged source/lock digests; nine static checks; bootstrap two no-ops, organization five creates, platform 20 no-ops | Satisfied after explicit operator scope expansion for exactly three private local imports; original issue prohibition was not silently bypassed |
| 3. Exact future application/rollback | No apply or cloud rollback for bootstrap/platform no-ops; exact organization three memberships/budget/empty dataset creation and partial rollback; future identity, state custody/adoption, fresh-plan and approval prerequisites | Satisfied as planning; no application authority |
| 4. Billing/cleanup controls and residuals | Daniel named approval/billing/custody contact; issue492-filtered USD20 budget explicitly not whole-estate cap; dataset not export activation; existing data/versions retained; no launch or POC changes | Satisfied as explicit controls/procedure, not deployed billing controls |
| 5. Independent complete-packet review | Independent #908 reviewer verified actual binary plans/state, three import receipts, all20 pre/post values and refresh-only drift; parent reviewed final metadata corrections | Satisfied; reviewer here reuses that independence because this reviewer implemented #909 |

Saved platform plan SHA256
`c1b26a420740315a3723194b36f29fec5c29b5f300cf3815db8bf4a7695759f3`;
reconstructed serial36 candidate SHA256
`056ae60722abd96b5102604c7241d36e2b8782c6576758164bf5a6dc47d15af4`.
Eight plan refresh entries are null-to-empty normalization, IAM etag and bucket
update timestamps; all20 proposed configuration actions are no-op. Pre/post
resource readbacks were exactly equal and planning left candidate bytes unchanged.

Before native cleanup, 124 private files were copied to stable Git-common custody
with matching source/destination hashes and restrictive permissions; all hashes
were rechecked before and after removal of the issue worktree. Private material
is not copied into this public review. Its manifest labels reconstructed evidence,
not authoritative adoption. Original state-loss cause remains unproven.

## Validation lanes, metrics and quality boundary

| Lane | Actual review/proof | Limits |
| --- | --- | --- |
| Gap/acceptance | All six #720 and five each #908/#909 obligations mapped above | #910 outstanding; no complete sprint verdict |
| Code | Actual UI startup/error/generation/timer paths and AWS dispatcher/projector/delta/validator inspected | No new cloud queries or broad code audit |
| Tests | Test source/failure assertions and retained run results inspected | This preparation turn reran no suites |
| Docs/privacy | Current application/maintenance distinctions, bounded projections and private custody inspected | Private raw state/credentials not published |
| Closeout | Live GitHub issue/PR state, exact merge/head parity and native terminal receipts checked | Prepublication card prose remains historical |
| CI | #939 path/tooling/demo/aggregation checks succeeded; #940/#949 path and aggregation checks succeeded | Other runtime/Rust/coverage execution lanes skipped by path policy |
| Metrics | No new line/branch coverage percentage or release-wide denominator measured | Numerical coverage N/A; green `adl-coverage` aggregation is not coverage measurement |
| Synthesis/quality | Findings, source map, proof provenance and residuals kept separate | Final #910 evidence and full sprint recommendation still required |

No issue, PR, milestone, card or cloud state was mutated by this review. Output
is confined to `.csdlc/evidence/934/review/` in the bound #934 worktree. Parent
owns integration of this partial review and final #910 acceptance synthesis.
