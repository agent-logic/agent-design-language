#!/usr/bin/env bash
# PVF: deterministic mocked Terraform contract; small local CPU; no release gate.
set -euo pipefail
repo_root="$(git rev-parse --show-toplevel)"
stack="$repo_root/infra/aws/csm-runtime-spot"
terraform -chdir="$stack" fmt -check
terraform -chdir="$repo_root/infra/aws/modules/csm-runtime-spot" fmt -check
terraform -chdir="$stack" init -backend=false -input=false -lockfile=readonly
terraform -chdir="$stack" validate -no-color
terraform -chdir="$stack" test -no-color
