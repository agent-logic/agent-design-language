# Structured Intent Prompt

Template: 1.0.0

Issue: 356

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Expose only minimal read-only redacted accessors on the sealed terminal #350 Observatory authority projection so #274 can consume authenticated state without caller-minted authority.

## Required Outcome

#274 can read the sealed projection's existing redacted identity references and scalar monotonicity/receipt fields, while private construction and all raw quorum, membership, OwnerCommit, lease, artifact, and secret authority remain inaccessible.

## Scope

- Read-only accessors on VerifiedObservatoryAuthorityProjection
- Focused accessor and mismatch/redaction proof
- Exact-head review, hosted CI, typed terminal finish

## Authority

- Projection construction remains private and only the #350 verifier can mint it
- No raw quorum, membership, OwnerCommit, lease, artifact, or secret authority is exposed
- #274 remains unbound until #356 is terminal and ancestral

## Assumptions

- none

## Operator Constraints

- Base is exact terminal #350 merge 5bff0099858f005bcc045b0aa7548be4892a2acb
- Use only the bound FastWork issue worktree
- No optional or paid runner
