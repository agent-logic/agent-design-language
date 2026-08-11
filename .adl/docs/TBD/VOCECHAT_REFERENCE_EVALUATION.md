# VoceChat Reference Evaluation For ADL

Status: Reference evaluation

Source repository: `/Users/daniel/git/vocechat-server-rust`

Source revision: `96c0aa5d100cc1c9090c004dbeb662ba3a08a6cd`

Evaluation date: 2026-08-10

## Recommendation

Use VoceChat as a product and interaction reference for Polis and the
Observatory. Do not adopt it as a dependency, fork its implementation, or add
it as a separately deployed chat server.

The useful communication capabilities should be implemented as a bounded
module inside the existing ADL Runtime. Polis and Observatory are consumers of
that Runtime-owned module, not alternate hosts or authorities. It must reuse
ADL's Axum, Rustls, ACIP, governed identity, capability, persistence,
observability, and replay boundaries. The module must not introduce a second
HTTP stack, TLS stack, identity system, persistence authority, or standalone
chat binary.

## Why It Is Worth Studying

VoceChat demonstrates a compact self-hosted communication product with:

- direct and group messaging;
- event subscriptions;
- bots and inbound/outbound webhooks;
- file transfer and archive/export behavior;
- OpenAPI-generated interfaces;
- SQLite-first local operation;
- authentication integrations;
- a small operational footprint; and
- straightforward single-host deployment.

Its personal-cloud philosophy also overlaps with the Polis goal of giving
participants durable, owned communication spaces rather than making a central
platform the sole authority over their interactions.

## Why ADL Should Not Adopt The Code

### Licensing ambiguity

`Cargo.toml` declares MIT, while the repository's `license` file contains the
Big Time Public License and the README describes separate commercial terms.
That inconsistency is a legal and procurement blocker for incorporating or
deriving ADL product code from this repository.

### Architectural mismatch

VoceChat uses Poem and poem-openapi. ADL has standardized Runtime-facing HTTP
and WebSocket behavior on Axum and Rustls. Incorporating VoceChat would create
another server framework and duplicate transport, middleware, authentication,
TLS, and API-description responsibilities.

VoceChat is also a centralized SQLite chat application. It does not provide
ADL's governed identity continuity, capability authority, deterministic replay,
ACIP boundary, distributed Guardian authority, or evidence-bearing execution
model.

### Maintenance and dependency risk

The public repository resembles a published source snapshot more than an
actively reviewed engineering repository. Its manifest uses older generations
of Tokio, SQLx, Poem, Rustls, Reqwest, and authentication dependencies. Adopting
it would require substantial modernization before security or compatibility
claims could be made.

### Security concerns

The repository includes CA certificate and private-key material under `cert/`
and compiles those values into runtime self-signed certificate generation in
`src/self_signed.rs`. This conflicts with ADL's production certificate policy.

Administrative user creation and update paths issue direct HTTP requests to
configured webhook URLs. Without a governed destination policy, DNS/IP checks,
redirect controls, and network egress restrictions, that pattern is
SSRF-sensitive.

The large authentication, user, group, upload, archive, webhook, and bot
surfaces would require a dedicated security review before reuse even if the
licensing problem were resolved.

## Capabilities Worth Reproducing

The ADL-native communication module should consider:

- channels and direct conversations;
- durable message history with explicit retention policy;
- replay-aware event subscriptions for browser and Unity consumers;
- bot identities governed through normal ADL capability authority;
- inbound and outbound integration endpoints with strict destination policy;
- attachments with content limits, provenance, redaction, and retention;
- archive and export operations with authorization and evidence receipts;
- generated API documentation from the canonical contract;
- local-first persistence and a low-resource single-node mode; and
- clean migration from local operation to distributed Polis operation.

These are product requirements and reference behaviors, not authorization to
copy VoceChat source or reproduce its internal architecture.

## Runtime Module Boundary

The communication capability should be an internal ADL Runtime module. The
Runtime owns execution, admission, authorization, persistence, transport, and
replay. Polis supplies the social domain and Observatory supplies governed
views and interaction surfaces.

It should:

- execute in the existing Runtime process unless measured isolation requires a
  later change;
- expose routes through the existing Axum server;
- use the existing Rustls certificate and trust-root configuration;
- represent participants through governed ADL identities;
- authorize channel, message, attachment, bot, and webhook actions through
  capability envelopes;
- encode inter-agent and external communication through ACIP where applicable;
- persist replayable events through the governed Runtime state boundary;
- publish redacted Observatory projections without leaking private state or
  control authority; and
- preserve a future distributed implementation behind the same contract.

It should not:

- become a separate `chat-server` binary;
- own a second listener, authentication system, or certificate lifecycle;
- generate production self-signed certificates;
- bypass Guardian admission or Runtime policy;
- treat SQLite rows as identity or continuity authority;
- permit unrestricted webhook egress; or
- turn the Observatory into the authoritative message store.

## Observatory Review Checklist

The Observatory work should compare its current design against the following
VoceChat-inspired workflows:

1. A governed participant enters a channel and receives replay plus live
   updates without duplicate or missing events.
2. Direct and group messages preserve sender identity, ordering, authorization,
   redaction, and causation metadata.
3. A bot participates only through an explicit identity and capability grant.
4. Attachments enforce size, type, storage, provenance, and retention policy.
5. Webhook delivery rejects private-network and otherwise unauthorized
   destinations, controls redirects, and retains bounded delivery evidence.
6. Export produces an authorized, portable, integrity-verifiable archive.
7. Browser and Unity consumers receive only governed public projections.
8. Local single-node operation can evolve into distributed Polis operation
   without changing user-facing message semantics or copying state as proof of
   continuity.

## Decision Boundary

This evaluation recommends studying VoceChat's product ergonomics while
rejecting code adoption and standalone-server integration. It does not approve
a final communication schema, persistence model, federation protocol, UI, or
implementation schedule. Those decisions remain with the relevant Polis,
Runtime, ACIP, and Observatory issues and architecture review.

## Validation Performed

- Inspected the exact local source revision named above.
- Reviewed the root manifest, lockfile, README, license, server/state layout,
  certificate implementation, webhook validation paths, migrations, and release
  workflow.
- Confirmed the recommendation requires no VoceChat code reuse.

No VoceChat build or runtime security test was executed. This is a bounded
architecture and product-reference evaluation, not a complete source-code or
penetration-test report.
