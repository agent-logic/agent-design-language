# Sprint 8 — Product Lanes Execution Packet

## Identity

- Umbrella: `#536`
- Milestone: `v0.92.1`
- Mode: `hybrid`
- Membership version: `5`
- Active members: `#51`, `#261`, `#262`, `#263`, `#264`, `#342`, `#511`, `#512`
- Independent backlog: `#84`; it is not active Sprint 8 work and does not gate `#512`.
- Machine-readable authority: `.csdlc/prepared/issues/536/sprint-execution-packet.yaml`

## Goal And Boundary

Coordinate the podcast and Observatory product lanes without implementing child
work in the umbrella. Every child keeps its own typed lifecycle, FastWork
worktree, session goal, proof, review, PR, finish, and cleanup.

## Opening Wave

## Child Issue Wave

The active child wave is `#261`, `#342`, `#262`, `#263`, `#264`, `#511`,
`#512`, and coordination closeout `#51`. Backlog `#84` is excluded from the
active denominator and is independent of `#512`.

## Recommended Execution Order

Open `#261` and `#511` independently. Advance the podcast chain through
`#342`, `#262`, and `#263`; run `#264` only with provider-specific operator
authorization. Start `#512` after `#511` is reviewed and terminal, then
reconcile the podcast lane in `#51`.

## Safe Parallel Lanes

| Lane | Issue | Initial status | Gate | Owned result |
|---|---:|---|---|---|
| Podcast foundation | `#261` | ready after typed child setup | Umbrella readiness | Approved show identity, artwork, rights, metadata, mailbox readiness |
| Observatory design | `#511` | ready after typed child setup | Umbrella readiness | Reviewed experience-design contract |

`#261` and `#511` are the only initial execution lanes. They have distinct
write sets. `#512` remains prepared until its actual dependency `#511` is
reviewed and terminal; `#84` remains independent backlog.

## Serial Gates

1. `#261 -> #342 -> #262 -> #263`.
2. `#264` additionally requires explicit provider-specific operator authorization.
3. `#511` is required before `#512`; `#84` is not a gate.
4. `#51` reconciles the podcast children after their terminal outcomes; a
   blocked `#264` counts only with an explicit operator-accepted disposition.
5. `#536` closes only after all current members and the integrated sprint review.

## Candidate Parallel Lanes

- `#261` and `#511` are safe opening candidates with distinct write sets.
- `#264` is blocked on serial prerequisites and external authorization.
- `#512` is blocked on `#511` only; preparation is not authority
  to start implementation.

## Operator Work

The operator must personally decide or authorize:

- the final podcast show identity;
- company-mailbox ownership and verification;
- each podcast directory submission and any public launch action;
- paid or externally mutating provider activity in the owning child.

No credentials, verification codes, recovery data, TLS private keys, or private
account material may enter retained evidence.

## Budget And Goal Accounting

The umbrella goal covers preparation only. Every implementation child creates
its own issue-bound goal after typed bind/readiness. Paid or externally mutating
work requires a child-specific budget and explicit authorization.

## Watcher Policy

Waiting children use issue-local watchers for dependencies, CI, review, merge,
or operator authority. Waiting is not completion and does not consume another
child's lifecycle authority.

## Watcher Plan

Watch `#264` for a provider authorization after `#263` is terminal. Watch
`#512` for terminal `#511`. Track `#84` independently in the backlog. Other
children advance only through the serial gates above.

## Parallelism Outcome Plan

Record actual lanes, collisions, prediction misses, and any reclassification in
the sprint activity log before expanding concurrency.

## Readiness And Review

- Sprint readiness: `.csdlc/evidence/536/readiness.json`
- Activity log: `.csdlc/evidence/536/activity.jsonl`
- Integrated sprint review: `.csdlc/evidence/536/sprint-review.md`
- One bounded exact-head review per child before publication.
- One integrated sprint review after all members are terminal or explicitly
  dispositioned; no repeated umbrella review loops without material change.

## Sprint Closeout Rollup Expectations

The final rollup names membership version 5, every completed child's reviewed
head, green checks, merge commit and ancestry, and every accepted blocked or
deferred disposition. It must not imply that backlog `#84` ran in Sprint 8.

## Initial Stop Conditions

- Any child lacks an issue-specific six-card bundle.
- Any write-set ownership collision is unresolved.
- `#512` is started before `#511` is reviewed and terminal.
- `#264` is started without explicit provider-specific authorization.
- A mock substitutes for an authentic Runtime route required by acceptance.
- Any retained evidence would expose private account or credential material.
