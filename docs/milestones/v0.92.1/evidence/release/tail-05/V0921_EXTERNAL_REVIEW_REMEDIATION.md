# v0.92.1 external-review remediation

External report candidate: `9c7e57d412d61898bd44ab00d53e31afbb779e5c`

## Finding dispositions

| Finding | Disposition | Evidence |
|---|---|---|
| P1 — mandated denominator not exercised | Partially remediated; remains open | A clean detached review at `9c7e57d412d61898bd44ab00d53e31afbb779e5c` executed the focused surfaces listed in `V0921_EXECUTABLE_REVIEW_ADDENDUM.md`. The retained artifact does not yet contain a complete per-lane command/output ledger, so it does not certify the full mandated denominator. It found #818 retained-proof drift and a non-current, blocked #835 release projection. These are evidence/projection blockers, not product-code defects. |
| P2 — #834 packet bound to an earlier source candidate | Resolved | `ISSUE_834_CANDIDATE_REVALIDATION.md` records a successful explicit validation at the review candidate: 14 findings, 8 merged owners, and two negative cases. The source candidate is an ancestral evidence floor, not a claim that the later candidate has identical bytes. |
| P3 — approved removals expose historical pending state | Clarified without rewriting historical packets | The nested `pending_operator_review` values are immutable pre-merge proposal evidence. Their closing PR merges are the terminal approval authority, as #834 records. Consumers must use the later reconciliation/projection rather than reinterpret the historical proposal field as current state. |

## Executable-review blockers

1. Refresh or supersede #818's 17-row retained corporate/Runtime packet at the
   immutable candidate. Its historical packet remains bound to `add8f488…` and
   correctly refuses a current-candidate claim.
2. Regenerate the release projection at the immutable candidate after all
   final-gate evidence is present. Its current `--require-ready` refusal is
   truthful and must remain fail-closed until that work is complete.

No Runtime or product implementation change is required by these findings.

## Closure boundary

This is not release approval. Issue #522 and issue #833 remain open until the
executable review completes, every returned release-blocking finding is fixed,
and the resulting exact head receives a current independent review.
