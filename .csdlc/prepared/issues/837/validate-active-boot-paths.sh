#!/usr/bin/env bash
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
cd "$root"

# PVF: review_docs; release-gating deterministic local contract; small CPU and
# filesystem profile; no network, cloud, credentials, or live Runtime required.
cargo test --manifest-path csdlc-v3/Cargo.toml --test command_manifest
bash adl/tools/test_editor_action.sh
git diff --check
