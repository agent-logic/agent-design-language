# Issue 113 Design: Live Polis Roster And Presence

## Outcome And Authority

Replace the bounded `AgentPopulationFeed::single()` sample with a versioned,
paginated Runtime-owned roster and agent-detail projection. The browser remains
a consumer: stable identity, visibility, redaction, presence, health,
freshness, capability summaries, location, and communication eligibility are
computed before serialization by Runtime policy and authority.

The local production source is the current Runtime v3 identity, admission,
component-state, and policy contracts on `main`. Merged distributed Polis
identity, membership, topology, failure, placement, migration, and projection
contracts remain compatible inputs for future non-local projection. Open issue
agent-logic/agent-design-language#142 gates only that non-local/distributed
adapter; it does not gate the local resident Shepherd roster outcome.

## Affected-Area Ownership

Issue #113 exclusively owns these new surfaces during execution:

- `adl-runtime-kernel/src/agent_roster.rs`
- `adl-runtime-kernel/tests/agent_roster.rs`
- `adl/tools/validate_v092_html_observatory_roster.mjs`

Issue #113 keeps changes to these shared integration surfaces isolated so they
can be rebased after concurrent #111 work. Neither open #83 nor open #142 gates
the local resident Shepherd projection under the current live #110/#113
authority:

- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/src/control.rs`
- `adl-runtime-kernel/src/bin/adl-runtime-kernel.rs`
- `adl-runtime-kernel/tests/control.rs`
- `docs/api/runtime-v3/v1/observatory.openapi.json`
- `demos/html-observatory/app.js`
- `demos/html-observatory/index.html`
- `demos/html-observatory/styles.css`

The C-SDLC package owns only `.csdlc/issues/113`,
`.csdlc/prepared/issues/113`, and `.csdlc/evidence/113`. Issue #83, #110,
#122, #142, every other WP-18C child, and legacy issue #83 records are
read-only. Scope expansion or active ownership overlap stops execution.

## Projection Contract

The Runtime exposes a versioned roster page and versioned agent detail. A page
binds a policy-subject digest, topology generation, roster revision, stable
sort key, page size, and opaque continuation token. A response reports visible
count, whether more pages exist, and whether policy redaction prevents any
claim about total Polis population. It never represents a sample as complete.

Each visible row carries stable agent identity, display name, public role,
presence, health, availability, bounded current-activity summary, capability
summary, node/location projection, communication eligibility, observed time,
source revision, freshness deadline, and provenance class. Detail uses the same
identity and policy decision and adds no private cognition or secret-bearing
state.

Presence is one of `ready`, `busy`, `sleeping`, `degraded`, `unreachable`,
`migrating`, or `unknown`. State derivation is deterministic from authenticated
Runtime evidence. Stale, contradictory, absent, or unauthorized evidence maps
to an explicit degraded or unknown public state; the browser cannot upgrade it.
Relocation changes node/location and revision while preserving stable agent
identity.

## Pagination And Events

Ordering is deterministic and stable within one roster revision. Continuation
tokens are opaque, bounded, integrity-protected, policy-subject-bound, and
rejected after incompatible revision, policy, sort, filter, or page-size
changes. Duplicate and out-of-order updates are idempotent or rejected without
regressing state. Reconnect resumes from a bounded cursor, deduplicates updates,
and forces a fresh page when retention or topology generation makes the cursor
unsafe.

Search, filter, sort, selection, and status-change rendering operate on the
versioned Runtime projection. Client filtering may narrow already-authorized
rows but never supplies visibility, health, identity, capability, or
communication authority.

## Failure Cases

- Unauthorized agents and private fields are omitted or redacted before JSON
  serialization and never shipped for CSS hiding.
- Invalid, replayed, cross-policy, stale-revision, or tampered page tokens fail
  closed with a stable reason and no partial page.
- Missing, stale, contradictory, or future-dated evidence yields explicit
  freshness and `unknown` or `degraded`, never false readiness.
- Migration preserves stable identity; duplicate identities, split ownership,
  stale locations, and unfenced previous owners fail closed.
- Event gaps, cursor expiry, duplicate events, out-of-order events, Runtime
  restart, and reconnect cannot silently omit or duplicate visible agents.
- Search and sorting remain deterministic for equal keys and bounded input.
- Large-Polis proof enforces page-size, response-size, memory, latency, event
  queue, and reconnect bounds without loading the whole roster in the browser.
- Detail lookup for an omitted, revoked, migrated, or unauthorized agent
  returns a policy-safe denial or absence without existence disclosure.

## Execution Gates

Local resident Shepherd implementation may proceed when:

1. Current `main` exposes the production Runtime v3 admission, component-state,
   policy, Observatory, and stable local identity inputs required by this
   projection. Open #83 is preserved source and no longer gates WP-18C.
2. Non-local/distributed roster projection remains deferred until #142 is
   terminal and ancestral. This issue must not invent or simulate that adapter
   while proving the local Shepherd outcome.
3. #122 remains non-gating for local work under its newer live status, which
   explicitly defers public exposure beyond v0.92. #110 remains a read-only
   umbrella and does not override that issue-local execution boundary.
4. Shared-path edits are isolated from the roster model and tests, with exact
   overlap reported for rebase against concurrent #111 work.

Closed #137 and legacy #5863/#5867/#5877/#5878 are compatible contract evidence,
not authority to claim non-local production behavior before #142 completes.

## Validation Plan

- Exact `agent_roster` Rust integration target: policy filtering, redaction,
  stable identity, all presence states, freshness, pagination/token integrity,
  deterministic ordering, relocation, restart, cursor gaps, and large-Polis
  bounds.
- Exact `control` integration target: authenticated Runtime route and WSS event
  parity, OpenAPI schema parity, detail denial, revision behavior, and no sample
  completeness claim.
- Browser validator: real Runtime-backed search, filter, sort, selection,
  status changes, pagination, reconnect, explicit failures, responsive layout,
  keyboard operation, clean console, and bounded DOM rows.
- Strict focused Clippy and diff hygiene before exact-head review.

All execution evidence must bind the exact candidate revision and nonzero test
targets. Fixture-only rendering and design-time card validation are not product
proof.

## Rollback

Disable the versioned roster/detail routes and event subscription, restore the
last compatible Observatory feed contract, and remove only #113-created UI and
test integration. Rollback must not reintroduce the sample as a complete roster,
weaken server-side policy filtering, alter stable identity, or disturb #83/#142
owned behavior and evidence.

## Non-Goals

No conversation persistence, private cognition, browser-defined health,
multi-agent rooms, attention inbox, public AWS exposure, distributed Runtime
implementation, #83 mutation during preparation, or work on another WP-18C
child.
