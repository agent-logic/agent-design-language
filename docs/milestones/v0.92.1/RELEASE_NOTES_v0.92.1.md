# Release Notes — v0.92.1

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

## Delivered implementation and bounded results

- **Repository and milestone opening:** Execution-wave creation receipt and repository planning delivered. [Evidence and limits](evidence/release/current-status/EVIDENCE_MAP.md#repo).
- **Corporate and IP:** Inventory, ownership transfer, operating-control acceptance and diligence packet retained. CORP-C has governed recordless acceptance. [Evidence and limits](evidence/release/current-status/EVIDENCE_MAP.md#corp).
- **C-SDLC v3:** Native command implementation and authenticated #505/PR591 cutover delivered. [Evidence and limits](evidence/release/current-status/EVIDENCE_MAP.md#csdlc).
- **Distributed multi-agent Runtime:** Distributed contract, continuity qualification and failure-mode implementation retained. [Evidence and limits](evidence/release/current-status/EVIDENCE_MAP.md#drt).
- **Podcast:** Episode/Studio package, feed hosting, HTTP feed/enclosure/range checks and public-hosting receipt retained. [Evidence and limits](evidence/release/current-status/EVIDENCE_MAP.md#pod).
- **Axum configuration hot reload:** Config reload implementation and retained validation supplied by #510. [Evidence and limits](evidence/release/current-status/EVIDENCE_MAP.md#hot).
- **Observatory redesign:** OBS-A captured absorption into OBS-B; authentic-browser proof retained under #512. [Evidence and limits](evidence/release/current-status/EVIDENCE_MAP.md#obs).
- **Runtime v2/v3 decoupling:** Runtime authority topology and generation ownership migration package retained. [Evidence and limits](evidence/release/current-status/EVIDENCE_MAP.md#dec).
- **Provider inference profiles:** Shared profile implementation and isolated comparison/shadow implementation retained. [Evidence and limits](evidence/release/current-status/EVIDENCE_MAP.md#prov).
- **GCP qualification sidecar:** Governed six-resident replay wrapper and retained run evidence delivered by #509. [Evidence and limits](evidence/release/current-status/EVIDENCE_MAP.md#gcp-drt).
- **AWS account move-in:** Seven phase implementation and receipt packages retained. [Evidence and limits](evidence/release/current-status/EVIDENCE_MAP.md#aws).
- **GCP account move-in:** Five phase packages plus #740 private/versioned backend recovery proof retained. [Evidence and limits](evidence/release/current-status/EVIDENCE_MAP.md#gcp).
- **Cross-cloud Terraform conversion:** Portable workload contract and AWS/GCP Terraform implementation retained. [Evidence and limits](evidence/release/current-status/EVIDENCE_MAP.md#xcl).
- **Rust resilience refactoring:** Resilience owner-boundary refactoring and retained behavior checks supplied by #499. [Evidence and limits](evidence/release/current-status/EVIDENCE_MAP.md#rust).

## Remaining release work

Resolve or explicitly govern the named remediation debts, refresh the complete candidate review and proof map, and obtain release authorization through the release tail. Podcast directory submission and launch require their own operator authorization. CodeFriend Beta 1 belongs to v0.92.2. No new demo or production mutation was performed for this documentation projection.
