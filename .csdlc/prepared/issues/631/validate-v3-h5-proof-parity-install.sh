#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
ROOT_DIR="$(cd "$ROOT_DIR/.." && pwd)"

cd "$ROOT_DIR"

fail() {
  echo "validate-v3-h5-proof-parity-install: $*" >&2
  exit 1
}

[[ -f csdlc-v3/src/main.rs ]] || fail "missing v3 single-binary entrypoint"
[[ -f docs/csdlc-v3/v3-command-manifest.json ]] || fail "missing v3 command manifest"

if git diff --name-only origin/main...HEAD -- csdlc-v2 | grep -q .; then
  fail "csdlc-v2 source changed in #631 scope"
fi

grep -Fq '"proof"' docs/csdlc-v3/v3-command-manifest.json || fail "proof route missing from manifest"
grep -Fq '"shadow"' docs/csdlc-v3/v3-command-manifest.json || fail "shadow route missing from manifest"
grep -Fq '"soak"' docs/csdlc-v3/v3-command-manifest.json || fail "soak route missing from manifest"
grep -Fq '"install"' docs/csdlc-v3/v3-command-manifest.json || fail "install route missing from manifest"

cargo test --manifest-path csdlc-v3/Cargo.toml --test proof_parity_install_commands

echo "validate-v3-h5-proof-parity-install: passed"
