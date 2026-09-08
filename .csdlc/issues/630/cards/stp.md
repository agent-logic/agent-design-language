# Structured Task Prompt

Template: 1.0.0

Issue: 630

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only the V3-H.4 finish, cleanup, and cutover command routes and issue-owned proof.

## Deliverables

- csdlc finish route behavior and tests
- csdlc clean route behavior and tests
- csdlc cutover decision route behavior and tests
- Real issue canary coverage for terminal and cleanup denial paths
- Issue-owned validation script and retained evidence

## Acceptance

1. AC-1: Finish derives terminal truth from authenticated live PR/issue state and rejects caller-forged terminal authority.
2. AC-2: Cleanup derives worktree authority from actual Git worktree registration, not caller booleans or self-derived digests.
3. AC-3: Cleanup preserves distinct states: absent, unregistered, dirty, live, already removed, removable, and removed.
4. AC-4: Cutover records explicit operator approval, rollback evidence, selected binary provenance, and fail-closed rollback/undo boundaries.
5. AC-5: Tests include positive terminal closeout, stale/nonmerged/part-of denial, dirty/live worktree denial, symlink/path escape denial, and rollback refusal cases.
6. AC-6: No v2 source changes.

## Dependencies

- #625 sprint umbrella
- #627 V3-H.1 command denominator
- #629 V3-H.3 GitHub/publication lane

## Inputs

- agent-logic/agent-design-language#630
- csdlc-v3/AGENTS.md
- docs/csdlc-v3/v3-command-manifest.json
- csdlc-v3/src/main.rs
- csdlc-v3/src/commands/remote/mod.rs

## Non Goals

- Perform #505 cutover
- Retire v2
- Merge or close #505
- Mutate GitHub from v3
- Change v2 source code
