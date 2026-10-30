#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# Keep this list aligned with the public adl-v2 workspace. Private source
# directories may remain as historical inputs during repository retirement.
manifests=(
  "$root/adl-v2/crates/adl-language/Cargo.toml"
  "$root/adl-v2/crates/adl-compiler/Cargo.toml"
)

# Check the complete public input set before invoking any validation command.
for manifest in "${manifests[@]}"; do
  if [ ! -f "$manifest" ]; then
    echo "missing public ADL v2 crate manifest: ${manifest#"$root/"}" >&2
    exit 2
  fi
done

for manifest in "${manifests[@]}"; do
  relative_manifest="${manifest#"$root/"}"
  echo "Validating $relative_manifest"
  bash "$root/adl/tools/run_cargo_validation.sh" cargo test --locked --manifest-path "$manifest"
  bash "$root/adl/tools/run_cargo_validation.sh" cargo fmt --manifest-path "$manifest" --all -- --check
  bash "$root/adl/tools/run_cargo_validation.sh" cargo clippy --locked --manifest-path "$manifest" --all-targets -- -D warnings
done
