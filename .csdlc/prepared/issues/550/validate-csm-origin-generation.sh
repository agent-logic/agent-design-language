#!/usr/bin/env bash
set -euo pipefail

if [[ -x adl/tools/test_csmctl_observatory_origins.sh ]]; then
  exec bash adl/tools/test_csmctl_observatory_origins.sh
fi

echo "missing adl/tools/test_csmctl_observatory_origins.sh; issue implementation has not added the CSM origin-generation validator yet" >&2
exit 1
