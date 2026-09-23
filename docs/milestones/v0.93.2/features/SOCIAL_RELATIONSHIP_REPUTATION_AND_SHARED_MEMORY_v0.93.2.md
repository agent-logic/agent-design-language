# v0.93.2 Feature: Social Relationship, Reputation, and Shared Memory

## Status

Forward-planning feature contract for `v0.93.2`.

## Purpose

Define the bounded social-cognition surfaces that sit between private Theory of
Mind and public governance:

- relationship model
- reputation and trust
- shared social memory

These surfaces are distinct from private ToM, standing, and constitutional
verdicts.

## Source Inputs

- `docs/milestones/v0.93.2/README.md`
- `docs/milestones/v0.93.2/CONSTITUTIONAL_CITIZENSHIP_AND_POLIS_GOVERNANCE_PLAN_v0.93.2.md`
- `docs/milestones/v0.93.2/features/THEORY_OF_MIND_AND_SOCIAL_COGNITION_v0.93.2.md`
- `docs/milestones/v0.93.2/features/README.md`
- `docs/planning/ADL_FEATURE_LIST.md`

## Scope

This feature should establish:

- relationship records as durable social context rather than loose narrative
- redacted, challengeable reputation projections
- shared social memory as a governance-facing summary distinct from private ToM
- explicit boundaries among private inference, remembered interaction, public
  projection, and standing action

## Non-goals

- public exposure of private ToM
- scalar moral verdicts
- irreversible reputation labels
- treating relationship state as constitutional judgment by itself

## Completion Target

`v0.93.2`

## Metadata

Template: feature_doc 1.1.0. Milestone: v0.93.2. Authoring issue: #1047. Status: first-pass planning; implementation and release acceptance not claimed.

## Template Rules

This document preserves its source requirements. The canonical execution graph supplies current scheduling and repository placement; generated sections and passing structure checks do not establish design approval or product proof.

## Context

Implementation follows the accepted v0.93.1 handoff and repository lockset. Preserve launched Beta 1 behavior through compatible upgrade and rollback; Runtime v4 is not a retroactive launch prerequisite. The source inputs above retain design context; newer operator decisions in the milestone decision register govern scheduling.

## Coverage / Ownership

Execution owners: agent-logic-runtime. Candidate results: GOV-08, GOV-09, GOV-10. Named people and exact repository identifiers are settled before execution. Shared contracts have one producer and versioned consumers; source is not duplicated between owners.

## Overview

- **GOV-08:** Relationship records preserve evidence and permitted scope across restart without becoming standing verdicts.
- **GOV-09:** Authorized redacted projections can inform review while retaining challenge and source lineage.
- **GOV-10:** Two governed consumers share evidence projections with preserved uncertainty, access rules and freshness.

## Design

The source scope and invariants above define the feature contract. Implement them in the owning product with explicit actor identity, authorized inputs, persisted evidence and a consumer-visible outcome. Denials retain reasons without disclosing protected state. The result-specific behavior is enumerated under Overview and Acceptance Criteria; any new design choice is recorded before implementation.

## Execution Flow

- GOV-08 follows GOV-06.
- GOV-09 follows GOV-05, GOV-07, GOV-08.
- GOV-10 follows GOV-09.

Follow the canonical dependency graph rather than document order. A candidate closes only when its complete consumer result and its negative cases are accepted.

## Determinism and Constraints

Pin source, installed artifacts, policy, configuration and fixtures. Preserve provenance and explicit failure/retry state across runs. Probabilistic model outputs require invariant and quality checks, not invented byte-for-byte determinism. No implicit authority, hidden private-state projection or schema-only completion.

## Integration Points

Consume the qualified product lockset and versioned contracts selected during migration. Runtime retains enforcement and shared Runtime services; enterprise providers may narrow authority; CodeFriend consumes product interfaces; public ADL stays independently usable. This feature adds no sibling-checkout dependency.

## Validation

Run the installed behavior described in Overview/Acceptance Criteria and these required negative cases:

- GOV-08: Untraceable relationship assertion.
- GOV-09: Raw ToM leak; Scalar moral verdict.
- GOV-10: Private model copied; Stale projection trusted.

Record exact versions, scenario counts, redacted evidence and skipped checks. QUALITY_GATE_v0.93.2.md defines release evidence; planning validation does not prove these behaviors.

## Acceptance Criteria

- **GOV-08:** Relationship records preserve evidence and permitted scope across restart without becoming standing verdicts.
- **GOV-09:** Authorized redacted projections can inform review while retaining challenge and source lineage.
- **GOV-10:** Two governed consumers share evidence projections with preserved uncertainty, access rules and freshness.

All declared negative cases must reject or recover as specified, with reviewable evidence and no unsupported completion claim.

## Risks

Primary failure risks are the negative cases under Validation. Cross-repository contract drift and overbroad task execution must be resolved before dependent acceptance. A successful fixture cannot establish an untested deployment class or customer/publication permission.

## Future Work

Only explicitly deferred source requirements belong to later work. Do not silently move a required v0.93.2 outcome into a successor; record a reviewed operator disposition for scope changes.

## Notes

See [execution specifications](../WP_EXECUTION_SPECIFICATIONS_v0.93.2.yaml), [decision register](../DECISIONS_v0.93.2.md) and [quality gate](../QUALITY_GATE_v0.93.2.md). First-pass implementation candidates are not yet created execution issues.

## Current Candidate Mapping

GOV-08, GOV-09, GOV-10; owner repositories: agent-logic-runtime. The current execution graph supersedes older sequencing or placement in retained source text.

## Approved split routing

This planned feature is owned by v0.93.2 under #922. Consume accepted v0.93.1/TAIL-10 and the pinned split lockset; do not repeat repository extraction. Numeric implementation issues and named owners remain pending. No execution, live-provider or release approval is inferred.
