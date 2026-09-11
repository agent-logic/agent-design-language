# v0.92.2 Sprint Plan

Status: planned; sequence expresses dependencies, not calendar deadlines.

## First sprint — C-SDLC v3 simplification alongside Runtime

SIM-UMBRELLA opens coordination; SIM-01 → SIM-02 → SIM-03 → SIM-04 → SIM-05 → SIM-06 → SIM-07 → SIM-08 → SIM-09 is one coherent sprint. It starts through its own readiness and dedicated issue launch, in parallel with Runtime, without waiting for WP-01 or unrelated prior closeout. The umbrella completes after SIM-09 and converges at TAIL-01. Resolve C-SDLC/Runtime shared-path and installed-binary ownership before overlapping writes. An eventual C-SDLC writer pause requires separate explicit authorization; it does not pause Runtime/provider services. See the [complete plan](cognitive-sdlc/C_SDLC_V3_SIMPLIFICATION_PLAN.md).

## CodeFriend wave 1 — Open and Establish Contracts

- WP-01 validates the package and opens only the approved work wave.
- Existing #848, #849, #852, #854, #855, #861, and #862 retain their mapped work-package authority; #848 decides whether repository-split implementation is admitted at all.
- CF-SHELL and CF-ADAPTER begin in parallel.
- CF-EVIDENCE establishes artifact identity, provenance, redaction, and retention contracts.
- PLAT-UTS, PLAT-RUST, OPS-AWS, OPS-GCP, PUB-MEDIUM, PUB-CSDLC, SPEC-RETEST, and ARCH-ADR begin as independent bounded tracks.
- PLAT-PROVIDER begins only after v0.92.1 issue #622 is merged; PLAT-MLX and the bounded PLAT-PAIR experiment follow the provider-definition contract.
- OBS-S3 begins only after WP-01 creates its issue and OBS-LIVE is complete; it applies the existing #679/PR #685 design and remains non-gating for product integration.

## CodeFriend wave 2 — Parallel Analysis Surfaces

- CF-COG, CF-GOV, CF-REVIEW, and CF-MEMORY proceed in parallel from the evidence contract.
- CF-UX connects the product shell to governed publication and output renderers.
- PLAT-MEMORY consumes the stabilized evidence and longitudinal-memory contracts.

## CodeFriend wave 3 — Proof and Integration

- CF-PROOF completes documentation, examples, deterministic fixtures, ADL self-review, and the bounded external open-source review.
- CF-INTEGRATE reconciles product flow, schemas, operator controls, failure behavior, supporting-track results, and proof artifacts.

## Milestone closeout — Canonical Release Tail

Run TAIL-01 through TAIL-10 in exact order. Individual issue closeout is asynchronous; downstream work depends on merged product authority and the stated release gate, not on bookkeeping receipts.

## Scope Control

Deferred connectors, autonomous mutation, public customer-scale or multi-tenant deployment, ATE, OCI model packaging, optional modernization, and Runtime v4 require separate admission. The bounded static Observatory sidecar does not authorize those broader deployment programs.

## Existing issues outside new-wave startup

Closed merged #717 and #718 are v0.92.1 predecessor inputs. #720 uses its own independent authority; #848, #849, #852, #854, #855, #861, and #862 use their mapped existing authority after WP-01. #848 is a decision row rather than split implementation. This schedules existing v0.92.2 work; #523 does not implement it.
