#!/usr/bin/env bash
set -euo pipefail
grep -Fq "redacted" .csdlc/prepared/issues/512/bootstrap-request.json
