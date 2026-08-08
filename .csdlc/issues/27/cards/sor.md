# Structured Output Record

Template: 1.0.0

Issue: 27

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Pre-execution output record.

## Artifacts

- none

## Execution

- none

## Validation

[
  {
    "command": [
      "ruby",
      "adl/tools/validate_v092_runtime_native_receipts.rb",
      "--self-test-policy"
    ],
    "purpose": "Prove order-independent roles, duplicate rejection, exact allowlisting, rename safety, clean-worktree enforcement, and proof ancestry.",
    "outcome": "passed",
    "evidence_ref": "adl/tools/validate_v092_runtime_native_receipts.rb"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace errors in the issue-local validator and lifecycle changes.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/prepared/issues/27/design.md"
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
