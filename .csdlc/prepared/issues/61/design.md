# Issue #61 design: canonical historical worktree topology during bind

## Decision

`csdlc-bind` will classify topology relevance from cheap record metadata before it loads or verifies the six cards and authored artifacts. A stored relative worktree path is interpreted against one canonical repository topology root, never against whichever listed worktree happens to contain a tracked copy of the historical record.

The implementation remains fail closed for records that are genuinely relevant by issue number, stored branch, or canonical worktree identity. Only records proven irrelevant are skipped before expensive/full verification.

## Observed defect

`bind_issue` enumerates every Git worktree and calls `issue_records` with a `Store` rooted at each listed path. `issue_records` currently evaluates:

```text
same_worktree(store.root(), stored_worktree, requested_worktree)
```

For a retained record whose stored worktree is `.`, `store.root()` is the worktree currently being scanned. The same tracked historical record therefore appears to own every listed worktree in turn. When that false match occurs, `verify_cards` reads the historical design and diagram before bind can reject the record as irrelevant. Issue #5791 demonstrates the failure: its retained record stores `worktree: "."` and intentionally untracked `.adl/local-artifacts/5791-bootstrap/*` references, producing an unqualified `ENOENT` while binding an unrelated issue.

## Proposed boundary

1. Run `git worktree list --porcelain`, take Git's first/primary worktree entry, canonicalize it, and require it to be an existing real directory whose Git common directory equals the invocation checkout's Git common directory. Zero entries, a missing or non-canonical primary entry, or a common-directory mismatch fails closed as unsafe topology.
2. Read each candidate `index.json` only far enough to obtain issue, branch, and stored worktree metadata.
3. Normalize stored paths against that primary root using the existing worktree-path rules: an existing absolute path is canonicalized; a nonexistent absolute path is rebuilt from its nearest existing canonical ancestor; `.` means the canonical primary root; every other relative value must contain only clean normal components and is resolved beneath the primary root with the same existing/nonexistent normalization.
4. Compute and retain three explicit predicates for every candidate: `same_issue`, `same_stored_branch`, and `same_canonical_worktree`. Mark a record relevant only when at least one predicate holds:
   - its issue equals the requested issue;
   - its stored branch equals the requested branch; or
   - its resolved canonical worktree equals the requested canonical worktree.
5. Only after relevance is established, load the typed record/cards and run full `verify_cards`. Carry the three predicates through verification; the scanned projection's listed branch/path identifies only where the projection was found and is never collision authority.
6. Make idempotence and reconciliation decisions from the verified record plus the retained predicates. `same_issue` permits only the existing exact bound/idempotent case; otherwise any verified `same_issue`, `same_stored_branch`, or `same_canonical_worktree` candidate fails with `reconciliation_required`, even when its projection was read from an unrelated listed worktree.
7. Wrap any remaining record, card, design, or diagram filesystem failure with the affected issue and path while preserving the typed error category.

This ordering prevents unrelated retained records from becoming validation dependencies while preserving full verification for actual topology owners.

## Invariants

- Git branch and registered worktree topology remain binding authority.
- The requested issue is always fully verified, even if its stored topology is incomplete or stale.
- A matching branch or matching canonical worktree is always fully verified and can still produce `reconciliation_required`.
- Relative paths never acquire meaning from an arbitrary scanned projection root.
- Retained #5791 evidence is immutable.
- No claim, lease, heartbeat, importer, publication, finish, or cleanup behavior changes.

## Implementation surface

- `csdlc-v2/src/lifecycle.rs`: separate canonical topology resolution, cheap relevance classification, predicate-carrying relevant-record verification, and predicate-based collision decisions; add issue/path context to surviving filesystem errors.
- `csdlc-v2/tests/gate2.rs`: add a real-binary regression with a #5791-shaped historical dot record and absent authored artifacts, plus genuine issue/branch/worktree collision controls.
- `csdlc-v2/operator/skills/csdlc-v2-bind/SKILL.md`: update only if the implemented typed diagnostic or topology rule needs operator-facing clarification.

## Regression design

The focused Gate 2 fixture will create a primary repository and distinct issue worktree, retain an unrelated historical record with `worktree: "."` and missing design/diagram paths, and prove the new issue binds successfully. Companion cases will prove that:

- the same issue remains relevant and fails closed when corrupt;
- a matching stored branch observed from an unrelated projection remains relevant and blocks reconciliation;
- a matching canonical stored worktree observed from an unrelated projection remains relevant and blocks reconciliation;
- a surviving filesystem failure names the historical issue and missing path instead of returning raw `os error 2` alone.

## Validation and estimates

- Exact Gate 2 regression `bind_topology_scan_uses_canonical_record_identity`: 540 seconds, 6,000 tokens.
- Strict C-SDLC v2 Clippy: 600 seconds, 2,500 tokens.
- Diff hygiene: 60 seconds, 500 tokens.
- Planned implementation elapsed time: 7,200 seconds.
- Planned implementation token estimate: 40,000 tokens.
- Planned validation time: 1,200 seconds.

## Stop conditions

- Correctness would require mutating retained historical issue records.
- The proposed classifier cannot prove a record irrelevant before full verification.
- A change would weaken verification for a matching issue, branch, or canonical worktree.
- Scope expands beyond bind topology classification, focused proof, and strictly necessary operator documentation.
