# Structured Intent Prompt

Template: 1.0.0

Issue: 508

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one final distributed Runtime qualification decision.

## Required Outcome

One final distributed Runtime qualification decision for requirements 185 through 187.

## Scope

- Map requirements 185 through 187 to one DRT-C qualification decision
- Prove identity, provider, and transport failures fail closed
- Bind Runtime-authentic Observatory evidence to the exact Runtime revision
- Bind bounded-soak, synthesis, and cleanup-zero evidence to the exact Runtime revision

## Authority

- Consume #507 DRT-B terminal merge d022d6c198669bcbc10cd98bee4d7c8520f9c4d4 as dependency authority
- Do not redesign the Observatory product
- Do not run unbounded soak
- Do not leave residual cloud or local runtime resources
- Do not absorb #509 GCP portability qualification

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes
- Use FastWork worktrees only for tracked execution
- Keep primary main clean
- Use standard CI runners only
- Record any paid or external proof gate truthfully before execution
