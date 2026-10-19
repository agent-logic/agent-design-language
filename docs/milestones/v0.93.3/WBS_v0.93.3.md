# v0.93.3 Wbs

## Metadata

Planning template set: 1.1.0. Target: v0.93.3. Setup issue: ADL #1296. Owner of planning: Planning #12. Product execution owners remain unassigned.

## Status

Planned successor only. No opening or execution is authorized. No deployment, provider spend, release or Runtime restart is authorized by these documents.

## WBS Summary

59 planned identities across four dependency-based sprints. Preserve all 67 earlier identities through [the allocation register](../v0.93.2/REALLOCATION_1296.json). 46 moved platform identities plus 13 version-specific opening, integration, qualification and release-tail gates. This is an allocation, not permission to launch a large parallel execution wave.

## Work Packages

| WP | Sprint | Repository | Outcome |
| --- | --- | --- | --- |
| WP-01 | 1 | agent-design-language | Open the separately reviewed Runtime v4, governance and GHB milestone after explicit authorization. |
| RV-01 | 1 | agent-logic-runtime | An accepted plugin contract defines topology precedence, epochs, durable activation journal, in-flight delivery, cancellation, versioned state/config migration, two-phase reconfiguration, dependency-safe removal and recovery classes. |
| RV-02 | 1 | agent-logic-runtime | Existing Runtime behavior and identity survive native adapter conversion with no second scheduler. |
| RV-03 | 1 | agent-logic-runtime | The durable generation transaction stages, quiesces, activates and rolls back to one fenced generation after interruption; in-flight work and cancellation retain their declared disposition. |
| RV-09 | 1 | agent-logic-runtime | An installed plugin migrates declared prior-version state to the selected version and preserves the prior generation when the migrator refuses or fails. |
| RV-10 | 1 | agent-logic-runtime | Versioned configuration migration and two-phase validation/activation apply one accepted configuration, or preserve the prior version and active configuration on refusal, failure or interruption. |
| RV-11 | 1 | agent-logic-runtime | A requested plugin removal checks dependencies and in-flight work, refuses unsafe removal, and completes authorized removal with durable recovery state. |
| RV-04 | 1 | agent-logic-runtime | One real non-core plugin uses authenticated bounded transport, deadlines and process supervision. |
| RV-05 | 1 | agent-logic-runtime | The canonical contract drives a default-deny WIT adapter with enforced memory, time and host-call limits. |
| RV-06 | 1 | agent-logic-runtime | One module has equivalent CSM-visible identity, messages, failure and state behavior across native, process and WASM adapters. |
| RV-07 | 1 | agent-logic-runtime | Provider instances consume host-owned credentials and versioned configuration through the common inference capability. |
| RV-08 | 1 | agent-logic-runtime | Installed package signing, checkpoint compatibility, generation recovery and Observatory operations pass with a retained known-good rollback. |
| GOV-01 | 2 | agent-logic-runtime | Runtime admits citizen conduct only through bound identity, Freedom Gate and signed trace; guest/operator action remains distinct. |
| GOV-02 | 2 | agent-logic-runtime | A Runtime policy evaluator returns rights/duties decisions with trace evidence for a bounded action. |
| GOV-03 | 2 | agent-logic-runtime | Standing maintenance, restriction, restoration and revocation consume existing state primitives and retain reasons and challenge context. |
| GOV-04 | 2 | agent-logic-runtime | A review consumer emits findings over trace/outcome/policy evidence without raw private-state access. |
| GOV-05 | 2 | agent-logic-runtime | A challenged finding progresses through appeal and recorded disposition without destroying original evidence. |
| GOV-06 | 2 | agent-logic-runtime | Signed evidence-backed model updates replay under standing, consent/scope and projection controls. |
| GOV-07 | 2 | agent-logic-runtime | Conflicting and stale entries change a bounded consumer decision through explicit uncertainty/decay events. |
| GOV-08 | 2 | agent-logic-runtime | Relationship records preserve evidence and permitted scope across restart without becoming standing verdicts. |
| GOV-09 | 2 | agent-logic-runtime | Authorized redacted projections can inform review while retaining challenge and source lineage. |
| GOV-10 | 2 | agent-logic-runtime | Two governed consumers share evidence projections with preserved uncertainty, access rules and freshness. |
| GOV-11 | 2 | agent-logic-runtime | Delegated action binds identity, standing, resource, capability and policy without exceeding the delegator grant. |
| GOV-12 | 2 | agent-logic-runtime | A selected/rejected upstream call retains citizen identity, policy, provenance and verification; ToM may inform selection but cannot override authority. |
| GOV-13 | 2 | agent-logic-runtime | A governed exchange succeeds while an associated unauthorized private-state/ToM inspection is denied. |
| GOV-14 | 2 | agent-logic-runtime | A policy consumer evaluates explicit polis/citizen obligations and emits reviewable evidence. |
| GOV-15 | 2 | agent-logic-runtime | Guild identity, membership, scoped resources and delegated actions persist with trace and challenge support. |
| GOV-16 | 2 | agent-logic-runtime | The operator receives evidence-linked standing, appeals and social-memory summaries without private-state leaks or scalar moral ranking. |
| CM-01 | 2 | agent-logic-runtime | A reviewed contract distinguishes same-identity migration and recovery from new-identity descendants, replication and independent clones; it defines authority, lineage, consent, state inheritance, fencing and recovery outcomes. |
| CM-02 | 2 | agent-logic-runtime | Two owned Runtime endpoints transfer one authorized citizen checkpoint and scoped continuity evidence; the source is fenced before destination activation and interruption can recover without two active holders or lost lineage. |
| CM-03 | 2 | agent-logic-runtime | An authorized parent creates one distinctly identified descendant with explicit lineage, selected inherited state and newly scoped authority while preserving the parent and protecting private state. |
| CM-04 | 2 | agent-logic-runtime | Independent installed scenarios exercise migration and descendant creation across restart, interruption and retry, verify lineage/privacy/fencing and expose the resulting identity and custody status to the operator. |
| DEMO-GOV | 2 | agent-logic-runtime | Execute D1-D8 and guild D13 through installed Runtime consumers with accepted and refused paths. |
| GHB-SPRINT | 3 | agent-logic-runtime | Open and coordinate the approved GHB program without granting execution, spend, or completion authority. |
| GHB-WP00 | 3 | agent-logic-runtime | Owner: program + Runtime. Dependencies: none. |
| GHB-WP01 | 3 | agent-design-language | Owner: ADL. Dependencies: WP00. |
| GHB-WP02 | 3 | agent-design-language | Owner: ADL. Dependencies: WP01. |
| GHB-WP03 | 3 | agent-logic-runtime | Owner: Runtime algorithm lane; exact module selected by WP00. Dependencies: WP02. |
| GHB-WP04 | 3 | agent-logic-runtime | Owner: Runtime algorithm lane. Dependencies: WP02; integrate WP03 selection after initial archive support. |
| GHB-WP05 | 3 | agent-logic-runtime | Owner: Runtime/execution. Dependencies: WP02. |
| GHB-WP06 | 3 | agent-logic-runtime | Owner: evaluation lane. Dependencies: WP00 protocol, WP02 contracts, WP03 statistics, WP05 executor. |
| GHB-WP07 | 3 | agent-logic-runtime | Owner: Runtime. Dependencies: WP00 audit; coordinate/reuse Runtime #112 rather than opening a duplicate. |
| GHB-WP08 | 3 | agent-logic-runtime | Owner: Runtime. Dependencies: WP02, WP05, WP06, WP07. |
| GHB-WP09 | 3 | agent-logic-runtime | Owner: Runtime resident integration. Dependencies: WP03–WP08. |
| GHB-WP10 | 3 | agent-logic-runtime | Owner: Runtime algorithm and activation owners. Dependencies: Gate A. |
| GHB-WP11 | 3 | agent-logic-runtime | Owner: evaluation + Runtime. Dependencies: WP10; test design begins during WP06. |
| GHB-WP12 | 3 | agent-logic-runtime | Owner: docs/research, with independent review. Dependencies: WP01 for vocabulary; WP11 for final claims. |
| INTEGRATE | 3 | agent-logic-runtime | One installed polis joins citizen action, upstream delegation, private cognition, policy enforcement, appeal and plugin restart without contract drift. |
| QUALIFY | 3 | agent-logic-runtime | Independent reviewer runs the accepted success/failure/recovery denominator on the exact multi-repository lockset. |
| TAIL-01 | 4 | agent-design-language | Deliver the complete quality gate result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-02 | 4 | agent-design-language | Deliver the complete documentation review and external-review handoff result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-03 | 4 | agent-design-language | Deliver the complete publication finalization result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-04 | 4 | agent-design-language | Deliver the complete internal milestone review result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-05 | 4 | agent-design-language | Deliver the complete external or third-party review result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-06 | 4 | agent-design-language | Deliver the complete accepted-findings remediation or explicit deferral capture result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-07 | 4 | agent-design-language | Deliver the complete next-milestone planning result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-08 | 4 | agent-design-language | Deliver the complete next-milestone closeout planning result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-09 | 4 | agent-design-language | Deliver the complete next-milestone planning review result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |
| TAIL-10 | 4 | agent-design-language | Deliver the complete release ceremony and milestone close result over the qualified product lockset, with exact artifact links and truthful approval/deferral status. |

## Sequencing

[Execution graph](EXECUTION_PLAN_v0.93.3.json), [specifications](WP_EXECUTION_SPECIFICATIONS_v0.93.3.yaml), and [issue wave](WP_ISSUE_WAVE_v0.93.3.yaml) carry exact acceptance and dependencies.

## Acceptance Mapping

Each row maps to the complete acceptance and negative cases in the specifications. Existing issue identities must be reused; new work packages are not yet newly created GitHub issues.

## Exit Criteria

No omitted identity, duplicate routing or unaccepted dependency.
