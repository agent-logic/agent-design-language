# Structured Intent Prompt

Template: 1.0.0

Issue: 280

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Prove large-Polis Observatory performance and recovery behavior for the exact integrated production-Polis-interface candidate after terminal #111/#112/#113/#114/#115/#116/#265/#270/#271/#276/#277/#278.

## Required Outcome

At one exact integrated candidate revision, the Observatory and Runtime-facing browser path remain bounded and truthful under large rosters, long transcripts, stream pressure, reconnect/restart, offline transitions, and version mismatch. Degradation and recovery are explicit and do not duplicate actions or hide stale state.

## Scope

- Deterministic large-roster and long-transcript Observatory performance proof
- Bounded latency, memory, DOM growth, retained transcript, stream resource, and render-state metrics
- Reconnect, Runtime restart, backpressure, offline, and version-mismatch recovery proof
- Machine-readable #280 metrics and public-safe evidence tied to one exact candidate revision
- Narrow Observatory-only source/test fixes if the proof finds in-scope performance or recovery defects
- Issue-local proof artifacts, validator, exact fresh review, typed publication, required CI, and typed finish

## Authority

- Runtime remains the sole communication, policy, authority, delivery, history, room-routing, and acknowledgement authority
- Browser code may represent degradation/recovery state but must not grant authority, invent acknowledgements, synthesize delivery, approve operations, or hide refusal/stale state
- #280 may record performance/recovery findings or narrowly fix Observatory presentation/state behavior; it must not redefine Runtime contracts or child product semantics
- #279 owns accessibility/responsive UX proof; #281 owns security/privacy/adversarial proof; #282 owns final qualification assembly
- #117 and #110 remain coordination parents and are not #280 implementation owners

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 owners for lifecycle, review, publication, and finish writes.
- Bind implementation only under /Volumes/FastWork/adl-worktrees.
- Do not touch #279, #281, #282, #117 parent, #110 parent, or sibling issue worktrees.
- Do not read credentials or synthesize live provider/runtime proof.
- Do not publish until exact-head fresh review passes and required PR checks are green.
