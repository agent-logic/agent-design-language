# Structured Output Record

Template: 1.0.0

Issue: 5455

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Retrospective recovery: merged PR #5460 implemented stale stable-owner provenance rejection and exact source-checkout binding.

## Artifacts

- csdlc-v2/src/operator.rs
- csdlc-v2/tests/gate10a.rs
- https://github.com/danielbaustin/agent-design-language/pull/5460

## Execution

- Install receipts record source checkout revision or content provenance fallback
- Coexistence verification rejects stale owner-binary provenance explicitly
- Gate 10A executes a freshly installed stable editor

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a"
    ],
    "purpose": "Prove current merged-head install provenance, stale rejection, atomic install, and stable editor execution behavior.",
    "outcome": "passed",
    "evidence_ref": "Fresh recovery run at PR #5460 head fb7d09a56: 9 passed, 0 failed"
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
