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
| `TPR-001` | P1 | #833 | PR #853 merged the failed immutable-candidate review and executable addendum. Its six returned findings are now mandatory inputs to this ledger; #833 remains administratively open pending their final disposition. |
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
- #851 was explicitly stopped and closed as `superseded`, not completed. The
  operator moved its five evidence-linkage rows and Runtime failure-event work
  to v0.92.2 issue #852. This is an owned residual and does not convert the
  missing executable proof into a v0.92.1 pass.
- #833 reviewed immutable candidate `9c7e57d412d61898bd44ab00d53e31afbb779e5c`.
  The retained report and executable addendum remain `FAIL — changes required`:
  they preserve the incomplete review denominator, stale candidate projection,
  historical #834 boundary, and explicit proof omissions. PR #853 merged as
  `99700441767c43aa69e379b7b59f0f2a2c82d879`; its report is now a terminal
  #522 input, not release approval.
- #856 owns the release-version reconciliation and native-v3 release-preflight
  repair returned during #526 preparation and preserved by #833. It remains a
  release blocker until reviewed and merged.

Final publication is fail-closed until #520 and #521 supply their merged
exact-revision source reports, #833/PR #853 supplies its terminal immutable
review, every returned finding has exactly one reviewed fix or explicit
operator-authorized residual disposition, #856 is complete, and the final
ledger reports no unresolved release blocker. This file records work routing
only; it is not a release decision or remediation proof.
