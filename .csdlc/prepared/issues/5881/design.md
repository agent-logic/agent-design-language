# Issue 5881 design: finish removing claims from C-SDLC v2

Status: design approved for preparation.

## Decision

Treat this as a residual-gap deletion and proof issue, not a new lifecycle
redesign. Current `main` already uses branch/worktree topology as canonical
ownership but still contains claim-specific legacy decoding. Execution
inventories the remaining active claim surfaces, performs a verified one-time
normalization of current records into canonical topology, and then deletes the
claim-specific structs and logic. If an acceptance criterion is already
satisfied, record exact evidence instead of writing replacement code.

## Publication authority

- Issue authority remains `danielbaustin/agent-design-language#5881`.
- Code publication targets `agent-logic/agent-design-language`; the PR body
  must use `Closes danielbaustin/agent-design-language#5881`.
- This is split issue/code publication authority, not repository cutover or
  issue migration.

## Scope and sequencing

1. Rebase after #5895 and #5883 so shared installer and operator-doc changes
   are classified once.
2. Inventory canonical models, schemas, CLI routes, operator skills, current
   docs, tests, and installed manifests. Separate active authority, private
   legacy normalization, historical evidence, and ordinary English uses of
   "claim".
3. Normalize current claim-bearing records through a verified one-time path,
   preserving branch/worktree topology and audit truth, then delete the
   claim-specific decoder, structs, and compatibility logic. Historical
   evidence remains immutable and is not rewritten.
4. Prove atomic and idempotent binding, interrupted-bind/crash recovery,
   different-issue concurrency without path reservations, completed record
   normalization, claim-free review/publication/finish/cleanup, and current
   operator guidance.

## Invariants

- One issue maps to one authoritative branch/worktree pair.
- No claim ID, lease, heartbeat, protected-path reservation, claim recovery,
  compatibility wrapper, or renamed claim abstraction enters canonical state.
- Exact-head review, card validation, publication, terminal authority, and
  cleanup remain fail closed.
- Historical records and terminal evidence remain immutable.
- Validation is focused; no broad workspace suite is required.

## Failure behavior

Fail closed if concurrent binds can create competing topology, interrupted
binding cannot recover safely, current records cannot normalize without manual
repair, any claim-specific production decoder remains after normalization, or a
public schema/operator route still requires claim state. Any genuinely separate
tooling defect becomes a follow-on issue.
