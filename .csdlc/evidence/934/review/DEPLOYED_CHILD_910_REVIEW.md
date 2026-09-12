# Bounded deployed-child #910 review

Review status: **interim; not final exact-head acceptance**. Reviewer: delegated Sprint 8 #909 agent. No cloud calls or mutations were performed for this review. No #910 files were modified.

Inspected checkout HEAD: `953cc5896b01a18f539ea04864e45e9b4a0f8ce1`. The deployed evidence and verifier were uncommitted when reviewed; the table below binds this review to file bytes rather than claiming they were contained in HEAD. Review manifest captured at 2026-09-12T02:19:40.265810+00:00. A newly present `browser-failure.json` was not inspected in this bounded review and is excluded from its conclusions.

## Findings and fix gates

1. **Browser acceptance pending (AC4/6).** No successful real Chrome acceptance receipt was available. HTTPS content fetches and the origin probe's 101 response do not establish that the Observatory renders live Runtime data. `verify-deployed.cjs` requires a connected UI, received WebSocket frames, disabled unauthenticated writes, no page errors and no write requests. Its frame count alone does not prove an accepted live telemetry payload; final proof should retain/assert the accepted Runtime schema and source/live-data state. Preserve any actual failure; do not claim authenticated write execution from public-read/disabled-control proof.
2. **Final invalidation correlation not independently established (AC2/6).** `upload-invalidation.json` says Completed with the correct three paths. The inspected private `invalidation.json` is the initial create response with InProgress. Retain or identify the final exact-ID `get-invalidation` readback and its sanitized correlation to the initial operation. This is an evidence gap, not a finding that the invalidation failed.
3. **Current documentation is stale.** `DEPLOYMENT_PLAN.md` lines 19, 35 and 37 still describe the origin 403/deployment pause and performed steps as pending. Mark these as historical preparation or replace them with current receipts and the genuine remaining browser gate before final publication.

## Verified observations

- Live source issue #910 acceptance was read; this review maps to application, authenticated posture, browser and rollback requirements, not just preparation-card wording.
- Private `apply.log` says 18 added, zero changed and zero destroyed. The preserved Terraform state contains 18 managed resources; its SHA-256 matches `deployed-posture.json`.
- The posture projection records Deployed/enabled CloudFront, exact target alias and A/AAAA aliases, ISSUED ACM certificate, TLSv1.2_2021, HTTPS redirect, always-signed SigV4 S3 OAC, private/versioned site bucket, log public-access blocks and 90-day expiration, logging enabled without cookies, and explicit CSP/security headers. The actual readback producer uses the approved business AWS profile. Its hard-coded action counts were cross-checked against the real apply log and state.
- All four private upload receipts have nonempty VersionIds. Downloaded exact-version bytes match their SHA-256 values. The sanitized upload projection equals those private receipts after removing VersionIds. The upload producer checks each source hash and exact-version response metadata/hash before proceeding, publishes index last, and creates invalidation only after four successes.
- Four HTTPS asset hashes match the reviewed upload manifest; headers match intended immutable hashed assets and uncached index/config. The direct anonymous S3 check records 403.
- The Runtime-origin receipt records the exact added site origin, preserved process identities and file ownership/mode, active configuration hash agreement, health 200 with exact CORS origin, and WSS 101/data. Its explicit boundary says no authentication exercise yet.
- Rollback receipt truthfully describes nonmutating exact-version recovery reconstruction for an initial deployment with no prior deployed release; zero copy operations and zero rollback invalidations. Four exact-version readbacks match; future index/config restore and invalidation are documented, with infrastructure rollback separate.
- No additional asset, privacy or custody correctness finding was identified in the inspected subset. This is not an exhaustive security audit or final deployment acceptance. Prior exact-plan/asset/custody review remains separate preparation evidence.

## Inspected evidence hash manifest

Private receipt contents, account/resource identifiers and credentials are not copied into this review. Helpers below are ignored local files; their hashes record the code inspected, without committing the helpers.

| Path in #910 checkout | State at review | SHA-256 |
| --- | --- | --- |
| `.csdlc/evidence/910/deployed-posture.json` | uncommitted / ignored private helper | `2e841975f68a9888cb2afb20c388e98573d2706dbfbb9ebb9e4820d63c1d7aa8` |
| `.csdlc/evidence/910/upload-invalidation.json` | uncommitted / ignored private helper | `1a65ce9fbb253670f805866c538f5a250a624727f93885fc3b191820eef63216` |
| `.csdlc/evidence/910/runtime-origin-change.json` | uncommitted / ignored private helper | `20b3085873ae05cacd4eed8d19471c6831c463bcb52726f8cad8f70c31a7fbc9` |
| `.csdlc/evidence/910/https-assets.json` | uncommitted / ignored private helper | `389ac65a57615993f3ae65b1140164d579268d62a58a3360920e897d43ab38b8` |
| `.csdlc/evidence/910/rollback-dry-run.json` | uncommitted / ignored private helper | `852fae475e2e3742fc3c518e0f752678525e418d941adbd4ec4eff329a90abc2` |
| `.csdlc/evidence/910/verify-deployed.cjs` | uncommitted / ignored private helper | `acaae1e7cb5182751e95ed12f8729e46bcd7fc41b52821bb98db7061fc0146a7` |
| `.csdlc/evidence/910/upload-manifest.json` | tracked | `d656805f793ecaab8636161b58d73c93ade89241b8aa977ef418a40e00218890` |
| `.csdlc/evidence/910/DEPLOYMENT_PLAN.md` | tracked | `ca99b531ad1755aa0c16b4baf02e42d6a2f431cc16fd4bcc719cb696ecd3a0c0` |
| `.adl/runs/910/readback_posture.py` | uncommitted / ignored private helper | `849ae4d5ed47a54cd8047fa5633ad98f152be53b6a6b83f8511d50083ee43aa7` |
| `.adl/runs/910/upload_verified.py` | uncommitted / ignored private helper | `feda879aacc850699838d9c9311682759f4f03025d7847a87e141ac100a470e1` |

## Handoff boundary

Root and the #910 worker received the findings. Renew review against the final candidate after browser results and the documentary/invalidation evidence gates are resolved. #934 overall review and closeout remain pending; this record does not approve #910 publication, merge or acceptance.

## Follow-up dispositions — 2026-09-12T02:22:20.359371+00:00

Read-only follow-up at the same checkout HEAD, over the following uncommitted corrections. No cloud calls or worker-file edits.

- Finding 2 **resolved**: independently compared initial and final private invalidation receipts. Exact IDs, complete batches and all three paths match; final status is Completed. The published ID SHA-256 and private raw-receipt SHA-256 both recompute correctly. The final response concerns the existing invalidation, not a newly created one.
- Finding 3 **resolved in substance**: both deployment plan and runbook now distinguish historical preparation from executed apply/uploads, replace the stale current-origin rejection with subsequent 200/101 proof, and retain failed/pending browser acceptance. Their references to invalidation reconciliation as open describe the review checkpoint before this follow-up; final consolidation should link this resolved disposition. No repeat-mutation instruction was introduced.
- Finding 1 **remains open**: no successful browser/live-rendered-telemetry acceptance is asserted. This follow-up does not approve final publication or close #910/#934.

| Corrected path | SHA-256 at follow-up |
| --- | --- |
| `.csdlc/evidence/910/DEPLOYMENT_PLAN.md` | `9d6edcc66ca101dcf5d717ef53b57adde53693331d59ec93896b5576969cc9c3` |
| `.csdlc/evidence/910/EXECUTION_RUNBOOK.md` | `eb375ac13470a38198615c48051e0d073b80525ad36cf92e34c024548aab0f90` |
| `.csdlc/evidence/910/upload-invalidation.json` | `d5e63af99a9953ad451e6020639d7ba3fa031a56496153e034c04f327eabdadf` |

Private final receipt SHA-256: `12f45ec408caefe32eb7c0c5663ab41e5dbc3540f0cf21c0d5af55965f03b42c`. Private identity values are not reproduced.

## Candidate d037039 review

Exact clean candidate `d03703999ce5c100c3070c5e7c7de547edb3018e` independently inspected. The committed browser verifier and result resolve finding 1: real Chrome loaded the documented `?runtime=v3&live=1` URL, received `adl.runtime_v3.observatory_feed.v3` over WSS, matched rendered agent count 3 to an actual frame, showed connected WebSocket state, hid the stale banner, and kept unauthenticated write controls disabled with zero write requests/page errors. No network interception is used. Temporary origin-scoped local-network permission and operator-local loopback reachability are explicitly bounded; authenticated write execution and remote-client reachability remain non-claims. The stale deployment/invalidation documentation has been consolidated correctly. No new cloud/browser calls by reviewer.

**New P2 metadata finding:** SOR retains contradictory preparation-only fields: required artifacts says deployed proof pending, sandbox invariants say no cloud writes, final PVF lane is read-only preflight, and several verification fields still say deployed proof pending. Worker was asked to normalize through native edits to actual deployed/browser, nondeterministic proof and authorized bounded cloud writes while preserving unknown metrics. Final exact-head acceptance remains pending that correction.

## Final implementation-candidate renewal

Exact metadata successor `28fbf649a258a89509fbd56a024ecff43c06e0b2` reviewed over `d03703999ce5c100c3070c5e7c7de547edb3018e`: **PASS, no actionable findings remain**. The eight-file native metadata tail corrects the SOR artifact, approved-cloud-write, final nondeterministic PVF, verification, ordering and privacy statements. Unknown metrics remain unknown and Runtime replay is not claimed. Deployment/source/browser evidence is unchanged. All findings in this review are resolved for the bounded implementation candidate.

This supports native review/publication of #910. It is not a claim of PR publication, green CI, merge, terminal closeout or overall #934 completion; those require subsequent exact remote/lifecycle observations. Operator-local browser permission/reachability and no authenticated-write execution remain explicit proof limits.
