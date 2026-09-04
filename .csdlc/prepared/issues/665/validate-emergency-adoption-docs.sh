#!/usr/bin/env bash
set -euo pipefail

doc_path="docs/tooling/EMERGENCY_BRANCH_ADOPTION.md"

test -f "${doc_path}"
rg -n "csdlc-bind" "${doc_path}" >/dev/null
rg -n "ready" "${doc_path}" >/dev/null
rg -n "bound" "${doc_path}" >/dev/null
rg -n "fail-closed|fail closed" "${doc_path}" >/dev/null
