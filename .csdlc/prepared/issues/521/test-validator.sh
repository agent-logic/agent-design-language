#!/usr/bin/env bash
set -euo pipefail
ruby .csdlc/prepared/issues/521/validate-external-review.rb fixture .csdlc/prepared/issues/521/fixtures/positive.json
if ruby .csdlc/prepared/issues/521/validate-external-review.rb fixture .csdlc/prepared/issues/521/fixtures/negative-vacuous.json; then
  echo "negative vacuous-review fixture unexpectedly passed" >&2
  exit 1
fi
