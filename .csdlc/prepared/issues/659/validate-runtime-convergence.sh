#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
design="$repo_root/.csdlc/prepared/issues/659/design.md"
spp="$repo_root/.csdlc/issues/659/cards/spp.values.json"
vpp="$repo_root/.csdlc/issues/659/cards/vpp.values.json"
source_file="$repo_root/adl/src/cli/csm_runtime_v3_cmd.rs"
config_file="$repo_root/adl-runtime-kernel/src/config.rs"

for required in "$design" "$spp" "$vpp" "$source_file" "$config_file"; do
  test -f "$required"
done

for term in service_convergence stop_timeout_millis unload_timeout_millis listener_timeout_millis readiness_timeout_millis; do
  rg -q "$term" "$design"
done

rg -q 'listener-open' "$design"
rg -q 'authenticated full `/v1/ready`' "$design"
rg -q '300,000 \(5 min\)' "$design"
rg -q '900,000 \(15 min\)' "$design"
rg -q '1,000.3,600,000' "$design"

jq -e '.content.values.affected_areas | index("adl-runtime-kernel/src/config.rs") != null' "$spp" >/dev/null
jq -e '.content.values.lanes | map(.lane) | index("runtime-convergence-preparation") != null' "$vpp" >/dev/null
jq -e '.content.values.lanes[] | select(.lane == "runtime-convergence-config") | .argv == ["cargo","test","--locked","--manifest-path","adl-runtime-kernel/Cargo.toml","--test","configuration","service_convergence_"]' "$vpp" >/dev/null
rg -q 'U -->\|deadline\| E' "$repo_root/.csdlc/prepared/issues/659/diagram.mmd"
! rg -q 'RUNTIME_V3_LAUNCH_AND_OBSERVATORY_RECOVERY_PLAN' "$repo_root/.csdlc/issues/659/cards/stp.values.json"

fixed_count="$(rg -c 'Duration::from_secs\(15\)' "$source_file" || true)"
fixed_count="${fixed_count:-0}"
test "$fixed_count" -eq 0

printf 'issue 659 convergence contract valid; fixed 15-second operational waits: %s\n' "$fixed_count"
