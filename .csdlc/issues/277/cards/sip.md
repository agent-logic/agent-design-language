# Structured Intent Prompt

Template: 1.0.0

Issue: 277

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement durable Runtime conversation continuity for sender and acknowledgement watermarks, attempt-local idempotency, replay ownership, ambiguous-dispatch outcomes, and delivery/response/ack receipts after #276 and #270 terminal caches validate.

## Required Outcome

Runtime can restart and reconcile conversation delivery state without duplicate execution, stale acknowledgement acceptance, lost receipts, or ambiguous dispatch lying about outcome.

## Scope

- Sender delivery watermarks
- Recipient acknowledgement watermarks consumed from trusted #270 acknowledgement truth
- Attempt-local idempotency and duplicate-attempt outcomes
- Replay ownership and restart reconciliation
- Ambiguous-dispatch outcome persistence and definite pre-dispatch retryability
- Delivery, response, and acknowledgement receipts stored through the #276 journal foundation

## Authority

- #276 owns durable conversation journal schema, storage, migrations, corruption recovery, retention, and deletion foundation
- #270 owns trusted recipient-acknowledgement Runtime API protocol, provenance, and signature trust
- #277 consumes #276 and #270 and must not redefine their authority
- #278 owns durable history/API/UI integration after #277
- #114 and #110 remain coordination parents and are not bound by this child

## Assumptions

- none

## Operator Constraints

- #276 terminal cache must validate canonical_match=true and merge 3e249f9857f392f7f569560fbd5fbfbc36b95b2f must be ancestral to refreshed origin/main before bind.
- #270 terminal cache must validate canonical_match=true and its merge must be ancestral to refreshed origin/main before bind.
- Use typed v2 lifecycle routes only; no raw GitHub lifecycle writes.
- Bind only /Volumes/FastWork/adl-worktrees/adl-issue-277-conversation-watermarks-idempotency-replay-receipts on branch codex/277-conversation-watermarks-idempotency-replay-receipts.
- Do not absorb #278, #114 parent, #115, API/UI/Observatory, cloud exposure, or provider transcript scraping.
