#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)
MANIFEST="$ROOT/tools/aws_remote_validation/Cargo.toml"
SOURCE_BIN="$ROOT/tools/aws_remote_validation/target/debug/adl-aws-remote-validation"
DEST_DIR=${ADL_OWNER_BIN_DIR:-$ROOT/.adl/bin}
DEST="$DEST_DIR/adl-aws-remote-validation-tool"
PROVENANCE_DIR="$DEST_DIR/.provenance"

source_hash=$(
  cd "$ROOT"
  git ls-files --cached --others --exclude-standard -- \
    tools/aws_remote_validation/Cargo.toml \
    tools/aws_remote_validation/Cargo.lock \
    tools/aws_remote_validation/src \
    tools/remote_validation/Cargo.toml \
    tools/remote_validation/src |
    LC_ALL=C sort |
    while IFS= read -r path; do
      [[ -f "$path" ]] && shasum -a 256 "$path"
    done |
    shasum -a 256 |
    awk '{print $1}'
)

if [[ -x "$DEST" && -f "$PROVENANCE_DIR/adl-aws-remote-validation-tool.sha256" \
    && "$(<"$PROVENANCE_DIR/adl-aws-remote-validation-tool.sha256")" == "$source_hash" ]]; then
  echo "aws-remote-validation-tool unchanged"
  exit 0
fi

cargo build --quiet --locked --manifest-path "$MANIFEST" --bin adl-aws-remote-validation
[[ -x "$SOURCE_BIN" ]] || { echo "aws remote validation tool build output missing" >&2; exit 1; }
mkdir -p "$PROVENANCE_DIR"
tmp=$(mktemp "$DEST_DIR/.adl-aws-remote-validation-tool.XXXXXX")
cp "$SOURCE_BIN" "$tmp"
chmod 0755 "$tmp"
mv "$tmp" "$DEST"
printf '%s\n' "$source_hash" >"$PROVENANCE_DIR/adl-aws-remote-validation-tool.sha256"
printf '{"binary":"adl-aws-remote-validation-tool","source_hash":"%s","manifest":"tools/aws_remote_validation/Cargo.toml"}\n' \
  "$source_hash" >"$PROVENANCE_DIR/adl-aws-remote-validation-tool.json"
echo "aws-remote-validation-tool installed: $DEST"
