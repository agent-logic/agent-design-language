# Raw-host bootstrap identities

Raw-host validation trusts the operator-approved checkout (the launcher clones
and checks it out before invoking `scripts/remote_validation_runner.sh`). That
checkout contains the runner and `scripts/verified_bootstrap.py`. Review the
checkout before running it; this is not a sandbox for hostile repositories.

Downloaded executables now require an approved identity manifest. The default
path is `tools/aws_remote_validation/bootstrap-identities.json` in that checkout;
`ADL_BOOTSTRAP_IDENTITIES` may select an operator-provisioned remote file. No
manifest with guessed digests is shipped. A cold host without Cargo therefore
fails closed until a reviewed rustup identity is provisioned, or an approved
host image/package source provides Cargo.

The manifest has schema `adl.remote-bootstrap.v1` and `tools` keyed by `sccache`,
`cargo-nextest`, or `rustup-init`, then by `x86_64-unknown-linux-gnu` or
`aarch64-unknown-linux-gnu`. Each identity requires:

- `version`: exact three-component release version, optionally prefixed by `v`.
- `sha256`: lowercase 64-character digest independently approved by the operator.
- `source`: `https` or `s3`.
- For HTTPS: `url`, an HTTPS artifact URL. `latest` and `stable` path segments
  are rejected. Redirects must remain HTTPS; bytes must match the approved hash.
- For S3: `bucket`, `key`, and a nonempty, non-`null` `version_id`. Downloads use
  `s3api get-object --version-id`, never the current unversioned object.
- For `rustup-init`: `toolchain`, an exact Rust release such as `1.92.0`. Its
  artifact is the executable installer, not a shell script or archive.

Hashes and version IDs must be recorded from a trusted release/provenance review,
not discovered from whatever the remote host downloads during the same run.
For S3, the AWS CLI and authenticated business-account host configuration must
already be available. The manifest must include every artifact needed by that
host and architecture. The existing tarball URL options remain supported only
when they exactly match their approved HTTPS identity; they do not supply trust.

An explicit manifest or tarball URL takes precedence over a preinstalled binary.
Any missing identity, mismatch, malformed archive, failed download, or failed
install terminates the run. There is no retry through another source and no
implicit upload or reuse of mutable tool archives in the build-cache bucket.
The bucket can still serve compiled-object caching through sccache.

Without a manifest or tarball URL, already-installed sccache/nextest or the
host's package-manager repositories remain supported. This explicitly trusts
that host and package-manager signing/configuration, and **does not claim pinned
package reproducibility**. Cargo already present on the host is also trusted;
when absent, rustup requires the approved manifest. Container and retained
Runtime qualification paths keep their existing image/package trust boundaries.

Before archive processing, the helper checks SHA-256 and compressed size.
It rejects absolute/traversal paths, links, special files, duplicate names,
multiple matching binaries, missing binaries, excessive member counts and
expanded size. It copies only the selected regular file to a private temporary
directory and atomically replaces the destination after validation. It never
executes downloaded sccache/nextest during extraction. Rustup is executed only
after its exact digest matches, with the specified toolchain.

Validation: `python3 tools/aws_remote_validation/scripts/test_verified_bootstrap.py`.
These are deterministic local contract and negative-regression tests, using
synthetic archives and mocked downloads/processes; they do not install software,
contact AWS, validate live upstream artifacts, or prove remote workload success.
PVF classification is in `scripts/verified_bootstrap.pvf.json`.
