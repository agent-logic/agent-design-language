# Structured Task Prompt

Template: 1.0.0

Issue: 83

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and prove only the HTML Observatory consumer paths owned by issue #83.

## Deliverables

- Live Runtime v3 HTTPS/WSS browser integration
- Authorized controls with visible accepted and denied outcomes
- Explicit trust, stale, unavailable, backpressure, and version-mismatch states
- Browser reconnect with bounded replay and no duplicate application
- Live Playwright validation entrypoint and retained browser evidence
- adl/tools/validate_v092_html_observatory_live.mjs

## Acceptance

1. The browser renders current Runtime v3 snapshots and WSS events with fresh correlation evidence
2. Every exposed menu, control, proof link, and packet link performs real behavior or shows an explicit unavailable state
3. Writes require authenticated authority and refusal cases remain denied before and after reconnect
4. TLS trust, origin refusal, version mismatch, stale data, backpressure, and Runtime unavailability are visible and never presented as live success
5. Reconnect uses bounded backoff and cursor continuity without duplicate event application or command replay
6. Live browser proof exercises reads, writes, redaction, refusal, disconnect, and reconnect without fixture substitution
7. No files outside the three declared owned paths change during implementation

## Dependencies

- #5800 trusted local browser HTTPS is terminal
- #5820 stable Runtime launch and API behavior is terminal
- #5832 versioned ACIP/A2A and WSS contract is terminal
- #5836 first-birthday interaction surface is terminal before final implementation credit
- #5837 supplies shared restart coordination for final integration

## Inputs

- AGENTS.md
- docs/milestones/v0.92/features/OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md
- docs/api/runtime-v3/v1/observatory.openapi.json
- demos/html-observatory/runtime-v3.config.json
- demos/html-observatory/app.js
- demos/html-observatory/styles.css

## Non Goals

- Unity client implementation or proof
- Runtime API, WSS, TLS, launch, or authentication changes
- Cross-client restart coordination
- Observatory redesign or serving UI assets from Runtime
- AWS or provider work
