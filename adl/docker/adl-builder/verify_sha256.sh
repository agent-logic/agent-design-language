#!/usr/bin/env bash
# Verify the reviewed construction input before any extraction or execution.
set -euo pipefail
[[ $# == 2 && "$1" =~ ^[0-9a-f]{64}$ ]] || { echo 'missing or invalid approved SHA-256' >&2; exit 2; }
[[ -f "$2" && ! -L "$2" ]] || { echo 'archive must be a regular file' >&2; exit 2; }
if command -v sha256sum >/dev/null 2>&1; then
  actual="$(sha256sum -- "$2")"
else
  actual="$(shasum -a 256 -- "$2")"
fi
[[ "${actual%% *}" == "$1" ]] || { echo 'construction input digest mismatch' >&2; exit 1; }
