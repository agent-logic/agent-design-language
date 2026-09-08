# Structured Intent Prompt

Template: 1.0.0

Issue: 740

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Repair #730/#738 GCP-B1 proof so impersonated remote-state evidence is credential-safe, authorization-bound, legacy-key-disposition complete, and Terraform-backend proven.

## Required Outcome

Issue #740 closes the post-merge #730 acceptance gaps with safe credential handling, authenticated authorization binding, legacy-key disposition, Terraform-backend canary proof, and exact-head review.

## Scope

- Repair merged #730 GCP-B1 live proof script and evidence contract.
- Add automated local validation for credential-retention prevention and backend-canary evidence shape.
- Run bounded live GCP proof using short-lived impersonation and retained redacted evidence.

## Authority

- This is a post-merge corrective issue for #730/#738 and must not claim retroactive pre-merge review truth.
- Cloud mutation remains bounded to project cs-host-377d41e71a824f92802120, region us-west2, bucket adl-tf-state-cs-host-377d41e71a824f92802120, and bootstrap service account impersonation.
- No static service-account key may be created, downloaded, selected, or required.
- Do not touch #446 or unrelated issue worktrees.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes.
- Work in a bound FastWork issue worktree for implementation.
- Use repo/worktree paths for diagnostics; do not write issue evidence to /private/tmp.
- Use GCP only within the authorized #740 scope and do not expose credential contents.
