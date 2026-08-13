# Structured Output Record

Template: 1.0.0

Issue: 203

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Reconciled the preserved #203 monolithic candidate against terminal #258, #259, and #260. Current origin/main is the implementation authority for all production and test paths; #203 retains only lifecycle, design, decomposition provenance, and integration-closeout proof surfaces.

## Artifacts

- .csdlc/evidence/203/ISSUE_203_DECOMPOSITION_PLAN.md
- .csdlc/evidence/203/provider-reviews
- .git/csdlc-v2/preservation/203-pre-main-reconciliation-20260813.md
- .git/csdlc-v2/preservation/203-three-way-disposition-20260813.md
- .git/csdlc-v2/quarantine/203-pre-main-reconciliation-20260813

## Execution

- Preserved the complete pre-reconciliation candidate as commit cb3770d03 plus a verified Git-common branch bundle and binary patch.
- Merged exact origin/main 0b5aefd6 and resolved every child-owned source, test, and Cargo.lock path to terminal main.
- Verified zero product diff against origin/main and retained no #202, #204, #205, #208, #258, #259, or #260 implementation absorption.
- Classified the historical 44-case synthetic receipt as superseded because the terminal child test denominator is now authoritative and current.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--exit-code",
      "origin/main",
      "--",
      "adl-runtime",
      "adl/Cargo.lock"
    ],
    "purpose": "Prove the reconciled #203 branch carries zero product or dependency-lock delta beyond terminal child main.",
    "outcome": "passed",
    "evidence_ref": ".git/csdlc-v2/preservation/203-three-way-disposition-20260813.md"
  },
  {
    "command": [
      "csdlc-finish",
      "--validate-cached-issue",
      "258|259|260"
    ],
    "purpose": "Validate all three terminal child caches against their canonical records before parent reconciliation.",
    "outcome": "passed",
    "evidence_ref": ".git/csdlc-v2/preservation/203-pre-main-reconciliation-20260813.md"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/203/produce-proof-receipt.rb"
    ],
    "purpose": "Audit whether the retained historical monolithic proof denominator remains current after terminal child delivery.",
    "outcome": "failed",
    "evidence_ref": ".csdlc/evidence/203/v2/identity-authority.stdout.log"
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
