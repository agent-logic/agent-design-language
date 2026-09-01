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
    "id": "F-345-12",
    "severity": "p1",
    "summary": "Paid authorization accepts any reviewed-revision suffix instead of the exact assigned immutable revision.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-345-13",
    "severity": "p1",
    "summary": "The runner re-resolves AMI and subnet after authorization verification, leaving a launch-time TOCTOU path to unbound infrastructure.",
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

- No paid GPU instance was launched, so the full guest and cleanup lane remains dynamically unproved.
- This bounded proof runner does not establish 24/7 service readiness.

## Review Result

Revision: Some("git-blake3:37b4fe46b16ffa76a16f1f2f56e5773de1df60ef:de5bc7e6aaaf8bc96a56753dd48919ea672d88dcfa2b778292458dd718b7da37")

Reviewer: Some("fresh-session:/root/issue_345_final_review")

Result: changes_required
