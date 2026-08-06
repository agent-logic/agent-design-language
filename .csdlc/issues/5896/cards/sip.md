# Structured Intent Prompt

Template: 1.0.0

Issue: 5896

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Migrate every pre-#5886 bound record with incomplete topology into truthful current C-SDLC v2 state.

## Required Outcome

All affected records receive a deterministic disposition and open never-bound issues become normally bindable without restoring claims.

## Scope

- The exact canonical bound-plus-incomplete-topology cohort
- One typed migration operation under the current csdlc-issue owner
- Per-issue disposition evidence and focused regression tests

## Authority

- Canonical .csdlc issue records and verified Git worktree topology
- Explicit live GitHub issue-state snapshot supplied as typed input
- No product-issue binding or implementation

## Assumptions

- none

## Operator Constraints

- Treat this as state migration, not tooling reliability work
- Do not restore claims, leases, heartbeats, or preparation state
- Do not hand-edit canonical issue records
- Keep root main clean
