# v0.92.2 Planned Issue Catalog

Status: planned catalog reconciled to the existing v0.92.2 issue inventory. All 69 core rows have canonical issue identities backed by the reviewed launch map. Issue assignment does not establish execution readiness. v0.92.1 issues #717 and #718 are predecessor inputs, not catalog rows.

| Order | Planned ID | Title | Creation boundary |
|---|---|---|---|
| 1 | WP-01 (#864) | Publish and open the v0.92.2 CodeFriend Beta 1 execution wave | Reuse existing; Own readiness; execution Sprint 1 |
| 2 | CF-SHELL (#891) | Operate a real repository review through the installed CodeFriend shell | Global core startup gate; WP-01, CF-REVIEW |
| 3 | CF-ADAPTER (#878) | Ingest a local checkout into a portable repository packet | Global core startup gate; WP-01 |
| 4 | CF-ADAPTER-GITHUB (#879) | Ingest a pinned GitHub revision or PR into a repository packet | Global core startup gate; CF-ADAPTER |
| 5 | CF-ADAPTER-CI (#880) | Ingest CI repository inputs into a repository packet | Global core startup gate; CF-ADAPTER |
| 6 | CF-EVIDENCE (#881) | Admit and retain governed evidence through the production adapter | Global core startup gate; CF-ADAPTER |
| 7 | CF-COG (#882) | Report repository dependency and boundary structure | Global core startup gate; CF-EVIDENCE |
| 8 | CF-COG-DRIFT (#886) | Report architecture drift between compatible revisions | Global core startup gate; CF-COG, CF-MEMORY |
| 9 | CF-COG-IMPACT (#883) | Report the impact of a scoped repository change | Global core startup gate; CF-COG |
| 10 | CF-COG-RATIONALE (#884) | Explain architectural quanta against recorded rationale | Global core startup gate; CF-COG |
| 11 | CF-GOV (#887) | Execute local architecture fitness functions | Global core startup gate; CF-EVIDENCE |
| 12 | CF-GOV-CI (#888) | Execute architecture fitness functions as a CI gate | Global core startup gate; CF-GOV |
| 13 | CF-REVIEW (#890) | Execute isolated four-perspective repository review | Global core startup gate; CF-EVIDENCE, RT-PROVIDER |
| 14 | CF-SYNTHESIS (#892) | Synthesize completed review perspectives | Global core startup gate; CF-REVIEW |
| 15 | CF-REMEDIATE (#893) | Generate a bounded remediation plan from review findings | Global core startup gate; CF-SYNTHESIS |
| 16 | CF-TESTPLAN (#894) | Generate a bounded test plan from review findings | Global core startup gate; CF-SYNTHESIS |
| 17 | CF-MEMORY (#885) | Stable second-run comparison and longitudinal review memory | Global core startup gate; CF-EVIDENCE |
| 18 | CF-UX (#895) | Enforce exact-artifact publication approval | Global core startup gate; CF-SHELL, CF-EVIDENCE |
| 19 | CF-RENDER-MD (#896) | Render an approved review as Markdown | Global core startup gate; CF-UX, CF-SYNTHESIS, CF-REMEDIATE, CF-TESTPLAN |
| 20 | CF-RENDER-HTML (#897) | Render an approved review as HTML | Global core startup gate; CF-RENDER-MD |
| 21 | CF-RENDER-PDF (#898) | Render an approved review as a verified PDF | Global core startup gate; CF-RENDER-MD |
| 22 | CF-PROOF (#915) | Execute independent installed Beta 1 qualification on ADL and an external repository | Global core startup gate; CF-INTEGRATE |
| 23 | CF-INTEGRATE (#914) | Connect the complete installed CodeFriend Beta 1 journey | Global core startup gate; CF-SHELL, CF-ADAPTER, CF-ADAPTER-GITHUB, CF-ADAPTER-CI, CF-EVIDENCE, CF-COG, CF-COG-DRIFT, CF-COG-IMPACT, CF-COG-RATIONALE, CF-GOV, CF-GOV-CI, CF-REVIEW, CF-SYNTHESIS, CF-REMEDIATE, CF-TESTPLAN, CF-MEMORY, CF-UX, CF-RENDER-MD, CF-RENDER-HTML, CF-RENDER-PDF, PLAT-PROVIDER, RT-PROVIDER, PLAT-MEMORY |
| 24 | PLAT-PROVIDER (#876) | Consume validated editable provider definitions | Global core startup gate; WP-01, RT-COST; v0.92.1-issue-622 |
| 25 | RT-PROVIDER (#855) | Run dynamic agent lifecycle through registered providers | Reuse existing; PLAT-PROVIDER; execution Sprint 2 |
| 26 | ARCH-SPLIT (#848) | Decide repository decomposition and public/private product boundaries | Reuse existing; WP-01; execution Sprint 2 |
| 27 | CSDLC-MERGE (#849) | Preserve publication linkage during native PR merge | Reuse existing; WP-01; execution Sprint 7 |
| 28 | QUAL-RUNTIME (#852) | Emit and correlate Runtime dispatch failure events | Reuse existing; WP-01; execution Sprint 5 |
| 29 | QUAL-RESIDENT (#900) | Execute resident workload and signed restore qualification | Global core startup gate; QUAL-RUNTIME, RT-PROVIDER |
| 30 | QUAL-PROVIDER (#901) | Execute real provider failure and recovery qualification | Global core startup gate; RT-PROVIDER |
| 31 | QUAL-INVENTORY (#899) | Measure the registered before and after validation inventory | Global core startup gate; WP-01 |
| 32 | QUAL-EVIDENCE (#902) | Validate criterion-bound Runtime qualification evidence | Global core startup gate; QUAL-RUNTIME, QUAL-RESIDENT, QUAL-PROVIDER, QUAL-INVENTORY |
| 33 | RT-COST (#854) | Stop metered cloud inference health-probe loops | Reuse existing; WP-01; execution Sprint 2 |
| 34 | CSDLC-MAN (#861) | Write complete C-SDLC v3 operator man pages | Reuse existing; WP-01; execution Sprint 1 |
| 35 | CSDLC-DECOMPOSE (#862) | Decompose the local C-SDLC command owner | Reuse existing; WP-01; execution Sprint 7 |
| 36 | CSDLC-REMOTE (#907) | Decompose the remote C-SDLC command owner | Global core startup gate; CSDLC-DECOMPOSE, CSDLC-MERGE |
| 37 | PLAT-MLX (#903) | Bounded MLX and Apple Metal provider adapter | Global core startup gate; PLAT-PROVIDER |
| 38 | PLAT-PAIR (#904) | NVIDIA PAIR multi-node local-inference experiment | Global core startup gate; PLAT-PROVIDER |
| 39 | PLAT-UTS (#877) | Install a versioned UTS package consumed by Runtime tool dispatch | Global core startup gate; WP-01 |
| 40 | PLAT-RUST (#906) | Complete one selected production Rust responsibility refactor | Global core startup gate; WP-01 |
| 41 | OPS-AWS (#908) | Produce one current AWS inventory packet from the #484 baseline | Global core startup gate; WP-01; completed-issue-484-baseline |
| 42 | OPS-GCP (#909) | Produce one apply-ready company GCP move-in execution packet | Global core startup gate; WP-01; merged-v0.92.1-gcp-foundations |
| 43 | PUB-MEDIUM (#912) | Write one complete selected Medium article manuscript | Global core startup gate; WP-01 |
| 44 | PUB-CSDLC (#913) | Complete one named C-SDLC manuscript revision | Global core startup gate; WP-01 |
| 45 | PLAT-MEMORY (#889) | Retrieve a compatible prior CodeFriend review through Memory Palace | Global core startup gate; CF-EVIDENCE, CF-MEMORY |
| 46 | SPEC-RETEST (#905) | Speculative-decoding requalification | Global core startup gate; WP-01 |
| 47 | OBS-LIVE (#720) | Remove retained-mode demo hazards | Reuse existing; Own readiness; execution Sprint 8 |
| 48 | OBS-S3 (#910) | Deploy the existing Observatory S3 and CloudFront sidecar | Global core startup gate; WP-01, OBS-LIVE; completed-v0.92.1-issue-679, merged-v0.92.1-pr-685 |
| 49 | ARCH-ADR (#911) | Generate and reconcile the ADRs required by v0.92.2 | Global core startup gate; WP-01 |
| 50 | SIM-UMBRELLA (#866) | Coordinate the first C-SDLC simplification sprint | Created first sprint; SIM-09 |
| 51 | SIM-01 (#867) | Make diagnostics observably read-only | Created first sprint; Own readiness |
| 52 | SIM-02 (#868) | One current installed command contract | Created first sprint; SIM-01 |
| 53 | SIM-03 (#869) | Execute the supported installed intent-command inventory | Created first sprint; SIM-02 |
| 54 | SIM-04 (#870) | Route operational mutations through one semantic transaction owner | Created first sprint; SIM-03 |
| 55 | SIM-05 (#871) | Derived cards and precise evidence invalidation | Created first sprint; SIM-04 |
| 56 | SIM-06 (#872) | Deliver one safe conversion rehearsal | Created first sprint; SIM-05 |
| 57 | SIM-07 (#873) | Produce one independent transition qualification | Created first sprint; SIM-06 |
| 58 | SIM-08 (#874) | Produce one transition operations packet | Created first sprint; SIM-07 |
| 59 | SIM-09 (#875) | Run one authorized consecutive-issue pilot | Created first sprint; SIM-08 |
| 60 | TAIL-01 (#916) | Quality gate | Global core startup gate; CF-INTEGRATE, CF-PROOF, PLAT-MLX, PLAT-PAIR, PLAT-UTS, PLAT-RUST, OPS-AWS, OPS-GCP, PUB-MEDIUM, PUB-CSDLC, SPEC-RETEST, OBS-LIVE, ARCH-SPLIT, CSDLC-MERGE, QUAL-RUNTIME, QUAL-RESIDENT, QUAL-PROVIDER, QUAL-INVENTORY, QUAL-EVIDENCE, RT-COST, CSDLC-MAN, CSDLC-DECOMPOSE, CSDLC-REMOTE, SIM-UMBRELLA |
| 61 | TAIL-02 (#917) | Documentation review and external-review handoff | Global core startup gate; TAIL-01 |
| 62 | TAIL-03 (#918) | Publication finalization | Global core startup gate; TAIL-02 |
| 63 | TAIL-04 (#919) | Internal milestone review | Global core startup gate; TAIL-03 |
| 64 | TAIL-05 (#920) | External or third-party review | Global core startup gate; TAIL-04 |
| 65 | TAIL-06 (#921) | Accepted-findings remediation or explicit deferral capture | Global core startup gate; TAIL-05 |
| 66 | TAIL-07 (#922) | Next-milestone planning | Global core startup gate; TAIL-06 |
| 67 | TAIL-08 (#923) | Next-milestone closeout planning | Global core startup gate; TAIL-07 |
| 68 | TAIL-09 (#924) | Next-milestone planning review | Global core startup gate; TAIL-08 |
| 69 | TAIL-10 (#925) | Release ceremony and milestone close | Global core startup gate; TAIL-09, OBS-S3, ARCH-ADR |

WP-01 is existing issue #864. It reuses #720, #848, #849, #852, #854, #855, #861, and #862 and records all 60 new child identities created under the operator's authorization for all eleven batches. #848 owns the decomposition decision only; any split implementation requires the resulting reviewed plan. This includes `OBS-S3`/#910 and `ARCH-ADR`/#911, each with a distinct complete result and final-tail acceptance. Closed merged #717/#718 are v0.92.1 predecessor capabilities. Other backlog is excluded. The operator-selected SIM sprint retains its dedicated launch authority.

Each catalog row owns one primary result. Lists of supporting artifacts or proof do not authorize additional independently valuable work; WP-01 must split any row whose execution contract cannot preserve that boundary before creating its issue.

## Deferred, Not Missing

Jira, Linear, Slack, broad Workspace integrations, autonomous mutation, public customer-scale or multi-tenant deployment, security tournaments, ATE, OCI packaging, optional OpenRewrite/modernization, and Runtime v4 are intentionally outside this catalog. The bounded static Observatory sidecar is admitted only through `OBS-S3`; NVIDIA PAIR and GCP move-in residuals are admitted only through `PLAT-PAIR` and `OPS-GCP`.

## First sprint creation state

SIM-UMBRELLA is #866 and SIM-01 through SIM-09 are #867 through #875 respectively. The [verified mapping](../../../.csdlc/evidence/864/sprint01-launch/issues.json) records native creation. Together with the ten later creation batches, the final wave has 69 assigned core issues: nine preexisting and 60 newly created identities, with zero unassigned tasks. Creation does not claim implementation or authorize live activation. The operator authorized all eleven creation batches; the final launch map records their created identities and independent reviews.

The [complete reviewed launch map](../../../.csdlc/evidence/864/all-issue-launch.json) binds all 69 core task IDs to verified issue numbers and independent creation reviews. This establishes issue inventory and review truth, not implementation or Beta 1 qualification.
