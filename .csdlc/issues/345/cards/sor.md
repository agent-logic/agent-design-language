# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the optional AWS GPU Shepherd proof runner with deterministic no-mutation contract proof, actual guest SSM proof choreography for the separately authorized paid lane, and deferred live AWS execution gates.

## Artifacts

- adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .csdlc/evidence/345/issue345-runner-contract.log
- .csdlc/evidence/345/issue345-diff-hygiene.log

## Execution

- Added issue-owned preflight, paid run, and owner-bound cleanup command for the AWS GPU Shepherd portability proof.
- Strengthened account, instance-profile role/policy, no-ingress security group, immutable artifact manifest/object-version, AMI/subnet, quota, price, stale-compute, and deadline-reaper predicates before paid launch.
- Upgraded the paid path to resolve a DLAMI and subnet, launch one tagged On-Demand GPU instance, wait for SSM, run a guest bootstrap proof, verify GPU residency and governed real-model Shepherd proof JSON, and cleanup owner-tagged resources.
- Added deterministic fake-AWS contract tests proving read-only preflight, fail-closed paid-run predicates, IAM drift rejection, lock collision behavior, fake successful SSM guest proof, post-launch failure cleanup, cleanup owner-token guard, no real paid launch, and redacted public output.
- Documented the optional portability boundary, required non-secret preflight inputs, paid execution authorization gate, cleanup command, and evidence hygiene.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Run the deterministic fake-AWS contract test for preflight, paid-run gates, SSM guest proof choreography, cleanup, and redaction without real AWS mutation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/345/issue345-runner-contract.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Reject patch hygiene defects.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/345/issue345-diff-hygiene.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
