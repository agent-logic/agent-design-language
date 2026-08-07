# Runtime, Observatory, Polis, and Protocol Sprint Design Execution Packet

## Metadata

- Sprint issue: `#5855`
- Milestone: `v0.92`
- Execution mode: `hybrid`
- Status: `prepared`
- Machine packet: `.csdlc/prepared/issues/5855/sprint-execution-packet.yaml`

## Sprint Goal

Deliver one resilient Runtime and Observatory path, then distributed, protocol, provider, and consumer integration.

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
| `#5800` | supporting | merged baseline | browser-trusted local Observatory HTTPS with reproducible browser and health proof | async closeout owner |
| `#5820` | WP-03 | initialized | one Guardian-owned launch path with resilient startup, configuration, recovery, and lifecycle behavior | child session owner |
| `#5795` | supporting | initialized | real local model-backed Shepherd dialogue through governed Runtime v3 and Observatory surfaces | child session owner |
| `#5821` | WP-04 | initialized | architecture and security gate followed by completion of the bounded 16-issue distributed-runtime program within v0.92 | child session owner |
| `#5832` | WP-14 | initialized | reconciled versioned protocol family, protobuf schema, public catalog, JSON projection, and authenticated full-duplex WSS carrier | child session owner |
| `#5837` | WP-18A | initialized | separate consumers integrated with the versioned Runtime API and WSS | child session owner |

## Recommended Execution Order

1. Treat merged `#5800` at `7dfb791ad2fc1ecbc1e3b3651815b1d37bfa060f` as the canonical TLS baseline; closeout continues asynchronously.
2. Route `#5820` immediately after readiness and collision checks.
3. Route `#5821` after `#5820` stabilizes Runtime ingress.
4. Route `#5832` after `#5821` and the separate `#5862` implementation sprint are terminal.
5. Route `#5795` after `#5820` and `#5832` stabilize Runtime and protocol contracts.
6. Route `#5837` only when its issue-wave dependencies and this packet serial gates are satisfied.

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
| `#5800` | async closeout owner | reconcile typed terminal truth without blocking Sprint 2 product work | truthful child closeout |
| `#5820` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5795` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5821` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5832` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |
| `#5837` | child session owner | bind, implementation, checks, review, merge | truthful child closeout |

## Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| lane 1 | `#5820` | Runtime launch and resilience is the first active product lane. | merged issue 5800 TLS baseline is ancestral |
| lane 2 | `#5795` preparation | Local-provider design preparation cannot redefine shared contracts. | integration waits for issues 5820 and 5832 |

## Candidate Parallel Lanes

| Lane | Classification | Issues | Expected write sets | Dependency gate | Collision posture |
|---|---|---|---|---|---|
| candidate 1 | active | `#5820` | dedicated FastWork child worktree | merged issue 5800 baseline | stop on any live path collision |
| candidate 2 | preparation only | `#5795` | dedicated FastWork child worktree | no shared product edits before issues 5820 and 5832 | collapse to serial on overlap |

## Serial Gates

| Gate | Blocks | Exit condition | Owner |
|---|---|---|---|
| gate 1 | issue 5820 | merged issue 5800 TLS baseline is ancestral; async closeout is non-blocking | sprint session |
| gate 2 | issue 5821 | issue 5820 Runtime ingress is stable | sprint session |
| gate 3 | issue 5832 | issues 5821 and 5862 are terminal | sprint session |
| gate 4 | issue 5795 | issues 5820 and 5832 provide stable Runtime and protocol contracts | sprint session |
| gate 5 | issue 5837 | issues 5820 and 5832 are terminal and WP-18 is ready | sprint session |

## PVF / Validation-Tail Notes

- Child VPP lanes remain the only authority for implementation proof.
- The umbrella validator proves membership, packet completeness, and routing boundaries only.
- Any overlap, unmet dependency, or unsupported completion claim fails closed.

## Parallelism Outcome Plan

- Start only the lanes classified safe in this packet.
- Collapse a lane to serial execution immediately if real write or proof surfaces overlap.
- Record planned versus actual parallelism in the sprint review; parallelism is an optimization, not acceptance evidence.

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
