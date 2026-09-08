#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
exec bash "$repo_root/.csdlc/prepared/issues/740/run-gcp-b1-corrective-proof.sh" "$@"
