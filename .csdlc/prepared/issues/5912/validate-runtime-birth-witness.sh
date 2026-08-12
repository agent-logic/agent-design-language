#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test birth_witness runtime_service_builds_validates_and_emits_receipt -- --exact
cargo test --manifest-path adl-runtime-kernel/Cargo.toml birth_witness --lib
cargo test --manifest-path adl-runtime/Cargo.toml --test guardian_cli guardian_cli_reports_successful_portable_child_as_json -- --exact
cargo check --manifest-path adl-runtime/Cargo.toml --bin adl-runtime-lifecycle-soak
