# v0.93.3 Demo Matrix

## Metadata

Planning template set: 1.1.0. Target: v0.93.3. Setup issue: ADL #1296. Owner of planning: Planning #12. Product execution owners remain unassigned.

## Status

Planned successor only. No opening or execution is authorized. No deployment, provider spend, release or Runtime restart is authorized by these documents.

## Purpose

Exercise complete consumer outcomes rather than scaffolding.

## Runtime Preconditions

Select exact source/artifact versions and a supported deployment class. Obtain scoped permission before paid or live operations.

## Demo Coverage Summary

| Package | Acceptance evidence | Required adverse cases |
| --- | --- | --- |
| WP-01 | Open the separately reviewed Runtime v4, governance and GHB milestone after explicit authorization. | Unaccepted predecessor; Missing opening approval; Duplicate issue identities |
| RV-01 | An accepted plugin contract defines topology precedence, epochs, durable activation journal, in-flight delivery, cancellation, versioned state/config migration, two-phase reconfiguration, dependency-safe removal and recovery classes. | Manifest self-grants; Undefined crash recovery |
| RV-02 | Existing Runtime behavior and identity survive native adapter conversion with no second scheduler. | Behavior parity regression |
| RV-03 | The durable generation transaction stages, quiesces, activates and rolls back to one fenced generation after interruption; in-flight work and cancellation retain their declared disposition. | Interrupted activation loses owner; Stale generation accepts work; Rollback duplicates in-flight delivery |
| RV-09 | An installed plugin migrates declared prior-version state to the selected version and preserves the prior generation when the migrator refuses or fails. | Unsupported state version; Migrator failure mutates prior state; Retry repeats non-idempotent migration |
| RV-10 | Versioned configuration migration and two-phase validation/activation apply one accepted configuration, or preserve the prior version and active configuration on refusal, failure or interruption. | Invalid configuration activated; Interrupted apply loses known-good configuration; Credential scope changes silently; Unsupported configuration version; Configuration migrator failure loses prior version |
| RV-11 | A requested plugin removal checks dependencies and in-flight work, refuses unsafe removal, and completes authorized removal with durable recovery state. | Live dependent orphaned; Unsafe removal accepted; Interrupted removal resurrects stale generation |
| RV-04 | One real non-core plugin uses authenticated bounded transport, deadlines and process supervision. | Orphan process; Hung handler; Unbounded queue |
| RV-05 | The canonical contract drives a default-deny WIT adapter with enforced memory, time and host-call limits. | Unapproved import; Timeout bypass |
| RV-06 | One module has equivalent CSM-visible identity, messages, failure and state behavior across native, process and WASM adapters. | Adapter-specific authority; State identity drift |
| RV-07 | Provider instances consume host-owned credentials and versioned configuration through the common inference capability. | Credential in manifest; Implicit provider fallback |
| RV-08 | Installed package signing, checkpoint compatibility, generation recovery and Observatory operations pass with a retained known-good rollback. | Bad signature; Incompatible checkpoint; Rollback cannot restore |
| GOV-01 | Runtime admits citizen conduct only through bound identity, Freedom Gate and signed trace; guest/operator action remains distinct. | Guest impersonates citizen; Instinct bypasses governance; Missing or inconsistent temporal anchor |
| GOV-02 | A Runtime policy evaluator returns rights/duties decisions with trace evidence for a bounded action. | Evidence-free verdict |
| GOV-03 | Standing maintenance, restriction, restoration and revocation consume existing state primitives and retain reasons and challenge context. | Unexplained restriction; Restoration loses history |
| GOV-04 | A review consumer emits findings over trace/outcome/policy evidence without raw private-state access. | Fabricated evidence; Raw state disclosure |
| GOV-05 | A challenged finding progresses through appeal and recorded disposition without destroying original evidence. | Replay changes verdict; Evidence lost on appeal |
| GOV-06 | Signed evidence-backed model updates replay under standing, consent/scope and projection controls. | Unsigned update; Guest scope exceeded |
| GOV-07 | Conflicting and stale entries change a bounded consumer decision through explicit uncertainty/decay events. | Conflict overwritten; Stale confidence unchanged |
| GOV-08 | Relationship records preserve evidence and permitted scope across restart without becoming standing verdicts. | Untraceable relationship assertion |
| GOV-09 | Authorized redacted projections can inform review while retaining challenge and source lineage. | Raw ToM leak; Scalar moral verdict |
| GOV-10 | Two governed consumers share evidence projections with preserved uncertainty, access rules and freshness. | Private model copied; Stale projection trusted |
| GOV-11 | Delegated action binds identity, standing, resource, capability and policy without exceeding the delegator grant. | Overbroad delegation; Revoked grant; Remote replay duplicates effect; Remote refusal ignored |
| GOV-12 | A selected/rejected upstream call retains citizen identity, policy, provenance and verification; ToM may inform selection but cannot override authority. | Provider becomes sovereign; Unverified answer accepted |
| GOV-13 | A governed exchange succeeds while an associated unauthorized private-state/ToM inspection is denied. | Message receipt grants inspection |
| GOV-14 | A policy consumer evaluates explicit polis/citizen obligations and emits reviewable evidence. | Policy text treated as executed behavior; Expired commitment treated as current; Missed deadline silently satisfied; Causally invalid commitment accepted |
| GOV-15 | Guild identity, membership, scoped resources and delegated actions persist with trace and challenge support. | Revoked membership acts; Guild exceeds citizen authority |
| GOV-16 | The operator receives evidence-linked standing, appeals and social-memory summaries without private-state leaks or scalar moral ranking. | Hidden uncertainty; Protected data in report |
| CM-01 | A reviewed contract distinguishes same-identity migration and recovery from new-identity descendants, replication and independent clones; it defines authority, lineage, consent, state inheritance, fencing and recovery outcomes. | Copy treated as same active identity; Implicit grant inheritance; Ambiguous ownership during transfer |
| CM-02 | Two owned Runtime endpoints transfer one authorized citizen checkpoint and scoped continuity evidence; the source is fenced before destination activation and interruption can recover without two active holders or lost lineage. | Stale or replayed transfer; Unauthorized destination; Interrupted handover; Source continues after destination activation; Wrong key or corrupt checkpoint |
| CM-03 | An authorized parent creates one distinctly identified descendant with explicit lineage, selected inherited state and newly scoped authority while preserving the parent and protecting private state. | Unauthorized cloning; Duplicate descendant on retry; Private state inherited without scope; Parent standing or credentials copied as authority |
| CM-04 | Independent installed scenarios exercise migration and descendant creation across restart, interruption and retry, verify lineage/privacy/fencing and expose the resulting identity and custody status to the operator. | Split-brain identity; Orphaned in-transit custody; Silent lineage loss; Retry creates extra descendant |
| DEMO-GOV | Execute D1-D8 and guild D13 through installed Runtime consumers with accepted and refused paths. | Demo replaces production path |
| GHB-SPRINT | Open and coordinate the approved GHB program without granting execution, spend, or completion authority. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP00 | Owner: program + Runtime. Dependencies: none. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP01 | Owner: ADL. Dependencies: WP00. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP02 | Owner: ADL. Dependencies: WP01. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP03 | Owner: Runtime algorithm lane; exact module selected by WP00. Dependencies: WP02. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP04 | Owner: Runtime algorithm lane. Dependencies: WP02; integrate WP03 selection after initial archive support. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP05 | Owner: Runtime/execution. Dependencies: WP02. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP06 | Owner: evaluation lane. Dependencies: WP00 protocol, WP02 contracts, WP03 statistics, WP05 executor. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP07 | Owner: Runtime. Dependencies: WP00 audit; coordinate/reuse Runtime #112 rather than opening a duplicate. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP08 | Owner: Runtime. Dependencies: WP02, WP05, WP06, WP07. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP09 | Owner: Runtime resident integration. Dependencies: WP03–WP08. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP10 | Owner: Runtime algorithm and activation owners. Dependencies: Gate A. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP11 | Owner: evaluation + Runtime. Dependencies: WP10; test design begins during WP06. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| GHB-WP12 | Owner: docs/research, with independent review. Dependencies: WP01 for vocabulary; WP11 for final claims. | Planning status treated as execution authority; Duplicate owner or issue identity; Fixture evidence substituted for installed behavior |
| INTEGRATE | One installed polis joins citizen action, upstream delegation, private cognition, policy enforcement, appeal and plugin restart without contract drift. | Authority lost across product boundary |
| QUALIFY | Independent reviewer runs the accepted success/failure/recovery denominator on the exact multi-repository lockset. | Wrong artifact head; Missing negative scenario |

## Coverage Rules

Record scenario counts, positive/negative/recovery outcomes, failures and skipped cases. Do not call a planned demonstration executed.

## Exit Criteria

Every mandatory scenario is accepted or has an explicit operator disposition before release.
