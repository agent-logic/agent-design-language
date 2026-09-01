# Structured Review Prompt

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
adl/tools/issue345_aws_gpu_prerequisites.cloudformation.yaml
docs/operations/cloud/aws/shepherd-gpu-proof/README.md
.csdlc/evidence/345
.csdlc/issues/345

## Prompts

- Can any path launch paid compute before account, authorization, cost, deadline, lock, IAM, no-ingress, artifact, quota, and reaper predicates are proven?
- Can interruption or a partial AWS response leave an unowned instance, volume, lock, or cleanup ambiguity?
- Can a stale source, artifact, runner, backend, or retained response be presented as current real-model GPU proof?
- Can public evidence expose credentials, AWS identifiers, private paths, prompts, responses, or environment values?
- Does the issue remain an optional portability proof without changing Runtime/local Shepherd acceptance or absorbing #256?

## Findings

[
  {
    "id": "F-345-8",
    "severity": "p1",
    "summary": "The live model test compares the plain manifest digest with Ollama's sha256-prefixed API digest and will reject the paid run.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-345-9",
    "severity": "p1",
    "summary": "The single-use authorization marker hashes raw JSON bytes, so semantically identical reformatted authorization can be replayed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-345-10",
    "severity": "p1",
    "summary": "Paid authorization does not bind the business AWS account or immutable infrastructure and artifact proof inputs.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-345-11",
    "severity": "p2",
    "summary": "AC-8 overstates executable negative coverage because several required guards are only asserted by source-text search.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No paid GPU instance was launched, so Guardian, Ollama, model, ACC, residency, and cleanup paths remain dynamically unproved.
- The bounded proof runner does not establish continuous 24/7 service readiness.

## Review Result

Revision: Some("git-blake3:8b3288aed60b6af2623bab64a477f60d3c785a26:8110ab0b1fc545af711c0d4359942c153dca225498363b5207decfef95e246d3")

Reviewer: Some("fresh-session:/root/issue_345_full_remediation_review")

Result: changes_required
