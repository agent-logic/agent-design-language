## Metadata

- Skill: `repo-dependency-review`
- Reviewer identity: Codex dependency/supply-chain specialist (`/root/review_313_tests`)
- Target: repository-wide review of exact revision `c6792e54df1db5969fa28c59b6dfe4c714ed5559`
- Date: 2026-08-25 UTC
- Artifact: `docs/reviews/v0.92/internal-review-5846/specialists/dependencies.md`
- Packet: `docs/reviews/v0.92/internal-review-5846`
- Finding count: 4 (`P1`: 2, `P2`: 2)

## Findings

- P1: Remote validation installs mutable executable artifacts without digest verification
  File or evidence path: `tools/aws_remote_validation/scripts/remote_validation_runner.sh:221`
  Role: dependency
  Scenario: A non-containerized remote validation host lacks sccache, cargo-nextest, Rust, or a cached tool, so the runner bootstraps it from GitHub, an operator-provided URL, S3, or rustup.
  Dependency surface: Remote builder executable bootstrap and shared S3 tool cache.
  Impact: The validation host can execute bytes that are neither tied to the reviewed commit nor verified against an approved digest. A mutable upstream release, overwritten cache object, compromised download endpoint, or operator URL can change the compiler/test toolchain while the repository SHA remains identical, undermining the authority of remote proof and creating a code-execution supply-chain boundary.
  Evidence: `install_github_release_binary` queries `/releases/latest`, selects the first matching asset, downloads it, and installs it without a checksum or signature (`tools/aws_remote_validation/scripts/remote_validation_runner.sh:221-252`). Arbitrary tarball URLs are downloaded and installed without digest verification (`:291-312`). S3 cache objects use mutable keys such as `tools/sccache.tar.gz`, are uploaded without immutable VersionId/digest metadata, and are later installed without content verification (`:315-340`). Missing Cargo triggers `curl https://sh.rustup.rs | sh` with no pinned installer digest (`:414-418`). Version-output smoke checks at `:343-368` prove executability, not provenance.
  Recommended follow-up owner: AWS remote-validation/toolchain owner. Require version plus SHA-256 (and S3 VersionId where applicable) for every downloaded executable/archive, reject `latest` and unpinned URLs, verify before extraction, and retain the verified provenance in the result contract.

- P1: The reusable builder image is not reproducible or fully integrity-pinned
  File or evidence path: `adl/docker/adl-builder/Dockerfile:1`
  Role: dependency
  Scenario: The builder image is rebuilt later under the same repository revision or image-tag contract.
  Dependency surface: Base image, operating-system packages, Rust toolchain, AWS CLI, sccache, cargo-nextest, and cargo-llvm-cov bootstrap.
  Impact: A rebuild can resolve materially different or compromised dependency bytes while retaining the same source revision and nominal tool versions. This weakens immutable-builder claims and can make CI/remote validation results non-reproducible across rebuilds.
  Evidence: The base is the mutable tag `ubuntu:24.04` rather than an image digest (`adl/docker/adl-builder/Dockerfile:1`); apt packages are unversioned (`:21-44`); `RUST_TOOLCHAIN=stable` floats and the downloaded rustup installer is executed without a checksum (`:3`, `:70-73`); AWS CLI is downloaded without checksum/signature verification (`:59-68`); sccache and cargo-llvm-cov archives are version-named but not digest-checked (`:75-85`, `:115-129`). Ruby and cargo-nextest demonstrate the required pattern by pinning and checking SHA-256 (`:6-10`, `:46-57`, `:87-103`).
  Recommended follow-up owner: Builder-image/release-engineering owner. Pin the base digest and Rust channel/version, add architecture-specific digests or verified signatures for every direct binary download, and retain a machine-readable resolved package/toolchain manifest with the image digest.

- P2: Several independent Cargo dependency graphs have no change-triggered validation owner
  File or evidence path: `.github/workflows/ci.yaml:386`
  Role: dependency
  Scenario: A pull request changes a manifest, lockfile, feature, or source dependency under `adl-characterization/**`, `adl-resilience/**`, or `tools/remote_validation/**` alone.
  Dependency surface: Independent Cargo manifests and lockfiles outside the `adl` workspace.
  Impact: CI can report a green required aggregate without resolving the changed lockfile or compiling the changed dependency graph. Broken lock consistency, MSRV drift, feature conflicts, or packaging failures can reach main undetected.
  Evidence: Ordinary Rust jobs run from `adl` and therefore own only its graph (`.github/workflows/ci.yaml:386-468`); the focused runtime job owns only `adl-runtime-kernel` (`:351-384`). The path policy creates standalone selection only for `csdlc-v2/*` and `adl-v2/*` (`adl/tools/ci_path_policy.sh:1669-1677`) and otherwise does not select Rust for these package roots (`:1737-1760`). Exact-target `cargo metadata --locked --no-deps` succeeds for all three today, but no routing contract preserves that property on package-only changes.
  Recommended follow-up owner: CI/PVF and package owners. Give each independent graph explicit path ownership with locked metadata/build/test/Clippy proof and routing regression fixtures. This overlaps the tests specialist's CI-denominator finding and should be deduplicated during synthesis, preserving the dependency-graph consequence.

- P2: The deterministic dependency scaffold mistakes lifecycle locks for dependency manifests
  File or evidence path: `docs/reviews/v0.92/internal-review-5846/evidence_index.json`
  Role: dependency
  Scenario: A dependency reviewer follows the skill-required deterministic scaffold generated from this packet.
  Dependency surface: Review-packet inventory and specialist routing.
  Impact: The scaffold spends its bounded candidate list on empty lifecycle lock sentinels and reports only a generic manifest surface, so it fails to route the actual Cargo/container/bootstrap supply-chain risks that the dependency skill expects it to expose.
  Evidence: Re-running `prepare_dependency_review.py` against the current packet emits `.csdlc/locks/10.lock` through other zero-length lifecycle locks as its only `dependency_surface_map.manifest` entries and as its supply-chain candidates. These are workflow lock sentinels, not package-manager lockfiles. The packet's subsequently repaired `specialist_assignments.json:2189-2250` does correctly list 61 actual dependency surfaces, demonstrating that the scaffold is consuming stale/misclassified `evidence_index.json` evidence rather than the corrected dependency assignment.
  Recommended follow-up owner: `repo-packet-builder` / dependency-review tooling owner. Classify manifests by basename and ecosystem context rather than extension alone, exclude lifecycle lock sentinels from dependency evidence, and have the scaffold consume the explicit dependency assignment or fail on disagreement between the evidence index and assignment.

## Dependency Surface Map

- Manifests and lockfiles: 12 top-level/nested Cargo dependency graphs identified by the packet, including `adl`, `adl-characterization`, `adl-resilience`, `adl-runtime`, `adl-runtime-kernel`, `adl-v2`, `csdlc-v2`, `tools/aws_remote_validation`, and `tools/remote_validation`; selected nested/example Cargo manifests were inventoried but not all were executed.
- Package-manager configuration: no tracked `rust-toolchain.toml`, Cargo source policy, `deny.toml`, or equivalent repository-wide dependency policy was found at the exact target. Cargo registry dependencies are lockfile-resolved where the reviewed commands use `--locked`.
- Runtime images and containers: `adl/docker/adl-builder/Dockerfile` is the primary reusable builder surface; it mixes apt, direct downloads, rustup, and checksum-verified archives.
- CI bootstrap and caches: GitHub Actions are commit-SHA pinned; Rust is selected as floating `stable`; cargo-nextest is version-pinned; remote AWS tool caches use S3 and direct GitHub downloads.
- Generated or vendored surfaces: no general vendored Cargo source tree was identified. Retained binary/evidence artifacts exist under `.csdlc/evidence/**` but were not treated as dependency authority.
- License and attribution: repository root `LICENSE`; crate manifests declare MIT-or-Apache or Apache-2.0; Vector installer records MPL-2.0 provenance and copies its license. No repository-wide third-party dependency attribution or automated license policy was found.
- Install/package proof: package-local Cargo suites and CI lanes exist unevenly; Vector has checksum/provenance checks, builder-image contract tests exist, and C-SDLC has an explicit supply-chain validator that records unavailable audit/SBOM tooling rather than claiming it.

## Reviewed Surfaces

- Packet: `repo_scope.md`, `repo_inventory.json`, `evidence_index.json`, `specialist_assignments.json`, and deterministic dependency scaffold output.
- Cargo manifests/locks for the nine principal graphs listed in Validation Performed, plus inventory review of the remaining nested Cargo manifests.
- `.github/workflows/*.yaml` and `.github/workflows/*.yml` action pinning, toolchain installation, cache, artifact, and AWS bootstrap surfaces.
- `adl/docker/adl-builder/Dockerfile` and its contract-test references.
- `tools/aws_remote_validation/scripts/remote_validation_runner.sh` direct-download, package-manager, S3-cache, and verification paths.
- `adl/tools/install_vector_component.sh` as a positive checksum/provenance control comparison.
- Root `LICENSE`, crate license declarations, and the tracked supply-chain validation scripts/docs.

## Candidate Supply-Chain Findings

- Adopt one immutable executable-bootstrap contract shared by local CI, builder images, and AWS remote validation: explicit version, digest/signature, source, architecture, and retained provenance.
- Replace mutable builder inputs (`ubuntu:24.04`, Rust `stable`, unversioned apt resolution) with a reviewed resolved-image/toolchain identity or explicitly classify rebuilds as new qualification events.
- Add explicit CI/PVF ownership for every independently locked Cargo graph.
- Repair dependency scaffold/evidence classification so `.csdlc/locks/*.lock` cannot masquerade as package-manager manifests.

## Candidate Dependency Test Gaps

- No negative contract proves remote bootstrap rejects a missing/mismatched executable digest, mutable S3 object, or unpinned GitHub `latest` asset.
- Builder-image tests assert some version strings and nextest checksums but do not require checksum/signature verification for AWS CLI, rustup, sccache, cargo-llvm-cov, or the base image digest.
- No routing fixture proves package-only changes in `adl-characterization`, `adl-resilience`, and `tools/remote_validation` run their locked dependency graph.
- No packet/scaffold test proves lifecycle `.lock` sentinels are excluded from dependency evidence while Cargo/container/bootstrap surfaces are included and assignment/index disagreement is rejected.

## Candidate License Review Notes

- Human review should confirm whether the shipped/distributed combination of MIT-or-Apache crates, Apache-only `adl-resilience`, and bundled MPL-2.0 Vector requires a consolidated third-party notice surface. This is a review cue, not a legal determination.
- The repository records that SBOM/license tooling is unavailable for at least the C-SDLC supply-chain lane; no repository-wide automated license allow/deny proof was observed.

## Validation Performed

- Ran the skill's deterministic `prepare_dependency_review.py` scaffold against the packet; it exposed the lifecycle-lock misclassification and was treated as routing evidence, not as a completed review.
- Used `git show`, `git grep`, and `git ls-tree` against exact revision `c6792e54df1db5969fa28c59b6dfe4c714ed5559` for all cited tracked source evidence.
- Ran `cargo metadata --locked --no-deps --format-version 1` successfully for `adl/Cargo.toml`, `adl-characterization/Cargo.toml`, `adl-resilience/Cargo.toml`, `adl-runtime/Cargo.toml`, `adl-runtime-kernel/Cargo.toml`, `adl-v2/Cargo.toml`, `csdlc-v2/Cargo.toml`, `tools/aws_remote_validation/Cargo.toml`, and `tools/remote_validation/Cargo.toml`. This proves current manifest/lock resolution for those graphs, not vulnerability or license safety.
- Confirmed GitHub Actions `uses:` references in the inventoried workflows are full commit SHAs. No external vulnerability feed, registry mutation, upgrade, or dependency installation was performed for this lane.

## Residual Risk

- No network vulnerability/advisory database, malware feed, signature transparency service, or paid data source was queried.
- No legal determination or exhaustive transitive-license analysis was performed.
- Docker images were not rebuilt, and downloaded executable signatures/checksums were not independently fetched from upstream.
- Nested demo/fixture manifests and retained historical binary evidence were inventoried but not individually built or provenance-audited.
- Successful locked metadata confirms consistency at the reviewed instant; it does not establish MSRV compatibility, reproducible builds, source authenticity, or future registry availability.
