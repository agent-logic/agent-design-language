# Exact-revision registered inventory — #899

This companion measures the `adl` package at the two immutable RUST-01 revisions. It supplements the historical #836 source inventory without changing that evidence. Registration, executable-case enumeration, ignored cases and behavioral execution are separate fields. No test bodies run, and no runtime qualification or source-size reduction is claimed.

## Measured result

| Measure | Baseline | Refactor merge |
|---|---:|---:|
| Cargo-registered targets | 33 | 33 |
| Listed harnesses | 33 | 33 |
| Enumerated cases | 2,483 | 2,487 |
| Ignored cases (included above) | 2 | 2 |
| Listed doctests | 0 | 0 |
| Test bodies run | 0 | 0 |

Both complete measurements passed transcript-derived verification. [Comparison](comparison.json) lists the four added resilience cases, no removed cases, and unchanged target registrations. This is an executed inventory process, not an assertion that any listed test passed. The compiler was Rust 1.92.0 on `aarch64-apple-darwin`.

The [bundle index](bundles.json) binds the two compressed transcript/inventory archives. Each archive contains `inventory.json`, the command ledger and all stdout/stderr transcripts. Extract into fresh issue-owned directories before running `verify` or `compare`; extraction does not execute the snapshots or binaries. For example:

```sh
mkdir -p .csdlc/evidence/899/retained-baseline .csdlc/evidence/899/retained-candidate
tar -xzf docs/milestones/v0.92.2/evidence/qual-inventory-899/baseline.tar.gz -C .csdlc/evidence/899/retained-baseline
tar -xzf docs/milestones/v0.92.2/evidence/qual-inventory-899/candidate.tar.gz -C .csdlc/evidence/899/retained-candidate
```

Use those extracted inventory paths with the commands below. The archive files were extracted and both inventories verified again before publication. Snapshot and target paths are normalized as `SNAPSHOT` and `TARGET`; home paths are normalized as `HOME`. Raw source snapshots and compiler products are not publication payloads.

## Reproduce

From an issue-bound checkout, choose fresh issue-owned output directories and a separate issue-owned Cargo target directory. The collector refuses to overwrite an existing output directory. It extracts the full Git revision, uses `adl/Cargo.toml`, default features, Rust 1.92.0 and the recorded host, then runs Cargo metadata and test-harness compilation. The collector rejects Cargo configuration files in the invocation ancestry or Cargo home, drops inherited build/compiler overrides, makes extracted source read-only, and verifies its Git blob identities before and after measurement. Each emitted harness is queried for all and ignored cases; rustdoc is queried separately for doctest names.

```sh
python3 adl/tools/revision_validation_inventory.py collect --repo . --revision a71d699d52831b32bb68ed9c7c7e837925949de4 --out .csdlc/evidence/899/baseline-run --target-dir .csdlc/evidence/899/target
python3 adl/tools/revision_validation_inventory.py collect --repo . --revision e986de6d06aacd385de93dd033def77a718c1581 --out .csdlc/evidence/899/candidate-run --target-dir .csdlc/evidence/899/target
python3 adl/tools/revision_validation_inventory.py verify --repo . --inventory .csdlc/evidence/899/baseline-run/inventory.json --expected-revision a71d699d52831b32bb68ed9c7c7e837925949de4
python3 adl/tools/revision_validation_inventory.py verify --repo . --inventory .csdlc/evidence/899/candidate-run/inventory.json --expected-revision e986de6d06aacd385de93dd033def77a718c1581
python3 adl/tools/revision_validation_inventory.py compare --repo . --baseline .csdlc/evidence/899/baseline-run/inventory.json --candidate .csdlc/evidence/899/candidate-run/inventory.json --out .csdlc/evidence/899/comparison.json
python3 adl/tools/test_revision_validation_inventory.py
```

Builds need local CPU/disk and downloaded Rust dependencies. No provider or paid-cloud calls are part of this profile. The shared owner binary is not replaced. See [proof profile](proof-profile.json) for PVF classification.

## Evidence interpretation

`verify` checks the expected immutable revision and source tree/manifest/lock identities, fixed package/feature profile, collector source digest, built-artifact features, exact command shapes, transcript digests and derived target/case sets. Omitted or duplicate targets, substituted source counts, changed cases and unsupported reduction claims are rejected. `compare` requires the exact baseline/candidate pair and equal compiler/platform descriptions.

These checks establish consistency of local retained measurements, not remote authenticated execution or independent approval. A party able to replace every transcript and its digest can author a different bundle; independent review of actual invocation and source provenance is a separate required acceptance gate. Test names may move across targets, so additions/removals are reported without claiming semantic movement or behavioral reduction.

Default features are the common declared scope. Feature-disabled cases are not enumerated; targets with unsatisfied required features are classified separately. Metadata includes other registered target kinds, but benches/examples and packages other than `adl` are outside this execution profile. Generated cases registered by selected harnesses are included. Rustdoc names are separate and its ignored-case population is not separately classified by the list output. Build/list failures remain incomplete with unavailable counts and cannot pass verification.

## Existing regression limitation

The unchanged `bash adl/tools/test_validation_inventory.sh` was also attempted. It fails because its assertions still require the retired `adl/src/bin/adl_pr_finish.rs` (and other former C-SDLC binary paths) in the source inventory. The path is absent at the current source head. This inherited failure is retained separately in the issue evidence; it is not reported as a passing regression or repaired by silently widening the companion's scope.
