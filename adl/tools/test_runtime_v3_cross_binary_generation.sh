#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE="$ROOT_DIR/.adl/tmp/runtime-v3-real-generation-test"
TARGET_DIR="$ROOT_DIR/.adl/cache/runtime-v3-cross-binary-target"
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

STATE_ROOT="$FIXTURE/state"
INIT="$FIXTURE/runtime-init.toml"
mkdir -p "$STATE_ROOT"
sed \
  -e "s#/var/lib/adl/runtime-v3#$STATE_ROOT#g" \
  -e "s#/opt/adl/bin/adl-runtime-kernel#$INSTALL_ROOT/current/bin/adl-runtime-kernel#g" \
  "$ROOT_DIR/infra/runtime-v3/runtime-init.toml" >"$INIT"

"$INSTALL_ROOT/current/bin/csm" runtime-v3 prepare-config --init "$INIT" >/dev/null
ADL_RUNTIME_V3_CONFIG_IDENTITY_CHECK=1 \
  "$INSTALL_ROOT/current/bin/adl-runtime-guardian" --init "$INIT" >/dev/null

cp "$INIT" "$INIT.saved"
printf '\n# content mismatch\n' >>"$INIT"
if ADL_RUNTIME_V3_CONFIG_IDENTITY_CHECK=1 \
  "$INSTALL_ROOT/current/bin/adl-runtime-guardian" --init "$INIT" >/dev/null 2>&1; then
  echo "Guardian accepted Runtime init content that disagreed with CSM receipt" >&2
  exit 1
fi
cp "$INIT.saved" "$INIT"

ACTIVE_REF="$FIXTURE/.runtime-init.toml.active-generation"
GENERATION="$(cut -d ' ' -f 1 "$ACTIVE_REF")"
RECEIPT="$FIXTURE/.runtime-config-generations/$GENERATION.json"
cp "$RECEIPT" "$RECEIPT.saved"
sed -e 's/real-cross-binary/incompatible-generation/' "$RECEIPT.saved" >"$RECEIPT"
if ADL_RUNTIME_V3_CONFIG_IDENTITY_CHECK=1 \
  "$INSTALL_ROOT/current/bin/adl-runtime-guardian" --init "$INIT" >/dev/null 2>&1; then
  echo "Guardian accepted a receipt for an incompatible binary generation" >&2
  exit 1
fi
cp "$RECEIPT.saved" "$RECEIPT"

printf '{not-json' >"$RECEIPT"
if ADL_RUNTIME_V3_CONFIG_IDENTITY_CHECK=1 \
  "$INSTALL_ROOT/current/bin/adl-runtime-guardian" --init "$INIT" >/dev/null 2>&1; then
  echo "Guardian accepted a malformed active configuration receipt" >&2
  exit 1
fi

echo "runtime v3 real cross-binary generation parity: PASS"
