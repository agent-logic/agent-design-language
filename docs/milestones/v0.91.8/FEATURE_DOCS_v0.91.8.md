# v0.91.8 Feature-Doc Index

## Metadata

- Milestone: `v0.91.8`
- Version: `v0.91.8`
- Date: `2026-07-14`
- Setup issue: `#5335`

## Status

Planned feature-doc package for the ADL Core Rearchitecture and integrated
platform deployment wave. These documents define intended contracts and proof
boundaries; they do not claim implementation, acceptance, or release.

## Implementation-Facing Features

| Feature doc | Surface | Execution WPs |
| --- | --- | --- |
| [`BASELINE_AND_ARCHITECTURE_v0.91.8.md`](features/BASELINE_AND_ARCHITECTURE_v0.91.8.md) | pinned incumbent denominator, ownership, dependency and size budgets | WP-02 |
| [`CHARACTERIZATION_AND_PARITY_v0.91.8.md`](features/CHARACTERIZATION_AND_PARITY_v0.91.8.md) | normalized behavior corpus, determinism, and mismatch disposition | WP-03, WP-11 |
| [`LANGUAGE_AND_COMPILER_v0.91.8.md`](features/LANGUAGE_AND_COMPILER_v0.91.8.md) | six-primitives language and deterministic execution-plan compiler | WP-04, WP-05 |
| [`PORTABLE_ENGINE_AND_CONTRACTS_v0.91.8.md`](features/PORTABLE_ENGINE_AND_CONTRACTS_v0.91.8.md) | bounded engine, records, signing, trust, provider and tool ports | WP-06, WP-07 |
| [`CLI_AND_ADAPTERS_v0.91.8.md`](features/CLI_AND_ADAPTERS_v0.91.8.md) | Runtime v3, provider/tool adapters, thin CLI, and selector | WP-08 through WP-10 |
| [`SHADOW_PARITY_AND_CUTOVER_v0.91.8.md`](features/SHADOW_PARITY_AND_CUTOVER_v0.91.8.md) | parity, soak, rollback, reversible switch, and deletion | WP-11 through WP-13 |
| [`PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md`](features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md) | ADL v2, Runtime v3, and C-SDLC v2 acceptance/deployment; Unity and Adaptive Learning dispositions | WP-14 |

## Proof And Closeout Docs

| Document | Purpose | Execution WPs |
| --- | --- | --- |
| [`FEATURE_PROOF_COVERAGE_v0.91.8.md`](FEATURE_PROOF_COVERAGE_v0.91.8.md) | map first-class features to proof owners and release gates | WP-16 through WP-20 |
| [`DEMO_MATRIX_v0.91.8.md`](DEMO_MATRIX_v0.91.8.md) | integrated reviewer-visible demonstration contract | WP-15 |
| [`QUALITY_GATE_v0.91.8.md`](QUALITY_GATE_v0.91.8.md) | product, deployment, rollback, deletion, and review gates | WP-16 |
| [`NEXT_MILESTONE_HANDOFF_v0.91.8.md`](NEXT_MILESTONE_HANDOFF_v0.91.8.md) | exact-revision v0.92 consumption and residual-risk boundary | WP-14, WP-21 through WP-22 |

## Closeout Boundary

WP-15 through WP-23, including WP-21A, retain separate canonical roles for
demo convergence, quality, documentation, internal review, external review,
remediation, next-milestone planning, next-milestone closeout planning,
next-milestone review, and release ceremony. No feature doc collapses those
gates or converts planned work into proof.

## Validation

- Validate every implementation-facing feature doc with the active planning-template registry.
- Resolve every relative link from the milestone directory.
- Keep moved WP-14 children visible until closed or operator-approved as evidence-backed blockers.
- Treat `FEATURE_PROOF_COVERAGE_v0.91.8.md` as a mapping contract, not acceptance evidence.
