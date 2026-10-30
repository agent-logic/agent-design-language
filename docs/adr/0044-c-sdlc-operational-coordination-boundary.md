# ADR 0044: C-SDLC Operational Coordination Boundary

- Status: Accepted
- Date: 2026-07-06
- Accepted in: v0.91.7
- Related issues: #4433, #4443, #4713, #4950, #4989
- Related ADRs: ADR 0024, ADR 0028, ADR 0033, ADR 0037
- Source evidence:
  - `docs/milestones/v0.91.7/WBS_v0.91.7.md`
  - `docs/milestones/v0.91.7/review/post_merge_closeout_watcher/POST_MERGE_CLOSEOUT_WATCHER_PROOF_4713.md`
  - `docs/milestones/v0.91.7/review/tooling_closeout/TOOLING_SPRINT_4806_CLOSEOUT_TRUTH_4959.md`
  - `docs/milestones/v0.91.7/review/V0917_WP03_REVIEW_4972.md`
  - `https://github.com/danielbaustin/agent-design-language/issues/4950`

## Context

v0.91.7 made C-SDLC coordination more operational: session ledger claims,
watchers, shepherd output, post-merge closeout packets, and PR validation state
now participate in issue execution. Those surfaces must coordinate work, not
silently take authority over implementation, review, merge, or closeout.

## Decision

ADL should treat the scheduler, watcher, shepherd, session ledger, and
post-merge closeout watcher as coordination infrastructure. They may classify
state, record ownership, create wait-state evidence, and route the next skill,
but they must not replace issue-bound goals, reviewer judgment, PR checks, or
operator-controlled merge/closeout.

## Consequences

- Multi-session work gets durable ownership and wait-state records.
- Stale or contradictory coordination truth becomes a tooling defect.
- Watchers and shepherds must be tested against closed, merged, stale, and
  validated-closeout states.

## Alternatives Considered

### Let watcher/shepherd output close issues automatically

Rejected. Routing evidence is not merge or closeout authority.

## Validation Notes

Review post-merge watcher proof, #4950 watch classification behavior, and
session-ledger doctor output for future changes.

## Non-Claims

- This ADR does not make watchers autonomous.
- This ADR does not claim every historical card is clean.
