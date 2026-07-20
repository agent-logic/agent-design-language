#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SOURCE_ROOT="${ROOT_DIR}/adl-runtime-kernel/src"
TEST_ROOT="${ROOT_DIR}/adl-runtime-kernel/tests"
CHALLENGE_TARGET=10000
REVIEWED_TARGET=12000
EXCEPTION_CEILING=20000
TEST_CEILING=1000

physical_lines="$({ find "${SOURCE_ROOT}" -type f -name '*.rs' -print0 | xargs -0 wc -l; } | awk 'END {print $1}')"
if [[ ! "${physical_lines}" =~ ^[0-9]+$ ]]; then
  echo "runtime_v3_loc=error reason=invalid_count" >&2
  exit 1
fi

test_count="$({ find "${SOURCE_ROOT}" "${TEST_ROOT}" -type f -name '*.rs' -print0 | xargs -0 grep -hEc '#\[(tokio::)?test\]'; } | awk '{ total += $1 } END { print total + 0 }')"
if [[ ! "${test_count}" =~ ^[0-9]+$ ]]; then
  echo "runtime_v3_loc=error reason=invalid_test_count" >&2
  exit 1
fi

disposition=within_challenge
if (( physical_lines > CHALLENGE_TARGET )); then
  disposition=within_reviewed_target
fi
if (( physical_lines > REVIEWED_TARGET )); then
  disposition=reviewed_exception_required
fi
if (( physical_lines > EXCEPTION_CEILING )); then
  echo "runtime_v3_loc=fail physical_lines=${physical_lines} exception_ceiling=${EXCEPTION_CEILING}" >&2
  exit 1
fi
if (( test_count >= TEST_CEILING )); then
  echo "runtime_v3_loc=fail test_count=${test_count} test_ceiling_exclusive=${TEST_CEILING}" >&2
  exit 1
fi

printf 'runtime_v3_loc=pass physical_lines=%s challenge_target=%s reviewed_target=%s exception_lines=%s exception_ceiling=%s test_count=%s test_ceiling_exclusive=%s disposition=%s\n' \
  "${physical_lines}" "${CHALLENGE_TARGET}" "${REVIEWED_TARGET}" \
  "$(( physical_lines > REVIEWED_TARGET ? physical_lines - REVIEWED_TARGET : 0 ))" \
  "${EXCEPTION_CEILING}" "${test_count}" "${TEST_CEILING}" "${disposition}"
