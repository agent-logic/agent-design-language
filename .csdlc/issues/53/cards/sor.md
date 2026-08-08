# Structured Output Record

Template: 1.0.0

Issue: 53

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Added an explicit v3 receipt contract that derives the evidence commit from the receipt introduction while preserving v2 exact-HEAD semantics.

## Artifacts

- .csdlc/prepared/issues/5862/proof-receipt-contract.rb
- .csdlc/prepared/issues/53/test-proof-receipt-contract.rb
- .csdlc/prepared/issues/53/design.md
- .csdlc/prepared/issues/53/diagram.mmd

## Execution

- Resolve the exact substantive source commit and machine-derive the unique evidence commit without embedding a self-referential evidence SHA.
- Require source-to-evidence ancestry, evidence-to-HEAD ancestry, evidence-only A..B changes, and zero later evidence drift.
- Validate source artifacts at the substantive commit and retain all command, log, negative, artifact, runner, and native-receipt checks.
- Add one focused temporary-repository regression for A/B/C success, v2 compatibility, and named fail-closed cases.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/53/test-proof-receipt-contract.rb"
    ],
    "purpose": "Prove non-self-referential A/B/C success, retained-v2 exact-HEAD behavior, and fail-closed ancestry, scope, source, receipt, and log tamper cases.",
    "outcome": "passed",
    "evidence_ref": "local:issue-53-receipt-regression:passed"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject malformed tracked changes before exact-head review.",
    "outcome": "passed",
    "evidence_ref": "local:issue-53-diff-check:passed"
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
