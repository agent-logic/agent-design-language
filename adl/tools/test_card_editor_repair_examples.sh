#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if [[ -e adl/tools/validate_structured_prompt.sh ]]; then
  echo "assertion failed: sunset structured-prompt validator was restored" >&2
  exit 1
fi

cargo run --quiet --locked --manifest-path csdlc-v2/Cargo.toml --bin csdlc-edit -- --help >/dev/null
cargo run --quiet --locked --manifest-path csdlc-v2/Cargo.toml --bin csdlc-validate -- --help >/dev/null

rg -q 'csdlc-edit' csdlc-v2/operator/skills/csdlc-v2-card-editor/SKILL.md
rg -q 'csdlc-validate' csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md

echo "PASS: card repair is owned by typed csdlc-edit plus csdlc-validate"
