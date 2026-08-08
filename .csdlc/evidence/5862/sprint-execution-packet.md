# WP-04-IMP Distributed Guardian Sprint Execution Packet

## Metadata

- Sprint issue: `#5862`
- Milestone: `v0.92`
- Execution mode: `hybrid`
- Follow-up policy: `post_sprint_follow_on` unless a finding invalidates safety,
  correctness, or integrated proof and is explicitly classified
  `must_land_before_sprint_close`.
- Product ownership: child-local only. The umbrella owns coordination and
  retained sprint evidence, not child product paths.

## Sprint Goal

Deliver the production Distributed Guardian implementation established by the
terminal `#5821` architecture and security gate, then prove the integrated
distributed surface before allowing WP-14 `#5832` to proceed.

## Child Issue Wave

| Order | WP | Issue | Depends on |
| ---: | --- | ---: | --- |
| 1 | WP-04.01 | `#5863` | terminal `#5821` gate |
| 2 | WP-04.02 | `#5864` | `#5863` terminal |
| 3 | WP-04.03 | `#5865` | `#5864` terminal |
| 4 | WP-04.04 | `#5866` | `#5865` terminal |
| 5 | WP-04.05 | `#5867` | `#5866` terminal |
| 6 | WP-04.06 | `#5868` | `#5867` terminal |
| 7 | WP-04.07 | `#5869` | `#5867` terminal |
| 8 | WP-04.08 | `#5870` | `#5868`, `#5869` terminal |
| 9 | WP-04.09 | `#5871` | `#5865` terminal |
| 10 | WP-04.10 | `#5872` | `#5865` terminal |
| 11 | WP-04.11 | `#5873` | `#5867`, `#5870`, `#5871`, `#5872` terminal |
| 12 | WP-04.12 | `#5874` | `#5864`, `#5870` terminal |
| 13 | WP-04.13 | `#5875` | `#5870`, `#5873`, `#5874` terminal |
| 14 | WP-04.14 | `#5876` | `#5875` terminal |
| 15 | WP-04.15 | `#5877` | `#5867`, `#5870`, `#5875`, `#5876` terminal |
| 16 | WP-04.16 | `#5878` | `#5863` through `#5877` terminal |

This is the exact sprint denominator. Scope changes require operator approval
and renewed architecture/security review.

## Recommended Execution Order

1. Trust spine: `#5863 -> #5864 -> #5865`.
2. After `#5865`, fan out `#5866`, `#5871`, and `#5872` in separate child
   worktrees.
3. After `#5866`, run `#5867`.
4. After `#5867`, fan out `#5868` and `#5869`.
5. Run `#5870` after both `#5868` and `#5869` are terminal.
6. Run `#5873` and `#5874` when each complete dependency predicate passes.
7. Run relocation spine `#5875 -> #5876 -> #5877`.
8. Run `#5878` only after all preceding fifteen children are terminal.
9. Reconcile the umbrella only after exact PR, merge, closure, receipt,
   ancestry, and integrated-proof evidence is current.

## Safe Parallel Lanes

| Lane | Issues | Safety predicate | Coordination |
| --- | --- | --- | --- |
| discovery-advertisements | `#5866`, `#5871`, `#5872` | `#5865` terminal | Exact disjoint source/test paths; no shared manifest edits. |
| failure-authority | `#5868`, `#5869` | `#5867` terminal | Membership output is read-only; sibling modules remain disjoint. |
| placement-catalog | `#5873`, `#5874` | Each full dependency set terminal, including `#5870` | Placement and snapshot-catalog paths remain disjoint. |

Parallel execution means separate child sessions, worktrees, goals, validation,
independent review, publication, watches, and closeout. The umbrella cannot act
for a child.

## Candidate Parallel Lanes

- `trust-spine`: `#5863`, `#5864`, `#5865`; `serial_gate`.
- `discovery-advertisements`: `#5866`, `#5871`, `#5872`;
  `safe_parallel` after `#5865` terminal.
- `failure-authority`: `#5868`, `#5869`; `safe_parallel` after `#5867`
  terminal.
- `placement-catalog`: `#5873`, `#5874`; `safe_parallel` only after each
  complete dependency predicate passes.
- `relocation-spine`: `#5875`, `#5876`, `#5877`; `serial_gate`.
- `integrated-proof`: `#5878`; `blocked_until_dependency` until all preceding
  children are terminal.

Any path collision, dependency drift, shared-manifest requirement, failed
proof, or superseded design reclassifies the affected lane as serial or
blocked.

## Serial Gates

| Gate | Blocks | Exit condition | Owner |
| --- | --- | --- | --- |
| G0 architecture | `#5863` | `#5821` terminal design is ancestral to current `main` | umbrella coordinator |
| G1 trust spine | `#5864`, `#5865` | preceding trust-spine child terminal | child closeout plus coordinator truth check |
| G2 authority | `#5870` | `#5867`, `#5868`, and `#5869` terminal | umbrella coordinator |
| G3 decisions | `#5873`, `#5874` | each issue's complete dependency set terminal | umbrella coordinator |
| G4 relocation | `#5875`, `#5876`, `#5877` | preceding relocation dependencies terminal | umbrella coordinator |
| G5 integration | `#5878` | all `#5863` through `#5877` terminal | umbrella coordinator |
| G6 protocol handoff | WP-14 `#5832` | `#5878` proof and `#5862` terminal reconciliation | umbrella coordinator |

Each gate consumes fresh GitHub and typed lifecycle truth. An open PR, prior
check, or merged branch without terminal reconciliation is not gate authority.

## Watcher Policy

- Each published child PR remains under an issue-local watcher through checks,
  review, merge, and typed closeout.
- Watchers are read-only unless explicitly routed to a bounded janitor repair.
- A watcher may not self-review, merge, close, fabricate a receipt, or advance
  the sprint matrix.
- Healthy pending checks remain watched. Failures, conflicts, requested
  changes, stale exact-head review, or dependency drift route to the child
  janitor.
- A merged child routes immediately to typed finish and cleanup; GitHub closure
  alone is insufficient.

## Budget And Goal Accounting

- Sprint `#5862` is descriptive coordination context, not a substitute for a
  child implementation goal.
- Every child session creates an issue-bound goal after bind/readiness and
  before implementation, naming sprint `#5862`, the child issue, and its
  bounded objective.
- Record available elapsed time and token use. Preserve `unknown` or
  `not_available`; never infer zero.
- The operator-wide 3% weekly-token floor stops new work, validation, review,
  retry, or repair immediately when reached.

## Watcher Plan

| State | Watch owner | Next route |
| --- | --- | --- |
| Child bound or implementing | child session | Continue issue-local execution and heartbeat. |
| PR open and healthy | issue-local watcher | Recheck the exact head through required checks. |
| Checks failed or conflict found | child janitor | Apply bounded child-owned repair, review again, republish. |
| PR merged or issue closed | child closeout session | Run typed finish, verify terminal receipt, then cleanup. |
| Child terminal | umbrella coordinator | Refresh matrix and evaluate newly ready DAG nodes. |
| Dependency or ownership ambiguity | umbrella coordinator | Stop the affected lane and use typed recovery or request operator direction. |

## Parallelism Outcome Plan

At sprint closeout, record which declared fan-outs ran concurrently, which
serialized, the exact cause of any prediction miss, and whether contention was
in source ownership, proof lanes, worker availability, or dependencies.

## Sprint Activity Log

- Declared path: `.csdlc/evidence/5862/activity.jsonl`
- Record child dispatch, bind, validation, review, PR, terminal, and gate-change
  events without converting waiting or unknown state into success.

## Sprint-Level Review

- Declared path: `.csdlc/evidence/5862/sprint-review.md`
- Review implementation, tests, security, documentation, failed or deferred
  lanes, and residual risk across the integrated distributed surface.

## Sprint Closeout Rollup Expectations

- Reconcile all sixteen children with PR URL, exact head and merge SHA,
  closing relation, receipt path/digest, ancestry, and cleanup state.
- Retain code, test, security, documentation, and synthesis review evidence.
- Retain `#5878` production distributed and native receipt validator results,
  including failed, skipped, deferred, or unavailable platforms.
- Record coverage source and Rust tracker counts, or a truthful reason either
  is not applicable.
- Run the terminal form of `validate-implementation-wave.rb` at the exact
  candidate head.
- Keep WP-14 `#5832` blocked until the integrated proof and umbrella terminal
  reconciliation both pass.
- Close sprint issue `#5862` last.
