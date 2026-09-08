#!/usr/bin/env bash
set -euo pipefail

printf 'BLOCKED: plan denominator validation requires bound worktree, private tfvars, backend confirmation, and no live apply\n' >&2
exit 2
