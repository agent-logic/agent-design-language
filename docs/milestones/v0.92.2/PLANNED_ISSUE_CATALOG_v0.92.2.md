# v0.92.2 Planned Issue Catalog

Status: planned catalog reconciled to the existing v0.92.2 issue inventory. Uncreated rows remain number-free. This file does not create or authorize new GitHub issues. v0.92.1 issues #717 and #718 are predecessor inputs, not catalog rows.

| Order | Planned ID | Title | Creation boundary |
|---|---|---|---|
| 1 | WP-01 (#864) | Publish and open the v0.92.2 CodeFriend Beta 1 execution wave | Reuse existing; Own readiness |
| 2 | CF-SHELL | Operate a real repository review through the installed CodeFriend shell | Separate creation authorization; WP-01, CF-REVIEW |
| 3 | CF-ADAPTER | Ingest a local checkout into a portable repository packet | Separate creation authorization; WP-01 |
| 4 | CF-ADAPTER-GITHUB | Ingest a pinned GitHub revision or PR into a repository packet | Separate creation authorization; CF-ADAPTER |
| 5 | CF-ADAPTER-CI | Ingest CI repository inputs into a repository packet | Separate creation authorization; CF-ADAPTER |
| 6 | CF-EVIDENCE | Admit and retain governed evidence through the production adapter | Separate creation authorization; CF-ADAPTER |
| 7 | CF-COG | Report repository dependency and boundary structure | Separate creation authorization; CF-EVIDENCE |
| 8 | CF-COG-DRIFT | Report architecture drift between compatible revisions | Separate creation authorization; CF-COG, CF-MEMORY |
| 9 | CF-COG-IMPACT | Report the impact of a scoped repository change | Separate creation authorization; CF-COG |
| 10 | CF-COG-RATIONALE | Explain architectural quanta against recorded rationale | Separate creation authorization; CF-COG |
| 11 | CF-GOV | Execute local architecture fitness functions | Separate creation authorization; CF-EVIDENCE |
| 12 | CF-GOV-CI | Execute architecture fitness functions as a CI gate | Separate creation authorization; CF-GOV |
| 13 | CF-REVIEW | Execute isolated four-perspective repository review | Separate creation authorization; CF-EVIDENCE, RT-PROVIDER |
| 14 | CF-SYNTHESIS | Synthesize completed review perspectives | Separate creation authorization; CF-REVIEW |
| 15 | CF-REMEDIATE | Generate a bounded remediation plan from review findings | Separate creation authorization; CF-SYNTHESIS |
| 16 | CF-TESTPLAN | Generate a bounded test plan from review findings | Separate creation authorization; CF-SYNTHESIS |
| 17 | CF-MEMORY | Stable second-run comparison and longitudinal review memory | Separate creation authorization; CF-EVIDENCE |
| 18 | CF-UX | Enforce exact-artifact publication approval | Separate creation authorization; CF-SHELL, CF-EVIDENCE |
| 19 | CF-RENDER-MD | Render an approved review as Markdown | Separate creation authorization; CF-UX, CF-SYNTHESIS, CF-REMEDIATE, CF-TESTPLAN |
| 20 | CF-RENDER-HTML | Render an approved review as HTML | Separate creation authorization; CF-RENDER-MD |
| 21 | CF-RENDER-PDF | Render an approved review as a verified PDF | Separate creation authorization; CF-RENDER-MD |
| 22 | CF-PROOF | Execute independent installed Beta 1 qualification on ADL and an external repository | Separate creation authorization; CF-INTEGRATE |
| 23 | CF-INTEGRATE | Connect the complete installed CodeFriend Beta 1 journey | Separate creation authorization; CF-SHELL, CF-ADAPTER, CF-ADAPTER-GITHUB, CF-ADAPTER-CI, CF-EVIDENCE, CF-COG, CF-COG-DRIFT, CF-COG-IMPACT, CF-COG-RATIONALE, CF-GOV, CF-GOV-CI, CF-REVIEW, CF-SYNTHESIS, CF-REMEDIATE, CF-TESTPLAN, CF-MEMORY, CF-UX, CF-RENDER-MD, CF-RENDER-HTML, CF-RENDER-PDF, PLAT-PROVIDER, RT-PROVIDER, PLAT-MEMORY |
| 24 | PLAT-PROVIDER | Consume validated editable provider definitions | Separate creation authorization; WP-01, RT-COST; v0.92.1-issue-622 |
| 25 | RT-PROVIDER (#855) | Run dynamic agent lifecycle through registered providers | Reuse existing; PLAT-PROVIDER |
| 26 | ARCH-SPLIT (#848) | Decide repository decomposition and public/private product boundaries | Reuse existing; WP-01 |
| 27 | CSDLC-MERGE (#849) | Preserve publication linkage during native PR merge | Reuse existing; WP-01 |
| 28 | QUAL-RUNTIME (#852) | Emit and correlate Runtime dispatch failure events | Reuse existing; WP-01 |
| 29 | QUAL-RESIDENT | Execute resident workload and signed restore qualification | Separate creation authorization; QUAL-RUNTIME, RT-PROVIDER |
| 30 | QUAL-PROVIDER | Execute real provider failure and recovery qualification | Separate creation authorization; RT-PROVIDER |
| 31 | QUAL-INVENTORY | Measure the registered before and after validation inventory | Separate creation authorization; WP-01 |
| 32 | QUAL-EVIDENCE | Validate criterion-bound Runtime qualification evidence | Separate creation authorization; QUAL-RUNTIME, QUAL-RESIDENT, QUAL-PROVIDER, QUAL-INVENTORY |
| 33 | RT-COST (#854) | Stop metered cloud inference health-probe loops | Reuse existing; WP-01 |
| 34 | CSDLC-MAN (#861) | Write complete C-SDLC v3 operator man pages | Reuse existing; WP-01 |
| 35 | CSDLC-DECOMPOSE (#862) | Decompose the local C-SDLC command owner | Reuse existing; WP-01 |
| 36 | CSDLC-REMOTE | Decompose the remote C-SDLC command owner | Separate creation authorization; CSDLC-DECOMPOSE, CSDLC-MERGE |
| 37 | PLAT-MLX | Bounded MLX and Apple Metal provider adapter | Separate creation authorization; PLAT-PROVIDER |
| 38 | PLAT-PAIR | NVIDIA PAIR multi-node local-inference experiment | Separate creation authorization; PLAT-PROVIDER |
| 39 | PLAT-UTS | Install a versioned UTS package consumed by Runtime tool dispatch | Separate creation authorization; WP-01 |
| 40 | PLAT-RUST | Complete one selected production Rust responsibility refactor | Separate creation authorization; WP-01 |
| 41 | OPS-AWS | Produce one current AWS inventory packet from the #484 baseline | Separate creation authorization; WP-01; completed-issue-484-baseline |
| 42 | OPS-GCP | Produce one apply-ready company GCP move-in execution packet | Separate creation authorization; WP-01; merged-v0.92.1-gcp-foundations |
| 43 | PUB-MEDIUM | Write one complete selected Medium article manuscript | Separate creation authorization; WP-01 |
| 44 | PUB-CSDLC | Complete one named C-SDLC manuscript revision | Separate creation authorization; WP-01 |
| 45 | PLAT-MEMORY | Retrieve a compatible prior CodeFriend review through Memory Palace | Separate creation authorization; CF-EVIDENCE, CF-MEMORY |
| 46 | SPEC-RETEST | Speculative-decoding requalification | Separate creation authorization; WP-01 |
| 47 | OBS-LIVE (#720) | Remove retained-mode demo hazards | Reuse existing; Own readiness |
| 48 | OBS-S3 | Deploy the existing Observatory S3 and CloudFront sidecar | Separate creation authorization; WP-01, OBS-LIVE; completed-v0.92.1-issue-679, merged-v0.92.1-pr-685 |
| 49 | ARCH-ADR | Generate and reconcile the ADRs required by v0.92.2 | Separate creation authorization; WP-01 |
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
| 60 | TAIL-01 | Quality gate | Separate creation authorization; CF-INTEGRATE, CF-PROOF, PLAT-MLX, PLAT-PAIR, PLAT-UTS, PLAT-RUST, OPS-AWS, OPS-GCP, PUB-MEDIUM, PUB-CSDLC, SPEC-RETEST, OBS-LIVE, ARCH-SPLIT, CSDLC-MERGE, QUAL-RUNTIME, QUAL-RESIDENT, QUAL-PROVIDER, QUAL-INVENTORY, QUAL-EVIDENCE, RT-COST, CSDLC-MAN, CSDLC-DECOMPOSE, CSDLC-REMOTE, SIM-UMBRELLA |
| 61 | TAIL-02 | Documentation review and external-review handoff | Separate creation authorization; TAIL-01 |
| 62 | TAIL-03 | Publication finalization | Separate creation authorization; TAIL-02 |
| 63 | TAIL-04 | Internal milestone review | Separate creation authorization; TAIL-03 |
| 64 | TAIL-05 | External or third-party review | Separate creation authorization; TAIL-04 |
| 65 | TAIL-06 | Accepted-findings remediation or explicit deferral capture | Separate creation authorization; TAIL-05 |
| 66 | TAIL-07 | Next-milestone planning | Separate creation authorization; TAIL-06 |
| 67 | TAIL-08 | Next-milestone closeout planning | Separate creation authorization; TAIL-07 |
| 68 | TAIL-09 | Next-milestone planning review | Separate creation authorization; TAIL-08 |
| 69 | TAIL-10 | Release ceremony and milestone close | Separate creation authorization; TAIL-09, OBS-S3, ARCH-ADR |

WP-01 is existing issue #864. It reuses #720, #848, #849, #852, #854, #855, #861, and #862 and creates only separately authorized unassigned child rows. #848 owns the decomposition decision only; any split implementation requires the resulting reviewed plan. This includes one issue for `OBS-S3` and one for `ARCH-ADR`; neither is created by this reconciliation. Closed merged #717/#718 are v0.92.1 predecessor capabilities. Other backlog is excluded. The operator-selected SIM sprint retains its dedicated launch authority.

Each catalog row owns one primary result. Lists of supporting artifacts or proof do not authorize additional independently valuable work; WP-01 must split any row whose execution contract cannot preserve that boundary before creating its issue.

## Deferred, Not Missing

Jira, Linear, Slack, broad Workspace integrations, autonomous mutation, public customer-scale or multi-tenant deployment, security tournaments, ATE, OCI packaging, optional OpenRewrite/modernization, and Runtime v4 are intentionally outside this catalog. The bounded static Observatory sidecar is admitted only through `OBS-S3`; NVIDIA PAIR and GCP move-in residuals are admitted only through `PLAT-PAIR` and `OPS-GCP`.

## First sprint creation state

SIM-UMBRELLA is #866 and SIM-01 through SIM-09 are #867 through #875 respectively. The [verified mapping](../../../.csdlc/evidence/864/sprint01-launch/issues.json) records native creation. These ten issues join nine preexisting bindings: 19 assigned issues and 50 unassigned tasks in the unchanged 69-row milestone. Creation does not claim implementation or authorize live activation. Later sprint creation remains separately authorized.
