# Structured Output Record

Template: 1.0.0

Issue: 59

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Prepared a reviewed, non-closing authority-routing checkpoint for the externally owned Codex blocked-goal replacement defect and stacked it on the typed issue 75 part_of publisher contract.

## Artifacts

- .csdlc/prepared/issues/59/design.md
- .csdlc/prepared/issues/59/diagram.mmd
- inherited issue 75 typed part_of publication contract at c405895a10440eb4a163ed7ebd084c4e8c617be7

## Execution

- .csdlc/issues/59
- .csdlc/prepared/issues/59

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject malformed authority-routing packet changes.",
    "outcome": "passed",
    "evidence_ref": "git diff --check passed at the issue 59 checkpoint candidate."
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate6"
    ],
    "purpose": "Prove default closing compatibility, explicit part_of linkage, qualified split-authority references, retained evidence, and mixed-linkage rejection.",
    "outcome": "passed",
    "evidence_ref": "gate6 passed 10 tests with 0 failures on the stacked issue 75 implementation."
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
