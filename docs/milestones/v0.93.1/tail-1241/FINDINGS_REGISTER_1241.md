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

## 2026-10-15 reconciliation

Inspected #1236 at `abb3d067c840be870a09f561ac216ffc7df5bd3b` and #1238 at `78894c8a44ebc61f7fc6ce2a5ccf10ed6086532a`. #1236 now correctly records merged #1223 and draft documentation PR #1276; its inventory remains explicitly not frozen. PREP-01 through PREP-05 therefore remain unresolved within these exact inputs. PREP-06 is historical workflow evidence: #1234 has since published, so do not continue treating its old SRP failure as current. #1236 native admission must be assessed by its owner, not inferred from this old symptom.

#1244's actual successor planning review records PLAN-01 (canonical #1259 reconciliation not applied) and PLAN-02 (complete #1241 residual input absent). These are successor planning completeness findings owned by #1242/#1243 and #1241 respectively. They are not substitutes for #1238/#1240 product findings. No author is authorized here to defer missing product evidence or to close launch-critical findings.

Final product finding denominator remains unknown until the actual internal and external reports arrive. This register is not complete acceptance merely because currently known preparation gaps have owners.

## Actual independent finding intake — 2026-10-15

Source: independent reviewer `/root/tail1240_independent`, exact ADL packet `22816165efff2416f0bba84d7b1e2c4b9a2acf2e`; tracked result in #1240 `tail-1240/INDEPENDENT_REVIEW_RESULT_1240.md`. This intake contains **one actual P3 finding**, no established P0–P2 defect in the examined scope; it is not a zero-gap release decision.

| Finding | Evidence / effect | Owner and disposition |
| --- | --- | --- |
| IR-1240-01 (P3) | Inventory says 185 unchanged public inputs; final correspondence has 182 unchanged plus three separately validated guard changes. Compiled product inputs remain unchanged. | #1236 wording fix assigned; open until exact correction is checked. No product rebuild required. Preserve intermediate and final counts. |

The raw report's “#1242/TAIL-06” routing is corrected here to #1241/TAIL-06; #1242 is successor planning. No severity or substantive observation changed.

CodeFriend #101 exact source `393dd2ee7c5db8e6108a27cf6441e380fb62f4c3` now has owner delivery evidence: 90/0 native tests, 6/6 CI checks, artifact `11994981448` from run `38777770708`, GitHub archive digest `2be7a8804199bf578388637915e399bf8e9cf23ac5057f4f4ef180b544c1f6f0`. Its 25-kind/50-view gallery preserves all final view bytes and explicitly limits readable-scale review coverage. PREP-02/03 are narrowed to final selected consumption and cross-version correspondence; new artifact existence and current CI are no longer missing. This intake did not download, hash or install the archive.

PLAN-01 is resolved by #1242 canonical reconciliation `99628d13ed1af84141dd038646363e82e5c64320`; #1244 retains the reviewed resolution. PLAN-02 and PREP-05 remain open for the actual #1238 result and final residual reconciliation. The combined internal/external denominator is still incomplete. Artifact availability, successful CI and a minor review finding do not alone establish milestone acceptance.

## IR-1240-01 fixed

Verified #1236 correction at `bd93dab710d99c076a7030b6908af4fd9e301a34`, inventory line 22: intermediate 185 inputs, final 182 unchanged plus three separately validated source-retirement guard changes; compiled product inputs remain unchanged. Root independently verified the same correction. IR-1240-01 is **fixed**; the original finding is retained above. No full review or product test was repeated. Internal #1238 findings and final correspondence remain pending.
