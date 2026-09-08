#!/usr/bin/env bash
set -euo pipefail

notes=docs/milestones/v0.92.1/RELEASE_NOTES_v0.92.1.md
receipt_dir=docs/milestones/v0.92.1/evidence/release/tail-10
test -s "$notes"
test -d "$receipt_dir"
find "$receipt_dir" -type f -size +0c | grep -q .
git diff --check
printf 'issue 526 ceremony receipt contract passed\n'
