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

## Combined actual finding denominator

Internal source: #1238 `0f363c1415a6e50f16a6b6718bb4c6e62ffc118c`, `docs/reviews/v0.93.1/internal-review-1238/INTERIM_SOURCE_REVIEW_1238.md`. Independent source: #1240 frozen packet review at `22816165efff2416f0bba84d7b1e2c4b9a2acf2e`, with P3 correction at `bd93dab710d99c076a7030b6908af4fd9e301a34`.

**Six actual findings received: five open, one fixed.** Internal: two P1 evidence/acceptance gaps and three P2 gaps. Independent: one P3 documentation finding fixed. Preparation PREP rows are evidence-gap precursors, not additional findings; PLAN rows are separately tracked planning completeness findings. This is the complete received interim-review denominator, not a claim of complete final candidate coverage.

| Original ID | Severity / scope | Assigned owner and next evidence | Disposition |
| --- | --- | --- | --- |
| IR-1238-01 | P1 artifact and installed correspondence | #24 verifies archive11994981448 once; #1236/#101 bind exact393dd2 archive, template and installed evidence. Current6/6CI is passed, not a failure. | Open; #24 verified archive integrity and exact manifest/source correspondence; selected consumption and affected qualification remain pending. |
| IR-1238-02 | P1 dated quality assessment | #1232/#1236 reconcile15gate rows against actual current101 proof and selected lockset. | Open; historical failed/unknown rows remain dated, not a claim currentCI failed. |
| IR-1238-03 | P2 affected interfaces | #21 reads PublicADL→Runtime/Infrastructure correspondence; #15 supplies CodeFriend→website, other role identities and consolidated before/after edge ledger. | Open; six graph edges plus separate CodeFriend disposition, no seven-build campaign. |
| IR-1238-04 | P2 reviewer-facing documentation | #26 refreshes #1234 and dates/corrects #1269's merged13-PR status, then identifies final pages. | Open; administrative PR state is distinct from product evidence. |
| IR-1238-05 | P2 recovery/access | #15/#1236 binds actual archive, deployment/stop owner, authorized reviewer access and existing prepare/safe-disable/failed-activation limits. | Open; undeployed readiness only, no demand for live rollback or prior deployed baseline. |
| IR-1240-01 | P3 count precision | #1236 correctionbd93, independently verified. | Fixed; original185/final182+3guard change distinction retained. |

PREP-01 maps to IR-1238-02/03; PREP-02/03 map to IR-1238-01; PREP-04 maps to IR-1238-05. PREP-05's missing-report condition is now replaced by the actual interim reports and their explicitly incomplete final coverage. PREP-06 is historical tooling evidence, not an added product finding. No unsupported deferrals, severity reductions or omitted raw IDs.

Successor handoff: carry these five open source-bound gaps with the owners above until fixed or explicitly authorized for deferral. None has been deferred into v0.93.2 by this register. #1242/#1243 can consume this concrete register now; their final residual decision must reflect later dispositions. Source reviews and current green CI do not independently establish release acceptance.

## Archive verification update

#24 verification at 2026-10-15T17:54:29Z fetched artifact11994981448 once: 262,057,006 bytes, digest `2be7a8804199bf578388637915e399bf8e9cf23ac5057f4f4ef180b544c1f6f0`, source `393dd2ee7c5db8e6108a27cf6441e380fb62f4c3`, tree `1808b045553b2e3ff1f54e841359cc0cd890c6df`, and template manifest `45bd407bd8cd850925d6177ee99814dc533d28779833e6cb8b01bc2337f68cd2` match. No missing package evidence was reported. Archive integrity and included manifest/source correspondence are now proven within that check; no second download or hash was performed here.

Preserve the proposed template-manifest status and packaged renderer proof's `acquisition_measurement_matches=false` for PNG/SVG. Their semantics are under owner investigation, not silently waived or labeled a newly proven product defect. This check did not execute binaries, install, build or requalify rendering. Selected consumer/affected-interface acceptance remains distinct and pending.

## Updated dispositions and additional interface finding — 2026-10-15

The current #1238 report at `1c7246e340740ec23de8642b4bf749db3eabd115` adds **IR-1238-06 (P1)**. The received denominator is now **seven findings: six open (including one partially remediated documentation gap), one fixed**. Earlier counts remain dated intake records.

- **IR-1238-04: source correction completed; final packet selection pending.** #1234 PR #1276 at `9a49f3fa0114ab7efd0433c257e7d87b1ed5b1fa` refreshes the handoff and corrects the #1269 record to thirteen merged documentation PRs. Focused local proof, independent and native review, native publication and ready transition passed; CI has four successful checks and three scope skips. This fixes stale documentation wording. The final #1236 manifest must still select these exact pages; the PR's pending merge is administrative, not a product-test failure. Do not repeat product tests for this documentation change.
- **IR-1238-06: confirmed, open, CodeFriend/website owners.** Website source `62b34f30a7042c10e35cbd5924841305bf68d106` admits legacy renderer identities and `codefriend.<format>_report_manifest.v1`; CodeFriend `393dd2ee7c5db8e6108a27cf6441e380fb62f4c3` emits newer renderer identities and report-manifest `.v2` (template exports have another schema). A separate bounded reproduction passes legacy identities and rejects both pre-#101 and current identities. The website adapter bytes are unchanged at observed `106d202`. This is a retained undeployed contract incompatibility, not an established #101 regression or deployed failure. Align the supported hosted-export contract and retain candidate-bound positive/negative proof; no journey exclusion or release deferral is authorized here. The durable follow-on issue is not yet created; creation awaits the applicable direct authorization. Preserve this finding regardless of issue-creation state.

Renderer acquisition comparison semantics are resolved: at CodeFriend `393dd2ee`, `verify_runtime.py` lines 268–284 enforces identical outputs from two renders within the native environment and records the acquisition comparison as informational. The retained PNG/SVG proof has `repeat_count=2` and successful child exits. `acquisition_measurement_matches=false` is documented cross-environment variance, not failure of that required gate; it does not establish cross-environment byte or visual equivalence. This disposition interprets existing source/proof only and adds no product defect, test, download or hash.

Final external review remains on hold until every finding is resolved, as directed by the operator. The existing independent diagnostic report remains evidence; it is not final external acceptance. No findings are waived or deferred by this update.

## Candidate-bound documentation and recovery reconciliation

Inspected #1236 candidate `4cbf44c91a3b1a00d26440b2f445a1c0812dcbfb` against `bb052ff40b0df087f9460a2b5d39ce2fdecb7224`; this was a bounded source-evidence comparison, with no new execution or review campaign.

**IR-1238-04 is fixed for the current review candidate.** Its inventory selects the #1234 handoff and refreshed #1269 checklist at `9a49f3fa0114ab7efd0433c257e7d87b1ed5b1fa`, with exact file identities, and selects #1232 `cd5e9098d9d1a0f65d460e43d1e000f57683a33b` as the adverse current assessment. The stale documentation and unselected-page conditions are resolved. Pending merges and final release acceptance remain separate; they do not reopen this source documentation finding. The received denominator is now **seven findings: five open, two fixed** (IR-1238-04 and IR-1240-01).

**IR-1238-05 is narrowed, still open for candidate/access readiness.** The candidate binds Linux x86_64 review archive `11994981448`, the selected template identity `45bd407bd8cd850925d6177ee99814dc533d28779833e6cb8b01bc2337f68cd2`, and the accepted website `62b34f30a7042c10e35cbd5924841305bf68d106` prepare/status/safe-disable/failed-activation procedure. Its recovery limits are now explicit: preserve state and credentials, revoke invitations before stopping, no reversal of dispatched work and no prior deployed application baseline. That documented undeployed recovery boundary is sufficient; live activation, live rollback, tester admission and an already-appointed future deployment operator are not requirements for this v0.93.1 review copy. CodeFriend #92 owns later launch/stop identity.

The remaining IR-1238-05 conditions are precise: establish selected candidate/host-platform and installed-consumer correspondence (shared with IR-1238-01/03 and the unresolved IR-1238-06 interface), and verify the final independent reviewer's authorized access to required private source/artifact/evidence before dispatch. Linux archive verification does not establish another platform's installation, and public-safe source availability does not establish private reviewer access. These are not demands for deployment or a second archive download. Keep shared causes linked rather than counting new duplicate findings. No final external dispatch occurs while these conditions or the other findings remain unresolved.

## Repair evidence intake — macOS and website

CodeFriend #118 / PR #119 now has reviewed source `43f955e674d2d2b8d9c90e5c5d2a09df7e4c788f`, seven successful checks in run `38837182759`, and artifact `12019981590`. The retained installed receipt (`codefriend.macos_installed_renderer.v1`, job `116768923664`) records archive `99703381a8a3ca5edc999b32b65f22638f413bcc19c10a8097a0c31c5ab0e49b`, macOS arm64, PASS: diagram-packet SVG/PNG and Mermaid SVG with two repeats each, tamper refusal, cleanup, stdout/stderr dispatch, and isolation negatives. It explicitly does not establish Linux qualification transfer, a physical-memory ceiling, a thread-count ceiling or release acceptance. Earlier incomplete prototype evidence is superseded for this installed-proof claim, not erased. This intake reads retained receipts without rerunning qualification or hashing.

Website #37 has an independent PASS at source `3ddd6976c3ce0c4e80aaa89c5bbd330d4b10c3e6` covering all five changed files, exact renderer versions, manifest/result fields, pinned digests, architecture binding and preserved authorization. Its retained review records 203 passing component tests; the reviewer did not repeat them or execute live native rendering. Native publication is still being completed by its owner. This is concrete repair evidence for IR-1238-06, not yet final selected candidate acceptance.

| Open finding | Remaining work after this intake |
| --- | --- |
| IR-1238-01 | Bind the proven macOS source/archive/installed receipt and retained Linux `393dd2ee` proof into the final #1236 candidate; assess their exact source correspondence without claiming automatic Linux transfer. The macOS installed-proof absence itself is resolved. |
| IR-1238-02 | Reconcile the candidate-bound quality decision with these repairs and the website result. The merged #1232 assessment remains historically adverse until that reconciliation; it is not a demand to rerun its tests. |
| IR-1238-03 | Complete the exact selected role/affected-edge ledger, retaining unchanged-role proof and distinguishing missing historical consumer provenance from proven producer acceptance. Include the repaired website consumer. |
| IR-1238-05 | Bind the final supported candidate/platform and authorized reviewer evidence access. The undeployed recovery boundary remains sufficient; no live activation or rollback is required. |
| IR-1238-06 | Owner finalizes the reviewed website repair's native publication/proof and binds it with its supported producer to the selected interface ledger. Reuse existing positive/negative evidence. |

The denominator remains seven findings, five open and two fixed, because the final selection and affected-edge decisions are not yet recorded. CodeFriend/website merge and native finish are owned separately; no unverified merge is claimed here. Once the five dispositions are complete, update the internal verdict and dispatch one independent final synthesis against the immutable packet. Existing diagnostic review is retained and is not relabeled final acceptance.
