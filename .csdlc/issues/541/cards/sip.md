# Structured Intent Prompt

Template: 1.0.0

Issue: 541

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Correct stale contributor-facing workflow documentation so it matches the current Gate 10D2 typed C-SDLC v2 authority and canonical repository identity.

## Required Outcome

Docs no longer route current lifecycle work through retired `adl_pr_cycle` or `pr ready` guidance, and instead state the typed v2 route, canonical repository, legacy remote boundary, installed binary location, and root/worktree expectations.

## Scope

- docs/onboarding.md
- adl/tools/README.md
- docs/tooling

## Authority

- Issue authority is agent-logic/agent-design-language#541
- Current lifecycle authority is Gate 10D2 typed C-SDLC v2
- Do not revive v1 wrappers or compatibility lifecycle routes
- Do not bind or implement during issue initialization
- Do not mutate runtime behavior or Git remotes

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle handling
- Keep root checkout on main
- Do not hand-edit cards
- Use focused docs validation
- Preserve canonical versus legacy repository truth
