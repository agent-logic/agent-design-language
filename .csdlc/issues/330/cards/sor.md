# Structured Output Record

Template: 1.0.0

Issue: 330

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the bounded #330 production repair for bridge-fed projection cleanup: cleanup rejects a raced final receipt before archive mutation using a new before_cleanup_node_mutation failpoint while preserving existing before_prefinal_receipt_chain_validation semantics, and recovery validation accepts post-cleanup retained recovery attempts only when strict cleanup-authority, manifest, completed-recovery receipt, operation-created payload, and final cleanup receipt predicates all validate. Publication and merge are not yet attempted.

## Artifacts

- .csdlc/evidence/330/r1/focused-issue-330.log sha256=fcfc8259877f11bef8fc4e0199fb89a81d073be61a3abd78cc83676e99ef8ba3
- .csdlc/evidence/330/r1/adjacent-archived-projection-cleanup.log sha256=782db846d78102964f6fe5ebc9416cfaf56df6489a08b9b314a09ef372b9d2b3
- .csdlc/evidence/330/r1/fmt-check.log sha256=1261237904e54b11c40f2123f36a4df20544f6804029973d71c1ca3436926b1b
- .csdlc/evidence/330/r1/diff-check.log sha256=ca9a6c1c0875981c9d803678b7cf194b9bebf38cb20762aec37d79d20c0cb001
- .csdlc/evidence/330/r1/strict-clippy.log sha256=828cdd35b042a8b9c9d66533a4dc0aac4a7b9dff4f5ba628a112a75e1b3238ae

## Execution

- csdlc-v2/src/projection_cleanup.rs: added pre-mutation raced final receipt rejection without moving the existing prefinal-validation failpoint
- csdlc-v2/src/projection_recovery.rs: validates exact cleanup authority before accepting a missing/cleaned retained rejected archive
- csdlc-v2/src/store.rs: skips cleanup ledger entries during ordinary recovery preflight only when the strict recovery cleanup-authority predicate validates
- csdlc-v2/tests/issue_330_bridge_cleanup_defect.rs: added focused bridge-fed regression tests for post-cleanup ordinary recovery validation and pre-mutation raced final zero-mutation behavior

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "issue_330_bridge_cleanup_defect",
      "--",
      "--nocapture"
    ],
    "purpose": "Focused #330 regression proof for bridge cleanup recovery validation and pre-mutation raced final zero-mutation behavior",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/330/r1/focused-issue-330.log sha256=fcfc8259877f11bef8fc4e0199fb89a81d073be61a3abd78cc83676e99ef8ba3 head=bb7ab591de6354e03e7f59fc342e083c73aee892 status=0"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "archived_projection_cleanup",
      "--",
      "--nocapture"
    ],
    "purpose": "Adjacent cleanup regression proof preserving existing cleanup failpoint and manifest-authority behavior",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/330/r1/adjacent-archived-projection-cleanup.log sha256=782db846d78102964f6fe5ebc9416cfaf56df6489a08b9b314a09ef372b9d2b3 head=bb7ab591de6354e03e7f59fc342e083c73aee892 status=0"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--check"
    ],
    "purpose": "Rust formatting check",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/330/r1/fmt-check.log sha256=1261237904e54b11c40f2123f36a4df20544f6804029973d71c1ca3436926b1b head=bb7ab591de6354e03e7f59fc342e083c73aee892 status=0"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "HEAD"
    ],
    "purpose": "Whitespace and patch hygiene check",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/330/r1/diff-check.log sha256=ca9a6c1c0875981c9d803678b7cf194b9bebf38cb20762aec37d79d20c0cb001 head=bb7ab591de6354e03e7f59fc342e083c73aee892 status=0"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Strict Clippy over all targets",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/330/r1/strict-clippy.log sha256=828cdd35b042a8b9c9d66533a4dc0aac4a7b9dff4f5ba628a112a75e1b3238ae head=bb7ab591de6354e03e7f59fc342e083c73aee892 status=0"
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
