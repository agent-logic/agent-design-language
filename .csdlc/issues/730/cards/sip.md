# Structured Intent Prompt

Template: 1.0.0

Issue: 730

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Correct #491 with an impersonation-only recoverable Terraform bootstrap.

## Required Outcome

Short-lived impersonation creates and proves the exact private remote-state backend, recovery, and zero local-state residue.

## Scope

- infra/gcp/bootstrap
- GCP-B runbook and focused validators
- gcp-b1 retained evidence

## Authority

- Only project cs-host-377d41e71a824f92802120 and us-west2.
- No GCP mutation without the issue's exact operator authorization.
- No static service-account key is a supported execution path.

## Assumptions

- none

## Operator Constraints

- Never write on main.
- Do not expose credentials.
- Do not mutate compute, network, DNS, billing, organization, folder, project, or unrelated IAM.
