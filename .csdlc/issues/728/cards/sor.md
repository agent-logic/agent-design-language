# Structured Output Record

Template: 1.0.0

Issue: 728

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and repaired the #728 disposable AWS-F Runtime proof runner and local proof package without performing live AWS mutation. The runner now binds mutable backend/input/route/receipt selectors to the authorization file and pairs Terraform-state cleanup with AWS-side absence readbacks for exact deployed selectors.

## Artifacts

- docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh
- docs/operations/cloud/aws/runtime-platform/README.md
- .csdlc/prepared/issues/728/validate_preparation_bundle.sh
- .csdlc/prepared/issues/728/design.md
- .csdlc/prepared/issues/728/diagram.mmd
- .csdlc/evidence/728/readiness-preflight-20260908.md
- .csdlc/evidence/728/no-auth-refusal.log

## Execution

- Added a fail-closed disposable proof runner that requires an explicit #728 authorization packet before any Terraform apply or destroy operation.
- Split the proof workflow into staged ALB, private-node, attach, external receipt, and reverse-destroy modes using the existing AWS-F Terraform roots.
- Enforced account, profile, region, roots, workspaces, backend config paths, tfvars paths, saved-plan paths and digests, external health URL, expected receipt marker, optional receipt path, cost ceiling, and deadline before mutation stages.
- Kept mutation-stage cleanup armed through post-apply output/readback capture so failures after successful apply still route to cleanup.
- Captured exact ALB, target group, listener, instance, and security-group selectors before destroy, then checked Terraform state emptiness and AWS describe/readback absence after destroy.
- Added a concise operator runbook section documenting the one script, required environment contract, authorization packet shape, and live proof sequence.
- Tightened .csdlc/prepared/issues/728/validate_preparation_bundle.sh so selector authorization and AWS absence-readback strings are mechanically checked.
- Retained readiness and no-authorization refusal evidence under .csdlc/evidence/728 without credentials or secret material.

## Validation

[
  {
    "command": [
      "bash",
      "-n",
      "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh"
    ],
    "purpose": "Prove the disposable proof runner shell syntax before any AWS mutation.",
    "outcome": "passed",
    "evidence_ref": "local command output: exit 0"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/728/validate_preparation_bundle.sh"
    ],
    "purpose": "Prove the #728 issue-owned preparation bundle, runner gates, mutable selector authorization checks, zero-residue cleanup surface, AWS absence-readback strings, and AWS-F module references are present.",
    "outcome": "passed",
    "evidence_ref": "local command output: PASS #728 preparation bundle validator"
  },
  {
    "command": [
      "docs/operations/cloud/aws/runtime-platform/run-disposable-proof.sh"
    ],
    "purpose": "Prove the runner refuses live AWS mutation when the exact authorization packet is absent.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/728/no-auth-refusal.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and conflict-marker defects in the bound worktree.",
    "outcome": "passed",
    "evidence_ref": "local command output: exit 0"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      "/Volumes/FastWork/adl-worktrees/adl-issue-728-aws-f-disposable-runtime-deployment-zero-residue",
      "--issue",
      "728"
    ],
    "purpose": "Verify typed lifecycle/card state is structurally clean after review recovery and the P1 remediation.",
    "outcome": "passed",
    "evidence_ref": "local command output: status pass, phase implemented, generation 8"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-728-aws-f-disposable-runtime-deployment-zero-residue",
      "issue",
      "--issue",
      "728"
    ],
    "purpose": "Verify typed issue validation passes after review recovery and the P1 remediation.",
    "outcome": "passed",
    "evidence_ref": "local command output: status pass, phase implemented, generation 8"
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
