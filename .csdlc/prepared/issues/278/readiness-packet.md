# Issue 278 readiness packet

## Live issue

- Issue: agent-logic/agent-design-language#278
- Title: [v0.92][WP-18C.04c][114.c] Expose re-authorized conversation history APIs and restore Observatory transcripts
- Parentage: part of #114 and #110

## Dependency gates

- #276 is closed by merged PR #346 with merge `3e249f9857f392f7f569560fbd5fbfbc36b95b2f`.
- #277 is closed by merged PR #348 with merge `3160fb8be575ba9a27748b05ea5dd911e4375deb`.
- #271 is closed by merged PR #382 with merge `6b200cfee83ea36a546123de4d24a6eda191b652`.
- #115 is closed by merged PR #384 with merge `22122c6c245b1f847aabcaf168a98660a3f11972`.

Each gate must validate through `csdlc-finish --validate-cached-issue` with `canonical_match=true` and the merge commit must be ancestral to `origin/main` before bind or implementation.

## Execution boundary

Bind only branch `codex/278-reauthorized-conversation-history-observatory-transcripts` and worktree `/Volumes/FastWork/adl-worktrees/adl-issue-278-reauthorized-conversation-history-observatory-transcripts`.

Do not mutate #114 parent staging, #115 room routing, #116 lifecycle/durability qualification, #117 integrated WP-18C qualification, #276 journal foundation semantics, #277 replay/idempotency semantics, or #271 authority-state presentation except as read-only inputs.
