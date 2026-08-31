#!/usr/bin/env bash
set -euo pipefail
grep -Fq "empty degraded recovery and revoked" .csdlc/prepared/issues/512/bootstrap-request.json
