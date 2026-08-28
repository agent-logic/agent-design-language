# Sprint 8 Product Lanes Design

## Purpose

Coordinate the v0.92.1 product-lane issues `#51`, `#261`, `#262`, `#263`,
`#264`, `#342`, `#511`, and `#512` without taking over child
implementation, proof, review, publication, or closeout authority.

## Execution Contract

- Use a hybrid execution model with independent podcast and Observatory lanes.
- Begin podcast work with `#261`; permit `#342` only after `#261` is terminal,
  then require `#342` and `#261` before `#262`, `#262` before `#263`, and
  explicit future operator authorization before any provider action in `#264`.
- Permit Observatory experience design `#511` independently.
- Move Unity integration `#84` to the backlog under explicit operator direction.
- Keep Observatory implementation `#512` prepared but deferred until both
  `#511` and a later reactivated `#84` are reviewed, merged, and terminal.
- Close coordination parent `#51` only after its children have truthful terminal
  outcomes, including an operator-accepted blocked disposition for `#264` when
  no submission authorization is granted.
- Require issue-bound FastWork worktrees, child-session goals, focused PVF
  proof, exact-head review, green PRs, typed finish, and cleanup for every child.

## Safety Boundaries

- The umbrella writes only its typed coordination record, Sprint Execution
  Packet, readiness evidence, activity log, and integrated sprint review.
- The readiness helper may be repaired only to recognize canonical typed v2
  card bundles; legacy v1 bundle creation remains forbidden.
- Credentials, mailbox verification secrets, provider tokens, recovery codes,
  TLS private keys, and private account data never enter retained evidence.
- Public podcast publication, directory submission, account mutation, and paid
  provider actions remain explicitly operator-controlled.
- A design or implementation must not invent Runtime fields or substitute mocks
  for required authentic Runtime routes.
- Existing ownership stays exact: `#342` owns episode packages, `#262` owns the
  production feed and hosting, backlog `#84` retains Unity integration, and
  `#512` owns only the HTML Observatory redesign surfaces declared in its issue.

## Review And Closeout

- Run one sprint-wide readiness review before child execution.
- Run one integrated code/test/docs/security review after all current members
  are terminal or have an explicit operator-approved disposition.
- Never close `#536` merely because children are waiting, published, or green;
  closure requires reviewed terminal truth and ancestral merges for completed
  implementation children.

## Non-Goals

- Implementing child work in the umbrella.
- Automatically submitting the podcast to any directory.
- Treating asynchronous typed finish or worktree cleanup as a dependency for a
  different child.
- Expanding Sprint 8 into cloud-foundation or distributed-runtime work owned by
  other sprints.
