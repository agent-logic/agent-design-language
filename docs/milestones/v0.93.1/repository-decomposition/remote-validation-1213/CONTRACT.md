# RD06 portable remote-validation distribution

ADL issue #1213 supplies the missing producer dependency for
`agent-logic/agent-logic-infrastructure#1`. The five existing RD03 distributions
and their acceptance remain unchanged.

The source archive is tracked under `tools/remote_validation_distribution/`:

- Package: `adl-remote-validation` version `0.92.1`.
- Source revision: `248f00e359e412f6bc0061ac88be9ff374e3a212`.
- Archive: `adl-remote-validation-0.92.1-source.tar.gz`, 15,289 bytes.
- SHA-256: `9ab5247741d12768f0e2bb866f59bf8ca4748bdb01005003c54b78dc100580f1`.
- `source-manifest.json` binds all five original files and `LICENSE` by Git blob,
  size and SHA-256. No source implementation has been changed.

This is a source distribution, not a crates.io release or platform executable.
The original crate includes local execution helpers as well as the portable
contract; this does not grant execution authority. The original Apache-2.0
license text and `MIT OR Apache-2.0` package metadata are preserved without a
new license grant. The manifest's `candidate_not_accepted` field describes its
original creation boundary. Acceptance must be established separately by the
reviewed producer merge and exact artifact digest; the manifest cannot grant it.

## Consumer procedure

Obtain the archive, verifier and source manifest from the same accepted producer
commit. Pin that commit and the archive digest in the consumer's dependency
record. Do not treat this document's presence on an unmerged branch as acceptance.

```sh
python3 -B tools/remote_validation_distribution/package.py verify \
  --artifact tools/remote_validation_distribution/adl-remote-validation-0.92.1-source.tar.gz \
  --extract /absolute/new/dependency-directory
```

The verifier checks the whole archive digest, exact entry inventory, file sizes,
payload hashes and package identity before writing files. It rejects links,
duplicate entries, traversal, omitted files and an existing extraction destination.
Use a new operator-owned destination. The verifier and pinned manifest are trusted
producer inputs; a self-supplied manifest is not an authentication mechanism.

Replace Infrastructure's producer-source include with its dependency on the
verified extracted `adl-remote-validation-0.92.1` directory, version `=0.92.1`.
The library exports `PortableRequest`, `PortableResult`, `AdapterKind`,
`AdapterPlan`, `adapter_plan` and `validate_result`. Registry dependencies remain
governed by the consumer lockfile. Do not copy ADL implementation into the
Infrastructure source tree.

## Reproduction and proof

```sh
python3 -B tools/remote_validation_distribution/package.py export \
  --repository . --output /absolute/new/export-directory
cargo test --offline --locked \
  --manifest-path tools/remote_validation_distribution/Cargo.toml --test distribution
python3 -B tools/remote_validation_distribution/verify_consumer.py \
  --output /absolute/new/consumer-proof-directory
```

Export requires the frozen Git objects. Compression output must match the pinned
archive or reproduction fails; it never silently changes the accepted digest.
Verification/extraction does not require a producer checkout or Git history.
Python 3.11+ and Rust 1.92+ are required for these proof tools. Offline consumer
proof requires the public registry dependencies already cached. The consumer uses
the tracked lockfile and records its digest and actual Rust version.

Five Cargo harness tests invoke five independent package boundary cases, including
five unsafe-entry variants. The artifact consumer executes the required API,
preserves revision/source-ref/budget/cancellation, and rejects wrong adapter and
result revision. It builds only against the extracted library plus registry crates;
metadata checks reject unexpected local dependencies. This is not an OS-sandbox
or denied-filesystem-access proof.

The unchanged original `tests/contract.rs` requires a Git fixture retaining the
`tools/remote_validation` layout; its twelve tests were run in that fixture.
Standalone compilation and artifact-consumer execution are separate evidence.
No cloud calls, deployed Runtime changes or installed Linux qualification are
claimed. CI runs the distribution tests and artifact consumer on Linux; the
result of that actual run must be recorded before claiming it passed.
