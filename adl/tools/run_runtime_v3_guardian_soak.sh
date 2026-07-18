#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
REPORT_PATH="${ADL_RUNTIME_V3_SOAK_REPORT:-${ROOT_DIR}/.adl/reports/runtime-v3/guardian-soak.json}"

mkdir -p "$(dirname "${REPORT_PATH}")"
ADL_RUNTIME_V3_SOAK_REPORT="${REPORT_PATH}" \
  cargo test \
    --manifest-path "${ROOT_DIR}/adl-runtime-kernel/Cargo.toml" \
    --test guardian_soak \
    bounded_runtime_v3_guardian_soak \
    -- \
    --exact \
    --ignored \
    --nocapture

test -s "${REPORT_PATH}"
python3 -m json.tool "${REPORT_PATH}" >/dev/null
printf 'runtime_v3_guardian_soak=pass report=%s\n' "${REPORT_PATH}"
