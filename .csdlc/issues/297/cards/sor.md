# Structured Output Record

Template: 1.0.0

Issue: 297

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Remediated the #297 bridge r2 review findings and revalidated the minimal production recovery-to-cleanup authority bridge. The bridge now stores cleanup authority under a validator-recognized cleanup-authority namespace so same-operation replay remains idempotent while conflicting cleanup-operation authority is rejected. Cleanup directory link-count relaxation is now limited to directories with authorized child nodes; leaf directories and regular files remain strict.

## Artifacts

- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/projection_cleanup.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/tests/archived_projection_cleanup.rs
- .csdlc/evidence/297/bridge-r3/gate5-bridge-replay.log#sha256=affcd237422a569bbe4bfdc30ae6e75201a967455ea0abdee7a9dd57041083dc
- .csdlc/evidence/297/bridge-r3/archived-cleanup-link-count.log#sha256=b32116712aa072c9d0556531e3c5e7485a147166a13fc905a39721b8e54732c6
- .csdlc/evidence/297/bridge-r3/gate5-full.log#sha256=0647f14cd74ae955d3d88afea3e405ac986cd19c05d935093129f68c675cc79c
- .csdlc/evidence/297/bridge-r3/gate-cleanup.log#sha256=e27f1067433eb2ae2b6f62e69a9ffd705b5858b72e4b038bfb6d3a1487499813
- .csdlc/evidence/297/bridge-r3/archived-cleanup-full.log#sha256=a5a7a630256f414b5d3d162d8b0c57d534432e257987f47cb9c435635748a571
- .csdlc/evidence/297/bridge-r3/strict-clippy.log#sha256=6368e7bc35f7e819b7c811817b1e927f336218a1f654c547cbadd8bd282515b8
- .csdlc/evidence/297/bridge-r3/fmt-check.log#sha256=d0c2216b26f44372de830b598d1177ff18843f3d495c101d0cdde608cbf606f3
- .csdlc/evidence/297/bridge-r3/diff-check.log#sha256=eb953affdeb057570a957c927cd19169e10f9e3ccad21ea7e9ded94b18ebb643
- .csdlc/evidence/297/bridge-r3/csdlc-validate.log#sha256=fdb367749e7d34428889f63afe3ad277da0de706ebb57061a9055726d9608bc6
- .csdlc/evidence/297/bridge-r3/csdlc-doctor.log#sha256=7269b6a31e73136b1cf0c9180a954ec1277ae098e3fa437a568c558a41518d2a

## Execution

- Recorded the r2 exact-head FAIL findings in typed review truth, then recovered review authority before source remediation.
- Moved bridge-produced canonical archive manifest and completed recovery receipt artifacts from the recovery attempt root into cleanup-authority/<cleanup-operation>/ so completed-recovery validation is not poisoned by bridge replay artifacts.
- Added cleanup-authority namespace validation that accepts only the two expected JSON artifacts per operation and rejects any unexpected artifact shape during completed-recovery validation.
- Added same-operation bridge replay proof and conflicting cleanup-operation rejection in gate5.
- Constrained cleanup identity matching so directory link-count drift is accepted only when the cleanup request owns descendants under that directory; leaf-directory and regular-file link-count drift are rejected.
- Added cleanup regressions for leaf directory link-count rejection, parent directory link-count drift after authorized child cleanup, and regular-file link-count rejection.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "recovery_bridge_emits_cleanup_authority_consumed_by_cleanup",
      "--",
      "--nocapture"
    ],
    "purpose": "Focused bridge proof at exact head 9aecbb18872036912a14d199c72a484c7ba08107: production bridge artifacts replay idempotently, conflicting cleanup operation is rejected, and cleanup consumes bridge authority.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/297/bridge-r3/gate5-bridge-replay.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "archived_projection_cleanup",
      "link_count",
      "--",
      "--nocapture"
    ],
    "purpose": "Focused cleanup link-count proof at exact head 9aecbb18872036912a14d199c72a484c7ba08107: leaf directory drift rejected, parent drift after authorized child cleanup accepted, regular-file drift rejected.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/297/bridge-r3/archived-cleanup-link-count.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "--",
      "--nocapture"
    ],
    "purpose": "Full gate5 recovery regression lane at exact head 9aecbb18872036912a14d199c72a484c7ba08107.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/297/bridge-r3/gate5-full.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_cleanup",
      "--",
      "--nocapture"
    ],
    "purpose": "Existing cleanup authority regression lane at exact head 9aecbb18872036912a14d199c72a484c7ba08107.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/297/bridge-r3/gate-cleanup.log"
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
    "purpose": "Full archived projection cleanup regression lane including new link-count boundary tests at exact head 9aecbb18872036912a14d199c72a484c7ba08107.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/297/bridge-r3/archived-cleanup-full.log"
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
    "purpose": "Strict Clippy over all csdlc-v2 targets at exact head 9aecbb18872036912a14d199c72a484c7ba08107.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/297/bridge-r3/strict-clippy.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--check"
    ],
    "purpose": "Rust formatting check at exact head 9aecbb18872036912a14d199c72a484c7ba08107.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/297/bridge-r3/fmt-check.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD",
      "&&",
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Committed-range and worktree diff hygiene at exact head 9aecbb18872036912a14d199c72a484c7ba08107.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/297/bridge-r3/diff-check.log"
  },
  {
    "command": [
      "csdlc-validate",
      "--root",
      ".",
      "issue",
      "--issue",
      "297"
    ],
    "purpose": "Typed issue validation after review recovery and bridge r3 evidence at exact head 9aecbb18872036912a14d199c72a484c7ba08107.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/297/bridge-r3/csdlc-validate.log"
  },
  {
    "command": [
      "csdlc-doctor",
      "--issue",
      "297"
    ],
    "purpose": "Typed doctor check after review recovery and bridge r3 evidence at exact head 9aecbb18872036912a14d199c72a484c7ba08107.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/297/bridge-r3/csdlc-doctor.log"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
