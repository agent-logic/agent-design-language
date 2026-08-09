# Issue 5832 Design: ACIP And A2A Contract Reconciliation

## Outcome And Boundary

Issue 5832 defines one versioned ACIP/A2A protocol family with a protobuf wire
schema, public catalog, deterministic JSON projection, and authenticated
full-duplex WSS carrier. It reconciles the existing Runtime v3 ACIP envelope,
A2A adapter boundary, trace/replay identity, and browser/native transport so
the same semantic message cannot drift across encodings or consumers.

This issue owns the protocol and carrier contract. It does not own distributed
Guardian authority (5821), Shepherd semantics (5795), Observatory/Unity UI
integration (5837), or cloud signal bridges.

## Source Baseline

- `adl-runtime/src/acip.rs` and `adl-runtime-kernel/src/acip.rs` are current
  Runtime v3 ACIP semantic owners.
- `adl-runtime/src/runtime_api_auth.rs` and
  `adl-runtime/tests/runtime_api_wss.rs` own this issue's authenticated
  full-duplex WSS contract and focused proof surface.
- `adl-runtime-kernel/src/protocol_adapters.rs`, `ingress.rs`, `control.rs`,
  and `operations.rs` own governed protocol admission and adapter dispatch.
- `docs/milestones/v0.92/features/ACIP_BINARY_SCHEMA_AND_WEBSOCKET_TRANSPORT_v0.92.md`
  requires a protobuf family, version negotiation, deterministic JSON,
  schema-derived public catalog, binary payload limits, and denied-access proof.
- ADR 0017 and prior ACIP/A2A/protobuf/WSS evidence define retained boundaries
  but do not establish the v0.92 reconciled contract.

## Design

Freeze a semantic envelope before encoding work: protocol family/version,
message and operation identity, sender/recipient, runtime/polis identity,
correlation/causation, trace/replay identity, capability/authority context,
payload type, ordering, acknowledgement/error, and bounded size. Protobuf is the
canonical binary wire schema. The public catalog is generated or validated
from that schema and records message direction, auth requirement, payload type,
and compatibility status.

JSON is a deterministic projection of the same semantic envelope, with stable
field names, ordering, integer/byte representation, omission rules, and
unknown-field policy. Round trips must preserve semantic identity across
protobuf and JSON. Version negotiation rejects unsupported major versions and
handles declared compatible minors without guessing.

The existing Axum/Rustls WSS endpoint carries authenticated bidirectional
frames. Admission verifies TLS, bearer/session credentials where applicable,
signed control authority, origin policy, frame/message limits, schema version,
and replay identity before dispatch. Backpressure, cancellation, reconnect,
and error frames are bounded and observable. No browser token appears in URLs,
logs, fixtures, or committed artifacts.

## Owned Paths

- `adl-runtime/src/acip.rs`
- `adl-runtime/src/runtime_api.rs`
- `adl-runtime/src/runtime_api_auth.rs`
- `adl-runtime-kernel/src/acip.rs`
- `adl-runtime-kernel/src/protocol_adapters.rs`
- `adl-runtime/tests/runtime_api_wss.rs`
- `adl-runtime/schemas/acip/v1/acip.proto`
- `adl-runtime/schemas/acip/v1/catalog.json`
- `docs/api/runtime-v3/v1/acip.openapi.json`
- `.csdlc/prepared/issues/5832/validate-acip-native-receipts.rb`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

## Invariants And Failure Semantics

- One semantic message has equivalent protobuf and deterministic JSON meaning.
- Unsupported versions, malformed/oversized frames, unknown required fields,
  replay, wrong runtime, and denied authority fail before operation dispatch.
- Authentication and signed command policy remain Runtime-owned.
- Binary payloads remain bounded; backpressure cannot create unbounded queues.
- Public catalog generation cannot silently diverge from the wire schema.
- No custom TLS, WebSocket, protobuf, or cryptographic implementation.

## Dependencies And Coordination

WP-04 gate issue #5821 must be terminal. WP-14 is an independent protocol lane
and its disjoint protocol, schema, catalog, deterministic JSON/protobuf
projection, negotiation, authentication, bounded WSS, and focused proof may
proceed immediately against current `main`. WP-04-IMP issue #5862 remains an
independent distributed sprint and consumes terminal WP-14 protocol evidence;
WP-14 has no reverse dependency on #5862.

Issue #5795 final browser integration consumes terminal #5832 merge evidence.
AWS GPU Shepherd sidecar #5907 follows terminal #5832 and local #5795; it is
independent and never gates either lane.
Each lane has its own branch, worktree, PR, focused proof, review, and closeout.
The lanes never share worktrees, cross owned-path boundaries, or close
provisionally. Handoffs are exact merged revisions and stable contracts only.
Existing ACIP stream and trace/replay baselines must be requalified at the
current implementation revision. Issue #5837 waits for the stable API/WSS
consumer contract. Consumer implementation files remain out of scope here.

## Validation Boundary

Deterministic lanes cover schema/catalog parity, protobuf/JSON round trips,
golden compatibility fixtures, ordering, limits, unsupported versions,
malformed frames, replay, and denied access. The issue-owned
`acip_schema_roundtrip_negatives` test is the mandatory nonzero denominator for
that lane.
The `runtime_api_wss` production target launches the production
Guardian/kernel, uses the real Rustls endpoint, performs authenticated
bidirectional binary and JSON exchanges, reconnects under backpressure,
verifies trace/replay identity, and retains exact-revision transcript digests.
It fails on fixture servers, plaintext, direct-kernel launch, zero exchanges,
schema drift, or auth bypass without adding a second shell-owned launch path.

`.csdlc/prepared/issues/5832/validate-acip-native-receipts.rb` requires distinct macOS,
Linux, and native Windows receipts bound to source revision, binary/schema
digests, exact argv, runner identity, nonzero exchanges, negative-case counts,
and output artifacts. These native receipts complete WP-14 independently and
become inputs to later distributed interoperability in #5862.
Consumer rendering remains deferred to #5837.

## Rollback

Rollback disables negotiation of the new protocol version, restores the prior
supported catalog/version set, and proves old compatible clients still fail or
succeed according to the retained matrix. It never downgrades authentication,
accepts ambiguous frames, or forks JSON semantics from protobuf.

## Non-Goals

- Distributed Guardian membership, migration, or fencing implementation.
- Observatory or Unity UI changes.
- Shepherd model behavior or provider integration.
- SNS/SQS/cloud bridge completion.
- Custom cryptography, TLS, WebSocket, or protobuf runtimes.
