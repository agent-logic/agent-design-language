# Issue 112 Slice 1 Design: Layer 8 Authority Core

This slice establishes the runtime-kernel authority primitives needed by issue
#112 without claiming the full WP-18C.02 product outcome. It is published as a
non-closing `part_of` PR so follow-on slices can integrate Runtime API delivery,
Observatory projection, and durable transcript/history behavior without
redefining this reviewed head as complete issue closeout.

The runtime kernel owns authenticated principal evidence, action-scoped
capabilities, policy intersection, public-safe refusal reasons, replay defense,
recipient validation, signed ACIP identity-message exchange, recipient-signed
acknowledgements, and redacted hash-chained audit records. Private signing keys
are loaded only from caller-supplied external paths and are never serialized in
messages, browser state, audit records, or repository state.

Authorization is fail-closed. Identity expiry, revocation, stale credential
generation, capability expiry/revocation/stale epoch, unavailable policy,
scope widening, recipient substitution, replay, non-canonical payloads, invalid
signatures, and audit unavailability all prevent a grant. Audit records hash
principals, Polis IDs, conversations, recipients, attachments, replay IDs,
capabilities, policies, and correlations instead of storing private content or
provider payloads.

Non-goals for this slice: browser/Observatory integration, Runtime API delivery
hooks, durable transcript or acknowledgement-watermark storage, rooms/fan-out,
and terminal issue closeout.
