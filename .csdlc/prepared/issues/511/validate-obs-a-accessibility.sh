#!/usr/bin/env bash
set -euo pipefail
grep -Fq "Keyboard and screen-reader flows" .csdlc/prepared/issues/511/bootstrap-request.json
