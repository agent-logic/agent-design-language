# ADL v0.92.1 — Integration and Product Readiness

v0.92.1 converts the remaining v0.92 planning residue into a bounded, parallel execution program. The package also promotes the separately authored AWS and GCP move-in plans, a cross-cloud Terraform conversion, a bounded Rust resilience refactor, Runtime v2/v3 decoupling, provider inference profiles, and the GCP six-resident qualification sidecar.

## Opening gate

Issue #432 removes tracked dependencies on local untracked paths. It is the repository-authority prerequisite for every execution lane. Issue #316 reconciles this planning package and the v0.92.2 successor package; it does not create execution issues. The reviewed planning merge makes WP-01 eligible but does not create it. When the operator declares v0.92.1 ready, the operator creates the number-free WP-01 conductor, which creates the remaining wave. Closed #431 is planning provenance only.

## Execution lanes

1. **Corporate and IP transfer** — complete the bounded corporate, ownership, licensing, and operational handoff records.
2. **C-SDLC v3** — convert the reviewed GitHub-inspired architecture into a typed implementation program using the tracked source in `sources/`.
3. **Distributed multi-agent Runtime qualification** — qualify governed multi-agent work, using UTS as a workload rather than inventing a separate UTS architecture program.
4. **Podcast publication and Studio** — finish the #51 / #261–#264 / #342 product chain with release evidence and operator-owned external decisions.
5. **Axum configuration hot reload** — deliver validated last-known-good configuration replacement, beginning with stateless strings and flags.
6. **Observatory redesign** — redesign the product around authentic Runtime authority, accessibility, and explicit empty/degraded states; invented data is prohibited.

7. **Runtime v2/v3 decoupling** — establish unambiguous source, manifest, import, test, compatibility, migration, and rollback ownership without beginning Runtime v4.
8. **Provider inference profiles** — define one provider-level profile contract, materialize Ollama settings deterministically, then run non-authoritative local-model shadow comparisons.
9. **GCP qualification sidecar** — repeat the six-resident workload on GCP after DRT-C, with separate identity, billing, cost, and zero-resource cleanup evidence.
10. **AWS account move-in** — inventory and normalize the company account through seven ordered resource, access/billing, Terraform, audit, adoption, platform, and retirement decisions.
11. **GCP account move-in** — establish five ordered hierarchy/cost, Terraform, organization/billing, private-platform, and GPU-readiness results before DRT-D.
12. **Cross-cloud Terraform conversion** — replace the exact #194/#268 CloudFormation behavior with a portable workload contract and explicit AWS/GCP Terraform implementations before retirement.
13. **Rust resilience refactoring** — perform one behavior-preserving owner-boundary extraction with no arbitrary LoC quota.

All execution roots depend on the WP-01/#480 conductor after #432 and the planning-package merge. They may execute independently where their explicit edges permit. XCL-01 consumes AWS-E and GCP-D; AWS-G consumes XCL-01; DRT-D consumes DRT-C, GCP-E, and XCL-01; PROV-B follows PROV-A. No lane silently absorbs another.

The execution wave was created by WP-01/#480. The exact planned-ID to issue mapping is retained in `evidence/wp-01/final-creation-receipt.json`; creation is not proof of implementation or acceptance. The number-free YAML/catalog remain source-time planning specifications. Current release-tail coordination is #538, covering #516 through #526. Premature placeholders #433-#438 and closed #431 remain historical provenance.

Closed issues #149–#190 were prematurely retired planning packets, not delivered execution. Their requirements remain in the routing denominator: corporate #153–#160 is consolidated into CORP-A through CORP-D; C-SDLC v3 #161–#180 into V3-A through V3-F; and Runtime #181–#187 into DRT-A through DRT-C. Integration routing is explicit: #188 informs INT-01 and TAIL-01 quality admission, #190 informs the TAIL-07 successor handoff, and #189 informs only TAIL-10 release ceremony. They are not reopened. The v0.92 tooling defect #387 is not part of the active milestone plan even if legacy labeling includes v0.92.1. #433–#438 were closed as premature placeholders and #439 was closed as redundant with #431. Issue #457 is historical provenance for the provider-profile sidecar only; it is not active execution authority.

Existing issues #122 (public exposure) and #345 (GPU Shepherd) retain their own proof obligations. #84 Unity integration and #251 TLS 1.2 are explicitly deferred backlog, retained visibly but excluded from admission gating. OBS-B/#512 consumes OBS-A/#511 and authentic Runtime projections; it does not require or claim Unity/TLS implementation. GPU-backed distributed qualification still consumes the required #345 evidence.

The existing podcast graph and #432 are likewise retained rather than recreated. #457 is provider-profile provenance only, and #269 remains excluded/backlogged. Cloud move-in does not change those dispositions.

## Canonical release tail

After every root named by INT-01 has merged reviewed authority, the planned integration conductor converges the canonical ten-step tail (successor-planning preparation may run alongside closeout under the operator clarification below): TAIL-01 Quality gate; TAIL-02 Documentation review and external-review handoff; TAIL-03 Publication finalization; TAIL-04 Internal review; TAIL-05 External / third-party review; TAIL-06 Review findings remediation; TAIL-07 Next-milestone planning; TAIL-08 Next-milestone closeout plan; TAIL-09 Next milestone review pass; and TAIL-10 Release ceremony. Individual issue closeout is asynchronous and never a downstream execution dependency.

## Boundaries

- Runtime v4 is a named rebaseline risk, not implicit v0.92.1 scope. Any incompatible authority change must trigger explicit replanning.
- Observatory implementation depends on stable Runtime authority APIs; its design work can proceed earlier.
- Paid infrastructure, publication, legal decisions, and external credentials remain operator-controlled.
- CodeFriend implementation is not part of v0.92.1.

## Successor

v0.92.2 is the **CodeFriend Beta 1** milestone. The release train must make CodeFriend available as an integrated beta by v0.95.

## Operator-promoted Runtime bugfixes

On 2026-09-09 the operator promoted #717 (versioned Polis capability orientation) and #718 (canonical-name A2A routing) from the successor plan into v0.92.1. They execute through their own typed issue authority, do not alter the original WP-01 wave, and must be included in v0.92.1 integration and closeout truth. v0.92.2 consumes their merged results as predecessor capabilities and must not recreate them.

## Package map

- [Vision](VISION_v0.92.1.md)
- [Design](DESIGN_v0.92.1.md)
- [Decisions](DECISIONS_v0.92.1.md)
- [Work breakdown](WBS_v0.92.1.md)
- [Sprint plan](SPRINT_v0.92.1.md)
- [Issue wave](WP_ISSUE_WAVE_v0.92.1.yaml)
- [Planned issue catalog](PLANNED_ISSUE_CATALOG_v0.92.1.md)
- [Canonical document inventory](CANONICAL_DOC_INVENTORY_v0.92.1.md)
- [Execution specifications](WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml)
- [Execution readiness](WP_EXECUTION_READINESS_v0.92.1.md)
- [Retirement ledger](WP_PREMATURE_ISSUE_RETIREMENT_v0.92.1.yaml)
- [Feature plans](features/README.md)
- [Feature proof coverage](FEATURE_PROOF_COVERAGE_v0.92.1.md)
- [Quality gate](QUALITY_GATE_v0.92.1.md)
- [Demo matrix](DEMO_MATRIX_v0.92.1.md)
- [Milestone checklist](MILESTONE_CHECKLIST_v0.92.1.md)
- [ADR plan](ADR_PLAN_v0.92.1.md)
- [Release plan](RELEASE_PLAN_v0.92.1.md)
- [Release notes](RELEASE_NOTES_v0.92.1.md)
- [Next milestone handoff](NEXT_MILESTONE_HANDOFF_v0.92.1.md)


The operator authorized #523/#524/#525 successor planning alongside current closeout on 2026-09-08. Their settled outputs retain #523 → #524 → #525 consumption/review ordering; late closeout findings are reconciled before release. Ceremony still requires both #522 remediation and #525 exact-revision planning review. See the release plan and issue wave for the matching gates.
