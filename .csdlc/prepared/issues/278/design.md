# Issue 278 design

## Scope

#278 owns the Runtime-owned history read layer for conversations that were already admitted through the #276 journal foundation, reconciled through #277 continuity receipts, rendered after #271 Observatory authority-state presentation, and related to #115 governed multi-agent room routing only as upstream conversation/room state that must not be reimplemented here.

The implementation must provide re-authorized paginated history, search, export, redaction, and Observatory transcript restoration after restart. Every read/export/redaction request must be authorized at read time by Runtime authority; a stale browser cursor, cached browser participant state, or private agent memory reference cannot authorize access.

## Dependency boundary

- #276 provides the durable append-only conversation journal foundation.
- #277 provides watermarks, idempotency, replay decisions, ambiguous dispatch outcomes, and delivery/response/ack receipts.
- #271 provides the Observatory authority and delivery-state UI integration required before transcript restoration is visible.
- #115 provides terminal governed multi-agent room routing and accepted-vs-delivered room state that #278 may read as history input but must not mutate, re-route, or re-authorize as room-routing work.
- #278 consumes those surfaces without redefining journal storage, acknowledgement trust, continuity semantics, governed room routing, or Observatory authority states.

## Required behavior

- Page visible conversation history with deterministic cursors and stale-cursor rejection.
- Search only authorized transcript records and never private memory.
- Export only public-safe redacted records with authority and redaction metadata.
- Record redaction markers and hide/redact affected records on subsequent reads, searches, exports, and restored Observatory transcripts.
- Restore Observatory transcript state after Runtime restart from Runtime-owned durable history, not browser-owned cached state.
- Preserve revocation behavior: a revoked or unauthorized principal cannot reuse an old cursor, export token, browser state, or prior room membership.

## Non-goals

- Global private-memory search.
- Provider transcript scraping.
- Browser-owned policy, signing, transcript authority, or cached authorization.
- Durable journal schema/storage foundation ownership.
- Redefining #270 acknowledgement trust or #277 replay/idempotency semantics.
- Reimplementing #115 governed multi-agent room routing or changing accepted-vs-delivered room semantics.
- #116 lifecycle/durability qualification or #117 integrated WP-18C qualification ownership.
- New cloud/public exposure work.
- Binding or implementing #114 parent directly.

## Validation plan

Focused proof should cover authorized page/read, stale cursor denial, revoked access denial, restart restoration, search, export, redaction, and Observatory transcript restoration using deterministic fixtures over the #276/#277 Runtime journal/continuity primitives.
