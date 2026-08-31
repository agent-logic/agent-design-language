# Structured Output Record

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implement the optional AWS GPU Shepherd proof runner with deterministic no-mutation contract proof and deferred paid/live AWS execution gates.

## Artifacts

- adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
- adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
- docs/operations/cloud/aws/shepherd-gpu-proof/README.md
- .csdlc/evidence/345/runner-contract.log

## Execution

- Added issue-owned preflight, paid run, and owner-bound cleanup command for the AWS GPU Shepherd portability proof.
- Added deterministic fake-AWS contract tests proving read-only preflight, fail-closed paid-run predicates, lock collision behavior, cleanup owner-token guard, no paid launch, and redacted public output.
- Documented the optional portability boundary, required non-secret preflight inputs, paid execution authorization gate, cleanup command, and evidence hygiene.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Run exact diff hygiene.",
    "outcome": "passed",
    "evidence_ref": "issue345-diff-hygiene.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "purpose": "Run the exact deterministic fake-AWS contract test for the issue-owned runner.",
    "outcome": "passed",
    "evidence_ref": "issue345-runner-contract.log"
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
