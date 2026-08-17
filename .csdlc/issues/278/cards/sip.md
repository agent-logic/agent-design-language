# Structured Intent Prompt

Template: 1.0.0

Issue: 278

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement re-authorized conversation history APIs and Observatory transcript restoration after #276, #277, and #271 terminal caches validate as ancestral to current origin/main.

## Required Outcome

Authorized operators can reopen conversations, page visible history, search/export/redact permitted transcript records, and restore Observatory transcript state after restart without trusting stale browser state or exposing private agent memory.

## Scope

- Re-authorized paginated conversation history APIs
- Bounded authorized search over transcript records
- Public-safe redacted export of transcript records
- Redaction markers applied to reads, search, export, and restored transcripts
- Observatory transcript restoration from Runtime-owned durable history after restart
- Focused stale cursor, revoked access, restart, export, search, and redaction proof

## Authority

- #276 owns durable conversation journal schema, storage, migrations, corruption recovery, retention, and deletion foundation
- #277 owns persisted watermarks, idempotency, replay ownership, ambiguous dispatch, and receipts
- #271 owns Observatory authority-state and delivery-state UI integration
- #278 consumes #276/#277/#271 and must not redefine their authority

## Assumptions

- none

## Operator Constraints

- #276, #277, and #271 terminal caches must validate canonical_match=true and their merge commits must be ancestral to refreshed origin/main before bind.
- Use typed v2 lifecycle routes only; no raw GitHub lifecycle writes.
- Bind only /Volumes/FastWork/adl-worktrees/adl-issue-278-reauthorized-conversation-history-observatory-transcripts on branch codex/278-reauthorized-conversation-history-observatory-transcripts.
- Do not absorb #114 parent, #115 room routing, #276 journal foundation, #277 continuity semantics, #271 authority presentation, cloud exposure, provider transcript scraping, or private-memory search.
