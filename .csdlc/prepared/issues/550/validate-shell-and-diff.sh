#!/usr/bin/env bash
set -euo pipefail

bash -n CSMctl
git diff --check
