# v0.93 CodeFriend Launch

## Status

Mandatory CodeFriend Beta 1 launch in v0.93, confirmed by the operator on 2026-09-16; planning under #1047, after qualified v0.92.2 Beta 1 and the opening repository split. This document plans the launch path; it does not authorize deployment, publication or customer-data use. Runtime v4 is mandatory in v0.93 and the installed launch candidate consumes its qualified artifacts.

## Source and precedence

The current product brief is [CodeFriend Beta 1 Plan — Second-pass working draft](https://docs.google.com/document/d/1_CDM8QydW8_Lmue82j5tTHRJpi-hWzOo2u3y6M312b0/edit), inspected in Google Drive on 2026-09-16; Drive reports last edit August 23. It defines engineering maintenance and intelligence, seven implementation workstreams, three demos and external-tester readiness. Its public SaaS boundary requires privacy controls first.

The tracked v0.92.2 [handoff](../../v0.92.2/NEXT_MILESTONE_HANDOFF_v0.92.2.md) prioritizes public/customer-scale deployment in the first milestone after qualified Beta 1. The operator requires CodeFriend Beta 1 to launch in v0.93 alongside completed Runtime v4. Audience, environment and initial capacity remain decisions; omission or silent deferral of the launch track is not the default.

[codefriend.ai issue #1](https://github.com/agent-logic/codefriend.ai/issues/1) owns the existing pre-alpha website launch planning task. It is open as observed on 2026-09-16. It supplies website, infrastructure, verification, rollback and publication inputs; it does not establish working product readiness. Preserve its identity and reconcile it before creating any overlapping website issue.

The older [v1 build plan](../../../planning/codefriend/CODEFRIEND_V1_BUILD_PLAN.md) preserves longer-term architecture cognition, governance and portfolio direction. Its old product/site combination and v0.93 working-alpha schedule do not override the current Beta 1 requirements or the operator's separate software/website decision. Full portfolio dashboards, autonomous application development and optional enterprise connectors are not inferred launch requirements.

## Required baseline mapping

CF-01 records accepted v0.92.2 proof for all seven Drive workstreams: repository engine, specialist reviews, diagrams, targeted test generation, documentation, report/safety gates and minimal product experience. Preserve all three demonstrations: ADL self-review, external OSS review and PR review. Every external-tester criterion in section 10 must map to exact evidence or an explicit current-milestone blocker. Do not rebuild or defer missing Beta 1 behavior as launch polish.

## Ownership and interfaces

The proposed private `codefriend` repository owns software, intake, review/report experience and product operations. Existing private `codefriend.ai` owns website content and deployment. Runtime owns plugin supervision and shared Runtime services. Infrastructure owns shared deployment modules and consumes pinned product artifacts. CF-02 depends on RV-08; no native-only Runtime substitute satisfies this launch candidate.

## Work packages

| ID | Completed result |
|---|---|
| CF-01 | Accepted Beta 1 evidence mapping and bounded launch target |
| CF-02 | Independently deployable product using the qualified Runtime v4 |
| CF-03 | Complete external-tester onboarding and report journey |
| CF-04 | Qualified product operations, cost controls and recovery |
| CF-05 | Independent launch-candidate rehearsal and quality acceptance |
| CF-06 | Website preview and handoff aligned to qualified product |
| CF-07 | Authorized live launch, tester admission, verification and retained rollback |

The canonical execution graph contains dependencies and negative cases. Website publication and customer admission require explicit authorization against the qualified candidate; a preview or a readiness review cannot claim those actions occurred.

## Acceptance Criteria

A non-ADL tester completes authorized repository intake through evidence-backed reports, diagrams, meaningful generated tests and documentation outputs. Scope, omissions, privacy, human approval and residual risk remain visible. Failure, cancellation, restart, rollback and data deletion are exercised in the intended deployment class. Record actual scenario counts and exact software, Runtime and website versions. Planning/schema checks and the coming-soon site are insufficient proof.

## Open decisions

CF-01 must resolve initial audience (invited testers versus broader access), deployment environment, capacity/cost limits, support and feedback owner, and permitted repository/data classes. These choices size the launch; they do not remove its workstream. No paid cloud, customer-data processing or public announcement is authorized by #1047.

## Launch execution boundary

CF-07 is future execution work. It requires explicit authorization of candidate, audience, environment and budget, then performs product/site publication, tester admission and live journey verification with rollback. If launch authorization has not been granted, launch remains unexecuted and v0.93 cannot close; CF-06 readiness cannot substitute for launch. #1047 does not perform these actions.

## Metadata

Template: feature_doc 1.1.0. Milestone: v0.93. Authoring issue: #1047. Status: first-pass planning; implementation and release acceptance not claimed.

## Template Rules

This document preserves its source requirements. The canonical execution graph supplies current scheduling and repository placement; generated sections and passing structure checks do not establish design approval or product proof.

## Purpose

Deliver the complete CodeFriend launch journey from accepted Beta 1 through authorized live verification.

## Context

Implementation follows accepted v0.92.2 closure and the opening repository split through RD-11. Existing Beta 1 behavior remains a predecessor obligation. The source inputs above retain design context; newer operator decisions in the milestone decision register govern scheduling.

## Coverage / Ownership

Execution owners: codefriend, codefriend.ai. Candidate results: CF-01, CF-02, CF-03, CF-04, CF-05, CF-06, CF-07. Named people and exact repository identifiers are settled before execution. Shared contracts have one producer and versioned consumers; source is not duplicated between owners.

## Overview

- **CF-01:** Map every Drive Beta 1 workstream, demo and external-tester criterion to accepted v0.92.2 evidence, then accept the launch audience, deployment mode, limits and remaining operational gaps. Missing Beta 1 behavior remains a predecessor blocker.
- **CF-02:** A clean target environment installs pinned CodeFriend and Runtime artifacts and completes a real repository review without a development checkout; deployment config identifies the approved environment and resource limits.
- **CF-03:** A non-ADL tester can connect an authorized local or GitHub repository, select scope, run a review and inspect/export its evidence packet with explicit privacy and publication state.
- **CF-04:** The deployed product restores a known-good release and retained project state after interruption, with bounded job cost, cancellation, deletion and operator diagnostics demonstrated.
- **CF-05:** An independent external-tester rehearsal repeats ADL, external OSS and PR-review journeys and evaluates evidence, diagrams, meaningful generated tests, documentation, redaction and report quality on exact installed versions.
- **CF-06:** The existing website launch plan and product entry point agree with qualified capabilities, support/feedback instructions, access limits and rollback; an approved preview is ready for separately authorized publication.
- **CF-07:** After explicit authorization of the exact candidate, audience, environment and budget, publish the approved product/site entry points, admit the authorized testers, verify the live end-to-end journey and retain a tested rollback. Until launch authorization is granted and live launch is verified, this result remains incomplete and v0.93 cannot close.

## Design

The source scope and invariants above define the feature contract. Implement them in the owning product with explicit actor identity, authorized inputs, persisted evidence and a consumer-visible outcome. Denials retain reasons without disclosing protected state. The result-specific behavior is enumerated under Overview and Acceptance Criteria; any new design choice is recorded before implementation.

## Execution Flow

- CF-01 follows RD-11.
- CF-02 follows CF-01, RV-08.
- CF-03 follows CF-02.
- CF-04 follows CF-03.
- CF-05 follows CF-04 and CT-05.
- CF-06 follows CF-05.
- CF-07 follows CF-06.

Follow the canonical dependency graph rather than document order. A candidate closes only when its complete consumer result and its negative cases are accepted.

## Determinism and Constraints

Pin source, installed artifacts, policy, configuration and fixtures. Preserve provenance and explicit failure/retry state across runs. Probabilistic model outputs require invariant and quality checks, not invented byte-for-byte determinism. No implicit authority, hidden private-state projection or schema-only completion.

## Integration Points

Consume the qualified product lockset and versioned contracts selected during migration. Runtime retains enforcement and shared Runtime services; enterprise providers may narrow authority; CodeFriend consumes product interfaces; public ADL stays independently usable. This feature adds no sibling-checkout dependency.

## Validation

Run the installed behavior described in Overview/Acceptance Criteria and these required negative cases:

- CF-01: Unproved Beta behavior deferred; Website readiness substituted for product readiness.
- CF-02: Private sibling dependency; Missing configuration silently defaulted; Incompatible Runtime accepted.
- CF-03: Unauthorized repository access; Unsafe source path; Unapproved export; Hidden skipped surface.
- CF-04: Cross-project disclosure; Cancellation leaves billable work running; Backup cannot restore; Deletion leaves accessible data.
- CF-05: Weak review passes quality gate; Unsupported diagram; Test unrelated to finding; Private material in report.
- CF-06: Coming-soon claims presented as live product; Broken product entry point; Unsupported availability claim.
- CF-07: Missing launch authorization; Live product differs from qualified artifact; Onboarding fails after publication; Rollback cannot restore access.

Record exact versions, scenario counts, redacted evidence and skipped checks. QUALITY_GATE_v0.93.md defines release evidence; planning validation does not prove these behaviors.

## Risks

Primary failure risks are the negative cases under Validation. Cross-repository contract drift and overbroad task execution must be resolved before dependent acceptance. A successful fixture cannot establish an untested deployment class or customer/publication permission.

## Future Work

Only explicitly deferred source requirements belong to later work. Do not silently move a required v0.93 outcome into a successor; record a reviewed operator disposition for scope changes.

## Notes

See [execution specifications](../WP_EXECUTION_SPECIFICATIONS_v0.93.yaml), [decision register](../DECISIONS_v0.93.md) and [quality gate](../QUALITY_GATE_v0.93.md). First-pass implementation candidates are not yet created execution issues.

## Current Candidate Mapping

CF-01, CF-02, CF-03, CF-04, CF-05, CF-06, CF-07; owner repositories: codefriend, codefriend.ai. The current execution graph supersedes older sequencing or placement in retained source text.


## Branded output launch dependency

CT-01–CT-10 deliver versioned artifact templates and styles. CF-05 consumes CT-05 catalog-wide qualification before CF-07; no optional post-launch deferral.

## Deferred Sprint 10 qualification

The operator authorized v0.92.2 Sprint10 closeout with #915 qualification explicitly incomplete/deferred. This does not qualify Beta1 for launch. CF-01 must map the three single-task successor obligations to exact issues once the current #915 owner creates/reuses them: source-grounded citation correctness, unknown second-run reconciliation/recovery, and the remaining independent12-cell24-obligation qualification. Their numeric mapping is pending; do not create duplicates or treat them as already complete. CF-05 must consume their accepted results before CF-07 launch. Existing six-format export observations are retained partial proof, not a passing qualification. The eight-sprint/83-core-candidate allocation remains unchanged pending exact cross-repository successor mapping; report these inherited obligations separately.
