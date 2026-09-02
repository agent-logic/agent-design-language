#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

design=.csdlc/prepared/issues/592/design.md
test -s "$design"
grep -q 'GCP project' "$design"
grep -q 'Vertex location' "$design"
grep -q 'secret JSON' "$design"
grep -q 'paid Vertex AI request' "$design"
grep -q 'project/location mismatch' "$design"

echo 'issue 592 Vertex configuration documentation contract: pass'
