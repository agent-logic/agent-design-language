# v0.91.8 Milestone Checklist

## Metadata
- Milestone: `v0.91.8`
- Version: `v0.91.8`
- Target release date: pending execution
- Owner: ADL maintainers

## Purpose

Ship/no-ship gate. Check items only when cited evidence exists.

## Planning
- [ ] Goal and scope reviewed in `VISION_v0.91.8.md` and `DESIGN_v0.91.8.md`
- [ ] WBS and GitHub issue mapping complete
- [ ] Decision and ADR plans reviewed
- [ ] Sprint execution packet approved

## Execution Discipline
- [ ] Every issue uses all six C-SDLC v2 cards
- [ ] Every implementation issue was bound to a dedicated worktree and goal
- [ ] Exact-revision review preceded publication
- [ ] SORs identify local proof, CI proof, deferrals, and residual risks

## Quality Gates
- [ ] Characterization corpus and normalizer reviewed
- [ ] Language/compiler/engine focused validation green
- [ ] Runtime/provider/tool adapter boundaries proven
- [ ] Shadow parity has no unclassified mismatch
- [ ] Warm/full validation and binary/dependency budgets pass
- [ ] Rollback proof passes
- [ ] Deletion is at least 80%, with 90% target disposition recorded
- [ ] ADL v2 stable installation, default selection, operations, and recovery pass
- [ ] Runtime v3 deployment topology, readiness, operations, recovery, and consumer proof pass
- [ ] C-SDLC v2 stable binaries, skills, init-to-closeout lifecycle, publication, and recovery pass
- [ ] Unity Observatory and Adaptive Learning DAG WP-14 children are closed or carry operator-approved evidence-backed blockers
- [ ] No unresolved blocker or critical review finding

## Release Packaging
- [ ] Release notes match shipped behavior
- [ ] v0.92 handoff names exact revisions and residual risks
- [ ] Tag and GitHub release drafted and verified
- [ ] Links and retained proof artifacts validated

## Post-Release
- [ ] Issues, cards, PRs, and milestone truth reconciled
- [ ] Compatibility expiry and retained-surface ownership recorded
- [ ] Follow-up bugs routed
- [ ] Retrospective and final deletion report retained

## Exit Criteria

All gates pass or the milestone does not release. The 80% deletion gate has no
exception path.
