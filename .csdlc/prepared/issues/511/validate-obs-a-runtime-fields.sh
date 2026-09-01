#!/usr/bin/env bash
set -euo pipefail
grep -Fq "No invented Runtime field" .csdlc/prepared/issues/511/bootstrap-request.json
test -f docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml
