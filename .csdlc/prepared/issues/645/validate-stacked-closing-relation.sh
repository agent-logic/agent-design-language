#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT"

export TMPDIR="$ROOT/.csdlc/tmp"
mkdir -p "$TMPDIR"

cargo test --manifest-path csdlc-v2/Cargo.toml --bin csdlc-publish stacked_closing_publication_without_remote_relation_fails_closed
