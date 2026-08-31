#!/usr/bin/env bash
set -euo pipefail
grep -Fq "Every view has a stable information contract" .csdlc/prepared/issues/511/bootstrap-request.json
grep -Fq "Runtime field" .csdlc/prepared/issues/511/bootstrap-request.json
