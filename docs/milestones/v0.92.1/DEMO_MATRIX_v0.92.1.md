# Demo Matrix — v0.92.1

## Current release disposition

**Release decision: blocked.** This is a reviewable projection of retained delivery and proof at `7ecc70b517ef2f7ee86a6456d8c778daf8c558d3`, observed 2026-09-09. It is a draft pending the substantive #522 remediation refresh required by #767; it grants no release authorization.

`implemented` means implementation or a governed delivery disposition exists; `proved` applies only to the explicitly named observed scope. `blocked` identifies outstanding proof, remediation or authorization. `deferred` records an explicit scope deferral; `not_applicable` identifies a requirement outside the governed scope. These states are not interchangeable.

The [evidence map](evidence/release/current-status/EVIDENCE_MAP.md) identifies all 35 execution work packages, existing podcast ownership, exact source revisions, retained proof classifications, current limits and debt owners. [Machine-readable status](evidence/release/current-status/status.json) binds source hashes and pending remediation observations. The full candidate review is owned by #761; retained candidate proof gaps by #764. A blocked proof column does **not** mean implementation failed or that historical accounting is incomplete.

<!-- release-status:start -->
| Lane ID | Lane | Delivery | Proof | Demo | Release | Evidence |
|---|---|---|---|---|---|---|
| REPO | Repository and milestone opening | implemented | blocked | blocked | blocked | [REPO](evidence/release/current-status/EVIDENCE_MAP.md#repo) |
| CORP | Corporate and IP | implemented | blocked | blocked | blocked | [CORP](evidence/release/current-status/EVIDENCE_MAP.md#corp) |
| CSDLC | C-SDLC v3 | implemented | blocked | blocked | blocked | [CSDLC](evidence/release/current-status/EVIDENCE_MAP.md#csdlc) |
| DRT | Distributed multi-agent Runtime | implemented | blocked | blocked | blocked | [DRT](evidence/release/current-status/EVIDENCE_MAP.md#drt) |
| POD | Podcast | implemented | blocked | proved | blocked | [POD](evidence/release/current-status/EVIDENCE_MAP.md#pod) |
| HOT | Axum configuration hot reload | implemented | blocked | blocked | blocked | [HOT](evidence/release/current-status/EVIDENCE_MAP.md#hot) |
| OBS | Observatory redesign | implemented | blocked | blocked | blocked | [OBS](evidence/release/current-status/EVIDENCE_MAP.md#obs) |
| DEC | Runtime v2/v3 decoupling | implemented | blocked | blocked | blocked | [DEC](evidence/release/current-status/EVIDENCE_MAP.md#dec) |
| PROV | Provider inference profiles | implemented | blocked | blocked | blocked | [PROV](evidence/release/current-status/EVIDENCE_MAP.md#prov) |
| GCP-DRT | GCP qualification sidecar | implemented | blocked | blocked | blocked | [GCP-DRT](evidence/release/current-status/EVIDENCE_MAP.md#gcp-drt) |
| AWS | AWS account move-in | implemented | blocked | blocked | blocked | [AWS](evidence/release/current-status/EVIDENCE_MAP.md#aws) |
| GCP | GCP account move-in | implemented | blocked | blocked | blocked | [GCP](evidence/release/current-status/EVIDENCE_MAP.md#gcp) |
| XCL | Cross-cloud Terraform conversion | implemented | blocked | blocked | blocked | [XCL](evidence/release/current-status/EVIDENCE_MAP.md#xcl) |
| RUST | Rust resilience refactoring | implemented | blocked | blocked | blocked | [RUST](evidence/release/current-status/EVIDENCE_MAP.md#rust) |
<!-- release-status:end -->

The Podcast demo `proved` cell covers retained HTTP feed/enclosure/range compatibility and public-hosting artifact checks only; actual browser/player playback remains unproved; it excludes directory submission and public-launch authorization.

### Explicit scope dispositions

- **UNITY: deferred** — Operator-deferred backlog; excluded from OBS-B release gate. [#84](https://github.com/agent-logic/agent-design-language/issues/84).
- **TLS: deferred** — Operator-deferred backlog; excluded from OBS-B release gate. [#251](https://github.com/agent-logic/agent-design-language/issues/251).
- **DRT-D-269: not_applicable** — DRT-D does not execute #269 or implicitly qualify Runtime v4. [#269](https://github.com/agent-logic/agent-design-language/issues/269).
- **PROVIDER-PROVENANCE: not_applicable** — Historical provider-profile provenance, not active execution authority. [#457](https://github.com/agent-logic/agent-design-language/issues/457).

Historical #517 / PR #752 accounting covers all 245 non-proving rows and five exception groups with zero unowned accounting rows. [The retained accounting boundary](evidence/release/tail-01/reconciliation/ownership.json) does not convert those rows into current product proof or authorize release.

Reproduce this projection with `python3 docs/milestones/v0.92.1/evidence/release/current-status/validate.py`. This checks documentation consistency and retained evidence bytes, not live cloud/product behavior.

## Observed demonstration scope

- **REPO — blocked:** Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection. [Evidence](evidence/release/current-status/EVIDENCE_MAP.md#repo).
- **CORP — blocked:** Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection. [Evidence](evidence/release/current-status/EVIDENCE_MAP.md#corp).
- **CSDLC — blocked:** Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection. [Evidence](evidence/release/current-status/EVIDENCE_MAP.md#csdlc).
- **DRT — blocked:** Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection. [Evidence](evidence/release/current-status/EVIDENCE_MAP.md#drt).
- **POD — proved:** Only retained HTTP feed/enclosure/range compatibility and public-hosting artifact checks; actual browser/player playback remains unproved; not directory submission or public launch. [Evidence](evidence/release/current-status/EVIDENCE_MAP.md#pod).
- **HOT — blocked:** Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection. [Evidence](evidence/release/current-status/EVIDENCE_MAP.md#hot).
- **OBS — blocked:** Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection. [Evidence](evidence/release/current-status/EVIDENCE_MAP.md#obs).
- **DEC — blocked:** Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection. [Evidence](evidence/release/current-status/EVIDENCE_MAP.md#dec).
- **PROV — blocked:** Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection. [Evidence](evidence/release/current-status/EVIDENCE_MAP.md#prov).
- **GCP-DRT — blocked:** Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection. [Evidence](evidence/release/current-status/EVIDENCE_MAP.md#gcp-drt).
- **AWS — blocked:** Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection. [Evidence](evidence/release/current-status/EVIDENCE_MAP.md#aws).
- **GCP — blocked:** Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection. [Evidence](evidence/release/current-status/EVIDENCE_MAP.md#gcp).
- **XCL — blocked:** Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection. [Evidence](evidence/release/current-status/EVIDENCE_MAP.md#xcl).
- **RUST — blocked:** Retained implementation/validation artifacts are linked below; a complete current candidate-bound demonstration is not established by this projection. [Evidence](evidence/release/current-status/EVIDENCE_MAP.md#rust).

## Required demonstrations

These retained scenarios define what demonstration proof must cover; listing a scenario is not an observed pass.

| Lane | Demonstration |
|---|---|
| Corporate and IP | Redacted transfer packet passes completeness and authority checks |
| C-SDLC v3 | One typed slice migrates, executes, rolls back, and retains evidence |
| Distributed multi-agent Runtime | Multiple resident agents complete governed UTS work across continuity |
| Podcast | Published-candidate episode validates feed, enclosure, playback, and Studio presentation |
| Axum configuration hot reload | Valid edit applies without restart; invalid edit retains last-known-good |
| Observatory redesign | Authentic authority projection renders accessible normal, empty, degraded, and recovery states |
| Runtime v2/v3 decoupling | Representative supported consumers resolve to one generation, pass compatibility proof, migrate, and roll back without cross-authority ambiguity |
| Provider inference profiles | One bounded profile materializes deterministic Ollama settings; an invalid profile preserves last-known-good; shadow output cannot mutate authority |
| GCP qualification sidecar | Six residents complete the same governed workload with provider identity, cost, and zero-resource cleanup receipts |
| AWS move-in | Reviewer walks the live-to-declared ownership register and one disposable private Runtime module deployment/cleanup without production traffic |
| GCP move-in | Reviewer inspects private foundation readback and the separately authorized L4 smoke-test cleanup packet |
| Cross-cloud Terraform | The same portable Runtime workload contract produces explicit AWS and GCP plans and comparable disposable-run receipts |
| Rust resilience refactor | Identical resilience behaviors run through the new owner boundaries with a smaller or truthfully unchanged validation-impact denominator |

Demonstrations supplement, but never replace, exact validation and review.
