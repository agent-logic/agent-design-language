#!/usr/bin/env bash
set -euo pipefail
bash adl/tools/test_release_ceremony.sh
bash adl/tools/release_ceremony.sh --version v0.92.1 --target-branch main
