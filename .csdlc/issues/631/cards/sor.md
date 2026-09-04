# Structured Output Record

Template: 1.0.0

Issue: 631

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented non-authoritative v3 proof, shadow, soak, and install construction routes under the single csdlc binary while preserving the #505 cutover authority boundary and binding proof/install evidence to the discovered repository root.

## Artifacts

- csdlc-v3/src/commands/proof.rs
- csdlc-v3/src/commands/mod.rs
- csdlc-v3/src/main.rs
- csdlc-v3/tests/command_manifest.rs
- csdlc-v3/tests/proof_parity_install_commands.rs
- csdlc-v3/tests/real_issue_canary.rs
- docs/csdlc-v3/v3-command-manifest.json
- docs/csdlc-v3/full-replacement-denominator.json
- .csdlc/prepared/issues/631/validate-v3-h5-proof-parity-install.sh

## Execution

- Added a typed csdlc-v3 proof command module for proof, shadow, soak, and install route request classification.
- Routed csdlc proof, shadow, soak, and install through implemented-construction CLI handling that emits machine-readable non-authoritative JSON reports.
- Kept all four routes read-only and blocked from lifecycle authority, provider side effects, binary installation, selector mutation, GitHub mutation, or #505 cutover.
- Updated the one-binary v3 command manifest and manifest tests so #631-owned routes no longer appear as placeholder fail-closed or partial routes.
- Replaced the placeholder #631 canary with behavior tests covering positive and negative proof, bounded shadow parity, soak hidden-state denial, full-replacement denominator status, and one-binary install gating.
- Bound proof/install evidence roots to the discovered Git repository root and rejected request-controlled scratch evidence roots even when their internal artifact, selector, and provenance bytes are self-consistent.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "proof_parity_install_commands"
    ],
    "purpose": "Run the focused #631 proof, shadow, soak, and install behavior tests, including caller-forged provenance and scratch-root rejection.",
    "outcome": "passed",
    "evidence_ref": "exact-head:69ce09689:4-passed"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "command_manifest",
      "--test",
      "real_issue_canary"
    ],
    "purpose": "Verify one-binary command manifest exposure and the full-replacement denominator canary after the evidence-root hardening.",
    "outcome": "passed",
    "evidence_ref": "exact-head:69ce09689:11-passed"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "purpose": "Reject Rust formatting drift after the evidence-root hardening.",
    "outcome": "passed",
    "evidence_ref": "exact-head:69ce09689:passed"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and conflict-marker artifacts before refreshed review.",
    "outcome": "passed",
    "evidence_ref": "exact-head:69ce09689:passed"
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
