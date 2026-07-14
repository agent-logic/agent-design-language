# Platform Acceptance And Deployment

## Metadata
- Feature Name: ADL v2 Runtime v3 and C-SDLC v2 acceptance and deployment
- Milestone Target: `v0.91.8`
- Status: planned
- Owner: WP-14
- Doc Role: primary
- Supporting Docs: `../DESIGN_v0.91.8.md`, `../NEXT_MILESTONE_HANDOFF_v0.91.8.md`
- Feature Types: runtime, policy, architecture
- Proof Modes: demo, tests, replay, review

## Template Rules

Every product must retain its own authority and evidence. Integrated deployment
does not authorize a new shared monolith.

## Purpose

Accept and deploy the three rearchitected owner products as the platform that
v0.92 can safely consume.

## Context

- Related milestone: `v0.91.8`
- Related issues: `#4641`, `#5358`, `#5361`, `#5352`, `#4758`-`#4763`,
  `#5007`, `#4739`, `#4741`, `#5332`, and `#5107`
- Dependencies: ADL parity, rollback, default switch, and deletion through WP-13

## Coverage / Ownership

- Primary owner doc: this document.
- Covered surfaces: stable installation, deployment topology, configuration,
  readiness, operations, recovery, rollback, lifecycle control, consumer
  integration, Unity Observatory, Adaptive Learning DAG, and v0.92 handoff.
- Related docs: demo matrix, quality gate, release plan.

## Overview

WP-14 is not a documentation-only handoff. It is the integrated acceptance gate
for ADL v2, Runtime v3, and C-SDLC v2. It remains open until each product is
installed into its stable owner path, deployed through its approved topology,
operationally proven, reviewed, and consumable by the next milestone.

## Design

### Core Concepts

- Product acceptance: contract, implementation, proof, operations, and review agree.
- Deployment acceptance: installation, configuration, health, recovery, and
  consumer behavior are proven in the approved environment.

### Architecture

- Inputs: exact product revisions, install receipts, deployment configuration,
  credentials posture, scenario manifests, and review approvals.
- Outputs: three-product acceptance matrix, deployment/rollback evidence,
  blocker ledger, and v0.92 handoff.
- Interfaces: ADL plan/engine contracts, Runtime v3 service/component contracts,
  C-SDLC v2 lifecycle binaries and skills.
- Invariants: separate authority; stable owner binaries; no disposable target
  path as operational truth; no credential exposure; no unsupported readiness claim.

### Data / Artifacts

- product acceptance matrix;
- install/deployment receipts and effective configuration;
- readiness, operations, recovery, rollback, lifecycle, and consumer packets;
- moved-issue disposition ledger.

## Execution Flow

1. Install exact reviewed ADL v2, Runtime v3, and C-SDLC v2 owner binaries.
2. Deploy Runtime v3 and its required operational components through the
   approved service topology and prove readiness, control, recovery, and rollback.
3. Prove the full C-SDLC v2 init, design, bind, goal, validate, review,
   publish, shepherd, merge-readiness, and closeout lifecycle.
4. Prove ADL v2 compilation/execution consumption through Runtime v3 and issue
   workflow consumption through C-SDLC v2.
5. Complete or explicitly block the Unity, Adaptive Learning, launch,
   activation, Memory Palace, capability, witness, birthday, and ADR children.
6. Record the exact v0.92 consumable handoff.

## Determinism and Constraints

- Install and configuration resolution are reproducible and provenance-bound.
- Local deterministic proof is separated from live deployment proof.
- Runtime/provider/external state is captured as evidence before supporting a claim.
- A child blocker requires evidence and explicit operator approval.

## Integration Points

| System / Surface | Integration Type | Description |
|---|---|---|
| ADL v2 | trigger/read | Compile and execute reviewed plans through explicit ports. |
| Runtime v3 | deploy/observe | Supervise components and expose readiness, operations, and recovery. |
| C-SDLC v2 | trigger/observe | Govern issue execution, validation, review, publication, and closeout. |
| Unity Observatory | read/observe | Consume deployed runtime state through the approved live/editor path. |
| v0.92 | read | Consume exact accepted contracts and residual-risk truth. |

## Validation

- Stable install and provenance verification for every owner binary.
- Runtime service health/readiness/control/recovery/rollback and retained evidence.
- Full C-SDLC lifecycle including negative, interruption, recovery, and closeout cases.
- ADL plan/run integration with mock and approved bounded live paths.
- Unity liveness/batch proof and Adaptive Learning dependency/readiness disposition.
- Bounded architecture, security, operations, and deployment review.

## Acceptance Criteria

- All three products are accepted and deployed at exact reviewed revisions.
- Required operational, rollback, recovery, publication, and closeout paths pass.
- Every WP-14 child is closed or blocked with evidence and operator approval.
- v0.92 handoff identifies exact contracts, deployments, non-claims, and risks.

## Risks

- Integration pressure could blur product authority; the acceptance matrix keeps
  owner and consumer roles explicit.
- Live deployment may depend on credentials or infrastructure; blocked evidence
  cannot be converted into readiness prose.

## Future Work

Production scaling and new provider/runtime capabilities remain separately
tracked after the accepted platform baseline.

## Notes

WP-14 supersedes the narrower v0.91.7 launch-handoff posture for its moved
issues; it does not supersede their substantive acceptance requirements.
