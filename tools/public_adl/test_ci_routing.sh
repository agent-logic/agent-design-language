#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
python3 -B -m unittest discover -s "$root/tools/public_adl" -p 'test_ci_policy.py' -v
