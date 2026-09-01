# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

The eighth Terraform attempt never created an instance: AWS repeatedly returned InsufficientInstanceCapacity for g6.xlarge in us-west-2a and reported availability in us-west-2b, 2c, and 2d. The controller was interrupted after the diagnosis to stop useless retries, then destroyed every temporary Terraform prerequisite and released the lock; direct inventory confirmed zero instances and volumes. The runner now accepts one explicit preferred subnet, but validates that it belongs to the authorized VPC, offers the GPU type, and has the required public route and network ACL before binding its hash in preflight and authorization. Read-only preflight passed for the public us-west-2b subnet. Final paid proof remains pending exact-head review and relaunch.

## Artifacts

- adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
- .adl/local/issue345/adl-issue345-20260901-131309/preflight.json
- .adl/local/issue345/adl-issue345-20260901-131309/cleanup-on-exit.json

## Execution

- Retained the reviewed two-node Terraform topology, one SSH key, mandatory /32 SSH ingress, private Ollama, immutable artifacts, single-use authorization, and USD 20 hard ceiling.
- Observed AWS reject g6.xlarge creation in us-west-2a for insufficient capacity before any instance or volume was created.
- Stopped the capacity retry loop after AWS explicitly identified available alternate AZs, then confirmed Terraform destroyed all temporary roles, profiles, policies, key, and security groups and released the lock.
- Added an optional explicit preferred-subnet input for capacity routing without weakening VPC, GPU-offering, route-table, network-ACL, preflight-digest, or authorization binding checks.
- Proved the preferred us-west-2b public subnet through live read-only preflight with a new authorization-bound subnet digest.
- Confirmed direct AWS inventory returned zero issue instances and zero issue volumes after cleanup.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Prove the no-paid topology, preferred-subnet fail-closed markers, authorization, recovery, and cleanup contracts.",
    "outcome": "passed",
    "evidence_ref": "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue345_aws_gpu_shepherd_proof.sh",
      "preflight"
    ],
    "purpose": "Validate and bind the selected us-west-2b public subnet, AMIs, network, SSH key and CIDR, model set, Terraform source, prices, quota, and zero stale compute without mutation.",
    "outcome": "passed",
    "evidence_ref": ".adl/local/issue345/adl-issue345-20260901-131309/preflight.json"
  },
  {
    "command": [
      "bash",
      "adl/tools/run_issue345_aws_gpu_shepherd_proof.sh",
      "run",
      "--commit",
      "904ffa5867297455252350b1601ab9292753dc00",
      "--run-id",
      "adl-issue345-20260901-131309",
      "--authorization-file",
      ".adl/local/issue345/operator-authorization-polis-domain-fix.json",
      "--execute"
    ],
    "purpose": "Attempt the reviewed two-node proof; AWS refused GPU capacity before instance creation and the controller cleaned all Terraform prerequisites.",
    "outcome": "failed",
    "evidence_ref": ".adl/local/issue345/adl-issue345-20260901-131309"
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
