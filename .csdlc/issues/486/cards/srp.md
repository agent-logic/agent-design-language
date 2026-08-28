# Structured Review Prompt

Template: 1.0.0

Issue: 486

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/486
.csdlc/prepared/issues/486
infra/aws/bootstrap
docs/operations/cloud/aws/terraform-bootstrap/AWS_TERRAFORM_BOOTSTRAP_RUNBOOK.md
docs/milestones/v0.92.1/evidence/cloud/aws-c/state-isolation-register.md
docs/milestones/v0.92.1/evidence/cloud/aws-c/run-terraform-bootstrap-readbacks.sh

## Prompts

- Does the bootstrap avoid copying or dual-owning existing website, DDNS, public-edge, or workload state?
- Are backend encryption, versioning, locking, recovery, and provider pins proven?
- Is the deployment role boundary least-privilege for this bootstrap scope?
- Does retained evidence avoid credentials and avoid overclaiming Runtime deployment or CloudFormation retirement?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer did not run the AWS readback lane, inspect credentials, perform cloud mutations, apply Terraform, publish, merge, or update lifecycle/GitHub state.
- Live AWS apply/readback remains gated on explicit operator approval with the approved agent-logic-admin profile.

## Review Result

Revision: Some("git-blake3:150b9e1bae67eb18521fd273c4005f0beb3c6a19:cf87a71fc6eb89ba98c4ed16e8aa81c3e9ab165c752173da5661b5bc81168d38")

Reviewer: Some("fresh-session:d4e0a850-acec-4d66-a4f0-a49e67203aac")

Result: pass
