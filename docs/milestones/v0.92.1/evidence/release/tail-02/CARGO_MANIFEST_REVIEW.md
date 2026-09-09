# Cargo manifest review — issue #518

This extends the initial three-manifest sample to **all 24 tracked Cargo.toml
files** at this branch revision. It is a separate denominator from the retained
737-document initial audit; that historical inventory has not been rewritten.

All 24 parse and pass `cargo metadata --offline --locked --no-deps`. All 20 local
dependency references, including dev/build, target-specific, workspace and patch
sections where present, resolve to tracked package manifests. Declared local
package names and requested local features agree with their targets. Cargo
resolves inherited package versions and workspace membership successfully.

The deterministic inventory is [cargo-manifest-audit.json](cargo-manifest-audit.json).
Reproduce it with `python3 .csdlc/prepared/issues/518/audit-cargo-manifests.py --check`
from the repository. This is a small, deterministic local documentation/PVF lane;
it uses no network or builds and cannot approve final release acceptance.

## Version observations

### 0.1.0

- `.csdlc/prepared/issues/467/terminal-digest-verifier/Cargo.toml`
- `.csdlc/prepared/issues/5340/source-authority-validator/Cargo.toml`
- `.csdlc/prepared/issues/5347/receipt-verifier/Cargo.toml`
- `csdlc-v3/Cargo.toml`

### 0.8.0

- `demos/transpiler_demo/Cargo.toml`

### 0.91.8

- `tools/aws_remote_validation/Cargo.toml`

### 0.92.0

- `adl-characterization/Cargo.toml`
- `adl-resilience/Cargo.toml`
- `adl-runtime-kernel/Cargo.toml`
- `adl-runtime/Cargo.toml`
- `adl-v2/crates/adl-adapters/Cargo.toml`
- `adl-v2/crates/adl-cli/Cargo.toml`
- `adl-v2/crates/adl-compiler/Cargo.toml`
- `adl-v2/crates/adl-engine/Cargo.toml`
- `adl-v2/crates/adl-language/Cargo.toml`
- `adl-v2/crates/adl-records/Cargo.toml`
- `adl-v2/crates/adl-runtime-v3-adapter/Cargo.toml`
- `adl-v2/crates/adl-workcell-conductor/Cargo.toml`
- `adl-v2/crates/adl-workcell-convergence/Cargo.toml`
- `adl-v2/crates/adl-workcell-task-adapter/Cargo.toml`
- `adl/Cargo.toml`
- `csdlc-v2/Cargo.toml`
- `tools/remote_validation/Cargo.toml`

### virtual workspace

- `adl-v2/Cargo.toml`

## Release disposition and limits

No manifest version was changed. `0.92.0` is the effective version for the main
Runtime/v2 package family; v3, standalone proof helpers, the transpiler demo and
the AWS validation tool retain their own declared versions. A milestone number
is not an instruction to mechanically bump every package. #519 publication
finalization must identify release artifacts and document any required coordinated
version change; this review records current values without approving them for a
new release. There is no Cargo.toml at the repository root.

`--no-deps` verifies Cargo's manifest/workspace model, not compilation or complete
dependency resolution. No registry freshness, vulnerability, license, feature
combination, full lockfile reproducibility or package-publishing test was run.
Those claims remain outside this documentation check. The #518 handoff includes #517's reviewed merge and preserves its blocked
release decision; this manifest check grants no release approval.
