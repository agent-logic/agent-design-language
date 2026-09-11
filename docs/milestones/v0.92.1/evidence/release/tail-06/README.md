# TAIL-06 third-party review remediation

The accepted finding denominator for #522 is 19: the 14 findings from the #520
internal review plus the five findings from the #521 third-party review.

## Internal-review findings

| Finding IDs | Remediation issue(s) | Disposition state |
| --- | --- | --- |
| `D520-RUNTIME-001`, `D520-RUNTIME-002`, `D520-SEC-004` | #814 | Merged; exact-head reconciliation is part of #834. |
| `D520-SEC-001`, `D520-SEC-002` | #815 | Merged; exact-head reconciliation is part of #834. |
| `D520-SEC-003`, `D520-TEST-001` | #816 | Merged; exact-head reconciliation is part of #834. |
| `D520-V3F-001` | #817, #843 | #817 merged; #843 is the required follow-on for the retained legacy ready-intent gap exposed by the #835 current-candidate proof. |
| `D520-REL-001`, `D520-DOC-003`, `D520-DOC-004`, `D520-EVID-001`, `D520-EVID-002` | #817 | Merged; exact-head reconciliation is part of #834. |
| `D520-RET-001` | #818, #819, #820, #821 | Merged remediation inputs; final projection is part of #835. |

## Third-party findings

| Finding | Severity | Remediation issue | Disposition state |
| --- | --- | --- | --- |
| `TPR-001` | P1 | #833 | Ready after #521 / PR #850 merges; must run last against the immutable post-remediation candidate. |
| `TPR-002` | P1 | #834 | Merged in PR #841. |
| `TPR-003` | P1 | #835 | PR #840 merged; repaired-candidate verification remains part of the final #833 review after #843 and #844. |
| `TPR-004` | P2 | #836 | Merged in PR #839. |
| `TPR-005` | P2 | #837 | Merged in PR #842. |

## Additional release blockers exposed during remediation

- #843 / PR #845 repairs the retained legacy ready-intent target gap and is now
  merged. It contributes to the existing `D520-V3F-001` disposition rather
  than adding a new finding.
- #844 / PR #847 added the native v3 PR merge operation and is merged at
  `25498d7709cb5fe13242aac81988ecdb344db968`.
- The post-merge reassessment found `MERGE-LINKAGE-001`: merge admission does
  not bind the reviewed closing/part-of relation. The operator explicitly
  deferred that repair to v0.92.2 as #849. This disposition is a residual risk,
  not a behavioral pass and not v0.92.1 release approval; four affected release
  criteria remain unproved until #833 classifies the final candidate.
- After #521 / PR #850 merges, #833 must bind the resulting immutable candidate,
  incorporate the complete 121-row historical and 51-row current reassessments,
  preserve #849 as a v0.92.2 deferred residual, and perform the final
  third-party review.

Final publication is fail-closed until #520 and #521 supply their merged
exact-revision source reports, all 19 findings have exactly one reviewed
disposition, and no release blocker remains. This file records work routing
only; it is not a release decision or remediation proof.
