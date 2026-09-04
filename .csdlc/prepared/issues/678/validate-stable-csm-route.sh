#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
cd "$ROOT_DIR"

mkdir -p .csdlc/evidence/678
bash adl/tools/test_runtime_v3_generation_install.sh | tee .csdlc/evidence/678/runtime-v3-generation-install.log
