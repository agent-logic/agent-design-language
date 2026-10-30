#!/usr/bin/env bash
# Shared source validation for the five retained public library/compiler packages.
# Installed-consumer qualification remains the responsibility of run_ci.sh.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

manifests=(adl-schema/Cargo.toml adl-uts/Cargo.toml adl-v2/crates/adl-language/Cargo.toml adl-v2/crates/adl-compiler/Cargo.toml adl-legacy-contracts/Cargo.toml)
packages=(
  adl-schema
  adl-uts
  adl-language
  adl-compiler
  adl-legacy-contracts
)
for index in "${!manifests[@]}"; do
  manifest="${manifests[$index]}"
  package="${packages[$index]}"
  cargo test --locked --manifest-path "$manifest" --package "$package"
  cargo fmt --manifest-path "$manifest" --package "$package" -- --check
  cargo clippy --locked --manifest-path "$manifest" --package "$package" --all-targets -- -D warnings
done
