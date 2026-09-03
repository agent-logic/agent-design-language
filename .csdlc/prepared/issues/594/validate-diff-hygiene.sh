#!/usr/bin/env bash
set -euo pipefail

git diff --check
printf 'issue=594\n'
printf 'lane=diff-hygiene\n'
printf 'argv=git diff --check\n'
printf 'status=passed\n'
