# Structured Output Record

Template: 1.0.0

Issue: 407

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

#407 adds a narrow typed implemented-phase SIP Goal recovery operation guarded by review-recovery provenance, with focused regression proof for #286-style repair and fail-closed broad SIP mutation rejection.

## Artifacts

- csdlc-v2/src/cards.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/issues/407
- .csdlc/prepared/issues/407

## Execution

- Added SemanticOperation::CorrectGoalAfterRecovery as a SIP-only recovery operation.
- Authorized the operation only for implemented issues with immediate typed review-recovery provenance and cleared downstream lifecycle truth.
- Recorded structured audit evidence with previous/new goal and recovery generation/sequence.
- Added focused gate5 regression covering recovered repair, unrecovered rejection, stale generation rejection, retained terminal rejection, and broad SIP mutation rejection.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "recovered_implemented_issue_can_correct_only_the_sip_goal",
      "--test",
      "gate5"
    ],
    "purpose": "Prove recovered implemented SIP Goal repair and fail-closed guard coverage.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2-focused.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
