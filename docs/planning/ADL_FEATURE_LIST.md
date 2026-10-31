# ADL Feature List

Updated2026-10-31 by documentation reconciliation #1407. This is the capability overview and planning index for the Agent Logic system, not a claim that the public ADL repository contains private Runtime, CodeFriend or C-SDLC implementation.

## Scope and source truth

Public ADL owns language/contracts, schemas and CLI dispatch. [Repository owners](../REPOSITORIES.md) build and qualify their own products. The [roadmap](../ROADMAP.md) and [approved allocation](../milestones/v0.93.3/REORGANIZATION_1405.md) govern current placement. Historical milestone/review results remain dated evidence; source presence, schema checks and CI do not establish installed product acceptance or release authorization.

The complete feature denominator is retained in the [127-row acceptance crosswalk](v095-reconciliation/FEATURE_CROSSWALK_1407.md) and [exact original baseline](v095-reconciliation/FEATURE_BASELINE_1407.json). Every original name, status, proof reference and target survives; the current crosswalk qualifies stale placement and completion claims. No feature or adverse-case acceptance is removed by this update.

## Completion model

- `baseline_exists`: a bounded reviewable slice exists; identify its exact source and proof scope.
- `subsystem_complete`: named done criteria and owner validation/review are satisfied.
- `integrated_for_milestone`: actual named consumers pass the milestone's accepted contract.
- `mvp_complete`: no accepted MVP-blocking requirement remains, supported by qualification and review.
- `partial_or_blocked`: name the residual, owner, proof gap and delivery home.
- `post_mvp_completion`: retain the required MVP baseline, explicit later boundary and rationale.
- `out_of_scope`: explicit operator disposition; an unassigned feature is not an exclusion.

MVP v0.95 is convergence, not universal subsystem completion. Every retained roadmap feature needs at least a reviewable baseline, proof posture and explicit residual/disposition; more stringent original contracts still apply. This documentation does not requalify historical claims.

## Current milestone allocation

| Milestone | Scope | Planned WPs |
| --- | --- | --- |
| [v0.93.2](../milestones/v0.93.2/README.md) | CodeFriend Beta1 and enterprise security | Existing execution graph |
| [v0.93.3](../milestones/v0.93.3/README.md) | Runtime v4 plus existing #1352 | 15 |
| [v0.93.4](../milestones/v0.93.4/README.md) | C-SDLC v4 plus eight governance/continuity foundations | 17 |
| [v0.93.5](../milestones/v0.93.5/README.md) | GHB | 17 |
| [v0.93.6](../milestones/v0.93.6/README.md) | Remaining governance/social cognition | 17 |
| [v0.93.7](../milestones/v0.93.7/README.md) | Affect8/humour4 plus integrated qualification/tail | 25 |
| [v0.94](../milestones/v0.94/README.md) | Secure execution/trust, signed/queryable trace, reasoning graph and mental time travel | Candidate wave8 |
| [v0.94.1](../milestones/v0.94.1/README.md) | Payments/settlement/economic follow-on; MVP subset unresolved | Candidate wave7 |
| [v0.95](../milestones/v0.95/README.md) | MVP convergence, bounded operator/editor/evaluation surfaces and feature freeze | Candidate wave13 |

Counts include coordination/acceptance gates and are not work estimates or one-PR-per-row requirements. Preserve the current allocation; no further split or one-day feasibility claim is made. Create child issues/cards only per separately activated sprint. Unity #84 and TLS #251 retain their explicit backlog disposition; no revival from historical feature prose.

## MVP evidence and decisions

[Reconciliation register](v095-reconciliation/README.md) covers all eight audit findings and all68document scope entries. [Decision and deferral register](v095-reconciliation/DECISIONS_AND_DEFERRALS_1407.md) retains the new decision-model programme, standalone UTS/SDK/ATEC/AVP boundaries, Shepherd/Gemma outcome question, payments subset, speculative backend decision, optional Zed and every named local proposal.

v0.95 must consume actual prior owner evidence for governance/guilds, GHB, affect/humour, Runtime recovery, Curiosity/Constructability, Memory Palace, AEE/arbitration and other retained capabilities. A prior bounded Runtime v2/v3 result is not automatically current Runtime v4 parity. Reuse unchanged exact-input proof; do not rerun every historical demo or create127implementation issues.

CodeFriend v1 portability consumes Beta1's actual onboarding/repository/report/export and current template/journal/full4+1 obligations where already required. Those journeys are not silently deferred after MVP. Broader billing/portfolio expansion remains separately scoped. Aptitude Atlas productization remains post-v0.95; independent ATEC/AVP product work is neither automatically included nor silently discarded.

## Full feature inventory

The source-status column is a dated snapshot at the baseline revision, not current qualification. Follow each stable ID in the acceptance crosswalk for current owner, delivery, proof and residuals.

| ID | Feature | Source status (historical snapshot) | Current delivery home |
| --- | --- | --- | --- |
| F-001 | Deterministic workflow execution | Implemented | v0.93.3/RV-08 |
| F-002 | ExecutionPlan runtime | Implemented | v0.93.3/RV-08 |
| F-003 | Sequential + fork/join coordination | Implemented | v0.93.3/RV-08 |
| F-004 | Bounded concurrency and retry/failure controls | Implemented | v0.93.3/RV-08 |
| F-005 | Run artifacts and replay-oriented inspection | Implemented baseline | v0.93.3/RV-08 |
| F-006 | Signing, verification, and trust policy | Implemented baseline | v0.93.2/WP-S1..WP-S6 |
| F-007 | Provider and transport substrate | Implemented baseline | v0.93.3/RV-08 |
| F-008 | Remote execution baseline | Implemented baseline | v0.95/WP-06; accepted v0.94 secure execution |
| F-009 | Human-in-the-loop pause/resume | Implemented baseline; AEE-required subset pulled forward | v0.93.3/RV-08 |
| F-010 | Structured authoring model | Implemented baseline | v0.93.4/CS4-01..CS4-06 |
| F-011 | Structured planning and Structured Review Prompt workflow | Implemented baseline | v0.93.4/CS4-01..CS4-06 |
| F-012 | Control-plane lifecycle | Implemented baseline; AEE-required truth surfaces pulled forward | v0.93.4/CS4-01..CS4-06 |
| F-013 | Editor and command-adapter surfaces | Implemented baseline | v0.95/WP-09 |
| F-014 | Review and validation surfaces | Implemented baseline | v0.93.4/CS4-01..CS4-06 |
| F-015 | Task-bundle workflow | Implemented baseline | v0.93.4/CS4-01..CS4-06 |
| F-016 | Agency, cognitive loop, and cognitive stack | Implemented baseline | v0.93.3/RV-08 |
| F-017 | Fast/slow thinking and cognitive arbitration | Implemented baseline | v0.93.7/AF-01..08,HU-01..04 |
| F-018 | Bounded Godel loop | Implemented baseline | v0.93.5/GHB-WP00..GHB-WP12 |
| F-019 | Godel agents and Godel-Hadamard-Bayes algorithm | Partially implemented | v0.93.5/GHB-WP00..GHB-WP12 |
| F-020 | ObsMem indexing, retrieval, and evidence-aware ranking | Implemented baseline | v0.93.3/RV-08 |
| F-021 | Shared ObsMem foundation | Implemented baseline | v0.93.4/GOV-01..03,GOV-11,CM-01..04; v0.93.6/GOV-04..16 as applicable |
| F-022 | Trace validation, trace review, and trace-to-memory ingestion | Implemented baseline | v0.94/WP-02..WP-05 |
| F-023 | Bounded cognitive path | Implemented baseline | v0.93.7/AF-01..08,HU-01..04 |
| F-024 | Freedom Gate baseline | Implemented baseline | v0.93.3/RV-08 |
| F-025 | Freedom Gate v2 | Implemented baseline | v0.93.4/GOV-01..03,GOV-11,CM-01..04; v0.93.6/GOV-04..16 as applicable |
| F-026 | Trace substrate | Implemented baseline | v0.94/WP-02..WP-05 |
| F-027 | Operational skills substrate | Implemented baseline | v0.93.3/RV-08 |
| F-028 | Runtime environment and lifecycle completion | Implemented baseline | v0.93.3/RV-08 |
| F-029 | Execution boundaries and capability-aware local execution | Implemented baseline | v0.93.3/RV-08 |
| F-030 | Local runtime resilience and Shepherd preservation | Implemented baseline | v0.93.3/RV-08 |
| F-031 | Chronosense / temporal substrate | Implemented baseline | v0.93.3/RV-08 |
| F-032 | Temporal query, retrieval, identity semantics, and continuity hooks | Implemented baseline | v0.93.3/RV-08 |
| F-033 | Commitments, deadlines, and bounded temporal causality | Implemented baseline | v0.93.4/GOV-01..03,GOV-11,CM-01..04; v0.93.6/GOV-04..16 as applicable |
| F-034 | Cost model, accounting primitives, and bounded economics hooks | Implemented baseline | v0.94.1/WP-02..WP-06 (existing bounded economics preserved) |
| F-035 | PHI-style integration metrics | Implemented baseline | v0.93.3/RV-08 |
| F-036 | Instinct and bounded agency | Implemented baseline | v0.93.7/AF-01..08,HU-01..04 |
| F-037 | Paper Sonata public-facing proof surface | Implemented baseline | v0.95/WP-07 |
| F-038 | Deep-agents comparative proof | Implemented baseline | v0.95/WP-07 |
| F-039 | AEE 1.0 convergence | Implemented baseline; subsystem closure explicitly pulled forward | v0.93.3/RV-08 |
| F-040 | Decision, action, and skill-governance surfaces | Implemented baseline | v0.93.4/GOV-01..03,GOV-11,CM-01..04; v0.93.6/GOV-04..16 as applicable |
| F-041 | Delegation, refusal, and coordination contracts | Implemented baseline | v0.93.4/GOV-01..03,GOV-11,CM-01..04; v0.93.6/GOV-04..16 as applicable |
| F-042 | Provider-extension packaging and safe extension boundaries | Implemented baseline | v0.93.2/WP-S1..WP-S6 |
| F-043 | Security, posture, and trust-under-adversary package | Implemented baseline | v0.93.2/WP-S1..WP-S6 |
| F-044 | Adversarial runtime, exploit/replay, and self-attack band | Implemented baseline | v0.93.2/WP-S1..WP-S6 |
| F-045 | Demo proof entry points and quality gate | Implemented baseline | v0.95/WP-07 |
| F-046 | Five-agent Hey Jude MIDI demo | Implemented baseline | v0.95/WP-07 |
| F-047 | arXiv paper writer and three-paper program | Implemented baseline | Historical publication packets; v0.95/WP-07,WP-11 consumption |
| F-048 | Long-lived supervisor, heartbeat, and cycle artifacts | Implemented baseline | v0.93.3/RV-08 |
| F-049 | Stock-league long-lived demo family | Implemented baseline | v0.95/WP-07 |
| F-050 | Minimal status/inspection boundary | Implemented baseline | v0.93.3/RV-08 |
| F-051 | CodeFriend review showcase and architecture-document generation | Implemented baseline | v0.93.2/CF-B1-01..05,CF-92 |
| F-052 | Coverage ratchet, test tracker, and quality tracking | Implemented baseline | v0.93.4/CS4-01..CS4-06 |
| F-053 | Rust refactoring tracker and evidence-driven maintenance | Implemented baseline | v0.93.4/CS4-01..CS4-06 |
| F-054 | Milestone compression and repo visibility prototypes | Implemented baseline | v0.95/WP-02 |
| F-055 | HTML milestone dashboard and compression reporting | MVP-scoped | v0.95/WP-02 |
| F-056 | Runtime v2 foundation prototype | Implemented baseline | v0.93.3/RV-08 |
| F-057 | CSM Observatory visibility and operator-report surfaces | Implemented baseline with retained Runtime v3 parity proof | v0.93.3/RV-08 |
| F-058 | Runtime v2 hardening, recovery, quarantine, and expanded invariants | Implemented baseline | v0.93.3/RV-08 |
| F-059 | First bounded CSM run | Implemented baseline | v0.93.3/RV-08 |
| F-060 | Third-party review and review-quality gates | Implemented baseline | v0.93.4/CS4-01..CS4-06 |
| F-061 | ANRM / shepherd-model experiments | Partially implemented | v0.95/WP-03 (mixed row also v0.93.6 governance) |
| F-062 | CSM Shepherd model and Gemma training path | MVP-scoped | v0.95/WP-03 (mixed row also v0.93.6 governance) |
| F-063 | Capability-testing evidence and Aptitude Atlas boundary | Post-MVP Aptitude Atlas productization; v0.95 evidence consumption only | v0.95/WP-04 |
| F-064 | Governed tool calls and capability contracts | Implemented baseline | Existing UTS release/adoption issues and accepted owner contracts |
| F-065 | Cognitive Compression Cost instrumentation | Implemented first pass | v0.95/WP-02 |
| F-066 | Web-based code editor integration | MVP-scoped | v0.95/WP-09 |
| F-067 | Reasoning graph baseline | Bounded Runtime v2 proof retained; Runtime v3 parity proof retained; broader provenance MVP-scoped | v0.94/WP-02..WP-05 |
| F-068 | Signed trace and trace query | MVP-scoped | v0.94/WP-02..WP-05 |
| F-069 | Wellbeing, affect, kindness, moral cognition, humor | Implemented Runtime v2 baseline; bounded Runtime v3 parity disposition retained | v0.93.7/AF-01..08,HU-01..04 |
| F-070 | Secure Agent Communication and Invocation Protocol | Implemented baseline plus active hardening | v0.93.3/RV-08 |
| F-071 | Inhabited runtime readiness | Implemented baseline | v0.93.3/RV-08 |
| F-072 | Runtime/polis architecture alignment | Implemented baseline | v0.93.3/RV-08 |
| F-073 | Agent lifecycle state model | Implemented baseline | v0.93.3/RV-08 |
| F-074 | CSM Observatory active agent runtime | Implemented baseline | v0.93.3/RV-08 |
| F-075 | Citizen standing and citizen state follow-on | Implemented baseline | v0.93.4/GOV-01..03,GOV-11,CM-01..04; v0.93.6/GOV-04..16 as applicable |
| F-076 | Memory, Theory of Mind, capability testing, intelligence metrics, governed learning, and ANRM/Gemma | Implemented baseline | v0.95/WP-03 (mixed row also v0.93.6 governance) |
| F-077 | ACIP hardening and local encryption boundary | Implemented baseline | v0.93.2/WP-S1..WP-S6 |
| F-078 | A2A adapter boundary | Implemented baseline | v0.93.3/RV-08 |
| F-079 | Runtime inhabitant proof | Implemented baseline | v0.93.3/RV-08 |
| F-080 | UTS + ACC multi-model benchmark and provider-native tool-call comparison | Implemented baseline | Existing UTS release/adoption issues and accepted owner contracts |
| F-081 | Runtime/test-cycle recovery and coverage ergonomics | Implemented baseline | v0.93.4/CS4-01..CS4-06 |
| F-082 | CodeFriend repo-review product layer | Implemented baseline | v0.93.2/CF-B1-01..05,CF-92 |
| F-083 | Review heuristics and reviewer demo lane | Implemented baseline | v0.95/WP-07 |
| F-084 | Google Workspace CMS bridge and Rust-native adapter boundary | Implemented baseline plus active hardening | Retained bounded bridge; future productization deferred |
| F-085 | Automated repository modernization and external refactoring integration | Implemented bounded packet | Retained bounded Moderne/OpenRewrite packet; broader service deferred |
| F-086 | Generic speculative decoding runtime acceleration | Implemented evaluation packet | Retained bounded evaluation; MVP inclusion unresolved |
| F-087 | Repo visibility follow-on | Implemented baseline | v0.95/WP-02 |
| F-088 | Publication packet program and GHB paper lane | Implemented packet | Historical publication packets; v0.95/WP-07,WP-11 consumption |
| F-089 | General-intelligence paper packet | Implemented packet | Historical publication packets; v0.95/WP-07,WP-11 consumption |
| F-090 | Rustdoc/doc cleanup | Implemented baseline | v0.93.4/CS4-01..CS4-06 |
| F-091 | Workflow guardrails | Implemented baseline | v0.93.4/CS4-01..CS4-06 |
| F-092 | Cognitive SDLC first slice and transition manifest | Implemented baseline | v0.93.4/CS4-01..CS4-06 |
| F-093 | Cognitive SDLC default operation and five-minute-sprint repeatability | Implemented baseline | v0.93.4/CS4-01..CS4-06 |
| F-094 | Logging, observability, and OTel-compatible proof-loop readiness | Retained bounded prerequisite and consumer proof | v0.93.3/RV-08 |
| F-095 | Resilience, citizen persistence, and operational sleep/wake | Retained bounded integration proof | v0.93.3/RV-08 |
| F-096 | Public prompt records export, redaction, validation, and indexing | Required pre-v0.92 implementation/proof surface | v0.93.4/CS4-01..CS4-06 |
| F-097 | Provider/model reliability and multi-agent readiness | Retained bounded implementation/proof | v0.93.3/RV-08 |
| F-098 | Security readiness and Continuous Adversarial Verification | Retained bounded implementation/proof and review remediation | v0.93.2/WP-S1..WP-S6 |
| F-099 | Curiosity Engine and Discovery Substrate | Bounded Runtime v2 cycle implemented/proven | v0.93.3/RV-08 |
| F-100 | Constructability Gate for shared ADL reality | Bounded Runtime v2 validator implemented/proven | v0.93.3/RV-08 |
| F-101 | Reasoning graph, loop runtime, and `adl.skill.v1` | Bounded Runtime v2 implementation/proof retained; Runtime v3 parity proof retained | v0.94/WP-02..WP-05 |
| F-102 | ACP / cognitive profiles runtime surface | Runtime resident-cycle integration in progress | v0.93.3/RV-08 |
| F-103 | ACIP binary schema and WebSocket carrier | Planned | v0.93.3/RV-08 |
| F-104 | Identity, stable name, and continuity substrate | Planned | v0.93.3/RV-08 |
| F-105 | Memory grounding, capability envelope, and birth witnesses/receipt | Production composition implemented | v0.93.3/RV-08 |
| F-106 | Memory Palace navigable context topology | Implemented Runtime-kernel production-authority slice | v0.93.3/RV-08 |
| F-107 | First true Gödel-agent birthday | Planned | v0.93.5/GHB-WP00..GHB-WP12 |
| F-108 | Opening repository split | Planned; operator-selected v0.93.1 opening phase | Delivered source allocation/history #1223 and repository map |
| F-109 | Runtime v4 CSM plugin system | Required v0.93.2 outcome; not implemented by planning | v0.93.3/RV-01..RV-11 |
| F-110 | CodeFriend launch after qualified Beta 1 | Required v0.93.1 release outcome; audience/environment/limits resolved before launch | v0.93.2/CF-B1-01..05,CF-92 |
| F-111 | CodeFriend versioned artifact templates and style guides | Required v0.93.1; operator confirmed | v0.93.2/CF-B1-01..05,CF-92 |
| F-112 | Citizen reproduction and migration | Required v0.93.2; operator confirmed | v0.93.4/GOV-01..03,GOV-11,CM-01..04; v0.93.6/GOV-04..16 as applicable |
| F-113 | Constitutional citizenship, rights/duties, and governance review | Planned | v0.93.4/GOV-01..03,GOV-11,CM-01..04; v0.93.6/GOV-04..16 as applicable |
| F-114 | Bounded Theory of Mind, relationship, reputation, and shared social memory boundary | Planned | v0.93.6/GOV-06..16,DEMO-GOV |
| F-115 | Delegation, upstream delegation, IAM, standing transition, and challenge/appeal governance | Planned | v0.93.4/GOV-01..03,GOV-11,CM-01..04; v0.93.6/GOV-04..16 as applicable |
| F-116 | Guilds and collective organization | MVP-scoped planned governance surface | v0.93.6/GOV-06..16,DEMO-GOV |
| F-117 | Enterprise security for the ADL polis | Planned | v0.93.2/WP-S1..WP-S6 |
| F-118 | Secure execution, policy, identity/auth, isolation, and provider-trust convergence | Planned | v0.94/WP-02..WP-05 |
| F-119 | Mental time travel / temporal self-projection | Planned | v0.94/WP-06 |
| F-120 | Payments, settlement, economic agency, and `x402` / Lightning adapters | Planned | v0.94.1/WP-02..WP-06 (existing bounded economics preserved) |
| F-121 | Bounded contract-market and resource-stewardship bridge | Implemented baseline | v0.94.1/WP-02..WP-06 (existing bounded economics preserved) |
| F-122 | Distributed execution integration | Partially implemented; AEE/multi-agent boundary subset pulled forward | v0.95/WP-06; accepted v0.94 secure execution |
| F-123 | CodeFriend Beta 1 and portable Adapter v2 | Planned complete beta milestone | v0.93.2/CF-B1-01..05,CF-92 |
| F-124 | Capability-testing evidence consumption / Aptitude Atlas boundary | MVP-scoped evidence consumption; productization post-v0.95 | v0.95/WP-04 |
| F-125 | Demo catalog and polished MVP walkthrough | Partially implemented | v0.95/WP-07 |
| F-126 | Control-plane Rust migration / tooling hardening | Partially implemented | v0.93.4/CS4-01..CS4-06 |
| F-127 | Zed integration | MVP-scoped decision | v0.95/WP-10 |

## Historical narrative

The former overview, chronological crosswalk and source narrative remain available in [the baseline revision](https://github.com/agent-logic/agent-design-language/blob/6b4126980209c7de68b91fd227612e1b161e87b1/docs/planning/ADL_FEATURE_LIST.md). They describe the original scope and proof claims; they do not override the current owner/target crosswalk or authorize product execution, publication, spend or deployment.
