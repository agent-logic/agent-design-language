# Structured Intent Prompt

Template: 1.0.0

Issue: 115

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Allow an authenticated Layer 8 operator to convene a bounded explicit set of policy-eligible agents and exchange attributed messages through governed Runtime routing.

## Required Outcome

Runtime owns versioned room, participant, membership, turn-routing, per-recipient delivery, replay, and attributed-response contracts, while the Observatory presents exact participants and deterministic partial outcomes without implicit broadcast or authority widening.

## Scope

- adl-runtime-kernel/src/conversation_rooms.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/src/ingress.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/operations.rs
- adl-runtime-kernel/tests/conversation_rooms.rs
- adl-runtime-kernel/tests/observatory.rs
- adl-runtime-kernel/tests/openapi_contract.rs
- docs/api/runtime-v3/v1/observatory.openapi.json
- demos/html-observatory/app.js
- demos/html-observatory/index.html
- demos/html-observatory/styles.css
- adl/tools/validate_v092_html_observatory_rooms.mjs
- docs/milestones/v0.92/features/GOVERNED_MULTI_AGENT_ROOMS.md
- .csdlc/issues/115
- .csdlc/prepared/issues/115
- .csdlc/evidence/115

## Authority

- Issue and code authority are agent-logic/agent-design-language#115
- Runtime owns room membership, routing, ordering, replay, delivery, attribution, and projection; browser state never grants authority
- Issue #112 remains sole owner of Layer 8 principal, exact multi-recipient policy, refusal, revocation, replay authority, and redacted audit
- Issue #111 remains sole owner of canonical conversation and turn contracts; issue #113 remains sole owner of roster and presence truth
- Issue #83, umbrella #110, dependencies #111-#113, sibling #114, and every other WP-18C child remain read-only

## Assumptions

- none

## Operator Constraints

- Do not bind or implement until #111, #112, and #113 are terminal, merged, ancestral, and ownership-compatible
- Use only C-SDLC v2 owner binaries and typed card editors in the dedicated FastWork context
- Do not publish, push, open a PR, merge, close, or mutate #83, #110, #111, #112, #113, or another issue during preparation
- No implicit broadcast, browser-selected authority, cross-Polis routing, private cognition, credentials, secrets, or raw provider payloads
- Every fan-out bound, replay identity, per-recipient outcome, event buffer, retry, timeout, response size, and UI list must be explicit and proven
