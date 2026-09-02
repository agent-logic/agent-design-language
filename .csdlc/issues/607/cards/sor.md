# Structured Output Record

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: sor

Status: draft

## Summary

Implemented and locally proved the restart-safe warm two-node AWS controller through revision 375ae8f88; live resume and two warm launches remain pending.

## Artifacts

- adl/tools/run_issue607_warm_polis.sh
- adl/tools/issue607_probe_runtime.py
- adl/tools/issue607_qualify_warm_polis.sh
- adl/tools/issue607_validate_saved_plan.sh
- adl/tools/test_issue607_warm_polis.sh
- infra/aws/runtime/gpu-proof
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .csdlc/evidence/607/local-validation-resume-375ae8f88.json

## Execution

- Separated retained warm storage, disposable preparation, and disposable compute ownership.
- Loaded the exact preparation instance output keys and covered their rejection boundary.
- Made zero, one, and two existing AMI continuation create exactly the missing prepared images.
- Made sealed snapshots, preparation cost recording, and terminal preparation completion idempotent at their bounded seams.
- Allowed a prepared artifact generation to continue only under a clean descendant controller while recording both identities in launch authorization evidence.
- Removed elapsed-time failure for healthy AWS image and snapshot transitions while retaining immediate failure for API and terminal-state errors.
- Added consumed-authorization, source, plan, Terraform state, owner, image, ledger, and terminal-checkpoint validation before resume.
- Prevented destructive recovery when a terminal preparation result exists.
- Allowed only exact retained prepared AMIs and root snapshots in zero-disposable-residue proof.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_issue607_warm_polis.sh",
      "all"
    ],
    "purpose": "Prove the bounded no-paid continuation seams including real preparation outputs, zero, one, and two AMI states, controller ancestry and manifest identity, checkpoint reconciliation, recovery refusal, and cost reuse.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-375ae8f88.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and conflict-marker residue.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-375ae8f88.json"
  },
  {
    "command": [
      "bash",
      "-n",
      "adl/tools/run_issue607_warm_polis.sh",
      "adl/tools/test_issue607_warm_polis.sh"
    ],
    "purpose": "Prove shell parse validity for the controller and regression suite.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-375ae8f88.json"
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
