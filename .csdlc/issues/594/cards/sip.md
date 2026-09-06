# Structured Intent Prompt

Template: 1.0.0

Issue: 594

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Regularly archive redacted Runtime v3 logs to private encrypted S3 storage without coupling archival availability to Runtime readiness.

## Required Outcome

A Terraform-owned S3 archive and least-privilege Vector delivery path provide identity-partitioned bounded uploads, observable failure, and live retrieval proof while the Runtime remains operational during S3 failure.

## Scope

- adl-runtime-kernel/src/observability/vector.rs
- adl-runtime-kernel/src/observability.rs
- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/tests/observability.rs
- adl-runtime-kernel/tests/configuration.rs
- adl-runtime-kernel/tests/guardian_soak.rs
- infra/aws/runtime/log-archive
- .csdlc/prepared/issues/594
- .csdlc/evidence/594
- .csdlc/issues/594

## Authority

- Issue authority is agent-logic/agent-design-language#594
- CloudWatch health and SSM recovery remain independent and are not replaced
- Live AWS mutation requires separate operator authorization and the agent-logic-admin business profile
- C-SDLC v3 is not live lifecycle authority before explicit operator-reviewed cutover

## Assumptions

- none

## Operator Constraints

- Never write tracked issue work on main
- Use Terraform rather than CloudFormation
- Never archive secrets, credentials, unredacted payloads, or machine-local paths
- Do not make S3 availability a Runtime readiness dependency
