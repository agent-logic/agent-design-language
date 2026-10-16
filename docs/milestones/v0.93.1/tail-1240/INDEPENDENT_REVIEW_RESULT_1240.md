# ADL #1240 independent agent source/evidence review

## Findings first

### IR-1240-01 — P3: distinguish intermediate and final source-correspondence counts

- Evidence: `docs/milestones/v0.93.1/publication-1236/CANDIDATE_INVENTORY.md:22` at ADL `22816165efff2416f0bba84d7b1e2c4b9a2acf2e` summarizes `FINAL_SOURCE_CORRESPONDENCE.json` as recording “185 unchanged public inputs” after removal of `run.rs`.
- The referenced JSON's `final_guard_correspondence` instead records `original_comparison_count=185`, `unchanged_at_final_head=182`, and three changed guard inputs: `tools/source_retirement/README.md`, `check.py`, and `test_check.py`. It separately preserves `compiled_product_inputs_unchanged=true` and actual final guard evidence.
- Impact: the historical intermediate count is readable as the final correspondence count in a final-candidate inventory, obscuring the separately validated guard delta. This is a documentation precision finding, not evidence of changed compiled product inputs or a reason to rebuild products.
- Owner/action: #1236 publication owner should state the intermediate 185-input comparison and final 182-unchanged/three-guard-change reconciliation explicitly, retaining the unchanged compiled-product conclusion and final guard proof. Route disposition through #1242/TAIL-06.

No P0, P1 or P2 source-document defect was established in the examined scope. The known acceptance gaps below remain release-blocking prerequisites, not newly discovered product defects.

## Verdict

**SOURCE_REVIEW_COMPLETED_WITH_MINOR_FINDING; RELEASE_ACCEPTANCE_DEFERRED.** This is an actual independent agent review of the frozen source/evidence packet. It is not a complete external approval of a qualified final lockset, and it does not satisfy all #1240 completion checks: #1238 internal results, exact new CodeFriend artifact/CI proof, final correspondence, and predecessor decisions were unavailable or explicitly pending at dispatch. Do not translate this verdict into an external PASS, native review authority, permission to publish release notes, or permission to deploy.

## Reviewer independence and identity

Reviewer: fresh delegated agent `/root/tail1240_independent`, assigned solely to #1240 external-review role on 2026-10-15. This agent did not author the candidate and did not perform its #1238 internal review. No #1238 findings were supplied or consumed before this assessment. This is independence of review role within the same agent system, not a third-party organization, external human, or separate institutional assurance claim. Parent authorization limited writes to this report.

Dispatch: exact two-file packet recorded by the coordinator; local invocation evidence retained separately. Exact examined candidate: ADL `22816165efff2416f0bba84d7b1e2c4b9a2acf2e`; observed worktree HEAD matched. Untracked lifecycle evidence directories were present; candidate inspection used committed Git objects rather than those directories.

## Exact inputs and observations

| Examined input | Identity / observation |
| --- | --- |
| `publication-1236/CANDIDATE_INVENTORY.md` under `docs/milestones/v0.93.1/` | Git blob `8c859eb41dd0927c1445d8b89cb1e75186dbb845`, SHA-256 `141d42ca52c10821c2a5e38644af96c8e9927cee4b2f7accd29ffae1f636587c`; dispatch blob matched. |
| `publication-1236/RELEASE_NOTES_CANDIDATE.md` under the same milestone | Git blob `dec29b769367f3618be40ab5ab52f54a23cb8cc9`, SHA-256 `8ec79922da1d7d2b0f401b003e64efe9f2dbba84c12e30cc9313fd836087f7d6`; dispatch blob matched. |
| `repository-decomposition/rd07-1223/FINAL_VALIDATION.json` at candidate | SHA-256 `cb7715c71a96d5b30a470723b7fbbe6159786da5e88731d3b39c3d17a51f96f4` matches inventory; records five installed package hashes, seven consumer cases, 21 fixtures, five negatives at `ad8204be76528a2c13ffabf5ee1e2672df475c8b`. These are retained results, not rerun evidence. |
| `repository-decomposition/rd07-1223/FINAL_SOURCE_CORRESPONDENCE.json` at candidate | SHA-256 `790dea5fbecd03729c8687499cb8428d7650e0cffc3c965f8ba11511d265ded4`; examined final guard reconciliation and compiled-product boundary. |
| `repository-decomposition/rd11-1193/interface-graph.json` at candidate | Six edges and separate historical CodeFriend no-Runtime-source-dependency disposition confirmed. Historical source/artifact pin differs from proposed new CodeFriend source; inventory correctly requires rebinding. |
| #1232 `quality-gate-1232/EVIDENCE_MATRIX.json` at `ea597506c370f6de3d997fbd46fa3ba2a0150d3f` | SHA-256 `9c0317b33d5ec8bd0e14ba62012cde326f19d8dbe9f2d1592123da349a96b5aa` matches; `NOT_READY_FOR_FINAL_RELEASE_ACCEPTANCE`, `acceptance_authority=false` confirmed. |
| #1234 `TAIL02_DOCUMENTATION_HANDOFF_1234.md` at `8205b366e50a1485460e3e090da580c11f823ba4` | SHA-256 `ffff996253d34ab43346c9e7370ff84eb14d58d44fc00541a39d7f8ed15cfece` matches; inspected candidate and documentation/claim handoff sections. |
| CodeFriend `393dd2ee7c5db8e6108a27cf6441e380fb62f4c3` | `Cargo.lock` SHA-256 `0f3088c2ce6d2f5e46144298b36e8e19b61b16e1696ab6985f8e21cc55974ca8`; template manifest SHA-256 `45bd407bd8cd850925d6177ee99814dc533d28779833e6cb8b01bc2337f68cd2`; both match. Compared changed-path inventory against `aa2cca757ba4e6c37cef6c1a6e013d3896cd1de1`: exactly 20 paths under production source/templates, including publication, renderer and composition. Other changes exist in tests, CI, packaging, docs and lifecycle records; 20 is not the entire diff count. |
| CodeFriend qualification and deployment-readiness JSON at `188dd2b701be0a4acce1ec510559fdff119fd4c7` | Respective SHA-256 values `3c662ded351af833179da02039ad5a4fd5c036f26058d4d3ac16c0cdebfdc1f0` and `04236d79a764f2aeb97b6c528089fbb164eabae5bcfd4e69f7aea8c58de8c6ea` match. Examined accepted original pair, 24 qualification result, no-deployment boundary, witness limitation and retained Linux failure. |
| Website `docs/APPLICATION_DEPLOYMENT.md` at `62b34f30a7042c10e35cbd5924841305bf68d106` | SHA-256 `789f32be5e3c58109a7defb594e2608b89427991f2db185bea67eb3b3ac1bf8f` matches; prepare/activate/status/safe-disable commands and no-prior-deployed-baseline limit agree with inventory. |

## Coverage and release gates

- **Source/evidence completeness:** hashes and declared freeze are traceable for examined inputs. The new product archive is absent from the accepted selection. The historical archive's availability and integrity are explicitly qualified; no claim that old qualification covers new bytes was found.
- **Compatibility:** six historical edges are documented. The inventory correctly calls for affected Runtime/Public ADL and Infrastructure/Public ADL correspondence, plus CodeFriend manifest/installed-proof rebinding and website interface review. It supplies a to-do ledger, not final edge dispositions or installed compatibility evidence. Owners #1232/#1236 must complete that mapping with exact selected artifacts; seven roles do not imply seven new builds.
- **No-deploy readiness:** source procedure and retained acceptance are consistent with disabled, undeployed readiness. Safe-disable preserves state and does not undo dispatched work. New CodeFriend candidate binding, final archive and actual operator/stop ownership remain unresolved. No deployment or live rollback is needed to establish this bounded source review.
- **Privacy:** the two frozen public-facing drafts contain identifiers, hashes and evidence boundaries, not credential values, tester content or raw provider traces observed in this review. Private source records were inspected locally; no private raw payload was reproduced here. Reviewer-access grants, public destinations and artifact access were not tested. This is a bounded manual disclosure review, not a whole-repository secret audit.
- **Release notes:** the unapproved draft confines 44-kind catalog and qualification claims to the retained original pair, retains the readability caveat, Linux failure and Q03 witness limitation, and defers launch to #92. It does not represent the new CodeFriend candidate as qualified or publicly available. Final lockset, documentation, artifact access and release authority remain prerequisites.
- **#1240 dependency:** #1238 findings and exact predecessor acceptance must be compared and dispositioned when supplied. This report can feed TAIL-06 now, but cannot establish completion of the final qualified-lockset review.

## Limitations and actions not performed

No builds, tests, provider calls, CI dispatches, downloads, remote writes, source edits, native lifecycle mutations or deployments were performed. Local Git reads, file reads and hashing were used. No live GitHub readback was performed, so run `38777770708`, PR status and expiration statements are reviewed as freeze-time claims, not independently verified current facts. New artifact bytes, installed rendering output, full implementation correctness, all owner lockfiles, and raw qualification executions were not examined. Retained scenario counts were checked where noted; no new executed-scenario credit is claimed.

The repo-review-docs skill guided claim checking. Its historical shared-suite reference was absent in the current primary checkout; no substitute authority or schema claim was invented. A narrow memory registry search served orientation only; no release verdict relies on memory.

## Coordinator reconciliation after source review

The original source-review observations above remain unchanged. The finding's phrase “#1242/TAIL-06” misidentifies the disposition issue: **#1241 is TAIL-06**; #1236 owns the wording correction, while #1242 receives any actual successor residual. This routing correction does not change the finding.

CodeFriend #101 supplied a delivery-completion record dated 2026-10-15T17:39:18Z for exact source `393dd2ee7c5db8e6108a27cf6441e380fb62f4c3`: 90 native tests with zero failures, six successful CI checks and native review/publication. Distribution artifact `11994981448`, run `38777770708`, reports GitHub archive digest `2be7a8804199bf578388637915e399bf8e9cf23ac5057f4f4ef180b544c1f6f0`. This is owner-supplied authenticated-delivery metadata, not a new download/hash or installation by this reviewer.

The record states 25 catalog kinds, 50 byte-identical final review views, overview inspection of all kinds and readable-scale inspection of representative/repair pages. It expressly does not claim every page was inspected at readable scale. These counts do not replace historical #47's 44-kind catalog denominator or establish all-journey qualification at the new source. The original report's absent-new-artifact/CI condition is superseded by this exact metadata; selected artifact consumption, affected-edge correspondence and final acceptance still need owner evidence.

IR-1240-01 remains open pending #1236's corrected exact revision. #1238's actual findings remain pending for a bounded coverage/disposition comparison. No second full review is required for unchanged packet inputs. Source review is complete; final #1240 acceptance is not yet established.

## IR-1240-01 fixed

Verified #1236 correction at `bd93dab710d99c076a7030b6908af4fd9e301a34`, inventory line 22: intermediate 185 inputs, final 182 unchanged plus three separately validated source-retirement guard changes; compiled product inputs remain unchanged. Root independently verified the same correction. IR-1240-01 is **fixed**; the original finding is retained above. No full review or product test was repeated. Internal #1238 findings and final correspondence remain pending.

## Internal-review comparison — no repeated source review

Consumed actual #1238 report `0f363c1415a6e50f16a6b6718bb4c6e62ffc118c`, `docs/reviews/v0.93.1/internal-review-1238/INTERIM_SOURCE_REVIEW_1238.md`. Its five stable findings IR-1238-01 through05 cover current artifact/installed correspondence, dated quality rows, affected interfaces, stale reviewer-facing documents, and recovery/access. They are consistent with this independent review's acceptance limitations; they do not contradict the finding that no P0–P2 source-document defect was established in this review's narrower examined scope.

The internal report reviewed path inventories and evidence summaries, not all product source bodies or installed outputs. This comparison adds no full-source-review coverage claim and no test execution. #1241 now preserves the six received actual finding IDs: five internal open gaps and one independent P3 fixed. The internal-input absence condition is replaced by an actual interim report; final candidate correspondence and complete final coverage remain pending. Do not turn a completed source review into a final qualified-lockset approval.

## Archive verification update

#24 verification at 2026-10-15T17:54:29Z fetched artifact11994981448 once: 262,057,006 bytes, digest `2be7a8804199bf578388637915e399bf8e9cf23ac5057f4f4ef180b544c1f6f0`, source `393dd2ee7c5db8e6108a27cf6441e380fb62f4c3`, tree `1808b045553b2e3ff1f54e841359cc0cd890c6df`, and template manifest `45bd407bd8cd850925d6177ee99814dc533d28779833e6cb8b01bc2337f68cd2` match. No missing package evidence was reported. Archive integrity and included manifest/source correspondence are now proven within that check; no second download or hash was performed here.

Preserve the proposed template-manifest status and packaged renderer proof's `acquisition_measurement_matches=false` for PNG/SVG. Their semantics are under owner investigation, not silently waived or labeled a newly proven product defect. This check did not execute binaries, install, build or requalify rendering. Selected consumer/affected-interface acceptance remains distinct and pending.
