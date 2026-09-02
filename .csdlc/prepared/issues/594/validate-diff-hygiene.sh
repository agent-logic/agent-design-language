#!/usr/bin/env bash
set -euo pipefail

git diff --check
head_sha="$(git rev-parse HEAD)"
printf 'issue=594\n'
printf 'lane=diff-hygiene\n'
printf 'argv=git diff --check\n'
printf 'head=%s\n' "${head_sha}"
printf 'status=passed\n'
