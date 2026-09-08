#!/usr/bin/env bash
set -euo pipefail

printf 'BLOCKED: mutation authorization packet requires saved plan digest, run ID, deadline, impersonated identity, rollback commands, and operator approval\n' >&2
exit 2
