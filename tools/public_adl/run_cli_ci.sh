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
native=(cargo run --locked --quiet --manifest-path tools/public_contracts/Cargo.toml --bin adl-public-contracts --)
"${native[@]}" metadata-check --input "$metadata"
cargo fmt --manifest-path adl/Cargo.toml --package adl -- --check
cargo clippy --locked --manifest-path adl/Cargo.toml --package adl --lib --bins \
  --test adl_tests --test resolve_tests --test schema_tests --test public_process \
  --test issue_327_removed_tooling --test public_observation -- -D warnings

# Owner integration suites retain separate obligations during the split. This
# lane names the public contract tests; it does not classify private tests as passed.
cargo test --locked --manifest-path adl/Cargo.toml --lib --bins \
  --test adl_tests --test resolve_tests --test schema_tests --test public_process \
  --test issue_327_removed_tooling --test public_observation
binary="$("${native[@]}" metadata-binary --input "$metadata")"
for contract in runtime-dispatch cli-refusals session codefriend-dispatch; do
  "${native[@]}" "$contract" --binary "$binary"
done
cargo build --locked --quiet --manifest-path tools/public_contracts/Cargo.toml --bins
"${native[@]}" runtime-owner
"${native[@]}" codefriend-owner
bash adl/tools/test_owner_binary_install.sh

# Version truth consumes the selected public build, never the docs-only job.
bash adl/tools/check_release_version_surfaces.sh
