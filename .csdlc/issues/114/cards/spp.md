# Structured Planning Prompt

Template: 1.0.0

Issue: 114

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Keep #114 initialized while #111 and #112 remain open; after both are terminal and ancestral, revalidate merged contracts and ownership, bind execution, implement the isolated durable store and narrow integrations, run the exact PVF DAG, resolve exact-head review, and only then publish under separate authority.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Reread #110, #111, #112, and #114 through the typed GitHub owner; prove #111 and then #112 terminal, merged, ancestral, and ownership-compatible before binding.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Freeze the merged #111 turn contract, merged #112 authorization contract, exact path ownership, schema versions, retention classes, cursor format, receipt chain, and migration generation contract through typed replanning if needed.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Implement the isolated atomic conversation history store, authorized APIs, stable paging/search/export, retention/deletion, migration, recovery, and bounded Observatory projection only in declared paths.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run the exact forty-two-case Rust, API, strict Clippy, browser, restart, corruption, migration, deletion, redaction, and diff-hygiene PVF lanes with nonzero selection.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Resolve fresh independent exact-head review with no actionable findings, then hand off to separate publication authority without merge or closeout.",
    "acceptance_ids": [
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- Only canonical #111 ordered public-safe turns and monotonic outcomes enter history
- Every history read or lifecycle action is freshly authorized by current #112 truth
- Browser caches, provider transcripts, lifelog data, and conversation history never grant execution or policy authority
- A transaction either commits the exact record, watermark, idempotency result, and receipt together or has no effect
- Migration never mutates the source generation in place and recovery never invents missing history
- Deletion makes exact scoped data unavailable before compaction and preserves only policy-required redacted evidence

## Risks

- Merged #111 or #112 contracts may change the planned shared integration paths or identity fields
- Search indexes or browser caches could bypass fresh authorization or retain deleted content
- Cursor, migration, or receipt-chain drift could produce duplicates, gaps, or false continuity
- History could accidentally absorb private cognition or become execution restore authority
- Partial writes, disk-full, reply loss, or corrupt metadata could be misreported as success

## Estimates

{
  "elapsed_seconds": 86400,
  "total_tokens": 240000,
  "validation_seconds": 21600
}

## Design

.csdlc/prepared/issues/114/design.md

Digest: b70cf7e77e06cad287166597bf0b70bfcd43392f6452325c905dcec6fab65c08

## Diagram

.csdlc/prepared/issues/114/diagram.mmd

Digest: b8e5984d673cff4cb398de9deeb653bfb7dda81c388243371d91bd7562bebf42

## Stop Conditions

- Issue #111 is not terminal through a merged PR ancestral to the selected execution base
- Issue #112 is not terminal through a merged PR ancestral to the selected execution base
- Merged dependency shapes overlap or invalidate the declared ownership set without a typed SPP/VPP replan
- The design would require private cognition, provider transcript scraping, browser authority, or execution restore authority
- Atomic append, fresh authorization, tombstone deletion, copy-validate-publish migration, or quarantine recovery cannot be preserved
- Any focused proof or exact-head review has an unresolved actionable finding

## Handoff

Proceed only after doctor readiness.
