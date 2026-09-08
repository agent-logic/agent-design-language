#!/usr/bin/env bash
set -euo pipefail
repo_root="$(git rev-parse --show-toplevel)"
exec ruby "${repo_root}/.csdlc/prepared/issues/516/validate-release-tail-admission.rb" "${1:-all}"
