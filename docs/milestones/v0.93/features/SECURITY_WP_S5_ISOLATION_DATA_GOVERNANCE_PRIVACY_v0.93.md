# Security WP-S5: Isolation, Data Governance, And Privacy v0.93

## Metadata

- Feature Name: Isolation, data governance, and privacy
- Milestone Target: v0.93
- Status: planned
- Doc Role: supporting enterprise-security feature contract
- Feature Types: architecture, policy, artifact
- Proof Modes: tests, fixtures, schema, demo, review

## Purpose

Define tenant, polis, citizen, tool, service, private-state, ToM, reputation,
and memory data boundaries so ADL can prove isolation, classification,
retention, deletion, projection, and redaction behavior.

## Dependencies

- WP-S1 zero-trust architecture.
- WP-S3 cryptographic trust.
- v0.90.3 private state, redacted projections, access control, sanctuary, and
  quarantine.
- v0.92 memory grounding, identity, and continuity.
- v0.93 ToM/reputation/shared-social-memory boundary.

## Required Work Products

- Data classification model for private state, ToM, reputation, shared social
  memory, audit records, incidents, tool outputs, and public projections.
- Isolation contract for tenant, polis, citizen, service, tool, and reviewer
  views.
- Retention, deletion, redaction, and projection fixtures.
- Leakage negative cases for unauthorized cross-boundary access.

## Invariants

- Private state is not public evidence.
- Private ToM is not public reputation.
- Retention and deletion decisions are traceable.
- Projection must preserve redaction class and authority context.
- Cross-polis, cross-tenant, and cross-citizen leakage fails closed.

## Demo Candidate

Show an unauthorized cross-boundary data access attempt. The system should deny
or redact based on classification, boundary, retention, and projection rules,
then emit a reviewable denial proof.

## Acceptance Criteria

- Every protected data class has an owner, allowed projections, and redaction
  policy.
- Leakage negative cases exist for private state, private ToM, reputation,
  memory, audit, and incident evidence.
- Reviewer packets can inspect decisions without raw protected data.
- Isolation composes with zero-trust and policy-enforcement decisions.

## Non-Goals

- No universal private-state browser.
- No deletion or retention legal-compliance claim.
- No cross-polis federation claim.

## Template Rules

This document preserves its source requirements. The canonical execution graph supplies current scheduling and repository placement; generated sections and passing structure checks do not establish design approval or product proof.

## Context

Implementation follows accepted v0.92.2 closure and the opening repository split through RD-11. Existing Beta 1 behavior remains a predecessor obligation. The source inputs above retain design context; newer operator decisions in the milestone decision register govern scheduling.

## Coverage / Ownership

Execution owners: agent-logic-enterprise-security. Candidate results: WP-S5. Named people and exact repository identifiers are settled before execution. Shared contracts have one producer and versioned consumers; source is not duplicated between owners.

## Overview

- **WP-S5:** Access, retention, deletion and projection operations enforce tenant/polis/citizen ownership and privacy.

## Design

The source scope and invariants above define the feature contract. Implement them in the owning product with explicit actor identity, authorized inputs, persisted evidence and a consumer-visible outcome. Denials retain reasons without disclosing protected state. The result-specific behavior is enumerated under Overview and Acceptance Criteria; any new design choice is recorded before implementation.

## Execution Flow

- WP-S5 follows WP-S3, GOV-10.

Follow the canonical dependency graph rather than document order. A candidate closes only when its complete consumer result and its negative cases are accepted.

## Determinism and Constraints

Pin source, installed artifacts, policy, configuration and fixtures. Preserve provenance and explicit failure/retry state across runs. Probabilistic model outputs require invariant and quality checks, not invented byte-for-byte determinism. No implicit authority, hidden private-state projection or schema-only completion.

## Integration Points

Consume the qualified product lockset and versioned contracts selected during migration. Runtime retains enforcement and shared Runtime services; enterprise providers may narrow authority; CodeFriend consumes product interfaces; public ADL stays independently usable. This feature adds no sibling-checkout dependency.

## Validation

Run the installed behavior described in Overview/Acceptance Criteria and these required negative cases:

- WP-S5: Cross-tenant leak; Raw ToM projection; Deletion leaves accessible data.

Record exact versions, scenario counts, redacted evidence and skipped checks. QUALITY_GATE_v0.93.md defines release evidence; planning validation does not prove these behaviors.

## Risks

Primary failure risks are the negative cases under Validation. Cross-repository contract drift and overbroad task execution must be resolved before dependent acceptance. A successful fixture cannot establish an untested deployment class or customer/publication permission.

## Future Work

Only explicitly deferred source requirements belong to later work. Do not silently move a required v0.93 outcome into a successor; record a reviewed operator disposition for scope changes.

## Notes

See [execution specifications](../WP_EXECUTION_SPECIFICATIONS_v0.93.yaml), [decision register](../DECISIONS_v0.93.md) and [quality gate](../QUALITY_GATE_v0.93.md). First-pass implementation candidates are not yet created execution issues.

## Current Candidate Mapping

WP-S5; owner repositories: agent-logic-enterprise-security. The current execution graph supersedes older sequencing or placement in retained source text.
