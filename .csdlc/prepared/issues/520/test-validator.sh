#!/usr/bin/env bash
set -euo pipefail
ruby .csdlc/prepared/issues/520/validate-internal-review.rb fixture .csdlc/prepared/issues/520/fixtures/positive.json
if ruby .csdlc/prepared/issues/520/validate-internal-review.rb fixture .csdlc/prepared/issues/520/fixtures/negative-empty.json; then
  echo "negative empty-denominator fixture unexpectedly passed" >&2
  exit 1
fi
