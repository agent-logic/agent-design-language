# Structured Intent Prompt

Template: 1.0.0

Issue: 258

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Split #203 and publish the authority-store security boundary as a focused runtime remediation slice.

## Required Outcome

Raw certificate, lease, and fencing store access is sealed behind typed authority access tokens; authority-bound adapter and published receipt view are reviewable without the broader #203 transport/peripheral/lifecycle monolith.

## Scope

- adl-runtime/src/distributed/authority_store_adapters.rs
- adl-runtime/src/distributed/certificates.rs
- adl-runtime/src/distributed/lease.rs
- adl-runtime/src/distributed/fencing.rs
- direct raw-store test fixture callers required to compile after sealing

## Authority

- Raw store mutation/open APIs require typed authority access tokens.
- Production publication still requires typed C-SDLC review and publish; Gemini review is advisory evidence only.
- No broad hosted/coverage jobs.

## Assumptions

- none

## Operator Constraints

- Use bound FastWork worktree.
- Use typed C-SDLC v2 lifecycle and GitHub routes.
- Do not use raw gh.
- Leave closeout/background bookkeeping to the background lane.
