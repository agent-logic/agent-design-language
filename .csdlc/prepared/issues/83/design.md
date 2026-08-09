# Issue 83 Design: HTML Observatory Runtime v3 Consumer

## Outcome And Boundary

Issue 83 makes the existing HTML Observatory a real browser consumer of the
versioned Runtime v3 HTTPS and WSS surfaces. It renders live snapshots and
events, binds approved controls to authorized Runtime commands, reconnects
without duplicate application, and makes trust, stale, unavailable, denied,
and version-mismatch states visible.

This issue does not change Runtime API/WSS behavior, Unity, or the shared
Guardian restart coordinator. Those remain upstream or parent-owned surfaces.

## Source Baseline

- `demos/html-observatory/app.js` already consumes `/v1/observatory`,
  `/v1/ready`, `/v1/control`, and `/v1/observatory/ws` through the existing
  Runtime v3 browser configuration.
- `docs/api/runtime-v3/v1/observatory.openapi.json` and the Runtime v3
  architecture projections are read-only schema inputs.
- `docs/milestones/v0.92/features/OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md`
  requires live reads and writes, redaction and refusal, reconnect, and visible
  browser proof without fixture substitution.
- Issue #5837 owns shared Runtime/WSS integration and cross-client restart
  reconciliation. Issue #84 owns the Unity consumer.

## Design

The browser keeps the existing Observatory application and visual design. A
single client state machine owns discovery, snapshot freshness, WSS session
state, reconnect cursor, authorization state, and visible failure status.
HTTP snapshots establish the initial projection. WSS events are accepted only
for the negotiated API/catalog version and are applied in stable order using
their correlation and reconnect metadata.

Read-only projection access does not imply write authority. Commands use the
existing authenticated control route and expose accepted, denied, expired,
and unavailable outcomes in the interface. Tokens and signing material never
appear in URLs, browser storage intended for durable presentation evidence,
screenshots, logs, or repository files.

Reconnect uses bounded exponential backoff with jitter and the last accepted
cursor. The client does not display retained data as live while disconnected;
it marks stale age explicitly and returns to live only after a fresh Runtime
correlation is observed. Duplicate or out-of-order events are rejected or
ignored according to the shared Runtime contract.

## Owned Paths

- `demos/html-observatory/app.js`
- `demos/html-observatory/styles.css`
- `adl/tools/validate_v092_html_observatory_live.mjs`

## Read-Only Inputs

- Runtime v3 HTTP/WSS implementation, schemas, authentication, certificates,
  and launch behavior.
- `demos/html-observatory/runtime-v3.config.json` and existing HTML structure.
- Issue #5837 restart coordinator and issue #84 Unity outputs.
- All sibling and dependency records.

## Invariants And Failure Semantics

- No fixture, static packet, or cached snapshot is labeled live.
- Public reads never widen command authority.
- TLS trust failure, CORS/origin refusal, API/WSS version mismatch, stale data,
  backpressure, authorization refusal, and Runtime unavailability are visible.
- Reconnect cannot duplicate events, replay commands, or escalate authority.
- The browser never receives raw private citizen state, keys, tokens, or sealed
  checkpoints.
- HTML remains a separate application and no UI code moves into Runtime.

## Dependencies And Execution Gate

Issues #5800, #5820, and #5832 must remain terminal and provide trusted local
HTTPS, stable Runtime launch/API behavior, and the versioned ACIP/WSS contract.
Issue #5836 must be terminal before final implementation credit or live
acceptance is claimed. Preparation may complete while that final product gate
is open; execution must report it truthfully.

## Validation Boundary

`adl/tools/validate_v092_html_observatory_live.mjs` drives the real browser
against Runtime HTTPS/WSS. It proves fresh live rendering, menu and control
behavior, authenticated writes, refusal and redaction, stale/unavailable
states, bounded reconnect, no duplicate application, and fresh post-reconnect
correlation. It retains screenshots and machine-readable assertions from the
same run. Static DOM checks and Runtime-only tests are useful supporting proof
but cannot satisfy this issue's live acceptance.

Shared Guardian-owned restart coordination remains in #5837; this issue only
provides the browser-side hooks and assertions consumed by that coordinator.

## Rollback

Rollback restores the prior browser client behavior, logs out write sessions,
and returns to an explicitly read-only or unavailable state. It must not replace
live Runtime data with a fixture or change Runtime itself.

## Non-Goals

- Unity implementation or proof.
- Runtime API, WSS, TLS, launch, or authentication changes.
- Cross-client restart orchestration.
- Observatory redesign or unapproved visual changes.
- Serving Observatory assets from Runtime.
- Provider or AWS work.
