# Structured Output Record

Template: 1.0.0

Issue: 665

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired pre-PR review findings for #665. Adoption now requires an exact expected repository matching the issue authority, successful adoption writes durable `.csdlc/issues/<issue>/adoption.v1.json` evidence with resulting generation and digest, and gate5 proves an adopted emergency worktree can continue through typed execution, implemented state, exact review recording, and publication-review readiness evaluation without bypassing ordinary gates.

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

## Validation

[]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
