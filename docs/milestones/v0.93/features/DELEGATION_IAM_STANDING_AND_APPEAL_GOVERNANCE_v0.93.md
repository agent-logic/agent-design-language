> **Superseded combined plan.** The operator-approved split is planned under #922 in [v0.93.1](../../v0.93.1/README.md) (repo split, templates and CodeFriend Beta 1 launch) and [v0.93.2](../../v0.93.2/README.md) (remaining platform work). The original combined scope below is retained for traceability; it does not require Runtime v4 before Beta 1 launch or authorize execution.

# v0.93 Feature: Delegation, Upstream Delegation, IAM, Standing Transition, and Appeal Governance

## Status

Forward-planning feature contract for `v0.93`.

## Purpose

Define the governance-facing authority chain for the ADL polis: who may
delegate, under what identity and standing, how authority is enforced, and how
standing/challenge/appeal surfaces remain evidence-based and fail closed.

## Source Inputs

- `docs/milestones/v0.93/CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.md`
- `docs/milestones/v0.93/README.md`
- `docs/milestones/v0.93/features/CITIZENSHIP_RIGHTS_DUTIES_AND_SOCIAL_CONTRACT_v0.93.md`
- `docs/milestones/v0.93/features/SOCIAL_RELATIONSHIP_REPUTATION_AND_SHARED_MEMORY_v0.93.md`
- `docs/milestones/v0.93/WBS_v0.93.md`
- `docs/planning/ADL_FEATURE_LIST.md`
- `.adl/docs/TBD/ADL_AGENT_UPSTREAM_DELEGATION.md`

## Scope

This feature should establish:

- delegation, upstream delegation, and IAM as trace-backed authority surfaces
- upstream delegation as governed cognitive escalation across local citizens,
  polis services, trusted external polis boundaries, and frontier providers
- standing maintenance, degradation, restoration, and revocation semantics
- challenge and appeal governance tied to evidence preservation
- communication without implicit inspection or hidden authority escalation
- failure-closed posture for missing authority, ambiguous identity, or policy
  conflict

## Non-goals

- replacing `v0.90.3` standing/access/state primitives
- collapsing delegation into unrestricted tool or citizen authority
- treating upstream providers as sovereign actors outside the delegating
  citizen's identity, policy, trace, and verification boundaries
- hidden operator overrides outside trace and review

## Completion Target

`v0.93`

## Metadata

Template: feature_doc 1.1.0. Milestone: v0.93. Authoring issue: #1047. Status: first-pass planning; implementation and release acceptance not claimed.

## Template Rules

This document preserves its source requirements. The canonical execution graph supplies current scheduling and repository placement; generated sections and passing structure checks do not establish design approval or product proof.

## Context

Implementation follows accepted v0.92.2 closure and the opening repository split through RD-11. Existing Beta 1 behavior remains a predecessor obligation. The source inputs above retain design context; newer operator decisions in the milestone decision register govern scheduling.

## Coverage / Ownership

Execution owners: agent-logic-runtime. Candidate results: GOV-03, GOV-05, GOV-11, GOV-12, GOV-13. Named people and exact repository identifiers are settled before execution. Shared contracts have one producer and versioned consumers; source is not duplicated between owners.

## Overview

- **GOV-03:** Standing maintenance, restriction, restoration and revocation consume existing state primitives and retain reasons and challenge context.
- **GOV-05:** A challenged finding progresses through appeal and recorded disposition without destroying original evidence.
- **GOV-11:** Delegated action binds identity, standing, resource, capability and policy without exceeding the delegator grant.
- **GOV-12:** A selected/rejected upstream call retains citizen identity, policy, provenance and verification; ToM may inform selection but cannot override authority.
- **GOV-13:** A governed exchange succeeds while an associated unauthorized private-state/ToM inspection is denied.

## Design

The source scope and invariants above define the feature contract. Implement them in the owning product with explicit actor identity, authorized inputs, persisted evidence and a consumer-visible outcome. Denials retain reasons without disclosing protected state. The result-specific behavior is enumerated under Overview and Acceptance Criteria; any new design choice is recorded before implementation.

## Execution Flow

- GOV-03 follows GOV-02.
- GOV-05 follows GOV-04.
- GOV-11 follows GOV-03.
- GOV-12 follows GOV-11, GOV-07.
- GOV-13 follows GOV-11.

Follow the canonical dependency graph rather than document order. A candidate closes only when its complete consumer result and its negative cases are accepted.

## Determinism and Constraints

Pin source, installed artifacts, policy, configuration and fixtures. Preserve provenance and explicit failure/retry state across runs. Probabilistic model outputs require invariant and quality checks, not invented byte-for-byte determinism. No implicit authority, hidden private-state projection or schema-only completion.

## Integration Points

Consume the qualified product lockset and versioned contracts selected during migration. Runtime retains enforcement and shared Runtime services; enterprise providers may narrow authority; CodeFriend consumes product interfaces; public ADL stays independently usable. This feature adds no sibling-checkout dependency.

## Validation

Run the installed behavior described in Overview/Acceptance Criteria and these required negative cases:

- GOV-03: Unexplained restriction; Restoration loses history.
- GOV-05: Replay changes verdict; Evidence lost on appeal.
- GOV-11: Overbroad delegation; Revoked grant.
- GOV-12: Provider becomes sovereign; Unverified answer accepted.
- GOV-13: Message receipt grants inspection.

Record exact versions, scenario counts, redacted evidence and skipped checks. QUALITY_GATE_v0.93.md defines release evidence; planning validation does not prove these behaviors.

## Acceptance Criteria

- **GOV-03:** Standing maintenance, restriction, restoration and revocation consume existing state primitives and retain reasons and challenge context.
- **GOV-05:** A challenged finding progresses through appeal and recorded disposition without destroying original evidence.
- **GOV-11:** Delegated action binds identity, standing, resource, capability and policy without exceeding the delegator grant.
- **GOV-12:** A selected/rejected upstream call retains citizen identity, policy, provenance and verification; ToM may inform selection but cannot override authority.
- **GOV-13:** A governed exchange succeeds while an associated unauthorized private-state/ToM inspection is denied.

All declared negative cases must reject or recover as specified, with reviewable evidence and no unsupported completion claim.

## Risks

Primary failure risks are the negative cases under Validation. Cross-repository contract drift and overbroad task execution must be resolved before dependent acceptance. A successful fixture cannot establish an untested deployment class or customer/publication permission.

## Future Work

Only explicitly deferred source requirements belong to later work. Do not silently move a required v0.93 outcome into a successor; record a reviewed operator disposition for scope changes.

## Notes

See [execution specifications](../WP_EXECUTION_SPECIFICATIONS_v0.93.yaml), [decision register](../DECISIONS_v0.93.md) and [quality gate](../QUALITY_GATE_v0.93.md). First-pass implementation candidates are not yet created execution issues.

## Current Candidate Mapping

GOV-03, GOV-05, GOV-11, GOV-12, GOV-13; owner repositories: agent-logic-runtime. The current execution graph supersedes older sequencing or placement in retained source text.


## Inherited feature acceptance — #922 / GOV-11

A bounded remote action accepts a current scoped delegation and rejects refusal, revocation and replay without applying the action twice.

Required negative scenarios: Remote replay duplicates effect; Remote refusal ignored.
