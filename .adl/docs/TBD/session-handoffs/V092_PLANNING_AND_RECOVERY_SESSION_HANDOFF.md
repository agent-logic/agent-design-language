# v0.92 Planning And Recovery Session Handoff

## Recovery Identity

- Recovery branch: `codex/session-handoff-recovery`
- Recovery base: `origin/main` at `e172257b50ec9d6e07bbb0ab62a69a001ad1774f`

Resume the indexed work from brand-new, non-cloned tasks using these documents.
Do not depend on retained conversation history as execution authority.

## Current Operator Decisions

- Never use or inspect `/private/tmp`.
- Worktrees and build output belong on FastWork, not the local drive.
- GitHub CI may use at most one paid 16-core runner per issue, only for the
  required proving job. Optional, redundant, benchmark, and speculative jobs
  must not run.
- The repository variable `ADL_HEAVY_RUNNER` was changed to `ubuntu-latest`
  during the funding stop. It has not been globally restored; paid-runner use
  should be selected per issue rather than enabling every heavy lane.
- Do not prune dirty, active, ambiguous, or unmerged worktrees.
- Preserve every active session handoff in repository `.adl/docs/TBD/` before
  retiring the source task.

## Sprint State Reviewed

- Legacy umbrella `#5854` is not ready to close. Operative children `#5835`,
  `#5836`, `#5838`, and `#5839` were still open when checked, although later
  handoff truth records subsequent WP-17 delivery and the active `#237` gate.
- Legacy umbrella `#5856` is not ready to close. Its WP-20 through WP-30
  release-tail children remain nonterminal.
- The authoritative current state for active lanes is in the four recovered
  handoffs indexed beside this file.

## VoceChat Evaluation

The retained evaluation is:

`../VOCECHAT_REFERENCE_EVALUATION.md`

Decision:

- Do not adopt, fork, embed, or ship VoceChat code.
- Use it only as a product and UX reference.
- Reimplement useful conversation, presence, room, history, moderation, and
  attachment concepts natively.
- The communication capability is a module inside the ADL Runtime, not a
  separate chat-server binary.
- Polis and Observatory consume the Runtime-owned module; they do not own
  authority, persistence, authentication, TLS, or transport.
- Preserve Axum, Rustls, ACIP, and Guardian governance boundaries.

The operator previously said not to share this evaluation with Observatory #2
while that task was busy. It is now preserved in the repository recovery
branch, but no cross-task design handoff should be sent until the operator
explicitly authorizes it.

## Local Session Storage Boundary

Local transcript diagnosis and archive details intentionally remain outside the
public repository. Recovery must archive directly to FastWork, verify checksums
before deletion, avoid `/private/tmp`, and obtain explicit operator approval
before destructive cleanup.

## Worktree Cleanup State

- No worktrees were pruned by this task.
- Local transcript storage, not worktrees, was the primary local-space problem.
- A safe prune still requires per-worktree proof of closed issue, merged PR,
  clean worktree, retained artifacts, and terminal truth.

## Exact Next Actions

1. Obtain final exact-head review of this recovery branch and resolve all
   actionable findings before opening its documentation-only PR.
2. Resume each sprint from a fresh non-cloned task using the indexed handoff.
3. Complete local session archive and checksum verification before deleting
   any local transcript.
4. Prune only worktrees that independently satisfy closed-and-merged safety
   proof.

## Non-Goals

- Do not resume implementation from this recovery branch.
- Do not mutate C-SDLC cards or lifecycle state here.
- Do not run CI, broad tests, cloud jobs, or paid runners for these documents.
- Do not delete transcript files or worktrees from this task.
- Do not write recovery files onto `main` directly.
