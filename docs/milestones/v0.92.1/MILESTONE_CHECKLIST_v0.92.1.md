# Milestone Checklist — v0.92.1

## Current release projection

**Release decision: blocked.** Frozen candidate: `64a99fd71b9770e15cb0dc393d669450d3a5f5b4`.

All 393 inventoried rows have exactly one disposition. Of 366 historically required rows, 143 criteria were explicitly removed by merged operator-reviewed PRs, leaving 223 requirements in the denominator. Removal changes the release requirement; it is **not a behavioral pass**. The 198-row remediation partition is fully joined to its successor evidence; the 32 preserved/separately accounted rows and 15 excluded accounting rows remain visible.

PR #829 merged preparation only. Its four final #821 obligations remain unresolved. The 51 #819 execution rows retain their original candidate and require refresh where proof producers changed. No current execution or release approval is inferred from issue closure, merge, or source presence.

[Machine-readable status](evidence/release/current-status/status.json), [quality gate](evidence/release/current-status/quality-gate.json), [blocker register](evidence/release/current-status/blockers.json), and [full evidence map](evidence/release/current-status/EVIDENCE_MAP.md) are generated together. Historical #517 and #767 packets remain source-time evidence in Git; this projection does not rewrite them.

| Disposition | Rows |
|---|---:|
| approved_removal | 143 |
| downstream_stage_obligation | 27 |
| execution_refresh_required | 51 |
| final_gate_proof_required | 4 |
| historical_evidence_retained | 136 |
| preserved_review_disposition | 32 |

<!-- release-status:start -->
| Feature | Delivery | Approved removals | Prior execution needing refresh | Release |
|---|---|---:|---:|---|
| REPO — Repository and milestone opening | implemented | 0 | 0 | awaiting_final_gate |
| CORP — Corporate and IP | implemented | 17 | 0 | awaiting_final_gate |
| CSDLC — C-SDLC v3 | implemented | 101 | 51 | awaiting_final_gate |
| DRT — Distributed multi-agent Runtime | implemented | 25 | 0 | awaiting_final_gate |
| POD — Podcast | implemented | 0 | 0 | awaiting_final_gate |
| HOT — Axum configuration hot reload | implemented | 0 | 0 | awaiting_final_gate |
| OBS — Observatory redesign | implemented | 0 | 0 | awaiting_final_gate |
| DEC — Runtime v2/v3 decoupling | implemented | 0 | 0 | awaiting_final_gate |
| PROV — Provider inference profiles | implemented | 0 | 0 | awaiting_final_gate |
| GCP-DRT — GCP qualification sidecar | implemented | 0 | 0 | awaiting_final_gate |
| AWS — AWS account move-in | implemented | 0 | 0 | awaiting_final_gate |
| GCP — GCP account move-in | implemented | 0 | 0 | awaiting_final_gate |
| XCL — Cross-cloud Terraform conversion | implemented | 0 | 0 | awaiting_final_gate |
| RUST — Rust resilience refactoring | implemented | 0 | 0 | awaiting_final_gate |
<!-- release-status:end -->

### Release-tail stages

- TAIL-05: awaiting_final_gate (#522).
- TAIL-06: awaiting_final_gate (#522).
- TAIL-10: ceremony_not_run (#526).

### Scope exclusions

- UNITY: deferred — Operator-deferred backlog; excluded from OBS-B release gate.
- TLS: deferred — Operator-deferred backlog; excluded from OBS-B release gate.
- DRT-D-269: not_applicable — DRT-D does not execute #269 or implicitly qualify Runtime v4.
- PROVIDER-PROVENANCE: not_applicable — Historical provider-profile provenance, not active execution authority.

Validate with `python3 .csdlc/prepared/issues/835/project_release.py --check`. `--require-ready` deliberately fails while release blockers remain.

## Complete checklist denominator

Retained status describes its bounded historical evidence, not a new candidate pass.

| ID | Obligation | Current disposition | Retained status | Owner |
|---|---|---|---|---|
| C01 | #432 reviewed, merged, and ancestral | retained_bounded_evidence | proved | none |
| C02 | Reviewed planning package merged; closed #431 retained as provenance only | retained_bounded_evidence | proved | none |
| C03 | Operator declared v0.92.1 ready to open | final_gate_review_required | blocked | 522 |
| C04 | Milestone operator created the number-free WP-01 opening conductor | retained_bounded_evidence | proved | none |
| C05 | No tracked milestone dependency on local untracked paths | final_gate_review_required | blocked | 522 |
| C06 | #316 planning reconciliation creates no execution issues or implementation claims | retained_bounded_evidence | proved | none |
| C07 | AWS-A through AWS-G deliver the seven ordered AWS move-in results; existing #122 remains separately owned | final_gate_review_required | implemented | 522 |
| C08 | GCP-A through GCP-E deliver the five ordered GCP move-in results before DRT-D | final_gate_review_required | implemented | 522 |
| C09 | XCL-01 converts the exact #194/#268 CloudFormation behavior into reviewed AWS/GCP Terraform implementations with rollback retained | final_gate_review_required | implemented | 522 |
| C10 | RUST-01 completes one behavior-preserving resilience owner-boundary refactor without a LoC quota | final_gate_review_required | implemented | 522 |
| C11 | Corporate and IP lane reviewed and merged | final_gate_review_required | implemented | 522 |
| C12 | C-SDLC v3 lane reviewed and merged | final_gate_review_required | implemented | 522 |
| C13 | Distributed multi-agent Runtime lane reviewed and merged | final_gate_review_required | implemented | 522 |
| C14 | #345 AWS GPU Shepherd hardening reviewed, merged, and consumed where required | final_gate_review_required | blocked | 522 |
| C15 | Podcast lane reviewed and merged | final_gate_review_required | implemented | 522 |
| C16 | Axum configuration hot reload lane reviewed and merged | final_gate_review_required | implemented | 522 |
| C17 | Observatory redesign lane reviewed and merged | final_gate_review_required | implemented | 522 |
| C18 | #122 public-exposure proof reviewed against its owned scope | final_gate_review_required | blocked | 522 |
| C19 | #84 Unity and #251 TLS 1.2 explicitly recorded as deferred backlog; neither claimed delivered nor required for OBS-B/#512 | retained_bounded_evidence | deferred | none |
| C20 | DEC-01 Runtime v2/v3 ownership, compatibility, migration, and rollback proof reviewed and merged | final_gate_review_required | implemented | 522 |
| C21 | PROV-A shared inference-profile/Ollama materialization reviewed and merged | final_gate_review_required | implemented | 522 |
| C22 | PROV-B non-authoritative local-model shadow comparison reviewed and merged | final_gate_review_required | implemented | 522 |
| C23 | DRT-D GCP six-resident replay, cost, and zero-resource cleanup reviewed and merged or explicitly operator-gated | final_gate_review_required | implemented | 522 |
| C24 | Individual issue closeout remains asynchronous and non-gating | retained_bounded_evidence | proved | none |
| C25 | Cross-lane dependency and collision review complete | final_gate_review_required | implemented | 522 |
| C26 | Runtime v4 compatibility disposition recorded | final_gate_review_required | blocked | 522 |
| C27 | Proof coverage, release notes, and residual risks reviewed | final_gate_review_required | implemented | 522 |
| C28 | v0.92.2 CodeFriend Beta 1 handoff accepted | final_gate_review_required | blocked | 522 |
| C29 | #188 is used only for convergence/quality, #190 only for successor planning, and #189 only for ceremony | retained_bounded_evidence | proved | none |
| C30 | #457 remains historical provider-profile provenance rather than active execution authority | retained_bounded_evidence | not_applicable | none |
| C31 | TAIL-01 quality gate | final_gate_review_required | implemented | 522 |
| C32 | TAIL-02 docs and release-truth pass | final_gate_review_required | implemented | 522 |
| C33 | TAIL-03 publication finalization | final_gate_review_required | implemented | 522 |
| C34 | TAIL-04 internal review | final_gate_review_required | implemented | 522 |
| C35 | TAIL-05 external review | final_gate_review_required | blocked | 522 |
| C36 | TAIL-06 remediation and release preflight | final_gate_review_required | blocked | 522 |
| C37 | TAIL-07 next-milestone planning | final_gate_review_required | implemented | 522 |
| C38 | TAIL-08 next-milestone closeout planning | final_gate_review_required | blocked | 522 |
| C39 | TAIL-09 next-milestone planning review | final_gate_review_required | blocked | 522 |
| C40 | TAIL-10 release ceremony | final_gate_review_required | blocked | 522 |
