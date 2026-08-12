#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test birth_witness runtime_service_builds_validates_and_emits_receipt -- --exact
