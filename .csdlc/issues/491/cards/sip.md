# Structured Intent Prompt

Template: 1.0.0

Issue: 491

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one recoverable GCP Terraform bootstrap.

## Required Outcome

One recoverable GCP state project backend and deployment-identity bootstrap.

## Scope

- GCP host-project Terraform backend bootstrap
- GCS remote-state privacy, versioning, auditability, and recovery
- Company-controlled Terraform bootstrap service account
- Operator-approved local static key bootstrap path for this sprint execution
- Provider pinning, saved-plan review, and local-state cleanup
- Future non-key identity hardening is outside #491 and must not block the sprint bootstrap

## Authority

- Use company GCP project cs-host-377d41e71a824f92802120
- Use bootstrap service account tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com
- Use the operator-approved static key only from /Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json for #491 sprint bootstrap commands
- Never print, copy, commit, or retain service-account private-key contents, token contents, ADC database contents, or refresh tokens
- GCP mutation is limited to the accepted host project and bootstrap resources
- Future non-key identity work is out of scope for #491 and must not block this sprint bootstrap

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes
- Bind beneath /Volumes/FastWork/adl-worktrees before tracked implementation edits
- Use standard runners only for hosted CI
- Record the key creation and org-policy window truthfully without retaining secrets
- Do not absorb GCP-C runtime, GCP-D security, GPU, AWS, or public-edge work
