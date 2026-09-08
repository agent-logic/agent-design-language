# Structured Review Prompt

Template: 1.0.0

Issue: 731

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

Exact commit 78490bd7bc8f0815914469fb4c0323d23437dd13 on branch codex/731-gcp-d1-private-foundation-non-gpu-disposal.
Terraform private-foundation implementation in infra/gcp/platform/main.tf, variables.tf, terraform.tfvars.example, and .terraform.lock.hcl.
Issue-owned #731 validators and evidence under .csdlc/prepared/issues/731, .csdlc/evidence/731, and .csdlc/issues/731.
Verify the live mutation was explicitly authorized in .csdlc/evidence/731/mutation-authorization-request.json and bound to plan digest e20e83dcafc5d8f7cd963660bbcc249be6fea81ca76d841e8e2e298a4426f6df.
Verify the successful foundation apply/readback proves the accepted denominator without public exposure, broad IAM, GPU, NAT/LB/DNS, or unintended workload resources.
Verify the disposable workload evidence proves exactly one labelled private e2-micro with no external IP, auto-delete standard boot disk, successful delete, and zero residual instances, run-labelled instances, disks, or addresses.
Verify SOR/SRP truth is current for publication and that the PR may use Closes #731 only if the live proof satisfies the issue contract.

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
