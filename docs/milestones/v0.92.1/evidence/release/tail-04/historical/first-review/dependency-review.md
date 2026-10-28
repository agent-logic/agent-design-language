# Issue #520 dependency and supply-chain review

## Review identity

- Candidate: `c24f8fa65ce445b03ce6cd69007307291d78b60c`
- Base: `f0a011a5c59d46c763d669f69a10308b3f870ba4`
- Review checkout: detached read-only checkout at the exact candidate SHA
- Review date: 2026-09-09 UTC
- Role: manifests, lockfiles, package managers, CI acquisition, Terraform providers, and supply-chain reproducibility

## Findings

### T520-DEP-001 — P2 — New XCL Terraform roots permit unbounded provider-major upgrades

- Evidence: `infra/aws/runtime/xcl-01/versions.tf:4-8` declares `hashicorp/aws >= 5.0`; `infra/gcp/workloads/xcl-01/versions.tf:4-8` declares `hashicorp/google >= 5.0`. Their lockfiles currently select AWS 6.62.0 (`infra/aws/runtime/xcl-01/.terraform.lock.hcl:4-6`) and Google 8.0.0 (`infra/gcp/workloads/xcl-01/.terraform.lock.hcl:4-6`) while retaining only the open-ended lower bound.
- Scenario: an operator regenerates either lockfile with `terraform init -upgrade`, or creates the root where the committed lock is unavailable.
- Impact: a future breaking provider major is eligible without a source review. The same Terraform source can then produce a different plan or fail after a routine dependency refresh, contrary to the release plan's pinned provider/module requirement.
- Required remediation/proof: choose and document supported provider major ranges (for example, a compatible-major constraint rather than a floor only), regenerate locks intentionally, and retain `terraform init -lockfile=readonly` plus validate/plan proof for both roots.

### T520-DEP-002 — P2 — Required Rust CI floats the compiler despite pinned action code and dependencies

- Evidence: `.github/workflows/ci.yaml:203-206` pins the `dtolnay/rust-toolchain` action commit but requests `stable`; no tracked `rust-toolchain` or `rust-toolchain.toml` exists, and the reviewed Cargo manifests declare no `rust-version` compatibility floor.
- Scenario: Rust publishes a new stable compiler between two identical runs of the same candidate and lockfiles.
- Impact: compile, Clippy, formatting, or MSRV behavior can change without any repository change. Exact-candidate validation is therefore not fully reproducible, and a future compiler can turn a previously green required lane red.
- Required remediation/proof: pin the CI compiler/toolchain channel to an explicit version (and declare the intended MSRV where appropriate), test the complete required Rust lanes with that version, and update it through an explicit dependency change.

## Dependency surface map

The complete base-to-candidate inventory was classified before review. It includes three changed `Cargo.toml` files, two changed Cargo lockfiles, 26 Terraform provider lockfiles, 138 Terraform configuration files, 19 Terraform variable/readback files, the CI acquisition workflow, and custom integrity/source manifests. Ten changed podcast `package.json` files are episode data envelopes rather than npm package-manager manifests: they declare no scripts or dependencies and do not require npm lockfiles.

The candidate's own Cargo audit traverses all 24 tracked Rust manifests and 20 local path dependencies. Its checks establish repository-relative local dependency resolution and absence of duplicate package/version identities; they do not establish compiler reproducibility, vulnerability status, license approval, or feature-combination behavior.

## Validation performed

- `python3 .csdlc/prepared/issues/518/audit-cargo-manifests.py --check`: **PASS** for 24 manifests and 20 local dependencies; the report truthfully retains `final_acceptance: false`.
- `cargo metadata --locked --no-deps` for `csdlc-v3/Cargo.toml`, `adl-runtime/Cargo.toml`, and `adl/Cargo.toml`: **PASS**.
- `terraform fmt -check -recursive infra`: **PASS**.
- All 26 changed Terraform lockfiles were structurally inventoried and their selected versions compared to declared constraints. No lock/source mismatch was found other than the two open-ended major ranges in T520-DEP-001.
- Changed GitHub Actions references are commit-pinned. The Rust compiler selected by the pinned action remains floating as described in T520-DEP-002.
- The review checkout remained clean after all checks.

## Supply-chain and license assessment

- No candidate evidence claims a current external vulnerability-feed scan, provenance attestation for every downloaded package, or exhaustive feature-matrix build. Those are unproved residuals, not inferred vulnerabilities.
- Cargo lockfiles provide version and checksum resolution for registry dependencies. Terraform lockfiles provide provider checksums for supported platforms. The checks do not eliminate registry compromise or establish artifact signing beyond those mechanisms.
- Rust package metadata consistently uses the repository's declared MIT/Apache-2.0 posture where specified. No new legal license determination was performed, and no incompatible license was identified from the reviewed manifest metadata.

## Residual risk

- No networked advisory, malware, or license database was queried; conclusions are limited to repository evidence and locked metadata.
- No paid cloud plan/apply was run. Provider schema and real-account behavior therefore remain dependent on the separately retained infrastructure proofs.
- Optional Cargo feature combinations and every supported Terraform platform checksum were not exhaustively executed. The two findings above are the actionable reproducibility gaps found in the complete assigned dependency surface.
