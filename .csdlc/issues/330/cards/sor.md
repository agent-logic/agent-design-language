# Structured Output Record

Template: 1.0.0

Issue: 330

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediated r2 review finding for #330. Recovery-side cleanup ledger authorization now rejects non-directory or non-empty private-delete cleanup namespaces before store/classification can skip cleanup ledger entries. Evidence was recaptured at immutable remediation head 63e3782a1d46dc86161eb81e9eb2fd43f1f7e562. Publication and merge are not yet attempted.

## Artifacts

- .csdlc/evidence/330/r3/focused-issue-330.log sha256=fa156d64aff998152b60b7e17f83508453a594d2529eef850b836dc89d2fddc8
- .csdlc/evidence/330/r3/adjacent-archived-projection-cleanup.log sha256=c03d1b12d66d2892ae6cb922e27e96bae6b9001247ae2e3a5dafddf3e78db73b
- .csdlc/evidence/330/r3/recovery-authority-gate5.log sha256=6365bc84c7f232e853760edab2bdd23d62977b17ad345358e396da3503a857cd
- .csdlc/evidence/330/r3/fmt-check.log sha256=72a12c267a914e78bde929382b3e4ac16f012fa102b3be6731f6cbd526e66c34
- .csdlc/evidence/330/r3/diff-check.log sha256=2b456c34a8e87292cba593625b94027b15e9463f5ca4dd850d7105db81d14fad
- .csdlc/evidence/330/r3/strict-clippy.log sha256=83db1c16aacf0a7d2285bbf65b36b01acb9aa63ee5423c8a994acdab01df96e2

## Execution

- csdlc-v2/src/projection_recovery.rs: recovery-side cleanup ledger validation now requires private-delete to be a directory and empty before authorizing recovery/store skip
- csdlc-v2/tests/issue_330_bridge_cleanup_defect.rs: added cleanup_private_namespace_residue_does_not_authorize_recovery_skip regression proving CorruptRecord and byte-for-byte no mutation

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
    "purpose": "Focused #330 regression proof including private-delete residue rejection, bridge cleanup recovery validation, forged cleanup final chain rejection, and pre-mutation raced final zero-mutation behavior",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/330/r3/focused-issue-330.log sha256=fa156d64aff998152b60b7e17f83508453a594d2529eef850b836dc89d2fddc8 head=63e3782a1d46dc86161eb81e9eb2fd43f1f7e562 status=0"
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
    "purpose": "Adjacent cleanup regression proof preserving cleanup authority and failpoint behavior",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/330/r3/adjacent-archived-projection-cleanup.log sha256=c03d1b12d66d2892ae6cb922e27e96bae6b9001247ae2e3a5dafddf3e78db73b head=63e3782a1d46dc86161eb81e9eb2fd43f1f7e562 status=0"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "preserved_projection_recovery",
      "--",
      "--nocapture"
    ],
    "purpose": "Existing recovery authority regression lane declared by #330 VPP",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/330/r3/recovery-authority-gate5.log sha256=6365bc84c7f232e853760edab2bdd23d62977b17ad345358e396da3503a857cd head=63e3782a1d46dc86161eb81e9eb2fd43f1f7e562 status=0"
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
    "evidence_ref": ".csdlc/evidence/330/r3/fmt-check.log sha256=72a12c267a914e78bde929382b3e4ac16f012fa102b3be6731f6cbd526e66c34 head=63e3782a1d46dc86161eb81e9eb2fd43f1f7e562 status=0"
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
    "evidence_ref": ".csdlc/evidence/330/r3/diff-check.log sha256=2b456c34a8e87292cba593625b94027b15e9463f5ca4dd850d7105db81d14fad head=63e3782a1d46dc86161eb81e9eb2fd43f1f7e562 status=0"
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
    "evidence_ref": ".csdlc/evidence/330/r3/strict-clippy.log sha256=83db1c16aacf0a7d2285bbf65b36b01acb9aa63ee5423c8a994acdab01df96e2 head=63e3782a1d46dc86161eb81e9eb2fd43f1f7e562 status=0"
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
