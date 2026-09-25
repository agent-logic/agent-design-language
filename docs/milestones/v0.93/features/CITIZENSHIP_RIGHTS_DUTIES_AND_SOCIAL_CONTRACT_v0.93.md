> **Superseded combined plan.** The operator-approved split is planned under #922 in [v0.93.1](../../v0.93.1/README.md) (repo split, templates and CodeFriend Beta 1 launch) and [v0.93.2](../../v0.93.2/README.md) (remaining platform work). The original combined scope below is retained for traceability; it does not require Runtime v4 before Beta 1 launch or authorize execution.

# v0.93 Feature: Citizenship, Rights, Duties, and Social Contract

## Status

Forward-planning feature contract for `v0.93`.

## Purpose

Define the governance-facing citizenship layer for the ADL polis:

- constitutional citizenship
- rights and duties
- social contract
- constitutional delegation

## Source Inputs

- `docs/milestones/v0.93/README.md`
- `docs/milestones/v0.93/CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md`
- `docs/milestones/v0.93/features/README.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Scope

This feature should establish:

- eligibility and actor-boundary rules for citizen participation
- explicit rights/duties semantics grounded in trace and policy
- a bounded social-contract representation
- delegation rules that remain reviewable and fail closed

## Non-goals

- legal personhood
- production constitutional sovereignty
- collapsing guest/operator action into citizen action without mediation

## Completion Target

`v0.93`

## Metadata

Template: feature_doc 1.1.0. Milestone: v0.93. Authoring issue: #1047. Status: first-pass planning; implementation and release acceptance not claimed.

## Template Rules

This document preserves its source requirements. The canonical execution graph supplies current scheduling and repository placement; generated sections and passing structure checks do not establish design approval or product proof.

## Context

Implementation follows accepted v0.92.2 closure and the opening repository split through RD-11. Existing Beta 1 behavior remains a predecessor obligation. The source inputs above retain design context; newer operator decisions in the milestone decision register govern scheduling.

## Coverage / Ownership

Execution owners: agent-logic-runtime. Candidate results: GOV-01, GOV-02, GOV-04, GOV-14, GOV-16. Named people and exact repository identifiers are settled before execution. Shared contracts have one producer and versioned consumers; source is not duplicated between owners.

## Overview

- **GOV-01:** Runtime admits citizen conduct only through bound identity, Freedom Gate and signed trace; guest/operator action remains distinct.
- **GOV-02:** A Runtime policy evaluator returns rights/duties decisions with trace evidence for a bounded action.
- **GOV-04:** A review consumer emits findings over trace/outcome/policy evidence without raw private-state access.
- **GOV-14:** A policy consumer evaluates explicit polis/citizen obligations and emits reviewable evidence.
- **GOV-16:** The operator receives evidence-linked standing, appeals and social-memory summaries without private-state leaks or scalar moral ranking.

## Design

The source scope and invariants above define the feature contract. Implement them in the owning product with explicit actor identity, authorized inputs, persisted evidence and a consumer-visible outcome. Denials retain reasons without disclosing protected state. The result-specific behavior is enumerated under Overview and Acceptance Criteria; any new design choice is recorded before implementation.

## Execution Flow

- GOV-01 follows RV-08.
- GOV-02 follows GOV-01.
- GOV-04 follows GOV-03.
- GOV-14 follows GOV-05, GOV-10, GOV-13.
- GOV-16 follows GOV-15.

Follow the canonical dependency graph rather than document order. A candidate closes only when its complete consumer result and its negative cases are accepted.

## Determinism and Constraints

Pin source, installed artifacts, policy, configuration and fixtures. Preserve provenance and explicit failure/retry state across runs. Probabilistic model outputs require invariant and quality checks, not invented byte-for-byte determinism. No implicit authority, hidden private-state projection or schema-only completion.

## Integration Points

Consume the qualified product lockset and versioned contracts selected during migration. Runtime retains enforcement and shared Runtime services; enterprise providers may narrow authority; CodeFriend consumes product interfaces; public ADL stays independently usable. This feature adds no sibling-checkout dependency.

## Validation

Run the installed behavior described in Overview/Acceptance Criteria and these required negative cases:

- GOV-01: Guest impersonates citizen.
- GOV-02: Evidence-free verdict.
- GOV-04: Fabricated evidence; Raw state disclosure.
- GOV-14: Policy text treated as executed behavior.
- GOV-16: Hidden uncertainty; Protected data in report.

Record exact versions, scenario counts, redacted evidence and skipped checks. QUALITY_GATE_v0.93.md defines release evidence; planning validation does not prove these behaviors.

## Acceptance Criteria

- **GOV-01:** Runtime admits citizen conduct only through bound identity, Freedom Gate and signed trace; guest/operator action remains distinct.
- **GOV-02:** A Runtime policy evaluator returns rights/duties decisions with trace evidence for a bounded action.
- **GOV-04:** A review consumer emits findings over trace/outcome/policy evidence without raw private-state access.
- **GOV-14:** A policy consumer evaluates explicit polis/citizen obligations and emits reviewable evidence.
- **GOV-16:** The operator receives evidence-linked standing, appeals and social-memory summaries without private-state leaks or scalar moral ranking.

All declared negative cases must reject or recover as specified, with reviewable evidence and no unsupported completion claim.

## Risks

Primary failure risks are the negative cases under Validation. Cross-repository contract drift and overbroad task execution must be resolved before dependent acceptance. A successful fixture cannot establish an untested deployment class or customer/publication permission.

## Future Work

Only explicitly deferred source requirements belong to later work. Do not silently move a required v0.93 outcome into a successor; record a reviewed operator disposition for scope changes.

## Notes

See [execution specifications](../WP_EXECUTION_SPECIFICATIONS_v0.93.yaml), [decision register](../DECISIONS_v0.93.md) and [quality gate](../QUALITY_GATE_v0.93.md). First-pass implementation candidates are not yet created execution issues.

## Current Candidate Mapping

GOV-01, GOV-02, GOV-04, GOV-14, GOV-16; owner repositories: agent-logic-runtime. The current execution graph supersedes older sequencing or placement in retained source text.


## Inherited feature acceptance — #922 / GOV-01

Installed cognitive-loop and instinct-originated actions traverse bound identity, Freedom Gate and temporal anchoring; guest or stale identity cannot acquire citizen authority.

Required negative scenarios: Instinct bypasses governance; Missing or inconsistent temporal anchor.


## Inherited feature acceptance — #922 / GOV-14

The installed obligations consumer evaluates accepted, expired and missed commitments and rejects causally invalid ordering using explicit temporal evidence and reviewable policy disposition.

Required negative scenarios: Expired commitment treated as current; Missed deadline silently satisfied; Causally invalid commitment accepted.
