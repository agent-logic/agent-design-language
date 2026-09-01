# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented and read-only-preflighted the hardened optional AWS GPU proof runner; paid GPU guest execution remains pending fresh exact-head review, typed publication, and the retained single-use authorization.

## Artifacts

- adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/issue345_aws_gpu_prerequisites.cloudformation.yaml
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .csdlc/evidence/345/issue345-runner-contract.log
- .csdlc/evidence/345/issue345-live-preflight.json
- .csdlc/evidence/345/issue345-diff-hygiene.log

## Execution

- Require and attest a configured set of at least two models, one real governed Shepherd result per model, and simultaneous nonzero GPU residency for the complete set.
- Exercise Guardian-supervised Runtime v3 lifecycle plus six real long-lived Runtime agents whose Ollama proposals execute through UTS, ACC, the Freedom Gate, and runtime.observe, while explicitly declining to claim a nonexistent Runtime-v3-to-Ollama transit path.
- Make retained paid authorization single-use through a create-only versioned S3 consumption marker that cleanup does not delete.
- Verify exact EC2 and Lambda trust policies, exact permission policies, the no-ingress group, immutable artifacts, deadline reaper, quota, price, and absence of issue-tagged instances or volumes.
- Bound worst-case cost across compute, 300-second reaper lag, a 200 GiB gp3 volume, public IPv4, and request overhead under the operator's 20 USD ceiling.
- Keep all real guest, model, ACC, residency, SSM, and cleanup claims deferred to the paid live lane; the local contract test makes no fake-AWS success claim.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Check real-Git, no-fake-AWS runner contracts without paid launch.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/345/issue345-runner-contract.log"
  },
  {
    "command": [
      "env",
      "ADL_ISSUE345_LIVE_PREFLIGHT=1",
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Run the real read-only AWS preflight and seven fail-closed negative checks without launching compute.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/345/issue345-live-preflight.json"
  },
  {
    "command": [
      "aws",
      "cloudformation",
      "validate-template",
      "--template-body",
      "file://adl/tools/issue345_aws_gpu_prerequisites.cloudformation.yaml"
    ],
    "purpose": "Validate the real AWS prerequisite template shape.",
    "outcome": "passed",
    "evidence_ref": "adl/tools/issue345_aws_gpu_prerequisites.cloudformation.yaml"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject patch hygiene defects.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/345/issue345-diff-hygiene.log"
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
