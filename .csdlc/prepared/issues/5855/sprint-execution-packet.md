# Runtime, Observatory, Polis, and Protocol Sprint Design Execution Packet

## Metadata

- Sprint issue: `#5855`
- Milestone: `v0.92`
- Execution mode: `hybrid`
- Status: `complete`
- Machine packet: `.csdlc/prepared/issues/5855/sprint-execution-packet.yaml`

## Sprint Goal

Deliver one resilient Runtime path with trusted Observatory TLS, the distributed
Guardian architecture gate, the versioned protocol contract, and the bounded
local Shepherd foundation.

## Sprint Boundary

In scope:

- Coordinate only the listed child issues through their existing typed v2 lifecycles.
- Preserve exact dependencies, separate worktrees, issue-bound goals, proof, review, and terminal truth.

Out of scope:

- Implementing child code or documentation in the umbrella session.
- Replacing child validation, review, publication, or closeout authority.

## Child Issue Wave

| Issue | Role | Status | Primary surface | Watcher |
|---|---|---|---|---|
| `#5800` | supporting | closed via PR #9 | browser-trusted local Observatory HTTPS with one shared certificate identity | terminal |
| `#5820` | WP-03 | closed via PR #28 | Guardian-owned launch path with resilient startup, recovery, and lifecycle behavior | terminal |
| `#5821` | WP-04 | closed via PR #39 | distributed Guardian architecture, security, and child-wave gate | terminal |
| `#5832` | WP-14 | closed via PR #76 | versioned protocol, protobuf, catalog, JSON projection, and authenticated WSS contract | terminal |
| `#5795` | supporting | closed via PR #72 | bounded local model-backed Shepherd foundation | terminal |

## Completed Execution Order

1. `#5800` established the trusted TLS baseline at `7dfb791ad2fc1ecbc1e3b3651815b1d37bfa060f`.
2. `#5820` completed Runtime launch and resilience at `b5bcfdfc13a6f454a715cbb9aa64e24bce3b7ba6`.
3. `#5821` completed the distributed architecture/security gate at `0ea81fd61b0bf598ece4ce368ae5cf1a1923127c`.
4. `#5832` completed the protocol and WSS contract at `a5021ab7e9bff220021e3600fa51b4f0848f5524`.
5. `#5795` completed the bounded local Shepherd foundation at `094797b6fe4be52549f447b0b7e513892c060436`.

Issue `#5837` and the split HTML/Unity issues `#83` and `#84` are explicitly
outside Sprint 2 and continue independently.

## Watcher Policy

- Each active child session owns its PR/check/review watch or explicitly hands it to a watcher.
- Waiting is not failure; blockers and changed gates are recorded without moving unrelated children.
- Product dependency gates use canonical merge ancestry. Typed closeout may
  reconcile asynchronously and is required only for final umbrella closeout.

## Budget And Goal Accounting

- No sprint-global token budget is preallocated.
- Every implementation session binds or verifies its dedicated FastWork
  branch/worktree with current typed `csdlc-bind`, then creates its own
  issue-bound goal before implementation.
- Actual time and token use are recorded per child when available and are never inferred as zero.

## Watcher Plan

| Issue | Watcher | Current focus | Next terminal state |
|---|---|---|---|
| `#5800` | complete | merged PR #9 and closed issue | terminal |
| `#5820` | complete | merged PR #28 and closed issue | terminal |
| `#5821` | complete | merged PR #39 and closed issue | terminal |
| `#5832` | complete | merged PR #76 and closed issue | terminal |
| `#5795` | complete | merged PR #72 and closed issue | terminal |

## Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| lane 1 | `#5820` | Completed after the ancestral #5800 TLS baseline. | satisfied |
| lane 2 | `#5795` | Completed after stable #5820 and #5832 contracts. | satisfied |

## Candidate Parallel Lanes

| Lane | Classification | Issues | Expected write sets | Dependency gate | Collision posture |
|---|---|---|---|---|---|
| candidate 1 | completed | `#5820` | dedicated FastWork child worktree | merged issue 5800 baseline | no unresolved collision |
| candidate 2 | completed | `#5795` | dedicated FastWork child worktree | terminal issues 5820 and 5832 | no unresolved collision |

## Serial Gates

| Gate | Blocks | Exit condition | Owner |
|---|---|---|---|
| gate 1 | issue 5820 | satisfied by ancestral issue 5800 TLS merge | complete |
| gate 2 | issue 5821 | satisfied by stable issue 5820 Runtime ingress | complete |
| gate 3 | issue 5832 | satisfied by the issue 5821 architecture gate | complete |
| gate 4 | issue 5795 | satisfied by stable issues 5820 and 5832 | complete |

## PVF / Validation-Tail Notes

- Child VPP lanes remain the only authority for implementation proof.
- The umbrella validator proves membership, packet completeness, and routing boundaries only.
- Any overlap, unmet dependency, or unsupported completion claim fails closed.

## Parallelism Outcome Plan

- The safe lanes completed without unresolved path collisions.
- Serial gates remained the acceptance authority; parallelism was not treated as proof.
- Exact merge ancestry and live issue closure are recorded in the sprint review.

## Sprint Activity Log

- Declared path: `.csdlc/evidence/5855/activity.jsonl`
- Record child start, bind, validation, review, PR state, terminal state, and any gate change.

## Sprint-Level Review

- Declared path: `.csdlc/evidence/5855/sprint-review.md`
- Review every child result, integration boundary, failed or deferred lane, and residual route before closing the umbrella.

## Sprint Closeout Rollup Expectations

- Roll up every child issue and PR state without converting unknown or waiting states into success.
- Record budget variance only from actual child goal data.
- Record which parallel lanes were safe, collapsed to serial, blocked, or not attempted.
- Close the umbrella only after every child has truthful terminal state.
