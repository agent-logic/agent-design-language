# v0.92.1 executable review addendum

Candidate: `9c7e57d412d61898bd44ab00d53e31afbb779e5c`

Checkout: clean detached worktree at the exact candidate.

Verdict: **FAIL — changes required**

## Findings

### P1 — Corporate/Runtime retained proof is stale

The #818 validator refused the candidate with `current_source_drift`. Its
reconciliation remains bound to `add8f488…`, records zero candidate execution,
17 pending operator approvals, and `release_ready: false`.

### P1 — Release projection is not candidate-current or ready

`project_release.py --require-ready` correctly refused readiness. The projection
is bound to `64a99fd…`; #835 records 51 execution refreshes and four final-gate
obligations unresolved.

### P2 — #834 is historical ancestral reconciliation

The #834 validator passes when explicitly evaluated against the candidate, but
the packet remains sourced from `64a99fd…`. It is valid historical
reconciliation, not exact-candidate execution proof.

## Passing executable coverage

- TAIL-05 production validator and eight adversarial fixtures
- #834 validator and two negative tests: 14 findings, eight merged owners
- #815 authorization forgery/replay matrix
- #816 29-path manifest and OBS-B redaction proof with 11 negatives
- #817 release-truth, current-status, and V3-F validators
- #819: 152/152 rows and 18 negative cases
- #820: 25/25 rows and 13 negative cases
- Runtime: eight greeting, two Shepherd-boundary, and eight config-reload tests
- #836 recursive measurement and three negative fixtures
- #837 boot-path suite: ten tests and six command-surface checks
- exact-range diff hygiene and final detached-worktree cleanliness

No candidate, `main`, GitHub, product code, or lifecycle state was modified by
the executable review.
