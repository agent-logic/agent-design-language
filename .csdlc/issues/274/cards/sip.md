# Structured Intent Prompt

Template: 1.0.0

Issue: 274

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement quorum-leased Observatory serving eligibility by consuming terminal #350's sealed projection through terminal #356/#358 read-only redacted action, predecessor, identity, and canonical-time accessors.

## Required Outcome

Only an exact successfully verified sealed Observatory authority projection can acquire, renew, transfer, revoke, or expire eligibility; every transition is deterministic, fenced, replay-safe, restart-safe, redacted, and fail-closed without caller-minted authority.

## Scope

- Observatory-only quorum serving-eligibility state machine
- Acquire, renew, fenced atomic transfer, revoke, and expiry semantics
- Exact sealed #350 projection and #356/#358 accessor bindings for authenticated action, predecessor, redacted identity references, log index, generation, fence, result digest, signer digest/count, and full canonical deadline/finalization time
- Deterministic redacted Observatory transition receipt and projection
- Focused Observatory tests, strict Clippy, scope proof, fresh exact-head review, and ordinary hosted CI after future bind

## Authority

- #205 remains coordination-only and owns no product implementation
- #274 consumes only the sealed terminal #350 projection through terminal #356/#358 read-only redacted accessors and cannot mint action, predecessor, authority, or deadline from local state
- #274 owns only a new Observatory-specific module and test; it does not own serving_authority.rs, the Shepherd module/test, or authority_store_adapters.rs
- distributed/mod.rs registration is shared and implementation is serialized after #273 is terminal and ancestral unless a fresh design review proves no registration touch
- Caller DTOs, raw tokens, permits, configuration, peer lists, cached booleans, and local clocks are never authority

## Assumptions

- none

## Operator Constraints

- Preparation base is exact origin/main cd0feef31240b95d344c5ae9b774325506586a5d
- Use only /Volumes/FastWork/adl-worktrees/adl-issue-274-observatory-serving-eligibility-preparation on codex/274-observatory-serving-eligibility-preparation for bootstrap
- Do not touch dirty primary main, #205 parent, #272 worktree, or any #273 preparation or implementation surface
- Approve and bind only after a new fresh UUID design PASS validates terminal #350/#356/#358 authority and exact scope
- A new #119-compliant fresh design review must explicitly prove module/test disjointness and the shared registration serial gate before any later approval or bind
- No optional or paid runner is authorized
