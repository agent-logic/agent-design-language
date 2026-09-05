# Structured Task Prompt

Template: 1.0.0

Issue: 693

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Repair only the brittle model-output-to-governed-A2A action selection boundary introduced after #662 and #675.

## Deliverables

- reliable first-class A2A action selection boundary
- governed dispatch integration preserving existing authority
- isolated live-style end-to-end acceptance
- compatibility and failure-path proof

## Acceptance

1. AC-1: Production conversation ingress selects A2A from ordinary model-style output without requiring perfect JSON.
2. AC-2: Runtime constructs and validates the governed initiation with canonical sender recipient work conversation turn and correlation identity.
3. AC-3: Recipient executes through its configured provider route and the correlated terminal result is distinct from the initiator's operator reply.
4. AC-4: Authoritative Observatory/feed activity exposes initiation dispatch and result.
5. AC-5: Missing or stale recipient unauthorized sender replay cancellation and provider failure retain explicit governed outcomes.
6. AC-6: Isolated production-ingress live-style proof passes repeatedly with non-perfect model output.
7. AC-7: Existing #662 primitive and compatible #675 tests remain green.
8. AC-8: Focused validation diff hygiene and independent exact-head review pass before non-draft publication.

## Dependencies

- #662 and PR #668 merged
- #675 and its implementation merged

## Inputs

- agent-logic/agent-design-language#693
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/control.rs
- runtime conversation and Observatory tests
- #662
- #675

## Non Goals

- Unrestricted autonomous messaging
- Broadcast or recursive fan-out
- Transcript-history restoration
- Live Wuji restart or config mutation
- Cloud/provider spend
- Changes to #686 or #689
