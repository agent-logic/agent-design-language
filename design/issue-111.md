# Issue 111 design: canonical human-agent conversation sessions

Status: preparation design complete; execution blocked on #83.

## Boundary

Issue #111 extends the #83 single-message Runtime/Observatory path into one bounded,
provider-neutral, one-operator-to-one-agent conversation session. Runtime owns the
session, turn ordering, idempotency, cancellation, timeout, and public response
projection. The browser is a client only. It may cache a view and reconnect with a
cursor, but it cannot create authoritative turns, infer delivery, or synthesize an
agent response.

This issue does not own durable searchable history (#114), multi-agent rooms or
broadcast (#115), the operator attention inbox (#116), downstream Layer 8 identity
policy semantics (#112, which depends on #111), agent roster/presence (#113), public
deployment (#122), or any mutation of #83 preparation truth.

## Dependency And Serial Gates

1. #92 is closed and supplies the current TLS baseline.
2. #83 must be terminal, independently validated, and ancestral to the #111 base
   revision before execution binding. Its live worktree currently owns overlapping
   Runtime ingress/control and Observatory surfaces, so #111 must not speculate over
   or copy its unmerged implementation.
3. #110 defines #111 as a foundational child after the first vertical slice and now
   explicitly records #122 as a deferred related issue that does not gate #111
   preparation, implementation, review, or publication.
4. #122 is open and deferred beyond v0.92. It owns future public exposure only and
   is not a #111 execution gate.

Preparation may reach design-ready while #83 is open. Execution binding, product
edits, review, and publication must stop until #83 is terminal and ancestral.

## Canonical Contracts

The implementation introduces versioned public contracts with bounded identifiers
and payloads:

- `adl.runtime.conversation.v1`: conversation identity, operator principal,
  recipient agent, lifecycle state, next sequence, and reconnect cursor.
- `adl.runtime.conversation_turn.v1`: turn identity, conversation identity, sender,
  recipient, sequence, correlation identity, bounded content, and submission key.
- `adl.runtime.conversation_delivery.v1`: accepted, delivered, refused, timed_out,
  cancelled, or failed outcome with the same turn and correlation identity.
- `adl.runtime.conversation_response.v1`: bounded public-safe agent response tied to
  the originating operator turn and its delivery outcome.

Unknown fields, invalid identifiers, oversized content, sequence gaps, sender or
recipient drift, and unsupported schema versions fail closed. Provider-specific
request/response payloads and private cognition never enter the public contracts.

## Runtime Ownership And Ordering

Add an issue-owned conversation module behind canonical ingress. A session registry
is held by Runtime and keyed by conversation ID. Each session binds one authenticated
operator principal to one policy-reachable agent and maintains the next accepted
sequence, a bounded idempotency index, ordered public turns, and cancellation state.

Submission is atomic at the session boundary:

1. Validate schema, bounds, stable identities, and the existing authenticated
   Layer 8 principal supplied by the #83 path.
2. Ask the existing recipient/policy boundary whether the agent is currently
   reachable. #111 consumes that decision and does not define broader authority.
3. Reserve exactly the expected sequence and submission key.
4. Emit `accepted`, dispatch through a provider-neutral agent execution adapter,
   then emit one terminal delivery outcome and at most one correlated public response.
5. Cache the terminal result by submission key so an exact duplicate returns the
   same result without dispatch; conflicting reuse is refused.

Per-conversation processing is serialized. Different conversations may progress
independently within existing bounded Runtime resource policy. No browser clock,
array order, acknowledgement hash, or provider completion ID is authoritative.

## Provider-Neutral Adapter Boundary

Define a narrow async adapter owned by the Runtime conversation module. Its input is
the canonical conversation/turn context plus bounded public content. Its output is
either a bounded public response or a typed refusal, timeout, cancellation, or
failure. Production assembly adapts the policy-selected agent execution route to
this trait. Tests use deterministic fakes. The adapter cannot return raw provider
payloads, credentials, private state, or uncorrelated text.

## Reconnect And Restart

Browser reconnect presents the conversation ID and last observed Runtime cursor.
Runtime returns only subsequent authoritative events, bounded by the session's
retention window. Repeated reconnect does not create or redeliver turns.

Long-term durable history is outside #111. A Runtime restart therefore does not
silently reconstruct a session from browser state. A pre-restart conversation ID is
reported as unavailable/failed and the operator must start a new session unless a
future #114 durable store supplies an explicitly versioned resume source. This
restart boundary is visible and tested.

## Observatory Integration

Extend the existing #83 authenticated chat path and WebSocket projection. The client
stores only the current conversation ID, last Runtime cursor, and rendered public
events. The composer submits the canonical signed turn envelope. UI state is derived
from Runtime delivery/response events and explicitly renders accepted, delivered,
refused, timed-out, cancelled, failed, disconnected, and restart-unavailable states.
An ingress acknowledgement or result hash is never rendered as an agent reply.

## Failure Cases

- Unknown, unavailable, or policy-ineligible recipient: refuse before sequence
  commitment or provider dispatch, with no authority disclosure.
- Malformed schema, invalid/changed identity, oversized content, or sequence gap:
  reject without mutating session state.
- Exact duplicate submission: return the cached authoritative outcome without a
  second dispatch or turn.
- Submission-key reuse with changed content or identity: conflict/refuse.
- Adapter timeout: terminate the turn as `timed_out`; late output is discarded.
- Cancellation before dispatch: `cancelled` with no dispatch. Cancellation during
  dispatch is propagated and any late response is discarded.
- Adapter/provider failure: `failed` with bounded public diagnostics and no raw
  provider payload.
- Browser disconnect: Runtime processing may finish; reconnect resumes from the
  Runtime cursor without duplication.
- Runtime restart: old in-memory session is unavailable and cannot be resumed from
  browser state.
- Queue saturation or shutdown: fail closed using existing Runtime bounded-admission
  semantics.

## Affected-Area Ownership And Intended Product Write Set

Primary issue-owned implementation paths:

- `adl-runtime-kernel/src/conversation.rs` (new canonical contracts/session engine)
- `adl-runtime-kernel/src/lib.rs` (module route only)
- `adl-runtime-kernel/src/ingress.rs` (conversation ingress kind/result integration)
- `adl-runtime-kernel/src/control.rs` (authenticated Observatory transport/projection)
- `adl-runtime-kernel/src/operations.rs` (provider-neutral adapter integration only
  if the post-#83 topology requires it)
- `adl-runtime-kernel/tests/conversation_sessions.rs` (new deterministic contract,
  ordering, idempotency, failure, reconnect, and restart tests)
- `adl-runtime-kernel/tests/observatory.rs` (live WSS conversation projection tests)
- `adl-runtime-kernel/tests/openapi_contract.rs` (schema/path parity)
- `docs/api/runtime-v3/v1/observatory.openapi.json` (versioned public contract)
- `demos/html-observatory/app.js` (Runtime-owned session/cursor client integration)

`demos/html-observatory/index.html` and `styles.css` are not pre-authorized writes;
add either only after a typed SPP/VPP replan proves it is necessary for explicit
failure-state rendering. No other provider, Runtime, or Observatory surface is owned
without the same typed replan.

## Execution Plan

1. Rebase from a terminal #83 revision and inspect its exact Runtime/Observatory
   contracts before changing the plan.
2. Add the canonical contracts and Runtime session engine with deterministic fake
   adapter coverage.
3. Integrate authenticated ingress/egress and production adapter routing without
   widening the inherited #83 authority or inventing downstream #112 semantics.
4. Integrate the Observatory client and reconnect cursor without browser authority.
5. Run the declared PVF lanes, then exact-head review focused on ordering,
   idempotency, cancellation races, redaction, and browser non-simulation.
6. Fix every actionable finding and only then use the normal v2 review/publication
   lifecycle in a separately authorized execution session.

## Validation Plan

PVF classification is recorded in VPP. The core lane is a deterministic, small-
resource Rust integration target dedicated to #111. Existing Observatory WSS and
OpenAPI targets prove transport and schema integration. JavaScript syntax and diff
hygiene are fast deterministic support lanes. Live provider proof is not required:
the provider-neutral boundary is proven with deterministic fakes, while #83 supplies
the real canonical ingress baseline.

All lanes fail closed. Missing issue-owned test targets are allowed only during this
initialized preparation phase and must exist before implementation finalization.
Skipped, pending, flaky, timed-out, credential-dependent, or unreviewed output is not
a pass.

## Execution Handoff

Do not bind from this preparation state until #83 is terminal and ancestral. At
handoff, rerun live issue reads, rebase or recreate from the terminal dependency
head, inspect overlapping #83 files, update SPP/VPP through `csdlc-edit` if paths or
tests changed, run doctor, and only then invoke `csdlc-bind`. Product implementation
starts after an issue-bound session goal is active in the bound worktree.
