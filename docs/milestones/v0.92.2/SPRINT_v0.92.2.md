# v0.92.2 Sprint Plan

Status: planned; sequence expresses dependencies, not calendar deadlines.

## Sprint 1 — Open and Establish Contracts

- WP-01 validates the package and opens only the approved work wave.
- CF-SHELL and CF-ADAPTER begin in parallel.
- CF-EVIDENCE establishes artifact identity, provenance, redaction, and retention contracts.
- PLAT-UTS, PLAT-RUST, OPS-AWS, PUB-MEDIUM, PUB-CSDLC, and SPEC-RETEST begin as independent bounded tracks.
- PLAT-PROVIDER begins only after v0.92.1 issue #622 is merged; PLAT-MLX follows the provider-definition contract.

## Sprint 2 — Parallel Analysis Surfaces

- CF-COG, CF-GOV, CF-REVIEW, and CF-MEMORY proceed in parallel from the evidence contract.
- CF-UX connects the product shell to governed publication and output renderers.
- PLAT-MEMORY consumes the stabilized evidence and longitudinal-memory contracts.

## Sprint 3 — Proof and Integration

- CF-PROOF completes documentation, examples, deterministic fixtures, ADL self-review, and the bounded external open-source review.
- CF-INTEGRATE reconciles product flow, schemas, operator controls, failure behavior, supporting-track results, and proof artifacts.

## Sprint 4 — Canonical Release Tail

Run TAIL-01 through TAIL-10 in exact order. Individual issue closeout is asynchronous; downstream work depends on merged product authority and the stated release gate, not on bookkeeping receipts.

## Scope Control

Deferred connectors, autonomous mutation, public customer-scale deployment, ATE, OCI model packaging, optional modernization, and Runtime v4 require separate admission.
