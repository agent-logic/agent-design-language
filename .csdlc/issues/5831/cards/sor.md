# Structured Output Record

Template: 1.0.0

Issue: 5831

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and locally proved governed WP-13A adaptive learning with discoverable fully validated reconciliation and durable-before-live MutationGate transaction semantics. After PR run 31429043531 proved both native producers passed but aggregate validation rejected the widened source manifest, the validator now mirrors the producer's reasoning.rs, durable_state.rs, and tests/durable_state.rs authority paths and locks them in its self-test. Replacement native execution remains required before merge.

## Artifacts

- adl-runtime-kernel/src/adaptive_learning.rs
- adl-runtime-kernel/src/durable_state.rs
- adl-runtime-kernel/src/reasoning.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/adaptive_learning.rs
- adl-runtime-kernel/tests/durable_state.rs
- adl-runtime-kernel/tests/fixtures/adaptive_learning/matrix.json
- docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md
- .csdlc/prepared/issues/5831/produce-native-receipt.rb
- .csdlc/prepared/issues/5831/validate-native-receipts.rb
- .github/workflows/wp13a-native-adaptive-learning.yml
- .csdlc/evidence/5831/adaptive-learning-runtime-v3.log
- .csdlc/evidence/5831/adaptive-learning-strict-clippy.log
- .csdlc/evidence/5831/adaptive-learning-native-scripts.log
- .csdlc/evidence/5831/local-validation-manifest.json

## Execution

- Execute accepted learning only through signed mutation, loop, cancellation, cognitive-profile, capability, and durable-state authorities.
- Bind policy, grant, mutation evidence, profile, capability, proposal, and complete predecessor history.
- Reserve one canonical bounded global pending intent with the exact before-gate snapshot and discover it at startup without caller-supplied identity.
- Fully validate pending canonical encoding, self-digest, snapshot, history, policy, profile, privacy, and MutationAuthority evidence before reconciliation.
- Hold gate and adaptation locks across candidate construction and the durable callback; publish live state only after durable success, leaving failures nonmutating.
- Reconcile reserved, committed, and aborted intents deterministically and allow one authoritative winner under concurrent adaptive execution.
- Persist complete sequence-addressed histories for restart and rollback while bounding durable text, collections, snapshots, and pending payloads.
- Bind the native source manifest and workflow triggers to adaptive_learning.rs, reasoning.rs, durable_state.rs, adaptive_learning tests, and durable_state tests.
- Align the independent native validator source manifest exactly with the producer's authority-bearing reasoning.rs, durable_state.rs, and tests/durable_state.rs paths, and retain a self-test that fails if any is omitted.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "adaptive_learning",
      "--test",
      "durable_state",
      "--no-tests=fail",
      "--status-level",
      "all"
    ],
    "purpose": "Prove fifteen adaptive authority/reconciliation/interruption/tamper/concurrency cases plus five durable atomic CAS/restart cases at exact revision 6c548cc36df074ab77f23d4e2f1aea9a459a8b89.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5831/local-validation-manifest.json"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "adaptive_learning",
      "--test",
      "durable_state",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove strict lint cleanliness for adaptive learning, the MutationGate transaction boundary, durable CAS, and focused tests.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5831/adaptive-learning-strict-clippy.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5831/produce-native-receipt.rb",
      "--self-test"
    ],
    "purpose": "Prove native log normalization and exact inclusion of all adaptive transaction and durable authority source paths.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5831/adaptive-learning-native-scripts.log"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
