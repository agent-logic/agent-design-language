# #277 Readiness Packet

## Scope

#277 owns the Runtime kernel continuity layer for sender/recipient-acknowledgement watermarks, attempt-local idempotency, replay ownership, ambiguous-dispatch outcomes, and delivery/response/ack receipts.

## Dependency Gates

- #276 terminal cache must validate with `canonical_match=true`.
- #276 merge `3e249f9857f392f7f569560fbd5fbfbc36b95b2f` must be ancestral to refreshed `origin/main`.
- #270 terminal cache must validate with `canonical_match=true`.
- #270 merge must be ancestral to refreshed `origin/main`.

## Boundary Gates

- Consume #276 `ConversationJournal` foundation read-only as storage substrate.
- Consume #270 trusted acknowledgement protocol; do not redefine acknowledgement trust.
- Do not absorb #278 durable history API/UI, #114 parent implementation, #115 room/UI behavior, Observatory, browser UI, cloud exposure, or provider transcript scraping.

## Bind Target

- Branch: `codex/277-conversation-watermarks-idempotency-replay-receipts`
- Worktree: `/Volumes/FastWork/adl-worktrees/adl-issue-277-conversation-watermarks-idempotency-replay-receipts`

## Required Proof Before Publication

- `python3 .csdlc/prepared/issues/277/validate_preparation_bundle.py`
- Focused Runtime kernel tests for #277 continuity behavior.
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check`
- Strict relevant Clippy.
- `csdlc-doctor --repo . --issue 277`
- `csdlc-validate --root . issue --issue 277`
- `git diff --check`
- Fresh no-context exact-head review PASS.
