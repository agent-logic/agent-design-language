#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$repo_root"
exec bash adl/tools/test_v092_first_birthday_demo.sh --positive
