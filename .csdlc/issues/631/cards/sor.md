# Structured Output Record

Template: 1.0.0

Issue: 631

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented non-authoritative v3 proof, shadow, soak, and install construction routes under the single csdlc binary while preserving the #505 cutover authority boundary.

## Artifacts

- csdlc-v3/src/commands/proof.rs
- csdlc-v3/src/commands/mod.rs
- csdlc-v3/src/main.rs
- csdlc-v3/tests/command_manifest.rs
- csdlc-v3/tests/proof_parity_install_commands.rs
- docs/csdlc-v3/v3-command-manifest.json
- .csdlc/prepared/issues/631/validate-v3-h5-proof-parity-install.sh

## Execution

- Added a typed csdlc-v3 proof command module for proof, shadow, soak, and install route request classification.
- Routed csdlc proof, shadow, soak, and install through implemented-construction CLI handling that emits machine-readable non-authoritative JSON reports.
- Kept all four routes read-only and blocked from lifecycle authority, provider side effects, binary installation, selector mutation, GitHub mutation, or #505 cutover.
- Updated the one-binary v3 command manifest and manifest tests so #631-owned routes no longer appear as placeholder fail-closed or partial routes.
- Replaced the placeholder #631 canary with behavior tests covering positive and negative proof, bounded shadow parity, soak hidden-state denial, and one-binary install gating.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Reject diff hygiene defects across the stack range.",
    "outcome": "passed",
    "evidence_ref": "v3-h5-diff-hygiene.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml"
    ],
    "purpose": "Run every csdlc-v3 unit and integration test.",
    "outcome": "passed",
    "evidence_ref": "v3-h5-full-v3-regression.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/631/validate-v3-h5-proof-parity-install.sh"
    ],
    "purpose": "Run the focused #631 validator, including the proof_parity_install_commands integration target.",
    "outcome": "passed",
    "evidence_ref": "v3-h5-issue-validator.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--check"
    ],
    "purpose": "Reject Rust formatting drift.",
    "outcome": "passed",
    "evidence_ref": "v3-h5-rustfmt.log"
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
