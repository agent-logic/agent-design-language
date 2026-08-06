# Issue 5896 design: pre-topology bound-record migration

## Decision

Add one bounded migration operation to the current `csdlc-issue` owner. It
classifies canonical records that say `bound` without complete branch/worktree
topology and rewrites only records whose truthful disposition is unambiguous.

## Classification

- A record outside the exact `bound` plus incomplete-topology predicate is a
  no-op.
- A terminal record is preserved and reported without reopening it.
- A record with complete, verified Git topology is preserved.
- An open nonterminal record with no matching branch or registered worktree is
  returned to `initialized`, with branch and worktree unset, so normal
  `csdlc-bind` remains the only binding authority.
- Ambiguous or partial live topology, invalid digests, malformed cards, or an
  issue-state lookup failure stops the migration before mutation.

## Transaction And Evidence

The command performs a full read-only classification pass before any write.
It verifies record/card digests through the store, uses the issue store's
atomic commit path, appends a migration audit event, and emits a typed report
with one disposition per issue. A second invocation is a no-op. Dry-run is the
default-safe proof mode and writes nothing.

## Boundaries

The migration does not restore claims, create branches or worktrees, bind
product issues, reopen terminal issues, or rewrite cards and authored evidence.
Live GitHub issue state is an explicit typed input snapshot so the migration is
deterministic and does not hide network access inside record mutation.
