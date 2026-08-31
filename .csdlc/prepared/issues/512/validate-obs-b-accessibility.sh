#!/usr/bin/env bash
set -euo pipefail
grep -Fq "Accessibility and recovery cases pass" .csdlc/prepared/issues/512/bootstrap-request.json
