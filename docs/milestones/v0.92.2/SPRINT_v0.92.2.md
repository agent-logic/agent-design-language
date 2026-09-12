# v0.92.2 Sprint Plan

Status: planned; sequence expresses dependencies, not calendar deadlines.

## Execution sprint assignments

Every core issue is assigned exactly once below, including all nine preexisting identities. The `execution_sprints` map in the [sprint registry](ISSUE_CREATION_BATCHES_v0.92.2.json) and [verified identity map](../../../.csdlc/evidence/864/all-issue-launch.json) records the same assignments. Execution sprint numbers are scheduling groups, distinct from immutable issue-creation batches. The 60 newly created tasks retain the corresponding batch number as their execution sprint; the nine reused tasks are now explicitly assigned. This adds no dependency edges or blanket requirement to wait for an entire earlier sprint. All core startup, task dependencies, accepted-output gates and ownership checks remain in force. Within a sprint, run prerequisites first; independent work may overlap when its actual gates permit.

| Sprint | Focus | Preexisting issues | Newly created issues |
|---|---|---|---|
| 1 | C-SDLC simplification and operator documentation | WP-01 (#864), CSDLC-MAN (#861) | SIM-UMBRELLA (#866), SIM-01 (#867), SIM-02 (#868), SIM-03 (#869), SIM-04 (#870), SIM-05 (#871), SIM-06 (#872), SIM-07 (#873), SIM-08 (#874), SIM-09 (#875) |
| 2 | Runtime/provider foundations and ingestion | ARCH-SPLIT (#848), RT-COST (#854), RT-PROVIDER (#855) | PLAT-PROVIDER (#876), PLAT-UTS (#877), CF-ADAPTER (#878), CF-ADAPTER-GITHUB (#879), CF-ADAPTER-CI (#880), CF-EVIDENCE (#881) |
| 3 | Architecture, governance and memory | None | CF-COG (#882), CF-COG-IMPACT (#883), CF-COG-RATIONALE (#884), CF-MEMORY (#885), CF-COG-DRIFT (#886), CF-GOV (#887), CF-GOV-CI (#888), PLAT-MEMORY (#889) |
| 4 | Review, action plans and publication renderers | None | CF-REVIEW (#890), CF-SHELL (#891), CF-SYNTHESIS (#892), CF-REMEDIATE (#893), CF-TESTPLAN (#894), CF-UX (#895), CF-RENDER-MD (#896), CF-RENDER-HTML (#897), CF-RENDER-PDF (#898) |
| 5 | Runtime repair and qualification | QUAL-RUNTIME (#852) | QUAL-INVENTORY (#899), QUAL-RESIDENT (#900), QUAL-PROVIDER (#901), QUAL-EVIDENCE (#902) |
| 6 | Hardware/provider qualification | None | PLAT-MLX (#903), PLAT-PAIR (#904), SPEC-RETEST (#905) |
| 7 | Rust and C-SDLC decomposition | CSDLC-MERGE (#849), CSDLC-DECOMPOSE (#862) | PLAT-RUST (#906), CSDLC-REMOTE (#907) |
| 8 | Cloud operations and Observatory | OBS-LIVE (#720) | OPS-AWS (#908), OPS-GCP (#909), OBS-S3 (#910) |
| 9 | ADRs and manuscripts | None | ARCH-ADR (#911), PUB-MEDIUM (#912), PUB-CSDLC (#913) |
| 10 | Installed integration then independent qualification | None | CF-INTEGRATE (#914), CF-PROOF (#915) |
| 11 | Canonical release tail | None | TAIL-01 (#916), TAIL-02 (#917), TAIL-03 (#918), TAIL-04 (#919), TAIL-05 (#920), TAIL-06 (#921), TAIL-07 (#922), TAIL-08 (#923), TAIL-09 (#924), TAIL-10 (#925) |

Sprint 1 completes WP-01 readiness before CSDLC-MAN, while the SIM chain retains its own declared entry conditions. Sprint 2 orders RT-COST → PLAT-PROVIDER → RT-PROVIDER. Sprint 5 completes QUAL-RUNTIME and the actual qualification producers before QUAL-EVIDENCE. Sprint 7 completes CSDLC-MERGE and CSDLC-DECOMPOSE before CSDLC-REMOTE. Sprint 8 completes OBS-LIVE before OBS-S3. Sprint 10 runs CF-INTEGRATE → CF-PROOF; Sprint 11 preserves TAIL-01 → TAIL-10 and the final OBS-S3/ARCH-ADR obligations. No dependency points from an earlier execution sprint to a later one.

Podcast #671 is separately scheduled sidecar work outside these eleven core sprint groups and the 69-task startup denominator. It retains its own explicit public-launch/provider-submission approval requirements.

## Sprint management umbrellas

Operator instruction under #926 adds one umbrella for each full execution sprint. These management issues preserve the existing child rosters and dependencies; they do not create an all-sprints serial barrier. #866 remains the narrower SIM coordinator inside Sprint 1.

| Sprint | Umbrella | Existing children |
|---|---|---|
| 1 | [#927](https://github.com/agent-logic/agent-design-language/issues/927) | #864, #861, #866, #867, #868, #869, #870, #871, #872, #873, #874, #875 |
| 2 | [#928](https://github.com/agent-logic/agent-design-language/issues/928) | #848, #854, #855, #876, #877, #878, #879, #880, #881 |
| 3 | [#929](https://github.com/agent-logic/agent-design-language/issues/929) | #882, #883, #884, #885, #886, #887, #888, #889 |
| 4 | [#930](https://github.com/agent-logic/agent-design-language/issues/930) | #890, #891, #892, #893, #894, #895, #896, #897, #898 |
| 5 | [#931](https://github.com/agent-logic/agent-design-language/issues/931) | #852, #899, #900, #901, #902 |
| 6 | [#932](https://github.com/agent-logic/agent-design-language/issues/932) | #903, #904, #905 |
| 7 | [#933](https://github.com/agent-logic/agent-design-language/issues/933) | #849, #862, #906, #907 |
| 8 | [#934](https://github.com/agent-logic/agent-design-language/issues/934) | #720, #908, #909, #910 |
| 9 | [#935](https://github.com/agent-logic/agent-design-language/issues/935) | #911, #912, #913 |
| 10 | [#936](https://github.com/agent-logic/agent-design-language/issues/936) | #914, #915 |
| 11 | [#937](https://github.com/agent-logic/agent-design-language/issues/937) | #916, #917, #918, #919, #920, #921, #922, #923, #924, #925 |

## First sprint — C-SDLC v3 simplification alongside Runtime

SIM-UMBRELLA opens coordination; SIM-01 → SIM-02 → SIM-03 → SIM-04 → SIM-05 → SIM-06 → SIM-07 → SIM-08 → SIM-09 is one coherent sprint. After all 69 core identities are created and all creation reviews pass, it starts through its own readiness in parallel with Runtime. Its graph does not add a WP-01 or unrelated prior-closeout dependency. The umbrella completes after SIM-09 and converges at TAIL-01. Resolve C-SDLC/Runtime shared-path and installed-binary ownership before overlapping writes. An eventual C-SDLC writer pause requires separate explicit authorization; it does not pause Runtime/provider services. See the [complete plan](cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md).

## CodeFriend implementation waves

CF-ADAPTER opens local ingestion; CF-ADAPTER-GITHUB and CF-ADAPTER-CI follow it. CF-EVIDENCE admits local packets. CF-REVIEW follows CF-EVIDENCE and RT-PROVIDER; CF-SHELL then proves a real review journey. CF-SYNTHESIS follows the lane runner; CF-REMEDIATE and CF-TESTPLAN follow synthesis. CF-COG provides structure, followed by CF-COG-IMPACT and CF-COG-RATIONALE; CF-COG-DRIFT additionally consumes CF-MEMORY. CF-GOV-CI follows the working local CF-GOV runner. CF-UX owns approval; CF-RENDER-MD consumes approval, synthesis and both action plans; CF-RENDER-HTML and CF-RENDER-PDF follow Markdown for parity. RT-COST precedes PLAT-PROVIDER (also consuming merged #622), then RT-PROVIDER/#855. PLAT-MLX and PLAT-PAIR consume PLAT-PROVIDER. PLAT-MEMORY consumes CF-EVIDENCE and CF-MEMORY. CF-INTEGRATE connects the complete product consumers, all ingestion routes, RT-PROVIDER and PLAT-MEMORY first. CF-PROOF then independently qualifies that installed integrated product on ADL and the pinned external repository. TAIL-01 consumes both completed results.

WP-01 records the complete operator-authorized creation wave; all eleven creation batches have passed independent review. All seven planning tasks remain required. Supporting tasks retain the exact dependencies in the issue wave. QUAL-EVIDENCE consumes the four independently completed Runtime repair/proof tasks. CSDLC-REMOTE follows local decomposition and CSDLC-MERGE; shared-path ownership with SIM is reconciled before execution.

## Milestone closeout — Canonical Release Tail

Run TAIL-01 through TAIL-10 in exact order. Individual issue closeout is asynchronous; downstream work depends on merged product authority and the stated release gate, not on bookkeeping receipts.

## Scope Control

Deferred connectors, autonomous mutation, public customer-scale or multi-tenant deployment, ATE, OCI model packaging, optional modernization, and Runtime v4 require separate admission. The bounded static Observatory sidecar does not authorize those broader deployment programs.

## Existing issue dependency boundaries

Closed merged #717 and #718 are v0.92.1 predecessor inputs. #720 uses its own independent authority after the global core startup gate; #848, #849, #852, #854, #855, #861, and #862 use their mapped existing authority after WP-01. #848 is a decision row rather than split implementation. This schedules existing v0.92.2 work; #523 does not implement it.

TAIL-10 waits for TAIL-09, OBS-S3 and ARCH-ADR. Final acceptance separately verifies authenticated Observatory deployment and the completed, source-grounded ADR set with explicit decision status. These obligations do not block CF-INTEGRATE or the early TAIL-01 quality gate, and do not change the canonical ten-step sequence.

## First sprint creation state

SIM-UMBRELLA is #866 and SIM-01 through SIM-09 are #867 through #875 respectively. The [verified mapping](../../../.csdlc/evidence/864/sprint01-launch/issues.json) records native creation. Together with the ten later creation batches, the final wave has 69 assigned core issues: nine preexisting and 60 newly created identities, with zero unassigned tasks. Creation does not claim implementation or authorize live activation. The operator authorized all eleven creation batches; the final launch map records their created identities and independent reviews.

## Global implementation startup gate

The operator requires all 69 core-plan issue identities to be created and all creation-batch reviews to pass before any implementation begins, including the independently scheduled SIM sprint. Once that global gate is satisfied, the declared task dependencies, native readiness, bound ownership and issue-level authority still govern execution. Creation batches add no dependency edges and confer no paid/cloud/provider, publication or writer-activation authority.

Existing podcast issue [#671](https://github.com/agent-logic/agent-design-language/issues/671) is a separately admitted v0.92.2 sidecar. It is outside the 69-task atomic plan, core startup denominator and dependency graph. Its own retained public-launch/provider-submission approval gates remain in force; milestone membership does not authorize those effects. The milestone has 82 issue memberships: 69 core-plan identities, this one sidecar, eleven sprint-management umbrellas (#927–#937), and setup issue #926. The management layer is recorded in [SPRINT_MANAGEMENT_v0.92.2.json](SPRINT_MANAGEMENT_v0.92.2.json); it does not change the 69-task implementation/startup denominator.

The [complete reviewed launch map](../../../.csdlc/evidence/864/all-issue-launch.json) binds all 69 core task IDs to verified issue numbers and independent creation reviews. This establishes issue inventory and review truth, not implementation or Beta 1 qualification.
