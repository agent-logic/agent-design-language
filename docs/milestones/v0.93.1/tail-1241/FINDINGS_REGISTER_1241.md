# TAIL-06 findings and evidence-gap register (#1241)

Snapshot: 2026-10-14. This is bounded preparation, not an acceptance result. The operator requested all Sprint 4 work in parallel after #1223 merged. PR #1273 merged as `791e44216659a14eeb95c8a05f79e965709b3421` from `129e5017508ac958311cf44553d20fddb997fa34`; #1223 is closed. Native finish/cleanup is asynchronous and is not represented as completed here. No new build, provider call, deployment, tag or release is authorized by this document.

This register seeds known evidence gaps; it is **not** the complete #1238/#1240 finding denominator. Their final raw reports are outstanding. No missing report is interpreted as zero findings.

| ID | Observed evidence / impact | Owner and next action | Disposition |
| --- | --- | --- | --- |
| PREP-01 | #1236 at `d1ec387dfb015cb2e0e2aa12285d5f2fa20d2831` has no final selected seven-role source/artifact/interface lockset. Historical identities cannot support final release claims. | #1236 with #1232: reconcile accepted #1223 results and affected role/edge proof. | Open evidence gap; no waiver. |
| PREP-02 | Same inventory records three different template manifest byte identities under version 1.0.0. Installed consumed manifest remains unselected. | #1236 / CodeFriend owner: bind actual installed template identity and assess changed evidence; keep CodeFriend #101 visible. | Open correspondence gap, not three asserted product defects. |
| PREP-03 | Product archive metadata confirms availability, but inventory did not download/re-hash the 260 MB archive. | #1236: reuse authentic retained integrity evidence if applicable; otherwise verify exact required bytes once. | Open integrity correspondence; no redundant download mandated here. |
| PREP-04 | Retained rollback describes private-prefix restore, Runtime retained generation and website disablement, not a complete selected-candidate rollback decision. | #1236: attach bounded owner-approved rollback evidence and limitations. | Open evidence gap. |
| PREP-05 | Internal review plan requires final findings; external review has not executed in this preparation. | #1238 / #1240: supply actual reports, candidate pins, coverage and limitations. | Missing review inputs; not a PASS. |
| PREP-06 | #1234/#1236 owners reported native SRP rendered-card drift preventing publication. | Existing tooling owner / root coordination: use existing repair evidence, no new duplicate repair or replay here. | Workflow symptom, separate from product quality. |

Source: #1236 `docs/milestones/v0.93.1/publication-1236/CANDIDATE_INVENTORY.md` and #1238 `docs/milestones/v0.93.1/review/INTERNAL_REVIEW_PLAN_1238.md`. #1223-open statements in those preparation snapshots are stale; PR #1273 is merged. The remaining observations must be reconciled with new upstream artifacts, not silently carried as current defects forever.

## Completing the register

Append every raw #1238/#1240 finding with its original ID, reviewer, candidate and evidence. Deduplicate by shared cause without dropping source IDs or lowering supported severity. Record one outcome: fixed with exact owning-repository commit and affected proof; evidence-backed non-finding with rationale; or explicitly authorized deferral with owner, risk, successor issue and release-note impact. Preparation gaps above are not automatically accepted review findings.

Only rerun proof invalidated by the actual fix. Preserve prior failures and distinguish source, installed, CI and review evidence. Unresolved launch-critical criteria remain unmet; a deferral or closed issue cannot manufacture acceptance. Acceptance waits for the complete review denominator and reviewed disposition of every row.
