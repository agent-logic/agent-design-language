# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and no-paid validated the Terraform-owned two-node AWS Runtime qualification; the real GPU apply remains pending fresh exact-head review, typed publication, and single-use run authorization.

## Artifacts

- infra/aws/runtime/gpu-proof
- adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/issue345_aws_gpu_prerequisites.cloudformation.yaml (removed)
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .csdlc/prepared/issues/345/design.md
- .csdlc/prepared/issues/345/diagram.mmd

## Execution

- Added an isolated Terraform root owning one regular Runtime node, one GPU Ollama node, two security groups, one shared EC2 key pair, least-privilege roles/profiles, encrypted disposable gp3 disks, and a two-node EventBridge Scheduler deadline.
- Pinned the Terraform AWS provider to agent-logic-admin so it cannot fall back to the operator's default account.
- Required public IPv4 and TCP/22 from one validated operator /32 on both nodes, while limiting GPU TCP/11434 to the Runtime security group.
- Replaced controller-side EC2/SSM orchestration with automatic cloud-init and retained SSM only as a recovery path.
- Replaced live Git checkout with a versioned digest-bound archive of the exact reviewed repository commit.
- Configured persistent Ollama keep-alive and complete simultaneous digest-checked GPU residency for at least two models.
- Ran Guardian/Runtime lifecycle, one governed Shepherd proof per model, and six real UTS/ACC/Freedom-Gate/runtime.observe agent cycles on the regular node.
- Bound single-use authorization to both nodes, disks, immutable artifacts, SSH/network identity, Terraform source, deadline, and combined compute/storage/IPv4/request cost; retained the exact applied saved-plan digest.
- Added controller destroy, guest systemd shutdown, and tag-constrained Scheduler termination for both nodes with zero-instance/volume verification.
- Removed the obsolete issue-345 CloudFormation prerequisite template and forced all run state beneath the bound worktree.

## Validation

[
  {
    "command": [
      "terraform",
      "-chdir=infra/aws/runtime/gpu-proof",
      "fmt",
      "-check"
    ],
    "purpose": "Reject Terraform formatting drift.",
    "outcome": "passed",
    "evidence_ref": "infra/aws/runtime/gpu-proof"
  },
  {
    "command": [
      "terraform",
      "-chdir=infra/aws/runtime/gpu-proof",
      "validate"
    ],
    "purpose": "Validate the complete two-node Terraform configuration.",
    "outcome": "passed",
    "evidence_ref": "infra/aws/runtime/gpu-proof"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Prove the no-paid two-node, shared-key, SSH, private-Ollama, authorization, source-archive, create-only receipt, state-containment, and cleanup contracts.",
    "outcome": "passed",
    "evidence_ref": "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue345_aws_gpu_shepherd_proof.sh",
      "preflight"
    ],
    "purpose": "Verify current business-account AMIs, pricing, GPU quota, immutable artifacts, network/SSH hashes, Terraform identity, combined 1.425111 USD worst-case cost, and zero stale resources without a paid launch.",
    "outcome": "passed",
    "evidence_ref": ".adl/local/issue345/preflight-artifact-manifest.json"
  },
  {
    "command": [
      "terraform",
      "-chdir=infra/aws/runtime/gpu-proof",
      "plan",
      "-refresh=false"
    ],
    "purpose": "Generate a real read-only plan through agent-logic-admin and verify exactly two instances, one key pair, one deadline schedule, and sixteen creates with no apply.",
    "outcome": "passed",
    "evidence_ref": ".adl/local/issue345/plancheck/terraform.tfplan"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject patch hygiene defects.",
    "outcome": "passed",
    "evidence_ref": "git diff"
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
