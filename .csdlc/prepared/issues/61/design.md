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

1. Determine one canonical repository topology root for the bind invocation from Git's registered worktree topology rather than from a scanned projection root.
2. Read each candidate `index.json` only far enough to obtain issue, branch, and stored worktree metadata.
3. Resolve absolute stored paths canonically. Resolve clean relative stored paths, including `.`, against the canonical topology root.
4. Mark a record relevant only when at least one identity predicate holds:
   - its issue equals the requested issue;
   - its stored branch equals the requested branch; or
   - its resolved canonical worktree equals the requested canonical worktree.
5. Only after relevance is established, load the typed record/cards and run full `verify_cards`.
6. Wrap any remaining record, card, design, or diagram filesystem failure with the affected issue and path while preserving the typed error category.

This ordering prevents unrelated retained records from becoming validation dependencies while preserving full verification for actual topology owners.

## Invariants

- Git branch and registered worktree topology remain binding authority.
- The requested issue is always fully verified, even if its stored topology is incomplete or stale.
- A matching branch or matching canonical worktree is always fully verified and can still produce `reconciliation_required`.
- Relative paths never acquire meaning from an arbitrary scanned projection root.
- Retained #5791 evidence is immutable.
- No claim, lease, heartbeat, importer, publication, finish, or cleanup behavior changes.

## Implementation surface

- `csdlc-v2/src/lifecycle.rs`: separate canonical topology resolution, cheap relevance classification, and relevant-record verification; add issue/path context to surviving filesystem errors.
- `csdlc-v2/tests/gate2.rs`: add a real-binary regression with a #5791-shaped historical dot record and absent authored artifacts, plus genuine issue/branch/worktree collision controls.
- `csdlc-v2/operator/skills/csdlc-v2-bind/SKILL.md`: update only if the implemented typed diagnostic or topology rule needs operator-facing clarification.

## Regression design

The focused Gate 2 fixture will create a primary repository and distinct issue worktree, retain an unrelated historical record with `worktree: "."` and missing design/diagram paths, and prove the new issue binds successfully. Companion cases will prove that:

- the same issue remains relevant and fails closed when corrupt;
- a matching stored branch remains relevant and blocks reconciliation;
- a matching canonical stored worktree remains relevant and blocks reconciliation;
- a surviving filesystem failure names the historical issue and missing path instead of returning raw `os error 2` alone.

## Validation and estimates

- Focused Gate 2 integration test: 540 seconds, 6,000 tokens.
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
