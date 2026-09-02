# Structured Output Record

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: sor

Status: draft

## Summary

Implemented the warm two-node AWS path through restart-safe controller revision f59fcbf6a; the prepared Runtime and GPU AMIs are available, while live resume and two warm launches remain pending.

## Artifacts

- adl/tools/run_issue607_warm_polis.sh
- adl/tools/issue607_probe_runtime.py
- adl/tools/issue607_qualify_warm_polis.sh
- adl/tools/issue607_validate_saved_plan.sh
- adl/tools/test_issue607_warm_polis.sh
- infra/aws/runtime/gpu-proof
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .csdlc/evidence/607/local-validation-resume-f59fcbf6a.json

## Execution

- Separated retained warm storage, disposable preparation, and disposable compute ownership.
- Prepared complete Runtime and GPU closures once and activated them without launch-time installation or downloads.
- Bound canonical saved-plan digests and three single-use actions to one aggregate authorization envelope.
- Made AMI and sealed-snapshot creation exact-state and idempotent across controller interruption.
- Removed elapsed-time failure for healthy AWS image and snapshot transitions while retaining immediate failure for API and terminal-state errors.
- Added consumed-authorization, source, plan, Terraform state, owner, image, and ledger identity validation before resume.
- Added executable regressions for permanent AWS errors and exact reuse of existing images and snapshots.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_issue607_warm_polis.sh",
      "all"
    ],
    "purpose": "Prove the complete bounded no-paid contract including restart-safe indefinite waits, terminal API failures, and exact image and snapshot reuse.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-f59fcbf6a.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and conflict-marker residue after restart-safety remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-f59fcbf6a.json"
  },
  {
    "command": [
      "bash",
      "-n",
      "adl/tools/run_issue607_warm_polis.sh",
      "adl/tools/test_issue607_warm_polis.sh"
    ],
    "purpose": "Prove shell parse validity for the controller and its regression suite.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-f59fcbf6a.json"
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
