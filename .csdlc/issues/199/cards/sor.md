# Structured Output Record

Template: 1.0.0

Issue: 199

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and review-remediated governed membership coordination. Authorization now binds the exact old and target stable maps plus target membership before Raft effects; history reconciliation requires joint/final entries newer than the authority operation; proof v4 replaces the misleading name facade with twelve behavior-specific cases and seven production assertions.

## Artifacts

- .csdlc/evidence/199/v4/execution-proof.json
- .csdlc/prepared/issues/199/produce-proof-receipt.rb
- .csdlc/prepared/issues/199/validate-proof-receipt.rb

## Execution

- Bound caller transition inputs byte-for-byte to the sealed PromoteVoter stable-map and target-membership digests before add_learner or change_membership
- Rejected retained membership-history entries at or before the active authority log index and required final history after the current joint entry
- Replaced thirty-six substituted public names with twelve behavior-specific public cases
- Expanded coordinator proof to seven cases covering authorized target binding and stale-history denial
- Retained v4 exact proof after strict library and integration Clippy

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/199/produce-proof-receipt.rb"
    ],
    "purpose": "Produce exact review-remediated evidence with behavior-specific and production-assertion denominators",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/199/v4/execution-proof.json"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/199/validate-proof-receipt.rb"
    ],
    "purpose": "Validate exact argv, 12 public cases, seven production assertions, protected source, immutable introduction, and current-main ancestry",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/199/v4/execution-proof.json"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings across the production Runtime library",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/199/v4/clippy-lib.stderr.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_membership_transition",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warnings across the behavior-specific public target",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/199/v4/clippy-integration.stderr.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
