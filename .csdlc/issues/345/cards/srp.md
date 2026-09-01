# Structured Review Prompt

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/tests/shepherd_local_model.rs
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
    "id": "F-345-14",
    "severity": "p2",
    "summary": "The runbook incorrectly requires the reviewed substantive commit to equal checkout HEAD instead of describing the clean reviewed or published metadata-tail lifecycle head.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-345-15",
    "severity": "p2",
    "summary": "Local proof does not execute the typed-review equality and substantive-drift guard or prove authorized SG, AMI, and subnet values reach launch arguments.",
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

- No paid GPU launch has yet exercised the guest and cleanup path.
- The bounded runner does not establish 24/7 service readiness.

## Review Result

Revision: Some("git-blake3:87c82f0b8049004b9f8fb755cddf398ef05aaf6f:e9425c9cb9c064167e6abf85c9ca7f8f59fd876102de94d1f43c0c4062911a62")

Reviewer: Some("fresh-session:/root/issue_345_launch_ready_review")

Result: changes_required
