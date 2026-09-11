# Demo Matrix — v0.92.1

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

## Bounded feature and demo evidence

### Repository and milestone opening

Retained demo status: blocked. Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection.

### Corporate and IP

Retained demo status: blocked. Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection.

### C-SDLC v3

Retained demo status: blocked. Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection.

### Distributed multi-agent Runtime

Retained demo status: blocked. Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection.

### Podcast

Retained demo status: proved. Only retained HTTP feed/enclosure/range compatibility and public-hosting artifact checks; actual browser/player playback remains unproved; not directory submission or public launch.

### Axum configuration hot reload

Retained demo status: blocked. Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection.

### Observatory redesign

Retained demo status: blocked. Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection.

### Runtime v2/v3 decoupling

Retained demo status: blocked. Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection.

### Provider inference profiles

Retained demo status: blocked. Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection.

### GCP qualification sidecar

Retained demo status: blocked. Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection.

### AWS account move-in

Retained demo status: blocked. Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection.

### GCP account move-in

Retained demo status: blocked. Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection.

### Cross-cloud Terraform conversion

Retained demo status: blocked. Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection.

### Rust resilience refactoring

Retained demo status: blocked. Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection.

