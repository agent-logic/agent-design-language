# Demonstration, Handoff, and Publication Sprint Execution Packet

## Metadata

- Sprint issue: `#5854`
- Milestone: `v0.92`
- Execution mode: `hybrid`
- Status: `ready_for_execution`
- Machine packet: `.csdlc/prepared/issues/5854/sprint-execution-packet.yaml`
- Split-authority bind requests: `.csdlc/prepared/issues/5854/split-authority-bind-requests.json`
- Live gate snapshot: `.csdlc/evidence/5854/live-gates.json`

## Sprint Goal

Produce real demonstrations, consumer proofs, governance handoff, and complete launch media without converting plans, checkpoints, or private artifacts into release claims.

## Sprint Boundary

The umbrella coordinates the listed children through their typed v2 lifecycles. Every child retains its own implementation, validation, review, publication, and closeout authority. This readiness change does not execute a demo, produce an episode, publish media, or authorize a release.

## Child Issue Wave

| Issue | Role | Current truth | Next action |
|---|---|---|---|
| `#5835` | WP-17 | prepared and unbound; `#5826`, `#5827`, and `#5834` are terminal | ready to bind when Sprint 5 execution starts |
| `#5836` | WP-18 | prepared and unbound; `#5825`-`#5830`, canonical WP-14 `agent-logic/agent-design-language#209` / PR `#215`, `#5833`, and `#5834` have reviewed ancestral merge proof; legacy `#5832` is superseded | ready to bind when Sprint 5 execution starts |
| `#5838` | WP-18B | prepared and unbound; canonical WP-14 is complete but `#5836` remains open | preserve the provider-proof gate |
| `#5839` | WP-19 | prepared and unbound; blocked on `#5834`, `#5835`, and accepted v0.93 allocation | preserve governance boundaries |
| `#5844` | WP-24 | product/GitHub complete; canonical issue `#10` and PR `#14` are merged; typed closeout remains asynchronous | no further product execution |

WP-20 (`#5840`) is not an operative Sprint 5 child. It consumes the completed
proof producers as the first child of the final release-tail sprint `#5856`.

### Out-Of-Band Stream

WP-24A (`#5845`) is independent of Sprint 5. Its episode work has no Sprint 5
dependency, is not coordinated by this umbrella, and cannot gate Sprint 5
readiness, execution, review, or closeout. Episode 001 is an informational
checkpoint only; nine episodes remain under WP-24A's separate ownership.

## Recommended Execution Order

For each child, submit its retained split-authority bind request only after the
listed gate is terminal. Ordinary doctor before bind is expected to report
repository identity drift; typed bind supplies the canonical code repository
during its pre-mutation diagnosis, and ordinary doctor runs after successful
binding.

1. Start `#5835` only after `#5826`, `#5827`, and `#5834` are terminal.
2. Start `#5836` only after `#5825`-`#5830`, canonical WP-14 `#209` / PR `#215`, `#5833`, and `#5834` have reviewed ancestral merge proof.
3. Start `#5838` after `#5836` is terminal and all of its other dependencies are satisfied.
4. Start `#5839` after `#5835` is terminal and the v0.93 allocation is explicit.
5. Hand the completed proof-producer set to WP-20 under release-tail sprint `#5856`.
6. Align final publication claims only after `#5843` and explicit operator authorization.

## Watcher Policy

- Each active child session owns its PR, check, and review watch or explicitly hands it to one bounded watcher.
- Waiting is not failure. Record changed gates without starting unrelated or blocked children.
- No optional proof job runs merely because capacity is available.
- Operative child completion requires live issue/PR truth and typed child terminal truth to agree. WP-24 product completion is reported separately from its asynchronous typed closeout.

## Budget And Goal Accounting

- No sprint-global token budget is preallocated.
- Every implementation child gets one issue-bound goal after typed bind and before implementation.
- Actual time and token use are recorded per child when available and are never inferred as zero.

## Watcher Plan

| Issue | Watcher | Current focus | Next terminal state |
|---|---|---|---|
| `#5835` | child session owner | ready to bind | truthful child closeout |
| `#5836` | child session owner | ready to bind | truthful child closeout |
| `#5838` | child session owner | dependency gate | truthful child closeout |
| `#5839` | child session owner | dependency gate | truthful child closeout |

## Safe Parallel Lanes

| Lane | Issues | Why parallel-safe | Required coordination |
|---|---|---|---|
| first downstream pair | `#5835`, `#5836` | Demo and migration planning retain separate child worktrees. | every prerequisite declared by each child STP is terminal |

## Serial Gates

| Gate | Blocks | Exit condition |
|---|---|---|
| migration prerequisites | `#5835` | `#5826`, `#5827`, and `#5834` terminal |
| birthday prerequisites | `#5836` | WP-16's accepted manifest proves reviewed ancestral merges for `#5825`-`#5830`, canonical WP-14 `#209` / PR `#215`, and `#5833`; `#5834` is terminal |
| provider proof | `#5838` | canonical WP-14 `#209` / PR `#215`, `#5834`, and `#5836` complete |
| governance handoff | `#5839` | `#5834` and `#5835` terminal plus explicit v0.93 allocation |
| release truth | final public claims | `#5843` terminal plus explicit operator authorization |

## PVF / Validation-Tail Notes

- Child VPP lanes remain the only authority for implementation proof.
- A deferred validator is a bind-readiness declaration, never validation evidence.
- The umbrella validator proves membership, current state classification, packet completeness, safe ownership, and routing boundaries only.
- Any overlap, unmet operative dependency, live-gate snapshot older than 24 hours, or unsupported completion claim fails closed.

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
- Close the umbrella only after the four operative children have truthful terminal state. WP-20 belongs to release-tail sprint `#5856`; WP-24A is excluded and cannot block closeout.
