# Issue #283 design: ADR 0065 ACIP evidence reconciliation

## Intent

Issue #283 is an evidence-reconciliation child of #207. It determines whether ADR 0065 has terminal, landed, artifact-bound ACIP schema catalog / governed projection evidence that is strong enough for the later #288 serialization step to represent truthfully.

## Evidence authority

- Historical baseline: `.csdlc/evidence/5832/acip-native-receipts.json` and native subdirectory receipts remain immutable prior evidence, but are not enough by themselves because later records identify #5832 as superseded/insufficient.
- Replacement authority candidate: issue #209 / PR #215, including live merged PR state, derived terminal cache `.git/csdlc-v2/derived-terminal/209.json`, local validation manifest `.csdlc/evidence/209/local-validation-manifest.json`, native validation manifest `.csdlc/evidence/209/native-validation-manifest.json`, and attached non-empty log/JSON artifacts.
- ADR authority boundary: #283 may record an issue-local reconciliation packet and residual gaps for #207. It must not move ADR 0065 to Accepted and must not serialize the shared ADR index, plan, manifest, or review packet; #288 owns those shared documents after #283-#287 are complete.

## Implementation map

1. Verify live #209 / PR #215 closure, merge commit, exact head, and issue closure linkage.
2. Verify the derived terminal cache for #209 binds repository, issue, PR, head, merge, disposition, and digest.
3. Verify the #209 local/native validation manifests are non-empty, exact-revision-bound, and reference non-empty artifact files.
4. Verify the #5832 historical evidence remains present but is classified as superseded input, not terminal promotion evidence.
5. Write issue-local #283 evidence summarizing promotion readiness, residual gaps, and #288 handoff language.

## Validation

Validation is evidence/documentation focused:

- `jq` sanity checks over `.git/csdlc-v2/derived-terminal/209.json`, `.csdlc/evidence/209/local-validation-manifest.json`, `.csdlc/evidence/209/native-validation-manifest.json`, and `.csdlc/evidence/5832/acip-native-receipts.json`.
- non-empty file checks for each referenced #209 and #5832 artifact.
- `git diff --check`.
- C-SDLC validation/doctor over #283 after cards and evidence are recorded.

