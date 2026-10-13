#!/usr/bin/env bash
# PVF local_contract: public CLI/dependency/dispatch proof; bounded local CPU/disk,
# deterministic fixtures, no provider calls; required public consumer gate.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"
metadata="$(mktemp "${TMPDIR:-/tmp}/adl-public-metadata.XXXXXX")"
trap 'rm -f "$metadata"' EXIT
host="$(rustc -vV | sed -n 's/^host: //p')"
cargo metadata --locked --manifest-path adl/Cargo.toml --format-version 1 --filter-platform "$host" >"$metadata"
python3 - "$metadata" <<'PY'
import json, sys
from pathlib import Path
metadata = json.loads(Path(sys.argv[1]).read_text())
allowed_local = {'adl', 'adl-schema', 'adl-legacy-contracts'}
local = {p['name'] for p in metadata['packages'] if p['source'] is None}
if local != allowed_local:
    raise SystemExit(f'public CLI local dependency closure changed: {sorted(local)}')
print('public CLI private dependency count: 0')
PY
cargo fmt --manifest-path adl/Cargo.toml --package adl -- --check
cargo clippy --locked --manifest-path adl/Cargo.toml --package adl --lib --bins \
  --test adl_tests --test resolve_tests --test schema_tests --test public_process \
  --test issue_327_removed_tooling --test public_observation -- -D warnings

# Owner integration suites retain separate obligations during the split. This
# lane names the public contract tests; it does not classify private tests as passed.
cargo test --locked --manifest-path adl/Cargo.toml --lib --bins \
  --test adl_tests --test resolve_tests --test schema_tests --test public_process \
  --test issue_327_removed_tooling --test public_observation
binary="$(python3 - "$metadata" <<'PY'
import json, sys
from pathlib import Path
print(Path(json.loads(Path(sys.argv[1]).read_text())['target_directory'])/'debug'/'adl')
PY
)"
python3 tools/public_adl/test_public_runtime_dispatch.py --binary "$binary"
python3 tools/public_adl/test_public_cli_refusals.py --binary "$binary"
python3 tools/public_adl/test_public_session.py --binary "$binary"
ADL_PUBLIC_CLI="$binary" python3 tools/public_adl/test_public_codefriend_dispatch.py
python3 tools/public_adl/test_runtime_owner.py
python3 tools/public_adl/test_codefriend_owner.py
bash adl/tools/test_owner_binary_install.sh

# Version truth consumes the selected public build, never the docs-only job.
bash adl/tools/check_release_version_surfaces.sh
