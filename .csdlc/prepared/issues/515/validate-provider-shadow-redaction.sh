#!/usr/bin/env bash
set -euo pipefail

evidence_dir="docs/milestones/v0.92.1/evidence/provider/prov-b"

test -d "$evidence_dir"

if grep -R -n -E '(AKIA[0-9A-Z]{16}|BEGIN (RSA|OPENSSH|EC|DSA) PRIVATE KEY|/Users/|/private/tmp|OPENAI_API_KEY|ANTHROPIC_API_KEY|AWS_SECRET_ACCESS_KEY)' "$evidence_dir"; then
  echo "unredacted sensitive marker found in PROV-B evidence" >&2
  exit 1
fi
