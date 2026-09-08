#!/usr/bin/env bash
set -euo pipefail

printf 'BLOCKED: read-only GCP preflight requires approved credential context and must redact sensitive output\n' >&2
exit 2
