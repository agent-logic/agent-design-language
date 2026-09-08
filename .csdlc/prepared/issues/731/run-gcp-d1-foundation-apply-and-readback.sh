#!/usr/bin/env bash
set -euo pipefail

printf 'BLOCKED: live foundation apply requires fresh explicit operator authorization; this script must fail closed until then\n' >&2
exit 2
