# Structured Intent Prompt

Template: 1.0.0

Issue: 450

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make the Runtime-kernel Memory Palace the single production authority consumed by resident agents while preserving resident context behavior and exact continuity/digest truth.

## Required Outcome

The live long-lived/resident path constructs agent context from one Runtime-kernel Memory Palace authority through one typed adapter, persists and rehydrates exact digest-bound state, and fails closed on divergence, rollback, gaps, duplicates, forged digests, or incompatible schema.

## Scope

- Runtime-kernel Memory Palace authority and validation contract
- One typed resident adapter replacing duplicate derivation logic
- Resident context construction and artifact integration
- Restart/rehydration continuity and divergence rejection
- Focused production-path and negative tests
- Canonical feature-list and v0.92 evidence truth

## Authority

- adl-runtime-kernel memory_palace owns topology, working set, record admission, authority/canonical/item/packet digests, and rejection semantics
- adl memory_palace becomes a loading and translation adapter only
- long_lived_agent consumes the authoritative packet and does not derive parallel writable truth
- Existing ObsMem, identity, continuity, trace, and redaction authorities are reused
- No third Memory Palace system is introduced

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only
- Use an issue-bound FastWork worktree before tracked implementation
- Leave #446 and all unrelated staging/worktrees untouched
- Preserve existing resident behavior unless an explicit reviewed schema migration is required
- Do not claim production integration from module existence or unit tests alone
