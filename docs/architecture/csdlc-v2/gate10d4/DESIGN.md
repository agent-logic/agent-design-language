# Gate 10D4 read-only importer sunset design

## Boundary

Remove only the read-only importer after `2026-08-12T02:03:02.808013Z`. The
decision requires trusted time, explicit approval, current v2 health, completed
migration evidence, and proof that no active contract still names the importer.

## Invariants

- Early, missing, stale, or ambiguous inputs yield zero mutation.
- Migration evidence remains durable after importer removal.
- Exact-revision review and green checks precede merge.

## Non-goals

No unrelated v1, ADL, or Runtime cleanup.
