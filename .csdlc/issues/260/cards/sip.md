# Structured Intent Prompt

Template: 1.0.0

Issue: 260

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prepare the #260 execution packet for migrating non-transport distributed Runtime callers to the governed authority adapter facade after #259 is terminal.

## Required Outcome

After #259 terminal, the #260 owner can bind immediately to migrate migration, recovery, placement, projection, resource-weather, snapshot-catalog, capability-advertisement, and related distributed Runtime callers without absorbing #258, #259, or #203 ownership.

## Scope

- adl-runtime/src/distributed migration callers outside #259 transport ownership
- adl-runtime/src/distributed recovery callers outside #259 transport ownership
- adl-runtime/src/distributed placement callers outside #259 transport ownership
- adl-runtime/src/distributed projection callers outside #259 transport ownership
- adl-runtime/src/distributed resource-weather callers outside #259 transport ownership
- adl-runtime/src/distributed snapshot-catalog callers outside #259 transport ownership
- adl-runtime/src/distributed capability-advertisement callers outside #259 transport ownership
- focused Runtime tests for the touched caller surfaces

## Authority

- #260 preparation may complete before #259 terminal only as initialized/ready design-card state; bind and source implementation remain gated.
- #260 must consume the #258 authority-store boundary and #259 governed transport binding; it must not redefine either.
- Parent #203 owns only final integration after #258/#259/#260 are terminal, reconciled, and ancestral.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle and card routes only.
- Do not bind or create the #260 worktree until #259 is terminal, reconciled, and ancestral.
- Do not touch #203 or #258 worktrees during #260 preparation.
- Do not implement #258 authority-store boundary changes.
- Do not implement #259 governed transport changes.
- Keep primary main tracked clean outside authorized initialized #260 staging.
