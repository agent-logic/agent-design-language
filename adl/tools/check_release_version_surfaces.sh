#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

native=(cargo run --locked --quiet --manifest-path tools/public_contracts/Cargo.toml --bin adl-public-contracts --)
manifest_version="$("${native[@]}" release-version-source adl/Cargo.toml adl/Cargo.lock)"
lock_version="$manifest_version"
metadata="$(mktemp "${TMPDIR:-/tmp}/adl-release-version.XXXXXX")"
trap 'rm -f "$metadata"' EXIT
cargo metadata --manifest-path adl/Cargo.toml --no-deps --format-version 1 >"$metadata"
"${native[@]}" release-version-metadata "$manifest_version" "$metadata" >/dev/null

cli_version="$(cargo run -q --manifest-path adl/Cargo.toml -- --version)"
if [ "$cli_version" != "$manifest_version" ]; then
  echo "release-version check: CLI reports $cli_version but manifest says $manifest_version" >&2
  exit 1
fi

echo "PASS release-version-surfaces manifest=$manifest_version lock=$lock_version cli=$cli_version"
