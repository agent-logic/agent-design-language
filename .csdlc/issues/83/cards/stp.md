# Structured Task Prompt

Template: 1.0.0

Issue: 83

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and prove the first live Layer 8 operator-to-agent chat vertical slice in the HTML Observatory, including only the minimal design-approved Runtime message, recipient-validation, public-response, schema, and focused test paths.

## Deliverables

- Live local Runtime v3 HTTPS/WSS browser integration
- Production-admitted Shepherd in the live roster with truthful running status
- Selected-Shepherd Layer 8 chat
- Signed Layer 8 message delivery with a correlated public-safe response or policy refusal
- Authorized controls with visible accepted and denied outcomes
- Explicit trust, stale, unavailable, backpressure, and version-mismatch states
- Browser reconnect with bounded replay and no duplicate application
- Live Playwright validation entrypoint and retained browser evidence
- adl/tools/validate_v092_html_observatory_live.mjs

## Acceptance

1. The local browser renders the current Runtime v3 roster with Shepherd present as an installed running agent backed by successful production-adapter admission
2. The operator can select Shepherd and send an ordinary signed Layer 8 message through canonical ingress, receiving a correlated public-safe response or policy refusal
3. Writes require authenticated authority and refusal cases remain denied before and after reconnect
4. TLS trust, origin refusal, version mismatch, stale data, backpressure, and Runtime unavailability are visible and never presented as live success
5. Reconnect uses bounded backoff and cursor continuity without duplicate event application or command replay
6. Live local browser proof exercises the real Shepherd roster, ordinary chat, redaction, refusal, disconnect, and reconnect without fixture substitution; no public exposure, ACM setup, or legacy birthday issue is required
7. No files outside the design-approved HTML, focused Runtime, API schema, test, validator, and issue lifecycle paths change during implementation

## Dependencies

- #5800 trusted browser HTTPS is terminal
- #5820 stable Runtime launch and API behavior is terminal
- #5832 versioned ACIP/A2A and WSS contract is terminal
- #92 supplies the Runtime TLS implementation baseline for local trusted development proof
- #5837 may consume the completed browser hooks for shared restart coordination but does not gate this lane

## Inputs

- AGENTS.md
- docs/milestones/v0.92/features/OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md
- docs/api/runtime-v3/v1/observatory.openapi.json
- demos/html-observatory/runtime-v3.config.json
- demos/html-observatory/index.html
- demos/html-observatory/app.js
- demos/html-observatory/styles.css
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/ingress.rs
- adl-runtime-kernel/tests/assembly.rs
- adl-runtime-kernel/tests/control.rs
- adl-runtime-kernel/tests/openapi_contract.rs
- adl/tools/test_html_observatory.sh
- adl/tools/validate_v092_html_observatory_live.mjs

## Non Goals

- Unity client implementation or proof
- Runtime launch, TLS, WSS transport, or general authentication redesign
- Durable conversation sessions, history, search, rooms, notifications, or cross-Polis messaging owned by #110 children
- Cross-client restart coordination
- Serving UI assets from Runtime
- AWS or provider work
