# v0.92 Planning And Recovery Session Handoff

## Session Identity

- Source task: `V0.9x Planning #4`
- Source task id: `019fe947-2d7d-7903-9260-f2dfb48de546`
- Recovery branch: `codex/session-handoff-recovery`
- Recovery worktree: `/Volumes/FastWork/adl-worktrees/adl-session-handoff-recovery`
- Recovery base: `origin/main` at `e172257b50ec9d6e07bbb0ab62a69a001ad1774f`

This task belongs to a transcript lineage affected by the Codex session-storage
duplication defect. Resume this work from a brand-new, non-cloned task using
this document, not by cloning or continuing the source task.

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
- The authoritative current state for active lanes is in the three recovered
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

## Session Storage Defect

Live read-only diagnosis on 2026-08-11:

- `/Users/daniel/.codex/sessions`: approximately 476 GB.
- August 2026 sessions: approximately 360 GB.
- 5,004 transcript files.
- 363 files larger than 500 MB.
- 227 files larger than 1 GB.
- `/System/Volumes/Data`: 95 percent full with approximately 45 GB available.

The defect is cross-task transcript duplication:

- July task `019f4b3e-...` repeats its own `session_meta` about 1,433 times.
- August 7 task `019fdf4d-...` includes about 1,428 July metadata records plus
  192 of its own.
- August 9 task `019fe947-...` includes three task ids: about 1,428 July, 116
  August 7, and 46 of its own.
- Old transcript files were still being modified on August 11.

The task `Codex Session Storage Recovery` received the full diagnosis and was
instructed to archive directly to FastWork, verify checksums before deletion,
avoid `/private/tmp`, and obtain explicit approval before destructive cleanup.

## Worktree Cleanup State

- No worktrees were pruned by this task.
- The repository had approximately 5.6 GB in repo-local worktrees, 454 MB in
  Codex-managed worktrees, and 7.8 GB in the Git database.
- Transcript storage, not worktrees, is the primary local-space problem.
- A safe prune still requires per-worktree proof of closed issue, merged PR,
  clean worktree, retained artifacts, and terminal truth.

## Exact Next Actions

1. Review and commit the recovery documents on this branch using force-add,
   because `.adl/` is intentionally ignored by the broad repository rule.
2. Push the recovery branch so the handoffs are not dependent on local
   worktrees.
3. Resume each sprint from a fresh non-cloned task using the indexed handoff.
4. Complete Codex session archive and checksum verification before deleting
   any local transcript.
5. Prune only worktrees that independently satisfy closed-and-merged safety
   proof.

## Non-Goals

- Do not resume implementation from this recovery branch.
- Do not mutate C-SDLC cards or lifecycle state here.
- Do not run CI, broad tests, cloud jobs, or paid runners for these documents.
- Do not delete transcript files or worktrees from this task.
- Do not write recovery files onto `main` directly.
