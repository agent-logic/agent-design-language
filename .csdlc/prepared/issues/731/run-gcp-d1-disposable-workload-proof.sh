#!/usr/bin/env bash
set -euo pipefail

printf 'BLOCKED: live disposable workload proof requires fresh explicit operator authorization; this script must fail closed until then\n' >&2
exit 2
