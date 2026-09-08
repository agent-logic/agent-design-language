#!/usr/bin/env bash
set -euo pipefail
ruby .csdlc/prepared/issues/522/validate-remediation-ledger.rb fixture .csdlc/prepared/issues/522/fixtures/positive-zero.json
ruby .csdlc/prepared/issues/522/validate-remediation-ledger.rb fixture .csdlc/prepared/issues/522/fixtures/positive-fixed.json
for fixture in negative-empty negative-undispositioned negative-invented-census negative-stale-review negative-false-validation negative-altered-finding-content negative-deferred-source-blocker negative-unmerged-source; do
  if ruby .csdlc/prepared/issues/522/validate-remediation-ledger.rb fixture ".csdlc/prepared/issues/522/fixtures/${fixture}.json"; then
    echo "negative ${fixture} fixture unexpectedly passed" >&2
    exit 1
  fi
done
