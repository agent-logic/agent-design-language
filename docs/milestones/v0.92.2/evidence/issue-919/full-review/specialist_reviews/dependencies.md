# Dependencies and supply chain review

Candidate ADL `5c4a6149771c637f3c805985b86231077965eab4`; referenced website `a45e339c13b24716edbd3fadf29dddff36ffe02e`.

## Findings

**DEP-001 P2 — privileged website deployment executes mutable action tags.** Website `.github/workflows/deploy-site.yml:30–33` loads `actions/checkout@v4` and `aws-actions/configure-aws-credentials@v4` in a workflow with `id-token: write`. Its exact-branch OIDC trust limits who invokes the role but does not bind these action bytes. The role can write/delete site objects and invalidate CloudFront. A retargeted tag changes executed privileged code without changing the reviewed website commit. Pin reviewed SHA identities. This is a retained integration-snapshot weakness, not an allegation of compromise or a new ADL regression.

**DEP-002 P1 — canonical builder reconstruction remains unpinned.** Rechecked historical SYN-005 against the current Dockerfile and setup route: mutable Ubuntu/Rust stable and unverified direct executable downloads remain. Digest-pinned consumption does not prove reproducible construction. Preserve historical P1 and route to build owner.

**DEP-003 P1 — raw-host tool bootstrap still installs unverified archives.** Rechecked historical SYN-004 at remote_validation_runner.sh230–333 and414–450: latest GitHub, supplied tarball and mutable S3 fallback bytes install without checksum/signature binding. Applies when non-containerized hosts need tools, not every remote run. Static proof; no cloud effects.

**DEP-004 P2 — generic dependency scaffold still selects lifecycle locks.** Fresh scaffold generation produced80 lifecycle lock entries and zero Cargo manifests/locks, reproducing historical SYN-007. Our explicit inventory replaces the faulty assignment locally; it does not fix the generic helper. This is review infrastructure, separate from product defects.

## Observations and coverage

Seven component lockfiles were parsed completely for registry source/checksum integrity. All registry entries have checksums, no Git dependencies were present, and local package identities match the split provider/runtime/UTS topology. Seven offline, locked, no-dependency metadata commands passed with unchanged lockfile bytes; they are metadata validation, not build or MSRV proof.

Reviewed changed manifests and bridge declarations, new pinned ADL workflows, dependency-setup sections of main CI, package assembly/build provenance, Terraform provider selection, and referenced website package manifests/lock/workflows/IAM. ADL new workflows pin actions and Rust1.92 and build with locked inputs. Website production npm packages are exact and all resolved packages retain integrity; CI uses npm ci with scripts disabled. Host assembly explicitly records dependencies not installed and refuses activation, so it does not falsely attest to a usable deployed package. Terraform locks pin AWS5.100.0 and archive2.8.1 with platform hashes; host AMI is explicitly selected.

Vector installer pins platform archive hashes and retains executable provenance. The unchanged builder still fetches some unverified download bytes and permits floating Ubuntu/stable inputs; see DEP-002; retain the original historical P1 severity. Runtime focused CI omits --locked while other key lanes use it: this allows lock updates if inputs drift, but no current lock drift was demonstrated, so no current build-break finding is asserted.

## Independence and limits

The lead authored issue917 bridge and cannot independently approve its behavior. Its dependency-free manifest shape is recorded here; independent tests/docs lanes own behavior review. No dependency source audit, CVE feed, license legal determination, clean-room build, package installation, cloud mutation, or deployed runtime claim was performed. Coverage ledger and metadata/lock proofs accompany this report.
