#!/usr/bin/env bash
# PVF: deterministic source-shape check, tiny local resource use; no SSM/host execution.
set -euo pipefail
source_path="adl/tools/polis_status_for_ssm_windows.ps1"
if [[ ! -r "$source_path" ]]; then
  echo "validate_polis_status_for_ssm_windows: cannot read $source_path" >&2
  exit 1
fi
missing=""
for fragment in schema_version generated_at_utc host_label os_name os_version repo_name repo_present git_branch git_commit_short ssm_agent_installed ConvertTo-Json; do
  if ! grep -Fq -- "$fragment" "$source_path"; then
    missing="${missing:+$missing, }$fragment"
  fi
done
if [[ -n "$missing" ]]; then
  echo "validate_polis_status_for_ssm_windows: missing required fragments: $missing" >&2
  exit 1
fi
echo "validate_polis_status_for_ssm_windows: ok"
