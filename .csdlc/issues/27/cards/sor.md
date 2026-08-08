# Structured Output Record

Template: 1.0.0

Issue: 27

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Hardened native receipt validation while preserving the exact-proof packet integration as an explicit upstream gate.

## Artifacts

- adl/tools/validate_v092_runtime_native_receipts.rb
- adl/tools/test_validate_v092_runtime_native_receipts.sh
- .csdlc/prepared/issues/27/design.md
- .csdlc/prepared/issues/27/diagram.mmd

## Execution

- Made artifact role denominator comparison explicitly order-independent while preserving separate uniqueness enforcement.
- Required proof ancestry and rename-disabled Git path enumeration for post-proof verifier changes.
- Removed proof evidence directories from the post-proof allowlist and rejected runtime or product drift.
- Required a clean validation worktree except for the exact issue-local C-SDLC lock.
- Added real temporary-Git regressions for allowed verifier repair, dirty product changes, rename attacks, and unrelated histories.

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

pr_open

## Publication

Publication: draft

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
