#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"

/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo "${ROOT}" --issue 485
