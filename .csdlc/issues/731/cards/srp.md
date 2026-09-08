# Structured Review Prompt

Template: 1.0.0

Issue: 731

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

Exact commit ee7aa44ff956729e91a504f3d19686c34460b40d on branch codex/731-gcp-d1-private-foundation-non-gpu-disposal, rebased onto origin/main after the operator merged PR #739 at cf8bc136a69501d9fd1e2f9cb5776b28f22ca763.
Follow-up remediation only: .csdlc/prepared/issues/731/validate-gcp-d1-authorization-packet.sh, .csdlc/prepared/issues/731/run-gcp-d1-foundation-apply-and-readback.sh, .csdlc/prepared/issues/731/run-gcp-d1-disposable-workload-proof.sh, .csdlc/prepared/issues/731/validate-gcp-d1-remediation-guards.sh, .csdlc/evidence/731/remediation-guards/**, and #731 SOR/SRP/index truth.
Verify reviewer fresh-session:b46c30c4-d885-4795-86e3-38a06bf88160 P1/P2 findings are actually resolved: every live gcloud mutation/readback/cleanup path uses the authorized impersonated service account from the packet, and SOR records the current typed validate/doctor evidence truthfully.
Verify authorization freshness, max 30-minute cleanup lifetime, expired-deadline rejection, and service-account impersonation enforcement before any live mutation path.
Verify disposable workload cleanup is armed before create, EXIT cleanup handles injected partial-create and post-create failures, the deadline reaper uses the authorized identity, success-path readiness and metadata service-account readback execute, and post-delete residue checks cover instances, disks, addresses, forwarding rules, firewall overrides, VM IAM, Terraform run labels, and storage objects/all versions.
Verify foundation apply/readback impersonation, exact project/subnet/firewall/OS Login/IAM/bucket/logging readbacks, and rollback-on-failure.
Verify retained evidence is no-live mock/local proof at the exact head; historical live GCP evidence remains historical only and no fresh live GCP mutation is claimed.
Verify diff hygiene and typed validate/doctor are clean, and evaluate whether a follow-up PR is appropriate because PR #739 was already merged by the operator before this remediation landed.

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
