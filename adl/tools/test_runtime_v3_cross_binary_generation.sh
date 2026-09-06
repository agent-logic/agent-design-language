#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT_DIR/.adl/tmp/runtime-v3-real-generation-test"
TARGET_DIR="$FIXTURE/target"
INSTALL_ROOT="$FIXTURE/install"
rm -rf "$FIXTURE"
mkdir -p "$TARGET_DIR" "$INSTALL_ROOT"
trap 'rm -rf "$FIXTURE"' EXIT

CARGO_TARGET_DIR="$TARGET_DIR" cargo build --locked --manifest-path "$ROOT_DIR/adl/Cargo.toml" --bin csm
CARGO_TARGET_DIR="$TARGET_DIR" cargo build --locked --manifest-path "$ROOT_DIR/adl-runtime/Cargo.toml" --bin adl-runtime-guardian
CARGO_TARGET_DIR="$TARGET_DIR" cargo build --locked --manifest-path "$ROOT_DIR/adl-runtime-kernel/Cargo.toml" --bin adl-runtime-kernel

"$ROOT_DIR/adl/tools/install_runtime_v3_generation.sh" install \
  --root "$INSTALL_ROOT" \
  --generation real-cross-binary \
  --csm "$TARGET_DIR/debug/csm" \
  --guardian "$TARGET_DIR/debug/adl-runtime-guardian" \
  --kernel "$TARGET_DIR/debug/adl-runtime-kernel" \
  --source-revision "$(git -C "$ROOT_DIR" rev-parse HEAD)" \
  --build-profile debug >/dev/null

"$ROOT_DIR/adl/tools/install_runtime_v3_generation.sh" verify --root "$INSTALL_ROOT" >/dev/null

printf '\0' >>"$INSTALL_ROOT/generations/real-cross-binary/bin/adl-runtime-kernel"
if "$ROOT_DIR/adl/tools/install_runtime_v3_generation.sh" verify --root "$INSTALL_ROOT" >/dev/null 2>&1; then
  echo "mixed real CSM/Guardian/Kernel generation was accepted" >&2
  exit 1
fi

echo "runtime v3 real cross-binary generation parity: PASS"
