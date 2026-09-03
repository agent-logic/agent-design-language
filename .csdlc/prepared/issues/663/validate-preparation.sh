#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
issue_root="$repo_root/.csdlc/issues/663"
design_root="$repo_root/.csdlc/prepared/issues/663"

test -f "$design_root/design.md"
test -f "$design_root/diagram.mmd"
test -f "$issue_root/index.json"

rg -q 'snapshot-catalog' "$design_root/design.md"
rg -q 'snapshot-launch-to-ready|snapshot-to-ready|snapshot launch' "$design_root/design.md" "$issue_root/cards"
rg -q 'disposable Runtime and Ollama/model disks restored from exact versioned snapshots' "$issue_root/cards"
rg -q 'image-family aliases are forbidden' "$design_root/design.md"
rg -q 'sync.*unmounts.*detaches' "$design_root/design.md"
rg -q 'explicit exact-generation retirement' "$design_root/diagram.mmd"

if rg -q 'normal startup.*(git clone|cargo build|ollama pull|apt-get|dnf install|yum install)' "$design_root/design.md"; then
  echo "normal-startup contract contains a forbidden executable action" >&2
  exit 1
fi

printf '%s\n' 'issue663_preparation=pass'
