# v0.91.8 Release Plan

## Metadata
- Milestone: `v0.91.8`
- Version: `v0.91.8`
- Release date: pending
- Release manager: ADL maintainer

## How To Use

Execute in WP-23 after WP-22. Ceremony confirms existing proof; it does not implement
or repair the product.

## 0. Release-Tail Convergence
- [ ] Baseline, parity, soak, selector, deletion, review, and blocker trackers current
- [ ] All closed issues have truthful SRP/SOR closeout
- [ ] README, CHANGELOG, Cargo manifests, feature list, and milestone docs agree
- [ ] Internal/external review findings resolved or explicitly rejected with rationale
- [ ] v0.92 handoff reviewed

## 1. Release Readiness
- [ ] Milestone checklist complete
- [ ] Quality gate passes
- [ ] Deletion report is at least 80%
- [ ] Go/no-go decision recorded

## 2. Branch And Tag Preparation
- [ ] `main` contains the reviewed selector and deletion result
- [ ] Working tree clean
- [ ] Version surfaces validated
- [ ] `v0.91.8` tag created and verified

## 3. GitHub Release Steps
- [ ] Draft release created from `v0.91.8`
- [ ] Body populated from approved release notes
- [ ] Key issues, PRs, parity, rollback, and deletion evidence linked
- [ ] Release published only after final approval

## 4. Verification
- [ ] Post-release required checks green
- [ ] Fresh install selects the intended generation
- [ ] Explicit compatibility behavior matches the recorded window
- [ ] Immediate regressions routed

## 5. Communication
- [ ] Internal platform handoff published
- [ ] v0.92 maintainers notified with exact consumption boundary
- [ ] Roadmap and status updated

## Exit Criteria

No hidden implementation remains; tag, release, closeout, and v0.92 handoff
are mutually consistent.
