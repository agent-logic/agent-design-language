#!/usr/bin/env bash
set -euo pipefail
ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)
grep -Fq 'tail -80 "$OLLAMA_LOG"' "$ROOT/adl/tools/run_issue268_remote_resident_qualification.sh"
grep -Fq 'OLLAMA_KEEP_ALIVE=-1' "$ROOT/adl/tools/run_issue268_remote_resident_qualification.sh"
grep -Fq 'OLLAMA_LLM_LIBRARY=cpu_avx2' "$ROOT/adl/tools/run_issue268_remote_resident_qualification.sh"
grep -Fq 'ADL_UTS_ALLOW_MULTI_MODEL_RESIDENCY=true' "$ROOT/adl/tools/run_issue268_remote_resident_qualification.sh"
grep -Fq "libggml-cpu-sapphirerapids.so' -print" "$ROOT/adl/tools/run_issue268_remote_resident_qualification.sh"
grep -Fq 'SAPPHIRE_BACKEND.disabled-issue268' "$ROOT/adl/tools/run_issue268_remote_resident_qualification.sh"
grep -Fq 'ollama-cpu-backend.txt' "$ROOT/adl/tools/run_issue268_remote_resident_qualification.sh"
grep -Fq 'Ollama preload failed; server diagnostics follow' "$ROOT/adl/tools/run_issue268_remote_resident_qualification.sh"
if grep -Fq 'provider-adapter/runtime-v1' "$ROOT/adl/tools/run_issue268_remote_resident_qualification.sh"; then
  echo "issue268: retired standalone provider adapter unexpectedly remains" >&2
  exit 1
fi
if grep -Fq 'git -C "$ROOT" diff' "$ROOT/adl/tools/run_issue268_remote_resident_qualification.sh"; then
  echo "issue268: Runtime startup must not inspect Git history for adapter reuse" >&2
  exit 1
fi
scratch=$(mktemp -d "$ROOT/.adl/issue268-remote-test.XXXXXX")
trap 'rm -rf "$scratch"' EXIT
touch "$scratch/continuity"; chmod +x "$scratch/continuity"
touch "$scratch/adl"; chmod +x "$scratch/adl"
cat >"$scratch/ollama" <<'SH'
#!/usr/bin/env bash
[[ "$1" == serve ]]
while :; do sleep 60; done
SH
chmod +x "$scratch/ollama"
mkdir -p "$scratch/agents"
continuity_sha=$(shasum -a 256 "$scratch/continuity" | awk '{print $1}')

printf '{}\n' >"$scratch/source-receipt.json"
cat >"$scratch/installer.py" <<'PY'
import hashlib,json,os,pathlib
continuity=pathlib.Path(os.environ['ADL_TEST_CONTINUITY'])
print(json.dumps({
  'ollama_binary':os.environ['ADL_TEST_OLLAMA'],
  'ollama_models':os.environ['ADL_TEST_MODELS'],
  'continuity_binary':str(continuity),
  'continuity_binary_sha256':hashlib.sha256(continuity.read_bytes()).hexdigest(),
  'runtime_binary':os.environ['ADL_TEST_RUNTIME'],
  'runtime_binary_sha256':hashlib.sha256(pathlib.Path(os.environ['ADL_TEST_RUNTIME']).read_bytes()).hexdigest(),
}))
PY
mkdir -p "$scratch/models" "$scratch/bin"
cat >"$scratch/bin/curl" <<'SH'
#!/usr/bin/env bash
exit 0
SH
chmod +x "$scratch/bin/curl"

cat >"$scratch/materializer.py" <<'PY'
import json,pathlib,sys
out=pathlib.Path(sys.argv[sys.argv.index('--output')+1]); out.write_text(json.dumps({'residents':[{'agent_id':str(i)} for i in range(6)]}))
PY
cat >"$scratch/orchestrator.py" <<'PY'
import json,pathlib,sys
a=sys.argv; evidence=pathlib.Path(a[a.index('--evidence-dir')+1]); evidence.mkdir(parents=True,exist_ok=True)
(evidence/'qualification-receipt.json').write_text(json.dumps({'schema':'adl.issue268.continuity_uts_qualification.v1','status':'passed','resident_count':6,'continuation_verified':True,'continuity_generation':1,'signed_population_sha256':'9'*64,'replay_denied':True,'residents':[{'agent_id':str(i)} for i in range(6)]})+'\n')
PY
cat >"$scratch/warmup.py" <<'PY'
import json,pathlib,sys
a=sys.argv; receipt=pathlib.Path(a[a.index('--receipt')+1])
receipt.write_text(json.dumps({'status':'passed','resident_model_count':3})+'\n')
PY
cat >"$scratch/guardian.sh" <<'SH'
#!/usr/bin/env bash
[[ "$1" == --suite && "$2" == six_hour_qualification ]]
printf 'ADL_ISSUE268_REPORT_BEGIN\n{"suite":"six_hour_qualification","measured_exposure_seconds":21600}\nADL_ISSUE268_REPORT_END\n'
SH
chmod +x "$scratch/guardian.sh"

output=$(PATH="$scratch/bin:$PATH" \
  ADL_RUN_ID=issue268-test-run \
  ADL_TEST_CONTINUITY="$scratch/continuity" \
  ADL_TEST_OLLAMA="$scratch/ollama" \
  ADL_TEST_MODELS="$scratch/models" \
  ADL_TEST_RUNTIME="$scratch/adl" \
  ADL_ISSUE268_REMOTE_EVIDENCE_ROOT="$scratch/evidence" \
  ADL_RUNTIME_CONTINUITY_ROOT="$scratch/volume/runtime" \
  ADL_RUNTIME_CONTINUITY_VOLUME_ID_SHA256="$(printf 'f%.0s' {1..64})" \
  ADL_ISSUE268_CONTINUITY_BIN="$scratch/continuity" \
  ADL_ISSUE268_RETAINED_RUNTIME_ROOT="$scratch/runtime" \
  ADL_ISSUE268_BUILD_CACHE_ROOT="$scratch/build" \
  ADL_ISSUE268_AGENT_SPEC_DIR="$scratch/agents" \
  ADL_ISSUE268_RUNTIME_VOLUME_IDENTITY_SHA256="$(printf 'f%.0s' {1..64})" \
  ADL_ISSUE268_S3_SOURCE_RECEIPT="$scratch/source-receipt.json" \
  ADL_ISSUE268_VOLUME_INSTALLER="$scratch/installer.py" \
  ADL_ISSUE268_414_REVIEWED_SHA="$(printf 'a%.0s' {1..40})" \
  ADL_ISSUE268_CONTINUITY_BIN_SHA256="$continuity_sha" \
  ADL_ISSUE268_MATERIALIZER="$scratch/materializer.py" \
  ADL_ISSUE268_CONTINUITY_UTS_RUNNER="$scratch/orchestrator.py" \
  ADL_ISSUE268_MODEL_WARMUP="$scratch/warmup.py" \
  ADL_ISSUE268_GUARDIAN_RUNNER="$scratch/guardian.sh" \
  bash "$ROOT/adl/tools/run_issue268_remote_resident_qualification.sh")
[[ "$output" == *ADL_ISSUE268_CONTINUITY_UTS_BEGIN* ]]
[[ "$output" == *ADL_ISSUE268_CONTINUITY_UTS_END* ]]
[[ "$output" == *ADL_ISSUE268_REPORT_BEGIN* ]]
[[ "${output%%ADL_ISSUE268_REPORT_BEGIN*}" == *ADL_ISSUE268_CONTINUITY_UTS_END* ]]

if PATH="$scratch/bin:$PATH" \
  ADL_RUN_ID=issue268-test-run \
  ADL_TEST_CONTINUITY="$scratch/continuity" \
  ADL_TEST_OLLAMA="$scratch/ollama" \
  ADL_TEST_MODELS="$scratch/models" \
  ADL_TEST_RUNTIME="$scratch/adl" \
  ADL_ISSUE268_REMOTE_EVIDENCE_ROOT="$scratch/provenance-failure" \
  ADL_RUNTIME_CONTINUITY_ROOT="$scratch/volume-failure/runtime" \
  ADL_RUNTIME_CONTINUITY_VOLUME_ID_SHA256="$(printf 'f%.0s' {1..64})" \
  ADL_ISSUE268_CONTINUITY_BIN="$scratch/continuity" \
  ADL_ISSUE268_RETAINED_RUNTIME_ROOT="$scratch/runtime-failure" \
  ADL_ISSUE268_BUILD_CACHE_ROOT="$scratch/build-failure" \
  ADL_ISSUE268_AGENT_SPEC_DIR="$scratch/agents" \
  ADL_ISSUE268_RUNTIME_VOLUME_IDENTITY_SHA256="$(printf 'f%.0s' {1..64})" \
  ADL_ISSUE268_S3_SOURCE_RECEIPT="$scratch/source-receipt.json" \
  ADL_ISSUE268_VOLUME_INSTALLER="$scratch/installer.py" \
  ADL_ISSUE268_414_REVIEWED_SHA="$(printf 'a%.0s' {1..40})" \
  ADL_ISSUE268_CONTINUITY_BIN_SHA256="$(printf '0%.0s' {1..64})" \
  bash "$ROOT/adl/tools/run_issue268_remote_resident_qualification.sh" >/dev/null 2>&1; then
  echo "issue268: mismatched #414 continuity provenance unexpectedly passed" >&2
  exit 1
fi
echo "PASS: issue268 remote resident qualification coupling"
