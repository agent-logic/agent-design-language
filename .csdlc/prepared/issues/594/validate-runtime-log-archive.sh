#!/usr/bin/env bash
set -euo pipefail

cargo nextest run \
  --locked \
  --manifest-path adl-runtime-kernel/Cargo.toml \
  --test observability \
  --test guardian_soak \
  --no-tests=fail \
  -E 'test(s3_archive)'
