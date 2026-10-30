#!/bin/bash
set -Eeuo pipefail

install -d -m 0755 /var/lib/adl/issue509 /var/log/adl
log=/var/log/adl/issue509-runtime.log
exec >"$log" 2>&1

echo "issue=509"
echo "node_role=runtime-csm"
date -u '+started_at=%Y-%m-%dT%H:%M:%SZ'

export DEBIAN_FRONTEND=noninteractive
for command in curl jq gcloud tar sha256sum python3; do
  command -v "$command" >/dev/null 2>&1 || {
    echo "missing preinstalled launch dependency: $command" >&2
    exit 2
  }
done

ollama_ip="$(curl -fsS -H 'Metadata-Flavor: Google' http://metadata.google.internal/computeMetadata/v1/instance/attributes/adl-ollama-private-ip)"
source_revision="$(curl -fsS -H 'Metadata-Flavor: Google' http://metadata.google.internal/computeMetadata/v1/instance/attributes/adl-source-revision)"
artifact_bucket="$(curl -fsS -H 'Metadata-Flavor: Google' http://metadata.google.internal/computeMetadata/v1/instance/attributes/adl-artifact-bucket)"
run_id="$(curl -fsS -H 'Metadata-Flavor: Google' http://metadata.google.internal/computeMetadata/v1/instance/attributes/adl-run-id)"
manifest_object="$(curl -fsS -H 'Metadata-Flavor: Google' http://metadata.google.internal/computeMetadata/v1/instance/attributes/adl-artifact-manifest-object)"
manifest_sha="$(curl -fsS -H 'Metadata-Flavor: Google' http://metadata.google.internal/computeMetadata/v1/instance/attributes/adl-artifact-manifest-sha256)"
manifest=/var/lib/adl/issue509/portable-model-bundle.json
receipt_object="models/ollama/issue509/receipts/${run_id}/runtime-final.json"

for _ in $(seq 1 180); do
  curl -fsS "http://${ollama_ip}:11434/api/ps" >/dev/null && break
  sleep 5
done

gcloud storage cp "gs://${artifact_bucket}/${manifest_object}" "$manifest"
printf '%s  %s\n' "$manifest_sha" "$manifest" | sha256sum -c -
runtime_object="$(jq -r '.artifacts[] | select(.kind == "runtime_bundle") | .object' "$manifest")"
runtime_relative="$(jq -r '.artifacts[] | select(.kind == "runtime_bundle") | .relative_path' "$manifest")"
runtime_sha="$(jq -r '.artifacts[] | select(.kind == "runtime_bundle") | .sha256' "$manifest")"
[[ -n "$runtime_object" && "$runtime_object" != "null" ]] || { echo "runtime_bundle missing from artifact manifest" >&2; exit 2; }
runtime_bundle="/var/lib/adl/issue509/artifacts/$runtime_relative"
mkdir -p "$(dirname "$runtime_bundle")" /opt/adl-runtime
gcloud storage cp "gs://${artifact_bucket}/${runtime_object}" "$runtime_bundle"
printf '%s  %s\n' "$runtime_sha" "$runtime_bundle" | sha256sum -c -
tar -xzf "$runtime_bundle" -C /opt/adl-runtime

jq '.provider.runtime_surface="gcp_private_ollama_http" | .provider.local_required=false | .provider.cloud_escalation_optional=false | .provider.cloud_escalation_authoritative=true' \
  /opt/adl-runtime/config/issue268_six_resident_uts_plan.json >/var/lib/adl/issue509/plan.json

remote_runner=/var/lib/adl/issue509/run-six-resident-gcp.py
sed "s#http://127.0.0.1:11434#http://${ollama_ip}:11434#g" \
  /opt/adl-runtime/config/run_issue268_six_resident_uts_cycle.py >"$remote_runner"

mkdir -p /var/lib/adl/issue509/agent-evidence
python3 "$remote_runner" \
  --phase pre \
  --state /var/lib/adl/issue509/agent-state.json \
  --evidence-dir /var/lib/adl/issue509/agent-evidence \
  --plan /var/lib/adl/issue509/plan.json \
  --task-panel /opt/adl-runtime/config/issue268_runtime_uts_task_panel.json \
  --runtime-bin /opt/adl-runtime/bin/adl \
  --runtime-root /var/lib/adl/issue509/runtime \
  >/var/log/adl/issue509-six-resident.log 2>&1

agents="$(jq -sc 'map(select(.agent_test_outcome=="executed" and .runtime_exit_code==0 and .runtime_receipt.decision=="executed"))|select(length==6)' /var/lib/adl/issue509/agent-evidence/pre-*.json)"
ollama_models="$(curl -fsS "http://${ollama_ip}:11434/api/ps" | jq -ce '[.models[] | {model_identity:(.name // .model), digest:(.digest // ""), size_vram:(.size_vram // 0)}] | sort_by(.model_identity)')"

jq -n \
  --arg source_revision "$source_revision" \
  --arg ollama_private_ip "$ollama_ip" \
  --arg artifact_manifest_sha256 "$manifest_sha" \
  --argjson residents "$agents" \
  --argjson ollama_models "$ollama_models" \
  '{
    schema:"adl.issue509.gcp_drt_d_runtime_receipt.v1",
    status:"passed",
    source_revision:$source_revision,
    topology:{node_count:2,runtime_node:"gcp_compute_instance.runtime",ollama_node:"gcp_compute_instance.ollama",ollama_private_ip:$ollama_private_ip,ollama_public:false},
    provider:{kind:"ollama",runtime_surface:"gcp_private_ollama_http",model_source:"gcs_object_storage",artifact_manifest_sha256:$artifact_manifest_sha256,model_count:($ollama_models|length),models:$ollama_models},
    residents:$residents,
    components_exercised:["gcp_compute_runtime_node","gcp_compute_ollama_gpu_node","six_resident_uts_runtime","private_ollama_http"]
  }' >/var/lib/adl/issue509/final.json
gcloud storage cp /var/lib/adl/issue509/final.json "gs://${artifact_bucket}/${receipt_object}"

date -u '+finished_at=%Y-%m-%dT%H:%M:%SZ'
