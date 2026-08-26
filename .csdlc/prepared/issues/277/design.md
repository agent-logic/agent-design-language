# #277 Design: Conversation Watermarks, Idempotency, Replay, and Receipts

## Issue Identity

- Issue: #277
- Title: `[v0.92][WP-18C.04b][114.b] Persist conversation watermarks, idempotency, replay, and receipts`
- Parent coordination issues: #114 and #110
- Required dependencies: terminal #276 durable conversation journal foundation and terminal #270 trusted recipient-acknowledgement Runtime API protocol

## Outcome

#277 adds the durable continuity layer on top of the #276 journal foundation. Runtime restart must be able to reconcile delivery state without duplicate execution, stale acknowledgement acceptance, lost receipts, or ambiguous-dispatch outcomes being reported as definite success.

## Owned Scope

- Sender watermarks for outbound delivery attempts.
- Recipient-acknowledgement watermarks consumed from #270 acknowledgement truth.
- Attempt-local idempotency keys and duplicate-attempt outcomes.
- Replay ownership for restart reconciliation.
- Ambiguous-dispatch outcome persistence and definite pre-dispatch retryability.
- Delivery, response, and acknowledgement receipts stored through the #276 journal foundation.
- Focused restart/replay/receipt/watermark proof.

## Non-Goals

- Redefining #270 acknowledgement trust, provenance, signing, served routes, or API semantics.
- Changing #276 journal schema/storage/corruption/migration foundation except through additive #277 event payload usage.
- Implementing #278 durable history APIs, transcript restoration, browser UI, or Observatory behavior.
- Binding or implementing #114 parent or #115 room/UI behavior.
- Cloud/public exposure, provider transcript scraping, or global private-memory search.

## Design

#277 should introduce a small Runtime kernel continuity module that consumes `ConversationJournal` as an append-only durable substrate. The module should model delivery state as explicit records rather than inferring it from transient in-memory control flow.

The intended minimum product surface is:

1. A durable continuity store that wraps `ConversationJournal`.
2. Event payloads for sender watermark advancement, acknowledgement watermark advancement, idempotency claim/results, replay decisions, and delivery/response/ack receipts.
3. A restart snapshot that rebuilds the latest known continuity state from the journal and fails closed if the underlying journal refuses to open.
4. Idempotency behavior that distinguishes:
   - duplicate completed attempts,
   - duplicate in-flight/ambiguous attempts,
   - definite pre-dispatch retryable attempts.
5. Receipt APIs that are local Runtime-kernel primitives only; no served public API or Observatory UI work belongs to this issue.

## Dependency Consumption

#276 is consumed as the durable storage authority. #277 may append foundation-compatible journal events and read snapshots, but it must not widen the journal foundation contract into public history or UI behavior.

#270 is consumed as acknowledgement protocol authority. #277 may store acknowledgement receipt/watermark facts that are already trusted by #270, but it must not validate new acknowledgement signatures or define new acknowledgement provenance rules.

## Validation Plan

- Preparation validator proves #276 and #270 canonical terminal caches are present and their merge SHAs are ancestral to current `origin/main`.
- Focused Runtime kernel tests prove restart, replay, duplicate/idempotent delivery, ambiguous dispatch, stale acknowledgement refusal, and receipt reconstruction behavior.
- Rust fmt and strict Clippy cover the touched Runtime kernel target.
- `csdlc-doctor`, `csdlc-validate`, and `git diff --check` prove lifecycle and patch hygiene.

## Stop Conditions

- #276 or #270 terminal cache validation fails.
- Bind target is not a dedicated FastWork #277 worktree.
- Implementation needs to redefine #270 trust or #276 storage foundation.
- Implementation begins touching #278, #114 parent, #115, API/UI/Observatory, or cloud exposure.
