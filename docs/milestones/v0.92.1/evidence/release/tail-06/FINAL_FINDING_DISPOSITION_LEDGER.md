# v0.92.1 final finding-disposition ledger

Status: **all dependency blockers resolved; final exact-head review and publication pending**

This ledger preserves two denominators instead of collapsing them:

- 19 original source findings: 14 accepted internal-review findings from #520
  and five retained failed-review findings from #521;
- six findings returned by the immutable-candidate #833 review retained in
  merged PR #853.

The terminal disposition denominator is therefore 25. The machine-readable
source records are `source-findings.json` and `returned-findings.json`.

## Original source findings

| Finding | Disposition | Reviewed remediation owners | Release consequence |
| --- | --- | --- | --- |
| `D520-RET-001` | Fixed, with separately disclosed proof residuals | #818, #819, #820, #821 | The accepted finding is remediated; #849/#852 and the seven remaining proof-insufficient rows are not promoted to behavioral passes. |
| `D520-V3F-001` | Fixed | #817, #843 | Native-v3 truth and retained ready-intent recovery are present in the candidate. |
| `D520-REL-001` | Fixed | #817 | Canonical release truth is retained; final version/preflight repair is separately owned by #856. |
| `D520-DOC-003` | Fixed | #817 | Corrected current release documentation is retained. |
| `D520-DOC-004` | Fixed | #817 | Corrected current release documentation is retained. |
| `D520-EVID-001` | Fixed | #817 | Candidate evidence and validation are retained. |
| `D520-EVID-002` | Fixed | #817 | Candidate evidence and validation are retained. |
| `D520-RUNTIME-001` | Fixed | #814 | Runtime greeting/recovery identity behavior is retained and executable. |
| `D520-RUNTIME-002` | Fixed | #814 | Runtime local-model boundary behavior is retained and executable. |
| `D520-SEC-001` | Fixed | #815 | Cloud mutation authorization is authenticated and fail-closed. |
| `D520-SEC-002` | Fixed | #815 | Cross-account/project replay and forged authorization are rejected. |
| `D520-SEC-003` | Fixed | #816 | Publication redaction is validated against production serialization and negative fixtures. |
| `D520-SEC-004` | Fixed | #814 | Loopback-provider origin validation and redirect denial are executable. |
| `D520-TEST-001` | Fixed | #816 | The retained redaction and hot-reload proof surfaces are non-vacuous. |
| `TPR-001` | Fixed | #833 | PR #853 retains the immutable candidate, failed review, executable review, and limitations without converting the review into a pass. |
| `TPR-002` | Fixed | #834 | Finalized predecessor reconciliation was explicitly revalidated at the #833 candidate. |
| `TPR-003` | Fixed | #835 | Current release projection preserves unresolved gates rather than requesting closure on blocked rows. |
| `TPR-004` | Fixed | #836 | Recursive Rust measurement and negative fixtures are retained. |
| `TPR-005` | Fixed | #837 | Active boot paths and retired command surfaces are explicitly documented and tested. |

## Findings returned by #833

| Finding | Disposition | Evidence owner | Release consequence |
| --- | --- | --- | --- |
| `D833-REPORT-001` | Fixed by executable follow-up; the chat-only report remains non-proving | #833 / PR #853 | The complete executable lane ledger, omissions, and limitations remain visible. |
| `D833-REPORT-002` | Fixed | #833 and #834 / PR #853 | Candidate-current #834 revalidation is retained without rewriting the historical packet. |
| `D833-REPORT-003` | Fixed by explicit historical-state clarification | #833 and #835 / PR #853 | Consumers use the later terminal projection; historical proposal fields remain immutable. |
| `D833-EXEC-001` | Fixed by narrow candidate-current supersession proof | #833 / PR #853 | Only the exact #818 17-row proposal/approval scope is superseded; no wider release claim is made. |
| `D833-EXEC-002` | Fixed | #856 / PR #858 | PR #858 merged at reviewed head `f4ea095a`; #833 consumed the repair in its later native terminal reconciliation. |
| `D833-EXEC-003` | Fixed | #833 and #834 / PR #853 | #834 remains historical ancestral evidence plus an explicit candidate revalidation, not exact-candidate execution proof. |

## Explicit residuals that are not passes

- `MERGE-LINKAGE-001` remains owned by #849 for v0.92.2. Four affected
  criteria remain unproved.
- The five #851 evidence-linkage rows were moved by explicit operator decision
  to v0.92.2 issue #852. #851 closed as superseded; it did not complete those
  rows.
- Five cloud-control rows and two execution-proof rows remain classified as
  insufficient proof. They are not invented findings in the original
  #520/#521 denominator and are not represented as behavioral passes.

These residuals are accepted planning limitations for the next milestone, not
release approval and not evidence that the omitted behavior passed.

## Closure boundary

Final publication remains pending until all of the following are true:

1. PR #858 is green, reviewed, and merged at its exact final head (**satisfied**);
2. #833 is terminally reconciled after the #856 repair (**satisfied**);
3. the machine-readable disposition and blocker records validate all 25 rows;
4. an independent exact-head review finds no unresolved ledger defect; and
5. the packet manifest binds every source, disposition, validation, and review
   artifact.

This ledger is not release ceremony, tag authority, publication approval, or a
claim that deferred proof passed.
