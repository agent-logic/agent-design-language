# Structured Task Prompt

Template: 1.0.0

Issue: 306

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #306 only; publication/finish-tail tooling and focused tests without active-lane mutation.

## Deliverables

- Safe publication-tail contract
- Fail-closed publish ordering or safe metadata classification
- Exact-clean finish readiness proof
- csdlc-v2/tests/publication_tail.rs

## Acceptance

1. AC-1: csdlc-publish either includes required publication metadata in the exact pushed head or records local metadata as explicitly safe, nonblocking, and absent from finish's exact-clean git status surface before publication reports success.
2. AC-2: No successful publication result can leave an uncommitted required lifecycle/publication tail or git-visible safe-cache tail that finish must consume or reject while the remote head does not contain it.
3. AC-3: Create, update, and noop publication retries are deterministic and do not duplicate or overwrite publication intent or record truth.
4. AC-4: csdlc-finish can verify the exact published head without requiring a second publication cycle solely to absorb publish-created metadata.
5. AC-5: Focused tests cover create, update, noop, interrupted-after-push, interrupted-after-intent, interrupted-after-record, and finish-readiness cases.
6. AC-6: Existing review-staleness behavior for committed typed metadata remains compatible.
7. AC-7: Fresh exact-head review has no unresolved actionable finding.

## Dependencies

- Observed on #295/#301/#298 publication-tail lanes
- Blocks truthful Sprint 6 terminal finish/closeout for affected open publication tails until fixed or explicitly worked around

## Inputs

- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/src/git.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/tests/gate6.rs

## Non Goals

- No implementation in #295, #301, #298, #258, #5913, or other active issue worktrees or root staging packets
- No PR publication or merge during planning
- No broad lifecycle redesign
- No weakening exact-head review, exact-clean finish, or GitHub remote identity checks
- No arbitrary untracked-file allowlist
