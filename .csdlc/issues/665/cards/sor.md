# Structured Output Record

Template: 1.0.0

Issue: 665

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Removed obsolete failed-adoption cleanup that could delete an existing target `.csdlc/issues/<issue>/adoption.v1.json` after a failed bind. Adoption evidence is staged inside the atomic issue projection replacement, so failed adoption now leaves any pre-existing regular evidence file untouched; focused regression coverage proves the preservation behavior.

## Artifacts

- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate5.rs
- docs/tooling/EMERGENCY_BRANCH_ADOPTION.md
- .csdlc/prepared/issues/665/validate-emergency-adoption-scope.sh
- .csdlc/prepared/issues/665/validate-emergency-adoption-docs.sh
- .csdlc/issues/665/index.json
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/tests/card_identity.rs
- csdlc-v2/tests/code_repository_migration.rs
- csdlc-v2/tests/gate10a.rs
- csdlc-v2/tests/gate2.rs
- csdlc-v2/tests/gate4.rs
- csdlc-v2/tests/issue_330_bridge_cleanup_defect.rs
- csdlc-v2/tests/projection_recovery_integration.rs
- docs/tooling/EMERGENCY_BRANCH_ADOPTION.md
- .csdlc/prepared/issues/665/record-review-finding-repairs.json
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/prepared/issues/665/record-atomic-adoption-repairs.json
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/prepared/issues/665/record-failed-adoption-preservation.json

## Execution

- Extended `BindRequest` and `BindResult` with explicit adoption authority and result fields while preserving ordinary bind-create behavior.
- Added fail-closed adoption validation for ready/unbound issue state, stale generation/digest, full expected HEAD, non-csdlc-bind actor, registered branch/worktree uniqueness, worktree branch, HEAD match, base ancestry, and clean target worktree.
- Recorded adoption evidence in the bound issue audit and result surface without claiming implementation, review, publication, merge readiness, or closeout.
- Rejected implicit adoption through ordinary bind when the requested worktree already exists and is not the current bound issue-local worktree.
- Added focused positive and negative gate5 regression coverage for exact-head adoption, stale digest rejection, dirty target rejection, explicit authority requirement, and ordinary bind-create preservation.
- Added operator documentation and issue-owned validators for the emergency adoption sequence and stop conditions.
- Added `expected_repository` to `BindRequest` and fail-closed validation against the ready issue record repository.
- Changed successful adoption result evidence from an audit-only reference to `.csdlc/issues/<issue>/adoption.v1.json` and added `resulting_digest` to `BindResult`.
- Recorded adoption evidence with expected repository, observed HEAD, resulting lifecycle generation, and resulting lifecycle digest.
- Expanded focused gate5 adoption coverage with wrong-repository rejection and downstream implemented/reviewed/publication-review readiness proof for adopted worktrees.
- Updated emergency adoption operator documentation to describe the expected repository and durable adoption evidence contract.
- Reloaded and revalidated the issue record under the binding and issue locks before bind diagnosis and topology work.
- Revalidated adoption authority against the materialized target record immediately before mutating it.
- Added Store support for issue-directory extra files inside the atomic projection staging swap.
- Moved adoption evidence into the staged projection replacement so successful adoption writes evidence and record truth together.
- Added focused regression coverage for stale adoption request freshness after source-record mutation and for replacing a conflicting evidence path with the atomic adoption evidence file.
- Removed the post-error `remove_file` cleanup for adoption evidence from `bind_issue`.
- Added a focused regression where a clean worktree has a tracked pre-existing adoption evidence file and a staging blocker; failed adoption returns an I/O error while preserving the existing file and ready/unbound record.
- Retained the staged atomic evidence path for successful adoption.

## Validation

[]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
