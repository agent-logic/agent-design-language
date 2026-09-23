> **Superseded combined plan.** The operator-approved split is planned under #922 in [v0.93.1](../../v0.93.1/README.md) (repo split, templates and CodeFriend Beta 1 launch) and [v0.93.2](../../v0.93.2/README.md) (remaining platform work). The original combined scope below is retained for traceability; it does not require Runtime v4 before Beta 1 launch or authorize execution.

# Security WP-S6: Security Operations, Adversarial Regression, And Provenance v0.93

## Metadata

- Feature Name: Security operations, adversarial regression, and provenance
- Milestone Target: v0.93
- Status: planned
- Doc Role: supporting enterprise-security feature contract
- Feature Types: runtime, artifact, policy, review
- Proof Modes: tests, demo, replay, review

## Purpose

Bind security operations to ADL's existing adversarial and review culture.
v0.93 should prove that threat-board hygiene, red/blue regression, provenance,
runtime hardening, incident response, and release review are connected rather
than separate manual rituals.

## Dependencies

- WP-S1 through WP-S5.
- v0.91.5 CAV/threat-model scheduling packet:
  `docs/milestones/v0.91.5/features/CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SCHEDULING_v0.91.5.md`.
- v0.91.5 CAV/threat-model source packet:
  `docs/milestones/v0.91.5/features/CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SOURCE_PACKET_v0.91.5.md`.
- v0.89.1 adversarial runtime, red/blue proof surfaces, exploit/replay, and
  self-attack work.
- Current CI, review, release-evidence, and milestone closeout gates.
- v0.90.5 governed-tool and provider/tool compatibility evidence.

## Required Work Products

- Security-ops runbook for threat-board review, incident triage, regression
  routing, and release evidence.
- CAV-aligned exploit artifact, replay, mitigation-verification, and security
  corpus scheduling boundaries, without claiming the full CAV runtime exists.
- Adversarial regression suite or matrix mapped to zero-trust, policy,
  cryptographic trust, audit, and isolation controls.
- Provenance checks for source, dependency, tool, provider, and artifact
  boundaries.
- Runtime-hardening report and incident-response drill evidence.

## Invariants

- Security findings must route to reviewable follow-up, not disappear into chat.
- Red/blue tests must be bounded and reproducible.
- Provenance evidence must identify source, dependency, tool, provider, and
  generated-artifact boundaries.
- Release review must include security residual risks.

## Demo Candidate

Run a bounded adversarial regression or incident-response drill that produces a
threat-board update, regression result, provenance evidence, incident record,
and release-review note.

## Acceptance Criteria

- The runbook identifies entry conditions, outputs, and stop boundaries.
- At least one adversarial regression maps to each major security control area.
- Provenance checks are explicit enough for review.
- Release evidence records unresolved security risk truthfully.

## Non-Goals

- No unbounded penetration test claim.
- No production security operations center claim.
- No supply-chain certification claim.

## Template Rules

This document preserves its source requirements. The canonical execution graph supplies current scheduling and repository placement; generated sections and passing structure checks do not establish design approval or product proof.

## Context

Implementation follows accepted v0.92.2 closure and the opening repository split through RD-11. Existing Beta 1 behavior remains a predecessor obligation. The source inputs above retain design context; newer operator decisions in the milestone decision register govern scheduling.

## Coverage / Ownership

Execution owners: agent-logic-enterprise-security. Candidate results: WP-S6. Named people and exact repository identifiers are settled before execution. Shared contracts have one producer and versioned consumers; source is not duplicated between owners.

## Overview

- **WP-S6:** One bounded red/blue/purple drill replays a seed per control area and produces mitigation, provenance, threat-board and release-risk evidence.

## Design

The source scope and invariants above define the feature contract. Implement them in the owning product with explicit actor identity, authorized inputs, persisted evidence and a consumer-visible outcome. Denials retain reasons without disclosing protected state. The result-specific behavior is enumerated under Overview and Acceptance Criteria; any new design choice is recorded before implementation.

## Execution Flow

- WP-S6 follows WP-S4, WP-S5.

Follow the canonical dependency graph rather than document order. A candidate closes only when its complete consumer result and its negative cases are accepted.

## Determinism and Constraints

Pin source, installed artifacts, policy, configuration and fixtures. Preserve provenance and explicit failure/retry state across runs. Probabilistic model outputs require invariant and quality checks, not invented byte-for-byte determinism. No implicit authority, hidden private-state projection or schema-only completion.

## Integration Points

Consume the qualified product lockset and versioned contracts selected during migration. Runtime retains enforcement and shared Runtime services; enterprise providers may narrow authority; CodeFriend consumes product interfaces; public ADL stays independently usable. This feature adds no sibling-checkout dependency.

## Validation

Run the installed behavior described in Overview/Acceptance Criteria and these required negative cases:

- WP-S6: Undeclared target; Non-replayable finding; Unproven mitigation.

Record exact versions, scenario counts, redacted evidence and skipped checks. QUALITY_GATE_v0.93.md defines release evidence; planning validation does not prove these behaviors.

## Risks

Primary failure risks are the negative cases under Validation. Cross-repository contract drift and overbroad task execution must be resolved before dependent acceptance. A successful fixture cannot establish an untested deployment class or customer/publication permission.

## Future Work

Only explicitly deferred source requirements belong to later work. Do not silently move a required v0.93 outcome into a successor; record a reviewed operator disposition for scope changes.

## Notes

See [execution specifications](../WP_EXECUTION_SPECIFICATIONS_v0.93.yaml), [decision register](../DECISIONS_v0.93.md) and [quality gate](../QUALITY_GATE_v0.93.md). First-pass implementation candidates are not yet created execution issues.

## Current Candidate Mapping

WP-S6; owner repositories: agent-logic-enterprise-security. The current execution graph supersedes older sequencing or placement in retained source text.
