#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

cargo test --locked --manifest-path tools/ci_contracts/Cargo.toml --test workflow
bash tools/public_adl/test_ci_routing.sh
bash tools/public_adl/run_cli_ci.sh

if [[ "$(uname -s)" != Darwin ]] || [[ ! -x /usr/bin/sandbox-exec ]]; then
  echo "public ADL qualification requires macOS sandbox-exec" >&2
  exit 2
fi

cargo test --locked --manifest-path tools/public_contracts/Cargo.toml

# These five package tests populate only their public registry dependency graph.
# The installed-consumer proof then runs offline inside the enforced sandbox.
bash tools/public_adl/run_package_checks.sh

cargo test --offline --locked \
  --manifest-path tools/public_adl/validator/Cargo.toml \
  --test public_adl_install_validator
