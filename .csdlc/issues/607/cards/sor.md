# Structured Output Record

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: sor

Status: draft

## Summary

Implemented and locally proved the restart-safe warm two-node AWS controller through revision b7b1ebd95; live resume and two warm launches remain pending.

## Artifacts

- adl/tools/run_issue607_warm_polis.sh
- adl/tools/issue607_probe_runtime.py
- adl/tools/issue607_qualify_warm_polis.sh
- adl/tools/issue607_validate_saved_plan.sh
- adl/tools/test_issue607_warm_polis.sh
- infra/aws/runtime/gpu-proof
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .csdlc/evidence/607/local-validation-resume-b7b1ebd95.json

## Execution

- Separated retained warm storage, disposable preparation, and disposable compute ownership.
- Loaded exact preparation output keys and made zero, one, and two existing AMI continuation create only missing images.
- Made sealed snapshots and terminal preparation completion idempotent at their bounded seams.
- Allowed an artifact generation to continue only under a clean descendant controller and recorded both identities in launch evidence.
- Waited indefinitely for healthy AWS transitions while failing immediately on API and terminal-state errors.
- Bound resume to consumed authorization, source, plans, Terraform state, owner, images, ledger, campaign, and terminal checkpoint.
- Bound destructive recovery to exact authorization, ledger, owner, campaign, generation, and owner-filtered AWS discovery.
- Made cost-ledger initialization atomic and rejected duplicate, malformed, arithmetically inconsistent, or wrong-input preparation entries.
- Prevented destructive recovery when a terminal preparation result exists and allowed only exact retained artifacts in residue proof.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_issue607_warm_polis.sh",
      "all"
    ],
    "purpose": "Prove bounded no-paid resume, exact destructive recovery identity, strict atomic cost reconciliation, controller ancestry, checkpoint reconciliation, and artifact reuse.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-b7b1ebd95.json"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject diff hygiene defects.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-b7b1ebd95.json"
  },
  {
    "command": [
      "bash",
      "-n",
      "adl/tools/run_issue607_warm_polis.sh",
      "adl/tools/test_issue607_warm_polis.sh"
    ],
    "purpose": "Prove shell parse validity.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/607/local-validation-resume-b7b1ebd95.json"
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
