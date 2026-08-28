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
- Short-lived impersonation as preferred execution
- Operator-approved local static key bootstrap path
- Provider pinning, saved-plan review, and local-state cleanup

## Authority

- Use company GCP project cs-host-377d41e71a824f92802120
- Use bootstrap service account tf-bootstrap@cs-host-377d41e71a824f92802120.iam.gserviceaccount.com
- Prefer short-lived service-account impersonation for steady-state Terraform execution
- Use the operator-approved static key only from /Users/daniel/keys/gcp-tf-bootstrap-cs-host-377d41e71a824f92802120-20260827.json when a key-backed bootstrap command is explicitly selected
- Never print, copy, commit, or retain service-account private-key contents, token contents, ADC database contents, or refresh tokens
- GCP mutation is limited to the accepted host project and bootstrap resources

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes
- Bind beneath /Volumes/FastWork/adl-worktrees before tracked implementation edits
- Use standard runners only for hosted CI
- Record the key creation and org-policy window truthfully without retaining secrets
- Do not absorb GCP-C runtime, GCP-D security, GPU, AWS, or public-edge work
