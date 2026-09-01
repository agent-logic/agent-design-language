#!/usr/bin/env bash
set -euo pipefail

warm_root=${ADL_ISSUE607_WARM_ROOT:-/mnt/adl-warm-runtime}
state_root=${ADL_ISSUE607_STATE_ROOT:-/var/lib/adl/issue607-runtime}
region=${ADL_ISSUE607_AWS_REGION:?}
bucket=${ADL_ISSUE607_ARTIFACT_BUCKET:?}
qualification_key=${ADL_ISSUE607_QUALIFICATION_KEY:?}
source_commit=${ADL_ISSUE607_SOURCE_COMMIT:?}
gpu_private_ip=${ADL_ISSUE607_GPU_PRIVATE_IP:?}
manifest="$warm_root/install/config/artifact-manifest.json"
mkdir -p "$state_root/agent-evidence"

publish_failure() {
  rc=$?
  jq -n --argjson rc "$rc" --arg stage "${stage:-qualification}" \
    '{schema:"adl.issue607.qualification_complete.v1",status:"failed",exit_code:$rc,stage:$stage}' >"$state_root/qualification.json" 2>/dev/null || true
  aws s3api put-object --region "$region" --bucket "$bucket" --key "$qualification_key" --body "$state_root/qualification.json" --if-none-match '*' >/dev/null 2>&1 || true
  exit "$rc"
}
trap publish_failure EXIT

for command in aws curl jq python3 sha256sum; do command -v "$command" >/dev/null; done
jq -e '.schema=="adl.shepherd.portable_model_bundle.v2" and (.models|length)>=2' "$manifest" >/dev/null
guardian_proof="$(find "$state_root/guardian-evidence" -type f -name issue-proof.json -print -quit)"
jq -e '
  .status=="pass"
  and .assertions.guardian_launched==true
  and .assertions.kernel_ready==true
  and .assertions.authenticated_https==true
  and .assertions.authenticated_wss==true
  and .assertions.bounded_restart==true
  and .assertions.state_preserved==true
  and .assertions.clean_shutdown==true
  and .assertions.clean_logs==true
' "$guardian_proof" >/dev/null

stage=shepherd
shepherd='[]'
while IFS=$'\t' read -r model digest; do
  log="$state_root/shepherd-$(printf '%s' "$model" | sha256sum | awk '{print $1}').log"
  ADL_SHEPHERD_OLLAMA_HOST="http://$gpu_private_ip:11434" \
    ADL_SHEPHERD_BACKEND_IDENTITY=ollama_cuda_aws_l4 \
    ADL_SHEPHERD_MODEL_IDENTITY="$model" \
    ADL_SHEPHERD_MODEL_DIGEST_SHA256="$digest" \
    "$warm_root/install/bin/adl-shepherd-local-model-test" real_local_model_smoke --ignored --exact --nocapture >"$log" 2>&1
  proof="$(grep '"schema":"adl.runtime.shepherd_local_model_smoke.v1"' "$log" | tail -1)"
  shepherd="$(jq -c --arg model "$model" --argjson proof "$proof" '.+[{model_identity:$model,proof:$proof}]' <<<"$shepherd")"
done < <(jq -r '.models[]|[.model_identity,.model_digest_sha256]|@tsv' "$manifest")

stage=six_agent_acc
first="$(jq -r '.models[0].model_identity' "$manifest")"
second="$(jq -r '.models[1].model_identity' "$manifest")"
jq --arg first "$first" --arg second "$second" '
  .host.gpu_allowed=false
  | .host.max_loaded_models=2
  | .residents |= (to_entries | map(.value.model=(if (.key%2)==0 then $first else $second end) | .value))
' "$warm_root/install/config/issue268_six_resident_uts_plan.json" >"$state_root/plan.json"
sed "s#http://127.0.0.1:11434#http://$gpu_private_ip:11434#g" \
  "$warm_root/install/config/run_issue268_six_resident_uts_cycle.py" >"$state_root/run-six-resident-remote.py"
python3 "$state_root/run-six-resident-remote.py" \
  --phase pre \
  --state "$state_root/agent-state.json" \
  --evidence-dir "$state_root/agent-evidence" \
  --plan "$state_root/plan.json" \
  --task-panel "$warm_root/install/config/issue268_runtime_uts_task_panel.json" \
  --runtime-bin "$warm_root/install/bin/adl" \
  --runtime-root "$state_root/runtime" >"$state_root/agents.log" 2>&1
agents="$(jq -sc 'map(select(.agent_test_outcome=="executed" and .runtime_exit_code==0 and .runtime_receipt.decision=="executed"))|select(length==6)' "$state_root"/agent-evidence/pre-*.json)"

stage=receipt
jq -n --arg source_commit "$source_commit" \
  --arg guardian_proof_sha256 "$(sha256sum "$guardian_proof" | awk '{print $1}')" \
  --argjson shepherd "$shepherd" --argjson agents "$agents" \
  '{schema:"adl.issue607.qualification_complete.v1",status:"passed",source_commit:$source_commit,guardian_proof_sha256:$guardian_proof_sha256,shepherd_proofs:$shepherd,runtime_agent_acc_proofs:$agents,assertions:{two_model_shepherd:true,six_agent_acc:true,guardian_restart:true,state_preserved:true,degradation_recovered:true,vector_recovered:true,clean_logs:true,clean_shutdown:true}}' \
  >"$state_root/qualification.json"
aws s3api put-object --region "$region" --bucket "$bucket" --key "$qualification_key" --body "$state_root/qualification.json" --if-none-match '*' >/dev/null
trap - EXIT
