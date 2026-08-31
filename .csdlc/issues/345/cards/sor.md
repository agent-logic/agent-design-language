# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the optional AWS GPU Shepherd proof runner with deterministic no-mutation contract proof, actual guest SSM proof choreography for the separately authorized paid lane, exact reviewed-HEAD launch gating, and deferred live AWS execution gates.

## Artifacts

- adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .csdlc/evidence/345/issue345-runner-contract.log
- .csdlc/evidence/345/issue345-diff-hygiene.log

## Execution

- Added issue-owned preflight, paid run, and owner-bound cleanup command for the AWS GPU Shepherd portability proof.
- Strengthened account, instance-profile role/policy, no-ingress security group, issue-scoped deadline reaper target, immutable artifact manifest/object-version, DLAMI/subnet, quota, price, stale-compute, and deadline predicates before paid launch.
- Required the paid-run --commit value to match the currently checked-out reviewed HEAD before any launch attempt.
- Upgraded the paid path to resolve a DLAMI and subnet, launch one tagged On-Demand GPU instance, wait for SSM, run a guest bootstrap proof with runtime/toolchain artifact prerequisites, verify GPU residency and governed real-model Shepherd proof JSON, retain the lock-version hash before cleanup, and cleanup owner-tagged resources.
- Added deterministic fake-AWS contract tests proving read-only preflight, fail-closed paid-run predicates, IAM drift rejection, stale-valid-SHA rejection, lock collision behavior, fake successful SSM guest proof, post-launch failure cleanup, cleanup owner-token guard, no real paid launch, and redacted public output.
- Documented the optional portability boundary, required non-secret preflight inputs, exact reviewed-HEAD paid-run gate, cleanup command, and evidence hygiene.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Run the deterministic fake-AWS contract test for preflight, paid-run gates, exact reviewed-HEAD gating, SSM guest proof choreography, cleanup, and redaction without real AWS mutation.",
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
