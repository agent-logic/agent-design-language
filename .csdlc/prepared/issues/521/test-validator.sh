#!/usr/bin/env bash
set -euo pipefail
ruby .csdlc/prepared/issues/521/validate-external-review.rb fixture .csdlc/prepared/issues/521/fixtures/positive.json
for fixture in negative-vacuous negative-self-authored-scope negative-omitted-internal-ref; do
  if ruby .csdlc/prepared/issues/521/validate-external-review.rb fixture ".csdlc/prepared/issues/521/fixtures/${fixture}.json"; then
    echo "negative ${fixture} fixture unexpectedly passed" >&2
    exit 1
  fi
done
