#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "$0")/../.." && pwd -P)
fast_root=${ADL_RUNTIME_V3_PROOF_ROOT:-/Volumes/FastWork/adl-runtime-v3-proof}
export CARGO_HOME=${CARGO_HOME:-$fast_root/cargo-home}
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$fast_root/target}

vector_binary=${ADL_RUNTIME_V3_VECTOR_BINARY:?set ADL_RUNTIME_V3_VECTOR_BINARY to the production Vector binary}
init_template=${ADL_RUNTIME_V3_INIT_TEMPLATE:-$repo_root/infra/runtime-v3/runtime-init.toml}

for path in "$vector_binary" "$init_template"; do
  test -f "$path" || { echo "required proof input is not a file: $path" >&2; exit 64; }
done

mkdir -p "$fast_root" "$CARGO_HOME" "$CARGO_TARGET_DIR"
proof_root=$(mktemp -d "$fast_root/run.XXXXXX")
mkdir -p "$proof_root/state"

cargo build --locked --manifest-path "$repo_root/adl-runtime-kernel/Cargo.toml" --bin adl-runtime-kernel
cargo build --locked --manifest-path "$repo_root/adl-runtime/Cargo.toml" \
  --bin adl-runtime-guardian --bin adl-runtime-lifecycle-soak

revision=$(git -C "$repo_root" rev-parse HEAD)
"$CARGO_TARGET_DIR/debug/adl-runtime-lifecycle-soak" \
  --guardian "$CARGO_TARGET_DIR/debug/adl-runtime-guardian" \
  --kernel "$CARGO_TARGET_DIR/debug/adl-runtime-kernel" \
  --vector "$vector_binary" \
  --init-template "$init_template" \
  --state-root "$proof_root/state" \
  --report "$proof_root/report.json" \
  --revision "$revision" \
  --suite preflight

printf 'runtime_v3_operational_proof=pass evidence=%s\n' "$proof_root/report.json"
