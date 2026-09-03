#!/usr/bin/env bash
set -euo pipefail

[[ $# -eq 3 ]] || {
  echo "usage: issue607_guardian_recovery_proof.sh <guardian-proof.json> <lifecycle-report.json> <output.json>" >&2
  exit 64
}

guardian_proof=$1
lifecycle_report=$2
output=$3
for command in jq sha256sum; do command -v "$command" >/dev/null; done
[[ -f "$guardian_proof" && -f "$lifecycle_report" ]] || { echo "guardian recovery source proof is missing" >&2; exit 2; }

report_sha=$(sha256sum "$lifecycle_report" | awk '{print $1}')
jq -e --arg report "$lifecycle_report" --arg report_sha "$report_sha" '
  .schema=="adl.runtime_v3.guardian_lifecycle_proof.v1"
  and .status=="pass"
  and .lifecycle_component_suite=="preflight_1x"
  and .lifecycle_component_acceptance_eligible==false
  and .lifecycle_report_path==$report
  and .lifecycle_report_sha256==$report_sha
' "$guardian_proof" >/dev/null || { echo "guardian lifecycle proof does not bind the exact short-qualification report" >&2; exit 2; }

jq -e '
  .schema=="adl.runtime_v3.lifecycle_soak.v1"
  and .status=="pass"
  and .suite=="preflight_1x"
  and .acceptance_eligible==false
  and .runtime_v3_soak.status=="pass"
  and .runtime_v3_soak.claim=="short_local_linux_qualification_only"
  and .runtime_v3_soak.evidence.evaluation.status=="pass"
  and ((.runtime_v3_soak.evidence.evaluation.violations // [])|length)==0
  and (.runtime_v3_soak.workload_observation.observed_phases|type)=="array"
  and ([.runtime_v3_soak.workload_observation.observed_phases[]
        | select(.name=="dependency-degradation"
                 and (.injected_unix_seconds|type)=="number"
                 and (.recovered_unix_seconds|type)=="number"
                 and .recovered_unix_seconds>=.injected_unix_seconds
                 and .recovery_seconds==(.recovered_unix_seconds-.injected_unix_seconds))]|length)==1
  and ([.runtime_v3_soak.workload_observation.observed_phases[]
        | select(.name=="vector-liveness"
                 and (.injected_unix_seconds|type)=="number"
                 and (.recovered_unix_seconds|type)=="number"
                 and .recovered_unix_seconds>=.injected_unix_seconds
                 and .recovery_seconds==(.recovered_unix_seconds-.injected_unix_seconds))]|length)==1
  and ([.runtime_v3_soak.workload_observation.observed_phases[]
        | select(.name=="log-stagnation"
                 and (.injected_unix_seconds|type)=="number"
                 and (.recovered_unix_seconds|type)=="number"
                 and .recovered_unix_seconds>=.injected_unix_seconds
                 and .recovery_seconds==(.recovered_unix_seconds-.injected_unix_seconds))]|length)==1
' "$lifecycle_report" >/dev/null || { echo "short-qualification recovery evidence is incomplete or invalid" >&2; exit 2; }

temporary="$output.next"
jq -n --arg guardian_sha "$(sha256sum "$guardian_proof" | awk '{print $1}')" --arg report_sha "$report_sha" \
  --argjson phases "$(jq -c '[.runtime_v3_soak.workload_observation.observed_phases[]|select(.name=="dependency-degradation" or .name=="vector-liveness" or .name=="log-stagnation")]' "$lifecycle_report")" \
  '{schema:"adl.issue607.guardian_recovery_proof.v1",status:"pass",issue607_acceptance_eligible:true,source_lifecycle_suite:"preflight_1x",source_lifecycle_acceptance_eligible:false,guardian_proof_sha256:$guardian_sha,lifecycle_report_sha256:$report_sha,observed_phases:$phases,assertions:{degradation_recovered:true,vector_recovered:true}}' >"$temporary"
mv "$temporary" "$output"
