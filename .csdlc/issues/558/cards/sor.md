# Structured Output Record

Template: 1.0.0

Issue: 558

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Stabilized the governed learner replication proof for coverage instrumentation by waiting for node 4 to observe a leader after promotion and extending the bounded machine-mutation wait used by this distributed harness.

## Artifacts

- adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs
- .csdlc/prepared/issues/558/validate-focused-proof.sh
- .csdlc/prepared/issues/558/validate-lifecycle-evidence.sh
- .csdlc/evidence/558/focused-learner-replication.log
- .csdlc/evidence/558/focused-learner-replication-llvm-cov.log
- .csdlc/evidence/558/diff-check.log
- .csdlc/evidence/558/lifecycle-evidence.log

## Execution

- Increased the learner transport test helper's machine-mutation observation timeout from 60s to 180s so coverage-instrumented scheduling has bounded room to catch up.
- Added an explicit node-leader observation wait for the promoted learner before the replication mutation is written.
- Kept all changes confined to the governed learner transport test harness; learner authorization, membership, transport, and Runtime product semantics are unchanged.
- Added issue-owned validators for focused ordinary and coverage-instrumented proof plus lifecycle evidence.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and patch hygiene drift.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/558/validate-focused-proof.sh"
    ],
    "purpose": "Run the issue-owned validator that exercises real_four_node_learner_replication normally and under cargo llvm-cov instrumentation.",
    "outcome": "passed",
    "evidence_ref": "focused-learner-replication.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/558/validate-lifecycle-evidence.sh"
    ],
    "purpose": "Verify issue-local lifecycle state and evidence directory exist before implementation finalization.",
    "outcome": "passed",
    "evidence_ref": "lifecycle-evidence.log"
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
