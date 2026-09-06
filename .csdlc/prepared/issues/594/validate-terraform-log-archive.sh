#!/usr/bin/env bash
set -euo pipefail

terraform -chdir=infra/aws/runtime/log-archive fmt -check -recursive
terraform -chdir=infra/aws/runtime/log-archive init -backend=false -input=false
terraform -chdir=infra/aws/runtime/log-archive validate
terraform -chdir=infra/aws/runtime/log-archive test
