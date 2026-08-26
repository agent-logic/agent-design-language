# Structured Task Prompt

Template: 1.0.0

Issue: 330

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

One production bridge-fed cleanup/recovery invariant defect; no broad projection recovery or cleanup redesign.

## Deliverables

- Narrow production repair in projection recovery/cleanup
- csdlc-v2/tests/issue_330_bridge_cleanup_defect.rs
- .csdlc/prepared/issues/330/validate_preparation_bundle.py
- Passing #300 bridge-fed projection_recovery_integration target after #330 terminal ancestry
- Strict Clippy, fmt/diff, typed validate/doctor evidence
- Fresh exact-head review and ready PR
- Terminal finish if hosted gates pass

## Acceptance

1. AC-1: A completed recovery whose rejected archive was removed by exact-authority cleanup remains valid for ordinary typed commits only when completed cleanup receipt and canonical/archive manifest authority prove the same recovery result.
2. AC-2: Missing, forged, mismatched, stale, or cross-operation cleanup authority still fails closed and never becomes a blanket bypass for retained recovery validation.
3. AC-3: A real final cleanup receipt appearing after early shortcut and before pre-final validation is rejected unless the whole operation ledger and final receipt chain match current request authority.
4. AC-4: Rejected final-receipt races preserve byte-exact ledger, namespace, receipt, and archived-node state.
5. AC-5: #300 bridge-fed integration tests pass without synthetic cleanup authority and without weakening mechanical matrix invocation.
6. AC-6: Fresh exact-head review records no actionable findings before publication.

## Dependencies

- #297 terminal/ancestral via PR #328 merge 5ebd2143e3f36638b16f6153446eff655116f76a
- #300 blocked-proof checkpoint a5253b9866f88aeeeff083c3d0a8e16c4fbdafd7

## Inputs

- agent-logic/agent-design-language#330
- .csdlc/evidence/300/bridge-fed-r2/projection-recovery-integration-failed.log
- csdlc-v2/tests/projection_recovery_integration.rs
- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/projection_cleanup.rs

## Non Goals

- Publishing or reviewing #300 before #330 terminal ancestry
- Broad lifecycle/GitHub/store redesign
- Weakening #299 cleanup authority checks
- Mutating #297 terminal state
