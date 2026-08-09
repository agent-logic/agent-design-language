# Demonstration, Handoff, and Publication Sprint Execution Packet

## Metadata

- Sprint issue: `#5854`
- Milestone: `v0.92`
- Execution mode: `hybrid`
- Status: `ready_for_execution`
- Machine packet: `.csdlc/prepared/issues/5854/sprint-execution-packet.yaml`
- Live gate snapshot: `.csdlc/evidence/5854/live-gates.json`

## Sprint Goal

Produce real demonstrations, consumer proofs, governance handoff, and complete launch media without converting plans, checkpoints, or private artifacts into release claims.

## Sprint Boundary

The umbrella coordinates the listed children through their typed v2 lifecycles. Every child retains its own implementation, validation, review, publication, and closeout authority. This readiness change does not execute a demo, produce an episode, publish media, or authorize a release.

## Child Issue Wave

| Issue | Role | Current truth | Next action |
|---|---|---|---|
| `#5835` | WP-17 | bind-ready after this readiness change; blocked on `#5834` | bind only after the dependency gate opens |
| `#5836` | WP-18 | bind-ready after this readiness change; blocked on `#5834` | bind only after the dependency gate opens |
| `#5838` | WP-18B | bind-ready after this readiness change; blocked on `#5834` and `#5836` | preserve the provider-proof gate |
| `#5839` | WP-19 | bind-ready after this readiness change; blocked on `#5835` and the v0.93 allocation | preserve governance boundaries |
| `#5840` | WP-20 | bind-ready after this readiness change; blocked on `#5836`, `#5837`, `#5838`, and `#5839` | run only after all proof producers finish |
| `#5844` | WP-24 | terminal; canonical issue `#10` and PR `#14` are merged | no further execution |
| `#5845` | WP-24A | active checkpoint; episode 001 landed in non-closing PR `#69`; nine episodes remain | continue episode checkpoints independently |

## Recommended Execution Order

1. Continue `#5845` as independent, non-closing episode checkpoints; do not treat one episode as WP-24A completion.
2. When `#5834` closes, start `#5835` and `#5836` in separate FastWork worktrees.
3. Start `#5838` after `#5836` is terminal and all of its other dependencies are satisfied.
4. Start `#5839` after `#5835` is terminal and the v0.93 allocation is explicit.
5. Start `#5840` only after every declared proof producer is terminal.
6. Align final publication claims only after `#5843` and explicit operator authorization.

## Watcher Policy

- Each active child session owns its PR, check, and review watch or explicitly hands it to one bounded watcher.
- Waiting is not failure. Record changed gates without starting unrelated or blocked children.
- No optional proof job runs merely because capacity is available.
- Completion requires live issue/PR truth and typed child terminal truth to agree.

## Budget And Goal Accounting

- No sprint-global token budget is preallocated.
- Every implementation child gets one issue-bound goal after typed bind and before implementation.
- Actual time and token use are recorded per child when available and are never inferred as zero.

## Watcher Plan

| Issue | Watcher | Current focus | Next terminal state |
|---|---|---|---|
| `#5835` | child session owner | dependency gate | truthful child closeout |
| `#5836` | child session owner | dependency gate | truthful child closeout |
| `#5838` | child session owner | dependency gate | truthful child closeout |
| `#5839` | child session owner | dependency gate | truthful child closeout |
| `#5840` | child session owner | dependency gate | truthful child closeout |
| `#5845` | child session owner | episode checkpoints 002-010 | truthful WP-24A closeout after ten episodes |

## Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| publication checkpoints | `#5845` | Each episode is a bounded non-closing package. | private/unpublished posture and exact checkpoint review |
| first downstream pair | `#5835`, `#5836` | Migration planning and birthday demo use disjoint child worktrees. | `#5834` terminal and no path overlap |

## Candidate Parallel Lanes

| Lane | Classification | Issues | Dependency gate | Collision posture |
|---|---|---|---|---|
| candidate 1 | active checkpoint | `#5845` | WP-24A remains open | serialize episode publication metadata updates |
| candidate 2 | safe after gate | `#5835`, `#5836` | `#5834` terminal | collapse to serial on real overlap |

## Serial Gates

| Gate | Blocks | Exit condition |
|---|---|---|
| birthday packet | `#5835`, `#5836` | `#5834` terminal |
| provider proof | `#5838` | `#5832`, `#5834`, and `#5836` terminal |
| governance handoff | `#5839` | `#5834` and `#5835` terminal plus explicit v0.93 allocation |
| proof coverage | `#5840` | `#5836`, `#5837`, `#5838`, and `#5839` terminal |
| release truth | final public claims | `#5843` terminal plus explicit operator authorization |

## PVF / Validation-Tail Notes

- Child VPP lanes remain the only authority for implementation proof.
- A deferred validator is a bind-readiness declaration, never validation evidence.
- The umbrella validator proves membership, current state classification, packet completeness, safe ownership, and routing boundaries only.
- Any overlap, unmet dependency, stale live-gate snapshot, or unsupported completion claim fails closed.

## Parallelism Outcome Plan

- Start only lanes whose dependency gates are satisfied.
- Collapse a lane to serial execution when real write or proof surfaces overlap.
- Record planned versus actual parallelism in sprint review; parallelism is an optimization, not acceptance evidence.

## Sprint Activity Log

- Declared path: `.csdlc/evidence/5854/activity.jsonl`
- Record child start, bind, validation, review, PR state, terminal state, and gate changes.

## Sprint-Level Review

- Declared path: `.csdlc/evidence/5854/sprint-review.md`
- Review every child result, integration boundary, failed or deferred lane, and residual route before closing the umbrella.

## Sprint Closeout Rollup Expectations

- Roll up every child issue and PR state without converting unknown or waiting states into success.
- Record budget variance only from actual child goal data.
- Record which parallel lanes were safe, collapsed, blocked, or not attempted.
- Close the umbrella only after every child has truthful terminal state.
