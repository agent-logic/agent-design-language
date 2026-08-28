#!/usr/bin/env bash
set -euo pipefail

manifest="adl-runtime-kernel/Cargo.toml"

cargo nextest run --locked --manifest-path "$manifest" --test configuration --no-tests=fail -E 'test(polis_identity)'
cargo nextest run --locked --manifest-path "$manifest" --test control --no-tests=fail -E 'test(polis_identity)'
cargo nextest run --locked --manifest-path "$manifest" --test observatory --no-tests=fail -E 'test(polis_identity)'
cargo nextest run --locked --manifest-path "$manifest" --test openapi_contract --no-tests=fail -E 'test(polis_identity)'
