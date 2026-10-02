#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

python3 tools/public_adl/test_ci_contract.py
bash tools/public_adl/test_ci_routing.sh

if [[ "$(uname -s)" != Darwin ]] || [[ ! -x /usr/bin/sandbox-exec ]]; then
  echo "public ADL qualification requires macOS sandbox-exec" >&2
  exit 2
fi

manifests=(
  adl-schema/Cargo.toml
  adl-uts/Cargo.toml
  adl-v2/crates/adl-language/Cargo.toml
  adl-v2/crates/adl-compiler/Cargo.toml
  adl-legacy-contracts/Cargo.toml
)
packages=(
  adl-schema
  adl-uts
  adl-language
  adl-compiler
  adl-legacy-contracts
)

# These five package tests populate only their public registry dependency graph.
# The installed-consumer proof then runs offline inside the enforced sandbox.
for index in "${!manifests[@]}"; do
  manifest="${manifests[$index]}"
  package="${packages[$index]}"
  cargo test --locked --manifest-path "$manifest" --package "$package"
  cargo fmt --manifest-path "$manifest" --package "$package" -- --check
  cargo clippy --locked --manifest-path "$manifest" --package "$package" --all-targets -- -D warnings
done

cargo test --offline --locked \
  --manifest-path tools/public_adl/validator/Cargo.toml \
  --test public_adl_install_validator
