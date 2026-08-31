# Structured Review Prompt

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl/tools/run_issue345_aws_gpu_shepherd_proof.sh
adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh
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

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review was read-only and did not run real AWS preflight or paid GPU execution; it confirmed local runner/test contract and the deferred-live boundary only.
- Paid GPU execution still requires a separate exact-run operator authorization naming the reviewed revision, run id, deadline, and maximum cost.

## Review Result

Revision: Some("git-blake3:549effd3da7dc399e2de9ed8517eadcc34e01a56:a9bc78cd08237731727d0792860748548ca3535ab2306e3c6c63b41ba78c3424")

Reviewer: Some("fresh-session:/root/issue_345_review_r4")

Result: pass
