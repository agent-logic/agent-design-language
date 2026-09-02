#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
ruby "$repo_root/.csdlc/prepared/issues/262/record-podcast-http-playback.rb" --profile ios-safari "$@"
