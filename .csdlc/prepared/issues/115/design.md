# Issue #115 Design: Governed Multi-Agent Rooms And Routing

## Outcome And Authority

Issue #115 extends the canonical one-to-one conversation contract from #111 into
bounded Runtime-owned rooms. An authenticated Layer 8 operator can create a room,
manage an explicit participant set, address an explicit recipient subset, receive
stable attributed responses, and observe joins, leaves, refusals, timeouts,
revocations, and partial delivery. Runtime remains the sole authority for room
membership, routing, delivery, ordering, replay, and public projection.

The browser may display eligible agents supplied by #113, but roster visibility or
selection never grants membership or delivery authority. Every room mutation and
turn consumes #112 authenticated principal, exact multi-recipient capability,
policy, revocation, replay, refusal, and redacted audit truth. Cross-Polis routing
fails closed unless a later separately governed contract authorizes it.

## Participant And Routing Security Contract

A room has a stable room identity, creator principal, Polis identity, lifecycle
state, monotonically increasing membership revision, bounded ordered participant
map, and bounded event cursor. A participant record binds stable agent identity,
join revision, current membership state, policy decision reference, and public
display projection. Display names, browser indexes, roster ordering, mentions, and
free text are never recipient authority.

Creation and membership mutation require an exact expected prior revision and an
explicit canonical participant set. Joins, leaves, and revocations create a new
revision. No participant is inherited from browser state, presence events, prior
turns, group labels, or wildcard syntax. Duplicate identities, unknown identities,
stale revisions, cross-Polis identities, unauthorized participants, and bound
overflow fail before mutation.

Each turn freezes the room identity, membership revision, sender identity, exact
sorted recipient identities, recipient-set digest, sequence, correlation,
causation, submission key, bounded content digest, capability digest, and policy
digest. An empty set, wildcard, omitted recipient set, nonparticipant, duplicate,
or substituted recipient fails closed. Mentions are display metadata and must be a
subset of the already authorized recipient set.

Authorization is atomic before fan-out: #112 must allow the exact whole recipient
set before any sequence commitment or dispatch. Delivery then records one
monotonic per-recipient outcome. A policy revocation, leave, unavailability,
timeout, or transport failure observed after admission may yield explicit partial
delivery; it can never add a recipient or hide an attempted recipient. Responses
are accepted only from a dispatched recipient, carry stable agent attribution,
and correlate to the triggering room turn and recipient delivery record.

## Deterministic Partial Delivery And Replay

Per-recipient outcomes are ordered by stable agent identity and transition only
forward through accepted, dispatched, delivered, responded, refused, revoked,
unavailable, timed_out, cancelled, or failed. The aggregate is derived from that
canonical map as delivered, partial_delivery, none_delivered, refused, timed_out,
cancelled, or failed. Completion order cannot change the serialized aggregate.

The replay fingerprint binds room, membership revision, sender, exact recipient
set and digest, turn sequence, correlation, content digest, capability and policy
digests, and submission key. An exact replay returns the retained aggregate,
per-recipient outcomes, responses, and receipts without membership mutation,
sequence allocation, authorization side effects, or redispatch. Conflicting reuse
of any replay identity fails closed. Duplicate events with the same identity and
digest are idempotent; conflicting duplicates fail. Reordered events are applied
only when the next room cursor is available within a bounded buffer; gaps or
overflow force a fresh Runtime snapshot and never fabricate continuity.

Membership changes after admission do not alter a frozen recipient set. A revoked
or departed recipient that has not crossed the delivery commit point receives an
explicit terminal nondelivery outcome. Already committed delivery is retained as
historical truth, and later responses are accepted only under the declared bounded
late-response policy. Runtime restart restores replay and room state only from an
approved dependency contract; otherwise the room is explicitly unavailable.

## Affected-Area Ownership

Exclusive issue-owned execution paths:

- `adl-runtime-kernel/src/conversation_rooms.rs`
- `adl-runtime-kernel/tests/conversation_rooms.rs`
- `adl/tools/validate_v092_html_observatory_rooms.mjs`
- `docs/milestones/v0.92/features/GOVERNED_MULTI_AGENT_ROOMS.md`

Shared integration paths, editable only after #111, #112, and #113 are terminal,
merged, ancestral, and have handed off compatible contracts:

- `adl-runtime-kernel/src/lib.rs`
- `adl-runtime-kernel/src/ingress.rs`
- `adl-runtime-kernel/src/control.rs`
- `adl-runtime-kernel/src/operations.rs`
- `adl-runtime-kernel/tests/observatory.rs`
- `adl-runtime-kernel/tests/openapi_contract.rs`
- `docs/api/runtime-v3/v1/observatory.openapi.json`
- `demos/html-observatory/app.js`
- `demos/html-observatory/index.html`
- `demos/html-observatory/styles.css`

Preparation owns only `.csdlc/issues/115`, `.csdlc/prepared/issues/115`, and
`.csdlc/evidence/115`. Root main, #83, #110, #111, #112, #113, and every sibling
issue remain read-only.

## Pairwise Overlap Notes

- #111 owns canonical conversation, turn, sequence, correlation, provider-neutral
  execution, and one-to-one session behavior. #115 consumes those contracts and
  owns only room membership, frozen recipient sets, fan-out aggregation, and room
  projection. Shared kernel, ingress, control, OpenAPI, and app paths require serial
  handoff.
- #112 owns principal, exact multi-recipient authorization, policy intersection,
  replay authority, refusal, revocation, and redacted audit. #115 does not redefine
  those decisions; it supplies exact room and recipient bindings and consumes the
  result before fan-out.
- #113 owns roster identity, presence, visibility, and communication-eligibility
  projection. #115 may use eligible visible agents as selection candidates, but
  reauthorizes exact participants through #112. Control, OpenAPI, and HTML paths
  overlap only after serial handoff.
- #114 owns durable history, search, retention, deletion, export, and receipts.
  #115 neither modifies `conversation_history` nor claims durable room history;
  any later room-history integration requires separate typed replanning after #114.

## Serial Gates And Execution Handoff

Do not bind or edit product code until #111, #112, and #113 are each terminal
through merged PRs, their merge revisions are ancestors of the selected execution
base, and no active owner holds an intended shared path. Re-read #110 and #115,
inspect exact merged dependency contracts, and update SPP/VPP through `csdlc-edit`
if topology, paths, schemas, or tests changed. Then run typed doctor, invoke
`csdlc-bind` in a separately authorized execution session, create the issue-bound
session goal, and begin only the declared implementation.

## Validation Plan

- Exact `conversation_rooms` target: membership revisions, participant bounds,
  exact recipient sets, authorization-before-dispatch, fan-out, attribution,
  partial delivery, replay, reorder, restart, and adversarial substitution.
- Exact Observatory target: authenticated room transport, room/participant lists,
  transcript, composer, delivery states, reconnect, and refusal projection.
- Exact OpenAPI parity target and live Runtime-backed browser room validator.
- Strict focused Clippy and diff hygiene before exact-head review.

All product lanes remain deferred during preparation because #111-#113 are open
and issue-owned targets do not yet exist. Typed card validation and doctor prove
readiness structure only, never product behavior.

## Failure Policy And Non-Goals

Fail closed on stale dependency ancestry, ownership collision, implicit recipients,
authorization ambiguity, recipient substitution, replay conflict, event gaps,
unbounded fan-out, unattributed response, cross-Polis routing, forbidden data,
missing proof, or unresolved exact-head findings.

No unbounded broadcast, browser-owned selection authority, cross-Polis federation,
durable room history, inbox/intervention workflow, public AWS deployment, Unity
work, product implementation, publication, push, PR, merge, closeout, or #83
mutation is in this preparation scope.
