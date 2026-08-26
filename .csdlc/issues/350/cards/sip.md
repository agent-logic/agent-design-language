# Structured Intent Prompt

Template: 1.0.0

Issue: 350

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Expose one sealed authenticated projection that supplies #274 with quorum, identity, lineage, operation, and committed deadline truth cross-bound to the exact #272 serving cut.

## Required Outcome

Only replicated published quorum authority cross-bound to the exact verified #272 cut can yield the unconstructible redacted Observatory authority projection; mismatch, caller substitution, stale state, corrupt restore, and deadline ambiguity fail closed.

## Scope

- Persist and restore the committed inclusive authority deadline
- Add a sealed Observatory-specific authenticated projection and cross-binding validator
- Focused canonical/noncanonical encoding, 2x2 cross-pair, quorum snapshot mutation, deadline, replay, restart, legacy-state, and redaction proof

## Authority

- Projection is returned only from replicated published authority and exact VerifiedServingAuthorityCut
- No caller-provided identity, quorum, lineage, operation, deadline, clock, or cut field is authority
- #274 state-machine behavior remains out of scope and unbound
- #273 Shepherd behavior and #203/#205/#275 surfaces remain unchanged

## Assumptions

- none

## Operator Constraints

- Base is exact origin/main 24c049c4e3ea71d0ad0633fc90ef35bfd57c2c4a
- Use only typed v2 lifecycle operations and FastWork worktrees
- No #274 bind or product edit until #350 is terminal and ancestral
- No optional or paid runner
