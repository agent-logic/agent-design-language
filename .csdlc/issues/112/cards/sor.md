# Structured Output Record

Template: 1.0.0

Issue: 112

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Forward-recovered the authoritative bound #112 branch to the decomposed Layer 8 authority core: preserved the historical cumulative evidence, removed child-owned Runtime ingress/API/Observatory product scope from the net product diff, aligned the runtime-kernel authority core with reviewed reference commit 613fddc9c017781420cb1854834c7376e96485b0, and validated the focused runtime-kernel core lanes.

## Artifacts

- adl-runtime-kernel/src/layer8_authority/audit.rs
- adl-runtime-kernel/src/layer8_authority/exchange.rs
- adl-runtime-kernel/src/layer8_authority/identity.rs
- adl-runtime-kernel/src/layer8_authority/mod.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/layer8_authority.rs
- .csdlc/issues/112
- .csdlc/prepared/issues/112/design.md
- .csdlc/prepared/issues/112/diagram.mmd
- .csdlc/evidence/112/layer8-authority-core-tests-forward.log
- .csdlc/evidence/112/layer8-authority-core-fmt-forward.log
- .csdlc/evidence/112/layer8-authority-core-clippy-forward.log
- .csdlc/evidence/112/layer8-authority-core-diff-check-forward.log

## Execution

- Cleared the superseded cumulative review assignment through typed csdlc-review recover, preserving audit history.
- Reapproved the core-only design and diagram through typed csdlc-edit approve-design.
- Restored child-owned Runtime ingress, Runtime API, Observatory, and feature-doc product files to origin/main or removed added child-only files from the net #112 diff.
- Aligned adl-runtime-kernel/src/layer8_authority/{audit.rs,exchange.rs,identity.rs,mod.rs}, adl-runtime-kernel/src/lib.rs, and adl-runtime-kernel/tests/layer8_authority.rs with the reviewed core reference behavior.
- Bound signed identity messages and communication identities to credential generation and rejected stale-generation signed messages.
- Narrowed typed lifecycle declared scope, authority boundary, deliverables, acceptance criteria, plan steps, affected areas, validation lanes, and review prompts to the Layer 8 authority core where supported by the v2 owner binary.

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "layer8_authority",
      "--",
      "--nocapture"
    ],
    "purpose": "Run the focused Layer 8 authority core integration test target with nonzero selection.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/112/layer8-authority-core-tests-forward.log (1 passed)"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Verify runtime-kernel Rust formatting for the core recovery diff.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/112/layer8-authority-core-fmt-forward.log (empty output; exit 0)"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Enforce strict warning-free clippy for the runtime-kernel crate and tests.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/112/layer8-authority-core-clippy-forward.log (exit 0)"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Verify whitespace and conflict-marker hygiene for the forward recovery diff.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/112/layer8-authority-core-diff-check-forward.log (empty output; exit 0)"
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
