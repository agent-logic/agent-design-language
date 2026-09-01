#!/usr/bin/env bash
set -euo pipefail
grep -Fq "Empty degraded recovery and revoked states" .csdlc/prepared/issues/511/bootstrap-request.json
