# v0.92.2 Work Breakdown Structure

Status: all 69 core rows have canonical issue identities; implementation and acceptance remain governed by their own contracts.

| WP | Work track | Primary outcome | Depends on |
|---|---|---|---|
| WP-01 (#864) | milestone_opening | Publish and open the v0.92.2 CodeFriend Beta 1 execution wave | Own readiness; execution Sprint 1 |
| CF-SHELL (#891) | product_shell | Operate a real repository review through the installed CodeFriend shell | WP-01, CF-REVIEW |
| CF-ADAPTER (#878) | repository_adapter | Ingest a local checkout into a portable repository packet | WP-01 |
| CF-ADAPTER-GITHUB (#879) | repository_adapter | Ingest a pinned GitHub revision or PR into a repository packet | CF-ADAPTER |
| CF-ADAPTER-CI (#880) | repository_adapter | Ingest CI repository inputs into a repository packet | CF-ADAPTER |
| CF-EVIDENCE (#881) | evidence_core | Admit and retain governed evidence through the production adapter | CF-ADAPTER |
| CF-COG (#882) | architecture_cognition | Report repository dependency and boundary structure | CF-EVIDENCE |
| CF-COG-DRIFT (#886) | architecture_cognition | Report architecture drift between compatible revisions | CF-COG, CF-MEMORY |
| CF-COG-IMPACT (#883) | architecture_cognition | Report the impact of a scoped repository change | CF-COG |
| CF-COG-RATIONALE (#884) | architecture_cognition | Explain architectural quanta against recorded rationale | CF-COG |
| CF-GOV (#887) | executable_governance | Execute local architecture fitness functions | CF-EVIDENCE |
| CF-GOV-CI (#888) | executable_governance | Execute architecture fitness functions as a CI gate | CF-GOV |
| CF-REVIEW (#890) | review_engine | Execute isolated four-perspective repository review | CF-EVIDENCE, RT-PROVIDER |
| CF-SYNTHESIS (#892) | review_engine | Synthesize completed review perspectives | CF-REVIEW |
| CF-REMEDIATE (#893) | review_engine | Generate a bounded remediation plan from review findings | CF-SYNTHESIS |
| CF-TESTPLAN (#894) | review_engine | Generate a bounded test plan from review findings | CF-SYNTHESIS |
| CF-MEMORY (#885) | longitudinal_memory | Stable second-run comparison and longitudinal review memory | CF-EVIDENCE |
| CF-UX (#895) | publication | Enforce exact-artifact publication approval | CF-SHELL, CF-EVIDENCE |
| CF-RENDER-MD (#896) | publication | Render an approved review as Markdown | CF-UX, CF-SYNTHESIS, CF-REMEDIATE, CF-TESTPLAN |
| CF-RENDER-HTML (#897) | publication | Render an approved review as HTML | CF-RENDER-MD |
| CF-RENDER-PDF (#898) | publication | Render an approved review as a verified PDF | CF-RENDER-MD |
| CF-PROOF (#915) | proof_and_documentation | Execute independent installed Beta 1 qualification on ADL and an external repository | CF-INTEGRATE |
| CF-INTEGRATE (#914) | integration | Connect the complete installed CodeFriend Beta 1 journey | CF-SHELL, CF-ADAPTER, CF-ADAPTER-GITHUB, CF-ADAPTER-CI, CF-EVIDENCE, CF-COG, CF-COG-DRIFT, CF-COG-IMPACT, CF-COG-RATIONALE, CF-GOV, CF-GOV-CI, CF-REVIEW, CF-SYNTHESIS, CF-REMEDIATE, CF-TESTPLAN, CF-MEMORY, CF-UX, CF-RENDER-MD, CF-RENDER-HTML, CF-RENDER-PDF, PLAT-PROVIDER, RT-PROVIDER, PLAT-MEMORY |
| PLAT-PROVIDER (#876) | provider_platform | Consume validated editable provider definitions | WP-01, RT-COST; v0.92.1-issue-622 |
| RT-PROVIDER (#855) | provider_platform | Run dynamic agent lifecycle through registered providers | PLAT-PROVIDER; execution Sprint 2 |
| ARCH-SPLIT (#848) | architecture_decisions | Decide repository decomposition and public/private product boundaries | WP-01; execution Sprint 2 |
| CSDLC-MERGE (#849) | csdlc_correctness | Preserve publication linkage during native PR merge | WP-01; execution Sprint 7 |
| QUAL-RUNTIME (#852) | runtime_quality | Emit and correlate Runtime dispatch failure events | WP-01; execution Sprint 5 |
| QUAL-RESIDENT (#900) | runtime_quality | Execute resident workload and signed restore qualification | QUAL-RUNTIME, RT-PROVIDER |
| QUAL-PROVIDER (#901) | runtime_quality | Execute real provider failure and recovery qualification | RT-PROVIDER |
| QUAL-INVENTORY (#899) | runtime_quality | Measure the registered before and after validation inventory | WP-01 |
| QUAL-EVIDENCE (#902) | runtime_quality | Validate criterion-bound Runtime qualification evidence | QUAL-RUNTIME, QUAL-RESIDENT, QUAL-PROVIDER, QUAL-INVENTORY |
| RT-COST (#854) | runtime_cost_control | Stop metered cloud inference health-probe loops | WP-01; execution Sprint 2 |
| CSDLC-MAN (#861) | csdlc_documentation | Write complete C-SDLC v3 operator man pages | WP-01; execution Sprint 1 |
| CSDLC-DECOMPOSE (#862) | csdlc_refactoring | Decompose the local C-SDLC command owner | WP-01; execution Sprint 7 |
| CSDLC-REMOTE (#907) | csdlc_refactoring | Decompose the remote C-SDLC command owner | CSDLC-DECOMPOSE, CSDLC-MERGE |
| PLAT-MLX (#903) | provider_platform | Bounded MLX and Apple Metal provider adapter | PLAT-PROVIDER |
| PLAT-PAIR (#904) | provider_platform_experiment | NVIDIA PAIR multi-node local-inference experiment | PLAT-PROVIDER |
| PLAT-UTS (#877) | uts_productization | Install a versioned UTS package consumed by Runtime tool dispatch | WP-01 |
| PLAT-RUST (#906) | rust_reduction | Complete one selected production Rust responsibility refactor | WP-01 |
| OPS-AWS (#908) | aws_inventory | Produce one current AWS inventory packet from the #484 baseline | WP-01; completed-issue-484-baseline |
| OPS-GCP (#909) | gcp_foundation | Produce one apply-ready company GCP move-in execution packet | WP-01; merged-v0.92.1-gcp-foundations |
| PUB-MEDIUM (#912) | publication_preparation | Write one complete selected Medium article manuscript | WP-01 |
| PUB-CSDLC (#913) | publication_preparation | Complete one named C-SDLC manuscript revision | WP-01 |
| PLAT-MEMORY (#889) | memory_palace | Retrieve a compatible prior CodeFriend review through Memory Palace | CF-EVIDENCE, CF-MEMORY |
| SPEC-RETEST (#905) | speculative_decoding | Speculative-decoding requalification | WP-01 |
| OBS-LIVE (#720) | observatory | Remove retained-mode demo hazards | Own readiness; execution Sprint 8 |
| OBS-S3 (#910) | observatory_deployment_sidecar | Deploy the existing Observatory S3 and CloudFront sidecar | WP-01, OBS-LIVE; completed-v0.92.1-issue-679, merged-v0.92.1-pr-685 |
| ARCH-ADR (#911) | architecture_decisions | Generate and reconcile the ADRs required by v0.92.2 | WP-01 |
| SIM-UMBRELLA (#866) | csdlc_simplification_sprint | Coordinate the first C-SDLC simplification sprint | SIM-09 (#875) |
| SIM-01 (#867) | csdlc_simplification_sprint | Make diagnostics observably read-only | Own readiness |
| SIM-02 (#868) | csdlc_simplification_sprint | One current installed command contract | SIM-01 (#867) |
| SIM-03 (#869) | csdlc_simplification_sprint | Execute the supported installed intent-command inventory | SIM-02 (#868) |
| SIM-04 (#870) | csdlc_simplification_sprint | Route operational mutations through one semantic transaction owner | SIM-03 (#869) |
| SIM-05 (#871) | csdlc_simplification_sprint | Derived cards and precise evidence invalidation | SIM-04 (#870) |
| SIM-06 (#872) | csdlc_simplification_sprint | Deliver one safe conversion rehearsal | SIM-05 (#871) |
| SIM-07 (#873) | csdlc_simplification_sprint | Produce one independent transition qualification | SIM-06 (#872) |
| SIM-08 (#874) | csdlc_simplification_sprint | Produce one transition operations packet | SIM-07 (#873) |
| SIM-09 (#875) | csdlc_simplification_sprint | Run one authorized consecutive-issue pilot | SIM-08 (#874) |
| TAIL-01 (#916) | release_tail | Quality gate | CF-INTEGRATE, CF-PROOF, PLAT-MLX, PLAT-PAIR, PLAT-UTS, PLAT-RUST, OPS-AWS, OPS-GCP, PUB-MEDIUM, PUB-CSDLC, SPEC-RETEST, OBS-LIVE, ARCH-SPLIT, CSDLC-MERGE, QUAL-RUNTIME, QUAL-RESIDENT, QUAL-PROVIDER, QUAL-INVENTORY, QUAL-EVIDENCE, RT-COST, CSDLC-MAN, CSDLC-DECOMPOSE, CSDLC-REMOTE, SIM-UMBRELLA |
| TAIL-02 (#917) | release_tail | Documentation review and external-review handoff | TAIL-01 |
| TAIL-03 (#918) | release_tail | Publication finalization | TAIL-02 |
| TAIL-04 (#919) | release_tail | Internal milestone review | TAIL-03 |
| TAIL-05 (#920) | release_tail | External or third-party review | TAIL-04 |
| TAIL-06 (#921) | release_tail | Accepted-findings remediation or explicit deferral capture | TAIL-05 |
| TAIL-07 (#922) | release_tail | Next-milestone planning | TAIL-06 |
| TAIL-08 (#923) | release_tail | Next-milestone closeout planning | TAIL-07 |
| TAIL-09 (#924) | release_tail | Next-milestone planning review | TAIL-08 |
| TAIL-10 (#925) | release_tail | Release ceremony and milestone close | TAIL-09, OBS-S3, ARCH-ADR |

## Parallelism

CF-ADAPTER opens local ingestion; CF-ADAPTER-GITHUB and CF-ADAPTER-CI follow it. CF-EVIDENCE admits local packets. CF-REVIEW follows CF-EVIDENCE and RT-PROVIDER; CF-SHELL then proves a real review journey. CF-SYNTHESIS follows the lane runner; CF-REMEDIATE and CF-TESTPLAN follow synthesis. CF-COG provides structure, followed by CF-COG-IMPACT and CF-COG-RATIONALE; CF-COG-DRIFT additionally consumes CF-MEMORY. CF-GOV-CI follows the working local CF-GOV runner. CF-UX owns approval; CF-RENDER-MD consumes approval, synthesis and both action plans; CF-RENDER-HTML and CF-RENDER-PDF follow Markdown for parity. RT-COST precedes PLAT-PROVIDER (also consuming merged #622), then RT-PROVIDER/#855. PLAT-MLX and PLAT-PAIR consume PLAT-PROVIDER. PLAT-MEMORY consumes CF-EVIDENCE and CF-MEMORY. CF-INTEGRATE connects the complete product consumers, all ingestion routes, RT-PROVIDER and PLAT-MEMORY first. CF-PROOF then independently qualifies that installed integrated product on ADL and the pinned external repository. TAIL-01 consumes both completed results.

## Work-Package Rule

Each implementation issue delivers one usable behavior through its production consumer, including supporting code, tests, failure handling and documentation. A schema, scaffold or authored packet alone is not delivery. Independent ingestion routes, analysis behaviors, review/action-planning stages and renderers are separate tasks. Evidence admission keeps identity, provenance and redaction as inseparable safety invariants. See [atomic task contracts](ATOMIC_TASK_CONTRACTS_v0.92.2.md) for the eight splits and eleven strengthened completion contracts.

The milestone inventory reconciles one bounded issue per expanded row. Every row must produce one named primary result. Supporting code, documentation, fixtures, and tests may travel with that result only when they are necessary to implement or prove it; independently useful results require separate rows before issue creation. WP-01 opens the CodeFriend wave; SIM-UMBRELLA coordinates its independently launched ten-issue sprint. Reuse #720, #848, #849, #852, #854, #855, #861, and #862; consume #717 and #718 as v0.92.1 predecessors; do not recreate completed #620 or predecessor #439. WP-01 is resolved as #864 and cannot create itself. The denominator is 69 rows. All 69 rows have canonical issue identities: nine preexisting and 60 newly created, with zero unassigned tasks. Issue creation does not establish implementation readiness or live activation authority.

## Immediate existing work

#717 and #718 are closed merged v0.92.1 predecessors and are consumed here as completed inputs. Existing #720, #848, #849, #852, #854, #855, #861, and #862 retain their own authority; #864 conducts planning. #848 delivers the decomposition decision only, not split implementation. Other backlog issues are not admitted by this package.

The [complete reviewed launch map](../../../.csdlc/evidence/864/all-issue-launch.json) binds all 69 core task IDs to verified issue numbers and independent creation reviews. This establishes issue inventory and review truth, not implementation or Beta 1 qualification.
