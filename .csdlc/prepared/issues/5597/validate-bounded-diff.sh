#!/usr/bin/env bash
set -euo pipefail

base_ref="${1:-origin/main}"
revision="${2:-HEAD}"
git diff --check "$base_ref...$revision"
while IFS= read -r path; do
  case "$path" in
    .csdlc/issues/5597/*|.csdlc/locks/5597.lock|.csdlc/prepared/issues/5597/*|csdlc-v2/*|docs/templates/prompts/current.json|docs/templates/prompts/README.md|docs/architecture/csdlc-v2/gate9/samples/*)
      ;;
    *)
      printf 'out-of-scope path: %s\n' "$path" >&2
      exit 1
      ;;
  esac
done < <(git diff --name-only "$base_ref...$revision")
