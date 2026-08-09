# Issue 74 design: stale foreign topology records

## Decision

Keep the relevance-first topology scan introduced by #61: parse only branch/worktree identity from each projection, skip unrelated records, and fully deserialize/verify only records that can collide by issue, branch, or canonical worktree. Add the exact missing regression using an unrelated legacy record containing the retired `claim` field.

If the real-binary canary passes on current main, this is a test-and-evidence issue; production code changes are unnecessary. Relevant malformed records and real issue/branch/worktree collisions must continue to fail closed. No stale foreign record is rewritten or deleted.

## Validation

One focused Gate 2 real-binary test covers: unrelated claim-bearing record succeeds; the same malformed record becomes relevant and fails; genuine branch/worktree collisions still fail.
