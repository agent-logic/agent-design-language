#!/bin/bash
set -Eeuo pipefail

install -d -m 0755 /var/lib/adl/issue509 /var/log/adl
log=/var/log/adl/issue509-ollama.log
exec >"$log" 2>&1

echo "issue=509"
echo "node_role=ollama-gpu"
date -u '+started_at=%Y-%m-%dT%H:%M:%SZ'

export DEBIAN_FRONTEND=noninteractive
for command in curl jq gcloud tar sha256sum systemd-run; do
  command -v "$command" >/dev/null 2>&1 || {
    echo "missing preinstalled launch dependency: $command" >&2
    exit 2
  }
done

artifact_bucket="$(curl -fsS -H 'Metadata-Flavor: Google' http://metadata.google.internal/computeMetadata/v1/instance/attributes/adl-artifact-bucket)"
run_id="$(curl -fsS -H 'Metadata-Flavor: Google' http://metadata.google.internal/computeMetadata/v1/instance/attributes/adl-run-id)"
manifest_object="$(curl -fsS -H 'Metadata-Flavor: Google' http://metadata.google.internal/computeMetadata/v1/instance/attributes/adl-artifact-manifest-object)"
manifest_sha="$(curl -fsS -H 'Metadata-Flavor: Google' http://metadata.google.internal/computeMetadata/v1/instance/attributes/adl-artifact-manifest-sha256)"
manifest=/var/lib/adl/issue509/portable-model-bundle.json
receipt_object="models/ollama/issue509/receipts/${run_id}/ollama-ready.json"

gcloud storage cp "gs://${artifact_bucket}/${manifest_object}" "$manifest"
printf '%s  %s\n' "$manifest_sha" "$manifest" | sha256sum -c -

jq -e '
  .schema == "adl.shepherd.portable_model_bundle.v2"
  and ((.models | map(.model_identity) | sort) == (["llama3.1:8b","qwen3:8b","phi4-mini:latest"] | sort))
  and ([.artifacts[] | select(.kind == "ollama_runtime")] | length == 1)
  and (([.artifacts[] | select(.kind == "ollama_model_manifest") | .model_identity] | sort | unique) == (["llama3.1:8b","qwen3:8b","phi4-mini:latest"] | sort))
  and ([.artifacts[] | select(.kind == "ollama_model_blob")] | length > 0)
  and all(.artifacts[]; (.sha256 | test("^[0-9a-f]{64}$")) and (.object | type == "string" and length > 0))
' "$manifest" >/dev/null

mkdir -p /var/lib/adl/issue509/artifacts /opt/adl-ollama-models
while IFS=$'\t' read -r kind object relative sha archive_format; do
  dest="/var/lib/adl/issue509/artifacts/$relative"
  mkdir -p "$(dirname "$dest")"
  gcloud storage cp "gs://${artifact_bucket}/${object}" "$dest"
  printf '%s  %s\n' "$sha" "$dest" | sha256sum -c -
  if [[ "$kind" == "ollama_runtime" ]]; then
    if [[ "$archive_format" == "tar.gz" ]]; then
      tar -xzf "$dest" -C /usr/local
    elif [[ "$archive_format" == "tar" ]]; then
      tar -xf "$dest" -C /usr/local
    else
      echo "unsupported ollama runtime archive_format: $archive_format" >&2
      exit 2
    fi
  elif [[ "$kind" == "ollama_model_manifest" || "$kind" == "ollama_model_blob" ]]; then
    install -d -m 0755 "$(dirname "/opt/adl-ollama-models/$relative")"
    cp "$dest" "/opt/adl-ollama-models/$relative"
  fi
done < <(jq -r '.artifacts[] | select(.kind == "ollama_runtime" or .kind == "ollama_model_manifest" or .kind == "ollama_model_blob") | [.kind,.object,.relative_path,.sha256,(.archive_format // "")] | @tsv' "$manifest")

systemd-run --unit=adl-ollama \
  --property=Restart=always \
  --property=RestartSec=1s \
  --property=StartLimitIntervalSec=0 \
  --setenv=HOME=/var/lib/adl/issue509 \
  --setenv=OLLAMA_MODELS=/opt/adl-ollama-models/models \
  --setenv=OLLAMA_HOST=0.0.0.0:11434 \
  --setenv=OLLAMA_KEEP_ALIVE=-1 \
  --setenv=OLLAMA_MAX_LOADED_MODELS=3 \
  /usr/local/bin/ollama serve

for _ in $(seq 1 120); do
  curl -fsS http://127.0.0.1:11434/api/version >/dev/null && break
  sleep 2
done

for model in llama3.1:8b qwen3:8b phi4-mini:latest; do
  jq -n --arg model "$model" '{model:$model,prompt:"Reply OK.",stream:false,keep_alive:-1,options:{num_predict:1}}' |
    curl -fsS http://127.0.0.1:11434/api/generate -d @- >/dev/null
done

models="$(curl -fsS http://127.0.0.1:11434/api/ps | jq -ce '
  [.models[] | {model_identity:(.name // .model), digest:(.digest // ""), size_vram:(.size_vram // 0)}]
  | sort_by(.model_identity)
')"

jq -n --argjson models "$models" \
  --arg artifact_manifest_sha256 "$manifest_sha" \
  '{schema:"adl.issue509.ollama_gpu_receipt.v1",status:"ready",ollama_public:false,model_source:"gcs_object_storage",artifact_manifest_sha256:$artifact_manifest_sha256,model_count:($models|length),models:$models}' \
  >/var/lib/adl/issue509/ollama-ready.json
gcloud storage cp /var/lib/adl/issue509/ollama-ready.json "gs://${artifact_bucket}/${receipt_object}"

date -u '+finished_at=%Y-%m-%dT%H:%M:%SZ'
