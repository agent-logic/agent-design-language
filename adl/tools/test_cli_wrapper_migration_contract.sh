#!/usr/bin/env bash
set -euo pipefail

# Original v2 command/Gate2/Gate4 assertions retain their independent owner/historical
# dispositions. This script proves local wrapper absence/schema guards only.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

for retired in \
  adl/tools/pr.sh \
  adl/tools/skills/workflow-conductor/scripts/route_workflow.py \
  adl/tools/validate_structured_prompt.sh \
  adl/tools/prompt_template.sh; do
  [[ ! -e "$ROOT_DIR/$retired" ]] || {
    echo "retired v1 route unexpectedly exists: $retired" >&2
    exit 1
  }
done

echo "PASS: retained local wrapper absence/schema guards only; no native edit, validation, or lifecycle equivalence claimed"
