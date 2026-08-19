# Issue 425 Design: Recordless Already-Merged Closeout Recovery

## Outcome

Add a typed C-SDLC v2 recovery/classification route for v0.92 issues that are already closed by merged GitHub PRs but have no local issue projection at the merged PR head. The route must let the closeout sweep move forward without synthesizing normal implementation cards or weakening ordinary active-issue publication requirements.

## Problem Boundary

Normal historical finish currently requires `Store::load_record(issue)` before it can derive terminal truth. That is correct for normal issue projections, but it leaves no typed way to classify or close already-merged legacy/current issues when the merged PR head does not contain `.csdlc/issues/<issue>/index.json`.

The route owns only C-SDLC v2 terminal recovery/classification behavior. It does not change product Runtime, Observatory, provider, Unity, AWS, or workflow-card content.

## Recovery Contract

Introduce a typed request that names:

- issue number;
- issue/code repository;
- merged PR number;
- expected PR head SHA;
- expected merge SHA;
- actor and reason;
- token source;
- recovery mode: `classify_only` or `recordless_terminal`.

The implementation must prove live GitHub state before recording anything:

- issue is closed;
- PR is merged;
- PR repository, base/head identity, head SHA, merge SHA, and closing issue linkage match the request;
- no existing local projection, publication evidence, terminal receipt, or contradictory retained publication path makes the result ambiguous.

`classify_only` emits a machine-readable classification and performs no closeout writes. `recordless_terminal` may emit a minimal terminal receipt only when the request proves enough exact authority to avoid pretending normal cards, review, publication, or implementation evidence existed.

## Contradictory Precedence

#248 must remain fail-closed unless a future typed operation resolves precedence between live merged PR #247 and retained publication evidence for PR #249. This issue must not falsely mark PR #249 merged or silently pick PR #247 over retained local publication truth.

## Validation

Focused Rust tests should cover:

- positive recordless merged closeout classification/materialization;
- no projection at PR head;
- wrong head SHA;
- wrong merge SHA;
- unmerged/open PR;
- missing closing linkage;
- wrong repository;
- existing contradictory publication evidence;
- #248-style contradictory terminal precedence.

The v0.92 sweep must be able to rerun after this fix and produce either 92/92 receipts or a retained blocker list containing only irreducible contradictory cases.

## Non-Goals

- No raw GitHub writes.
- No manual card synthesis.
- No product implementation changes.
- No weakening normal publish/review/finish gates for active issues.
- No treating checkpoint or `part_of` PRs as terminal closing authority.
