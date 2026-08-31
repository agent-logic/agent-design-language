#!/usr/bin/env bash
set -euo pipefail

rg -n \
  "/Users/daniel/keys|approved local Z\\.ai key source|duration_ms=3748|\\.adl/provider-smoke/glm-5-3-flash/result\\.json|\\.adl/provider-smoke/glm-5-3-flash/reviewer-proof-result\\.json" \
  .csdlc/issues/578 \
  .csdlc/prepared/issues/578 \
  docs/milestones/v0.92.1/evidence/provider/glm-5-3-flash \
  --glob '!**/verify-no-glm-secret-paths.sh' \
  -S \
  && exit 1

exit 0
