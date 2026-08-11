# Structured Task Prompt

Template: 1.0.0

Issue: 114

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

After direct dependency gates clear, implement only the declared durable history store and narrow Runtime API and Observatory integration paths without absorbing session, authority, roster, room, attention, deployment, or private-memory scope.

## Deliverables

- adl-runtime/src/conversation_history.rs
- adl-runtime/tests/conversation_history.rs
- adl/tests/conversation_history_runtime_api.rs
- adl/tools/validate_v092_html_observatory_history.mjs
- docs/milestones/v0.92/features/DURABLE_CONVERSATION_HISTORY.md
- Narrow module, Runtime API, OpenAPI, and demos/html-observatory/app.js integration
- Exact forty-two-case product proof and fresh exact-head independent review

## Acceptance

1. AC-1 History survives bounded Runtime and browser restart without loss, duplication, reordering, terminal-outcome rewrite, or use as execution restore authority
2. AC-2 Every persisted turn and outcome binds canonical #111 identities, exact sequence and predecessor receipt, and commits atomically with its watermark and idempotency result
3. AC-3 Every page, search, export, deletion, and receipt read is freshly authorized through #112; expiry, revocation, policy drift, recipient substitution, and cross-Polis or cross-principal access fail closed
4. AC-4 Opaque authenticated cursors provide bounded stable-snapshot paging and reject tampering, expiry, stale generation, changed policy, or principal reuse without skips or duplicates
5. AC-5 Search and export operate only on bounded authorized public-safe projections and cannot expose private cognition, provider payloads, credentials, secrets, signing material, or unauthorized conversations
6. AC-6 Explicit bounded retention and monotonic tombstone deletion are idempotent, auditable, immediately unavailable to reads, and leave no hidden index or export residue
7. AC-7 Copy-validate-publish schema migration is deterministic, receipt-bound, restart-safe, and fail closed on unknown, lossy, partial, ambiguous, or unsafe rollback state
8. AC-8 Startup and retry recovery validate generations, watermarks, sequences, outcomes, idempotency, indexes, tombstones, and receipt chains; corruption or ambiguity quarantines rather than presenting partial history as complete
9. AC-9 Exact forty-two-case focused Rust/API/browser proof, strict Clippy, diff hygiene, typed validation, and fresh exact-head review pass before publication

## Dependencies

- Serial gate 1: issue #111 must be terminal through a merged PR and its merge revision ancestral to the #114 execution base
- Serial gate 2: issue #112 must then be terminal through a merged PR and its merge revision ancestral to the #114 execution base
- Issue #110 remains open umbrella scope and sequencing authority but is not a terminal prerequisite
- Issue #83 is a transitive read-only baseline through #111 and #112 and receives no #114 mutation

## Inputs

- agent-logic/agent-design-language#110
- agent-logic/agent-design-language#111
- agent-logic/agent-design-language#112
- agent-logic/agent-design-language#114
- .csdlc/prepared/issues/114/design.md
- .csdlc/prepared/issues/114/diagram.mmd
- adl-runtime/src/runtime_api_auth.rs
- adl-runtime/src/continuity_history.rs
- adl/src/csm_runtime_api.rs
- docs/api/runtime-v3/v1/openapi.json
- docs/api/runtime-v3/v1/observatory.openapi.json
- demos/html-observatory/app.js

## Non Goals

- Implementing or mutating #83, #110, #111, #112, or any sibling WP-18C issue
- Global memory search, agent-private state, provider transcript scraping, private cognition, or execution checkpoint/lifelog authority
- Indefinite retention, browser-owned transcript authority, silent deletion, or ungoverned export
- Rooms, roster/presence, attention inbox, Unity, AWS, public deployment, model/provider work, publication, merge, or closeout
