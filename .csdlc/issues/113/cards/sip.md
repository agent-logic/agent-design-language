# Structured Intent Prompt

Template: 1.0.0

Issue: 113

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Let an authorized Observatory operator navigate a complete policy-filtered live Polis roster and understand each visible agent's current presence, health, activity, location, capabilities, and communication eligibility.

## Required Outcome

Runtime exposes versioned paginated roster and agent-detail projections with stable identity, server-side policy filtering, deterministic presence and freshness, bounded revisioned updates, and an accessible searchable Observatory consumer that never presents a sample as complete.

## Scope

- adl-runtime-kernel/src/agent_roster.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- adl-runtime-kernel/tests/agent_roster.rs
- adl-runtime-kernel/tests/control.rs
- docs/api/runtime-v3/v1/observatory.openapi.json
- demos/html-observatory/app.js
- demos/html-observatory/index.html
- demos/html-observatory/styles.css
- adl/tools/validate_v092_html_observatory_roster.mjs
- .csdlc/issues/113
- .csdlc/prepared/issues/113
- .csdlc/evidence/113

## Authority

- Issue and code authority are agent-logic/agent-design-language#113
- Runtime identity, topology, health, policy, and communication authority remain canonical; the browser is a read-only consumer of the filtered projection
- New agent_roster module, exact tests, and roster browser validator are exclusive to #113
- Control, OpenAPI, and HTML Observatory paths are serially shared and may change only after #83 and #142 are terminal and ancestral
- Issue #83, umbrella #110, deferred deployment #122, operational Runtime #142, every sibling WP-18C child, and their lifecycle records remain read-only

## Assumptions

- none

## Operator Constraints

- Do not bind or implement until every declared serial gate is satisfied and revalidated live
- Use only C-SDLC v2 owner binaries and typed card editors in the dedicated FastWork issue context
- Do not publish, push, open a PR, merge, close, or mutate issue #83, #110, #122, #142, or another WP-18C child during preparation
- Do not expose private cognition, credentials, secret-bearing state, unauthorized agent existence, or raw provider output
- Use exact nonzero validation targets and retain large-Polis resource evidence
