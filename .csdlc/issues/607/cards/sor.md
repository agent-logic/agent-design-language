# Structured Output Record

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: sor

Status: draft

## Summary

Implemented and locally proved the restart-safe warm two-node AWS controller through revision b1ca07fb2; live resume and two warm launches remain pending.

## Artifacts

- adl/tools/run_issue607_warm_polis.sh
- adl/tools/issue607_probe_runtime.py
- adl/tools/issue607_qualify_warm_polis.sh
- adl/tools/issue607_validate_saved_plan.sh
- adl/tools/test_issue607_warm_polis.sh
- infra/aws/runtime/gpu-proof
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .csdlc/evidence/607/local-validation-resume-b1ca07fb2.json

## Execution

- Separated retained warm storage, disposable preparation, and disposable compute ownership.
- Prepared complete Runtime and GPU closures once and activated them without launch-time installation or downloads.
- Bound canonical saved-plan digests and three single-use actions to one aggregate authorization envelope.
- Made partial AMI creation, sealed snapshots, cost recording, and terminal preparation completion idempotent across controller interruption.
- Allowed a prepared artifact generation to continue only under a clean descendant controller while recording both identities separately.
- Removed elapsed-time failure for healthy AWS image and snapshot transitions while retaining immediate failure for API and terminal-state errors.
- Added consumed-authorization, source, plan, Terraform state, owner, image, ledger, and terminal-checkpoint validation before resume.
- Allowed only the exact retained prepared AMIs and root snapshots in zero-disposable-residue proof.
- Added executable regressions for partial-image recovery, controller-generation ancestry, checkpoint reconciliation, permanent AWS errors, and exact image, snapshot, and cost reuse.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_issue607_warm_polis.sh",
      "all"
    ],
    "purpose": "Prove the complete bounded no-paid contract including partial-image recovery, generation ancestry, terminal checkpoint reconciliation, indefinite healthy waits, and exact artifact reuse.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-b1ca07fb2.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and conflict-marker residue after restart-safety remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-b1ca07fb2.json"
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
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-b1ca07fb2.json"
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
