# Structured Review Prompt

Template: 1.0.0

Issue: 731

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Exact commit 992d0485c593926617a9474b194fa8758ac7b64e on branch codex/731-gcp-d1-private-foundation-non-gpu-disposal.
Terraform provider-correct OS Login metadata resource change in infra/gcp/platform/main.tf.
No-mutation static readiness, saved Terraform plan denominator, read-only GCP preflight, pending authorization packet, and SOR truth under .csdlc/issues/731, .csdlc/prepared/issues/731, and .csdlc/evidence/731.
Verify the PR must not claim full #731 completion, must not use a closing keyword, and must preserve the explicit live GCP apply/run stop boundary pending exact operator authorization.

## Prompts

- Does the saved Terraform plan exactly match the #731/#493 denominator with no public exposure, broad IAM, replacement, or extra resources?
- Are live operations gated by fresh explicit authorization naming every required identity, digest, deadline, and rollback command?
- Does the proof use short-lived impersonation without static keys or credential exposure?
- Does the disposable VM prove RUNNING without external IP, then destroy the VM and boot disk within the authorized window?
- Does independent zero-residue readback cover every required resource class and exact run label?
- Does the final SOR distinguish read-only preflight, live authorized mutation, cleanup, deferred CI, and non-goals truthfully?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
