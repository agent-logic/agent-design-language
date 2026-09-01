#!/usr/bin/env bash
set -euo pipefail
test -f demos/html-observatory/app.js
grep -Fq "OBS-A contracts are implemented" .csdlc/prepared/issues/512/bootstrap-request.json
