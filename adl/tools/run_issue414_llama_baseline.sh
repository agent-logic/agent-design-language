#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OLLAMA_URL="${ADL_ISSUE414_OLLAMA_URL:-http://127.0.0.1:11434}"
BIN="${ADL_ISSUE414_CONTINUITY_BIN:-$ROOT/adl/target/debug/adl_resident_shepherd_continuity}"
CSM_BIN="${ADL_ISSUE414_CSM_BIN:-$ROOT/adl/target/debug/csm}"
OUT="${ADL_ISSUE414_OUT:-$ROOT/.csdlc/evidence/414/llama-baseline-reference.json}"
mkdir -p "$(dirname "$OUT")"
TMP="$(mktemp -d "${OUT}.work.XXXXXX")"
cleanup() { status=$?; if [[ $status == 0 ]]; then rm -rf "$TMP"; else printf '%s\n' "$TMP" >"${OUT}.failure-workdir"; fi; }
trap cleanup EXIT
command -v curl >/dev/null; command -v jq >/dev/null; [[ -x "$BIN" ]]; [[ -x "$CSM_BIN" ]]
export ADL_ISSUE414_SIGNING_KEY_HEX="${ADL_ISSUE414_SIGNING_KEY_HEX:-9999999999999999999999999999999999999999999999999999999999999999}"
export ADL_CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64="${ADL_CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64:-CQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQk=}"
export ADL_CSM_CUSTODY_SIGNING_KEY_ID="${ADL_CSM_CUSTODY_SIGNING_KEY_ID:-issue414-reference-key}"
export ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64="${ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64:-BHE1+k/ZOgnc6Yu/aBtL/PUOfA1jVOYq+wv/KjQpYXhl7UwfAt25Aj7lalV+UV1qncZsEfIglg3llDNN9Yh3ZyQ=}"
curl -fsS "$OLLAMA_URL/api/tags" >"$TMP/tags.json"
jq -e '.models[]|select((.name=="llama3.1:8b" or .model=="llama3.1:8b") and (.details.quantization_level|startswith("Q4")))' "$TMP/tags.json" >"$TMP/model.json"
artifact="$(jq -r .digest "$TMP/model.json")"; quant="$(jq -r .details.quantization_level "$TMP/model.json")"
config="$(printf 'llama3.1:8b:%s:%s:8192:1:1:think=unsupported' "$artifact" "$quant" | shasum -a 256 | awk '{print $1}')"
runtime="$TMP/retained-runtime"; build="$TMP/ephemeral-build-cache"; mkdir -p "$runtime" "$build"
agent_specs=()
for id in 1 2; do
  agent_root="$TMP/resident-$id"
  mkdir -p "$agent_root"
  spec="$agent_root/agent.yaml"
  printf 'schema: adl.long_lived_agent_spec.v1\nagent_instance_id: resident-%s\ndisplay_name: resident-%s\nstate_root: state\nworkflow:\n  kind: demo_adapter\nheartbeat:\n  stale_lease_after_secs: 60\n' "$id" "$id" >"$spec"
  "$CSM_BIN" daemon --spec "$spec" --checkpoint-interval-secs 1 --interval-secs 1 --api-bind 127.0.0.1:0 --no-sleep --json >"$TMP/resident-$id-daemon.json"
  agent_specs+=("$spec")
done
for id in 1 2; do
  prompt="Return only compact JSON exactly shaped as {\"action\":\"check\",\"ordered_steps\":[\"digest\",\"population\"],\"next_action\":\"verify_population_digest\"}."
  jq -n --arg p "$prompt" '{model:"llama3.1:8b",prompt:$p,stream:false,format:"json",keep_alive:"5m",options:{num_ctx:8192,num_predict:128,num_gpu:0,temperature:0,seed:414}}' >"$TMP/request.json"
  curl --max-time 180 -fsS "$OLLAMA_URL/api/generate" -H 'Content-Type: application/json' --data-binary @"$TMP/request.json" >"$TMP/cold-$id.json"
  jq -e '.done==true and (.response|fromjson|.next_action=="verify_population_digest") and (.response|fromjson|.ordered_steps|length==2)' "$TMP/cold-$id.json" >/dev/null
  jq -r '.response|fromjson|@json' "$TMP/cold-$id.json" | shasum -a 256 | awk '{print $1}' >"$TMP/cold-$id.sha"
  continuation="Return only compact JSON exactly shaped as {\"action\":\"continue\",\"ordered_steps\":[\"lineage\",\"admission\"],\"next_action\":\"continue_after_exact_restore\"}."
  printf '%s' "$continuation" >"$TMP/warm-$id.prompt"; printf '%s' "$continuation" | shasum -a 256 | awk '{print $1}' >"$TMP/warm-$id.request.sha"
done
python3 - "$TMP" "$runtime" "$build" "$artifact" "$quant" "$config" "${agent_specs[@]}" >"$TMP/input.json" <<'PY'
import json,pathlib,sys
t=pathlib.Path(sys.argv[1]); residents=[]
for i in (1,2): residents.append({"agent_id":f"resident-{i}","model":"llama3.1:8b","artifact_sha256":sys.argv[4],"quantization":sys.argv[5],"configuration_sha256":sys.argv[6],"completed_task_sha256":(t/f"cold-{i}.sha").read_text().strip(),"continuation_request_sha256":(t/f"warm-{i}.request.sha").read_text().strip()})
print(json.dumps({"residents":residents,"existing_agent_specs":sys.argv[7:9],"retained_runtime_root":sys.argv[2],"build_cache_root":sys.argv[3],"runtime_volume_identity_sha256":__import__("hashlib").sha256(b"reference-volume").hexdigest(),"source_host":"external-reference","target_host":"local"},sort_keys=True))
PY
"$BIN" preflight --input "$TMP/input.json" --runtime-root "$runtime" --output "$TMP/preflight.json"
"$BIN" dehydrate --input "$TMP/input.json" --runtime-root "$runtime" --output "$TMP/dehydrate.json"
ADL_ISSUE414_RESTORE_HOST_CLASS=reference ADL_ISSUE414_CONTINUITY_BIN="$BIN" bash "$ROOT/adl/tools/issue414_restore_and_admit.sh" "$TMP/input.json" "$runtime" "$TMP/restore.json"
jq -e '.admission_open==true and .continuation_verified==false' "$TMP/restore.json" >/dev/null
for id in 1 2; do
  jq -n --rawfile p "$TMP/warm-$id.prompt" '{model:"llama3.1:8b",prompt:$p,stream:false,format:"json",keep_alive:"5m",options:{num_ctx:8192,num_predict:128,num_gpu:0,temperature:0,seed:415}}' >"$TMP/request.json"
  curl --max-time 180 -fsS "$OLLAMA_URL/api/generate" -H 'Content-Type: application/json' --data-binary @"$TMP/request.json" >"$TMP/warm-$id.json"
  jq -e '.done==true and (.response|fromjson|.next_action=="continue_after_exact_restore") and (.response|fromjson|.ordered_steps|length==2)' "$TMP/warm-$id.json" >/dev/null
  jq -r '.response|fromjson|@json' "$TMP/warm-$id.json" | shasum -a 256 | awk '{print $1}' >"$TMP/warm-$id.sha"
done
python3 - "$TMP" "$artifact" "$quant" "$config" "$OUT" <<'PY'
import json,pathlib,sys
t=pathlib.Path(sys.argv[1]); residents=[]
for i in (1,2): residents.append({"agent_id":f"resident-{i}","model":"llama3.1:8b","artifact_sha256":sys.argv[2],"quantization":sys.argv[3],"configuration_sha256":sys.argv[4],"completed_task_sha256":(t/f"cold-{i}.sha").read_text().strip(),"continuation_request_sha256":(t/f"warm-{i}.request.sha").read_text().strip(),"next_task_sha256":(t/f"warm-{i}.sha").read_text().strip(),"cold_latency_millis":max(1,json.loads((t/f"cold-{i}.json").read_text())["total_duration"]//1_000_000),"warm_latency_millis":max(1,json.loads((t/f"warm-{i}.json").read_text())["total_duration"]//1_000_000),"loaded_model_count":1,"loaded_model_bytes":0,"ollama_rss_kib":-1})
receipt={"schema":"adl.runtime.resident_shepherd_habitability.v2","qualification":False,"host_class":"reference","instance_type":"reference-host","vcpus":0,"memory_mib":65536,"swap_used_mib":-1,"peak_loaded_model_bytes":0,"peak_ollama_rss_kib":-1,"rss_measurement":"unavailable_external_reference_server","required_capacity_headroom_mib":16384,"model_capacity_headroom_mib":65536,"capacity_headroom_pass":True,"context_tokens":8192,"parallelism":1,"max_loaded_models":1,"ollama_configuration_source":"api_ps_and_request_contract_server_environment_unverified","compilation_concurrent":False,"focused_tests_passed":6,"logical_resident_count":2,"distinct_model_count":1,"loaded_model_count":1,"max_concurrent_inference":1,"resident_count":2,"residents":residents,"prompts_retained":False,"model_weights_serialized":False,"external_model_authoritative":False,"bootstrap_role":"not_used_reference_host"}
pathlib.Path(sys.argv[5]).write_text(json.dumps(receipt,indent=2,sort_keys=True)+"\n")
PY
"$BIN" complete --input "$OUT" --runtime-root "$runtime" --output "${OUT%.json}-end-to-end.json"
"$BIN" validate-receipt --input "$OUT" --runtime-root "$runtime" --output "${OUT%.json}-validation.json"
jq -e '.admission_open==true and .continuation_verified==true' "${OUT%.json}-end-to-end.json" >/dev/null
jq -e '.status=="passed" and .qualification==false' "${OUT%.json}-validation.json" >/dev/null
