# Structured Intent Prompt

Template: 1.0.0

Issue: 199

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make OpenRaft learner, joint, final, removal, and governed rejoin transitions agree exactly with Runtime MembershipState, AuthorityMembership, route, and pending-exclusion authority without any node, leader, caller, or stale local history self-promoting.

## Required Outcome

Every voter-set change consumes an opaque #201 membership-operation token, catches learners to the exact committed boundary, observes durable OpenRaft joint and final configurations, publishes exact concrete authority parity through a crash-reconciled checkpoint, and denies removed or stale voters from the pending phase onward.

## Scope

- Crash-resumable MembershipTransitionCoordinator
- Opaque #201 membership-operation token consumption
- Learner enrollment, canonical snapshot or log catch-up, and readiness proof
- Standard OpenRaft joint and final membership transition orchestration
- Exact MembershipState, AuthorityMembership, and route-cut parity publication
- Pending removal exclusion and governed stale-state rejoin
- Focused real-node transition, restart, rollback, and fault proof

## Authority

- Only an opaque #201 token plus exact concrete old-cut parity authorizes a transition
- OpenRaft remains the sole joint-consensus authority; the coordinator observes its committed joint and final states rather than reimplementing voting
- Caller routes are hints for an authorized learner and cannot choose voters, roles, certificates, or Raft ids
- Pending removal denies new membership, endorsement, routing, renewal, Shepherd, and Observatory authority but does not claim the concrete #200 FencingStore mutation
- Rejoin always begins as a governed learner and cannot reuse retained local voting authority
- Certificate, lease, concrete fence, owner, Shepherd, Observatory, migration, and recovery store application belongs to #200

## Assumptions

- none

## Operator Constraints

- Do not bind or edit product source until PR #197, #201, and learner-transport prerequisite #202 are externally reviewed, merged, and ancestral
- Use typed C-SDLC v2 lifecycle commands and an issue-bound execution worktree
- Keep #199 bounded to membership transition coordination; route concrete authority stores to #200
- Run fresh independent exact-head review before publication
- Open a ready PR for visibility but never merge before operator review and authorization
- No AWS use and no lifecycle closeout
