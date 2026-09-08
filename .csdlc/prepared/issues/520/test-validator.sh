#!/usr/bin/env bash
set -euo pipefail
ruby .csdlc/prepared/issues/520/validate-internal-review.rb fixture .csdlc/prepared/issues/520/fixtures/positive-authority.json
for fixture in negative-empty negative-self-authored-base negative-omitted-live-pr negative-incomplete-pagination negative-jointly-truncated; do
  if ruby .csdlc/prepared/issues/520/validate-internal-review.rb fixture ".csdlc/prepared/issues/520/fixtures/${fixture}.json"; then
    echo "negative ${fixture} fixture unexpectedly passed" >&2
    exit 1
  fi
done
