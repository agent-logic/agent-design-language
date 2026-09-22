# v0.92.2 Cargo manifest review

All **27 tracked Cargo.toml files** were parsed and checked with `cargo metadata --offline --locked --no-deps`. All **23 declared local dependency references** resolve to tracked manifests; package names, requested local features, inherited versions and workspace members agree. The repository has no root Cargo.toml; the virtual workspace is `adl-v2/Cargo.toml`.

The [complete audit](CARGO_MANIFEST_AUDIT.json) records each manifest digest and dependency. This is metadata validation, not compilation, full dependency resolution, minimum-Rust-version proof, registry publication readiness or release approval. No Cargo manifest or lockfile was changed.

## Package versions and release disposition

| Manifest | Effective version |
|---|---|
| `.csdlc/prepared/issues/467/terminal-digest-verifier/Cargo.toml` | 0.1.0 |
| `.csdlc/prepared/issues/522/blake3-file/Cargo.toml` | 0.1.0 |
| `.csdlc/prepared/issues/5340/source-authority-validator/Cargo.toml` | 0.1.0 |
| `.csdlc/prepared/issues/5347/receipt-verifier/Cargo.toml` | 0.1.0 |
| `adl-characterization/Cargo.toml` | 0.92.1 |
| `adl-provider-core/Cargo.toml` | 0.1.0 |
| `adl-resilience/Cargo.toml` | 0.92.1 |
| `adl-runtime-kernel/Cargo.toml` | 0.92.1 |
| `adl-runtime/Cargo.toml` | 0.92.1 |
| `adl-uts/Cargo.toml` | 0.1.0 |
| `adl-v2/Cargo.toml` | virtual workspace |
| `adl-v2/crates/adl-adapters/Cargo.toml` | 0.92.1 |
| `adl-v2/crates/adl-cli/Cargo.toml` | 0.92.1 |
| `adl-v2/crates/adl-compiler/Cargo.toml` | 0.92.1 |
| `adl-v2/crates/adl-engine/Cargo.toml` | 0.92.1 |
| `adl-v2/crates/adl-language/Cargo.toml` | 0.92.1 |
| `adl-v2/crates/adl-records/Cargo.toml` | 0.92.1 |
| `adl-v2/crates/adl-runtime-v3-adapter/Cargo.toml` | 0.92.1 |
| `adl-v2/crates/adl-workcell-conductor/Cargo.toml` | 0.92.1 |
| `adl-v2/crates/adl-workcell-convergence/Cargo.toml` | 0.92.1 |
| `adl-v2/crates/adl-workcell-task-adapter/Cargo.toml` | 0.92.1 |
| `adl/Cargo.toml` | 0.92.1 |
| `csdlc-v2/Cargo.toml` | 0.92.1 |
| `csdlc-v3/Cargo.toml` | 0.1.0 |
| `demos/transpiler_demo/Cargo.toml` | 0.8.0 |
| `tools/aws_remote_validation/Cargo.toml` | 0.91.8 |
| `tools/remote_validation/Cargo.toml` | 0.92.1 |

The main Runtime/ADL/v2 package family remains **0.92.1**. Native C-SDLC v3, provider-core, UTS and standalone proof helpers declare **0.1.0**; the demo and AWS validation tool retain their independent versions. The v0.92.2 milestone name does not prove that binaries identify as v0.92.2. Publication finalization #918 must select the actual release artifacts and record any coordinated version/lockfile changes before final candidate freeze; #925 must consume that exact release identity. Do not relabel retained source proof as a new release or bump historical helper packages mechanically.

Declared license and minimum Rust version differ by package family (1.85 and 1.92). Those declarations are recorded metadata, not a new license or compiler-compatibility determination. Explicit readme paths resolve. No current manifest declares a stale legacy repository/homepage URL. Missing optional registry metadata is not treated as proof of publishability.

## Reproduce the manifest audit

From the repository root using Python 3.11+ (tomllib), Cargo and locally available dependency metadata:

```sh
python3 - <<'PYCODE'
import importlib.util
import json
from pathlib import Path
source = Path('.csdlc/prepared/issues/518/audit-cargo-manifests.py')
spec = importlib.util.spec_from_file_location('manifest_audit', source)
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)
result = module.audit()
print(json.dumps({
    'status': result['status'],
    'manifest_count': result['manifest_count'],
    'local_dependency_count': result['local_dependency_count'],
    'release_approved': False,
}, indent=2))
PYCODE
```

This reuses the established manifest checker without its historical report-writing mode; it does not overwrite #518 evidence. The #917 handoff independently derives all tracked Cargo.toml paths and binds their bytes, so a newly added or changed manifest cannot silently escape this inventory.
