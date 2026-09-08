#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

git diff --check origin/main...HEAD
git diff --check

printf 'PASS: #731 diff hygiene\n'
