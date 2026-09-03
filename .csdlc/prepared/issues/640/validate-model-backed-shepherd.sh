#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

if [[ "${1:-}" == "--live-wuji" || "${1:-}" == "--capture-live-wuji-before" ]]; then
  if [[ -n "${2:-}" ]]; then
    echo "unsupported live Wuji acceptance option: $2" >&2
    exit 64
  fi
  primary_root="$(dirname "$(git rev-parse --git-common-dir)")"
  init="$primary_root/.adl/runtime-v3/live/runtime-init.toml"
  evidence_dir="$repo_root/.adl/runs/issue-640"
  mkdir -p "$evidence_dir"
  status="$evidence_dir/wuji-service-status.json"
  feed="$evidence_dir/wuji-observatory-feed.json"
  ready="$evidence_dir/wuji-readiness.json"
  before="$evidence_dir/wuji-service-before-restart.json"
  source_kernel="$repo_root/adl-runtime-kernel/target/release/adl-runtime-kernel"
  installed_kernel="$(python3 -c 'import sys,tomllib; print(tomllib.load(open(sys.argv[1], "rb"))["binaries"]["kernel_path"])' "$init")"
  installed_guardian="$primary_root/.adl/runtime-v3/current/bin/adl-runtime-guardian"
  installed_kernel_realpath="$(realpath "$installed_kernel")"
  installed_guardian_realpath="$(realpath "$installed_guardian")"
  local_address="$(python3 -c 'import sys,tomllib; print(tomllib.load(open(sys.argv[1], "rb"))["api"]["address"])' "$init")"
  cargo build --locked --release --manifest-path adl-runtime-kernel/Cargo.toml --bin adl-runtime-kernel
  source_kernel_sha256="$(shasum -a 256 "$source_kernel" | awk '{print $1}')"
  installed_kernel_sha256="$(shasum -a 256 "$installed_kernel_realpath" | awk '{print $1}')"
  if [[ "$source_kernel_sha256" != "$installed_kernel_sha256" ]]; then
    echo "installed Runtime kernel does not match the exact checked-out source build" >&2
    exit 1
  fi
  deployed_kernel_count="$(find "$primary_root/.adl/runtime-v3/releases" -type f -perm -111 -name 'adl-runtime-kernel*' | wc -l | tr -d ' ')"
  if [[ "$deployed_kernel_count" != "1" ]]; then
    echo "expected exactly one deployed Runtime kernel executable, found $deployed_kernel_count" >&2
    exit 1
  fi
  if [[ "${1:-}" == "--capture-live-wuji-before" ]]; then
    curl -fsSk "https://$local_address/v1/ready" >"$before"
    before_runtime_pid="$(jq -er '.runtime_process_id' "$before")"
    before_guardian_pid="$(jq -er '.guardian_process_id' "$before")"
    echo "captured live Wuji pre-restart state for Runtime PID $before_runtime_pid and Guardian PID $before_guardian_pid"
    exit 0
  fi
  if [[ ! -f "$evidence_dir/wuji-service-before-restart.json" ]]; then
    echo "capture pre-restart state with --capture-live-wuji-before, perform a controlled service restart, then run --live-wuji" >&2
    exit 1
  fi
  curl -fsSk "https://$local_address/v1/ready" >"$status"
  before_runtime_pid="$(jq -er '.runtime_process_id' "$before")"
  before_guardian_pid="$(jq -er '.guardian_process_id' "$before")"
  after_runtime_pid="$(jq -er '.runtime_process_id' "$status")"
  process_kernel="$(lsof -a -p "$after_runtime_pid" -d txt -Fn | awk '/^n/ {print substr($0, 2); exit}')"
  process_kernel_realpath="$(realpath "$process_kernel")"
  process_kernel_sha256="$(shasum -a 256 "$process_kernel_realpath" | awk '{print $1}')"
  if [[ "$process_kernel_realpath" != "$installed_kernel_realpath" || "$process_kernel_sha256" != "$installed_kernel_sha256" ]]; then
    echo "running Runtime process is not executing the configured exact-build kernel" >&2
    exit 1
  fi
  after_guardian_pid="$(jq -er '.guardian_process_id' "$status")"
  process_guardian="$(lsof -a -p "$after_guardian_pid" -d txt -Fn | awk '/^n/ {print substr($0, 2); exit}')"
  process_guardian_realpath="$(realpath "$process_guardian")"
  if [[ "$process_guardian_realpath" != "$installed_guardian_realpath" ]]; then
    echo "running Guardian process is not executing the configured Guardian binary" >&2
    exit 1
  fi
  public_base_url="$(python3 -c 'import sys,tomllib; print(tomllib.load(open(sys.argv[1], "rb"))["api"]["public_base_url"])' "$init")"
  for _ in $(seq 1 180); do
    curl -fsSk "$public_base_url:20997/v1/observatory" >"$feed"
    if jq -e '.agents.sample[] | select(.id == "shepherd" and .state == "ready" and .communication_eligible == true)' "$feed" >/dev/null; then
      break
    fi
    sleep 5
  done
  curl -fsSk "$public_base_url:20997/v1/ready" >"$ready"
  python3 - "$status" "$feed" "$ready" "$source_kernel_sha256" "$installed_kernel" "$installed_kernel_realpath" "$process_kernel_realpath" "$before_guardian_pid" "$deployed_kernel_count" "$installed_guardian_realpath" "$process_guardian_realpath" <<'PY'
import json, os, socket, subprocess, sys
status = json.load(open(sys.argv[1]))
feed = json.load(open(sys.argv[2]))
ready = json.load(open(sys.argv[3]))
installed_kernel_sha256 = sys.argv[4]
configured_kernel_path = sys.argv[5]
configured_kernel_realpath = sys.argv[6]
process_kernel_realpath = sys.argv[7]
before_guardian_pid = int(sys.argv[8])
deployed_kernel_count = int(sys.argv[9])
configured_guardian_realpath = sys.argv[10]
process_guardian_realpath = sys.argv[11]
assert status["ready"] is True and status["observability_ready"] is True
shepherd = next(agent for agent in feed["agents"]["sample"] if agent["id"] == "shepherd")
assert shepherd["name"] == "beacon.axioma"
assert shepherd["provider"] == "ollama" and shepherd["model"] == "qwen3:8b"
assert shepherd["state"] == "ready" and shepherd["communication_eligible"] is True
assert "governed inference probe passed" in shepherd["detail"]
assert feed["health"]["observability_ready"] is True
assert ready["ready"] is True and ready["observability_ready"] is True
assert ready["runtime_process_id"] == status["runtime_process_id"] == feed["runtime_process_id"]
before = json.load(open(os.path.join(os.path.dirname(sys.argv[1]), "wuji-service-before-restart.json")))
assert before["runtime_process_id"] != status["runtime_process_id"]
receipt = {
  "schema":"adl.issue_640.wuji_acceptance.v2",
  "status":"pass",
  "candidate_revision":subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip(),
  "host":socket.gethostname(),
  "command":["bash", ".csdlc/prepared/issues/640/validate-model-backed-shepherd.sh", "--live-wuji"],
  "restart_mechanism":"externally_controlled_service_restart",
  "guardian_process_before":before_guardian_pid,
  "guardian_process_id":status["guardian_process_id"],
  "configured_kernel_path":configured_kernel_path,
  "configured_kernel_realpath":configured_kernel_realpath,
  "process_kernel_realpath":process_kernel_realpath,
  "configured_guardian_realpath":configured_guardian_realpath,
  "process_guardian_realpath":process_guardian_realpath,
  "installed_kernel_sha256":installed_kernel_sha256,
  "process_kernel_sha256":installed_kernel_sha256,
  "deployed_kernel_executable_count":deployed_kernel_count,
  "runtime_process_before":before["runtime_process_id"],
  "runtime_process_after":status["runtime_process_id"],
  "restart_proved":True,
  "governed_inference_proved":True,
  "readiness_feed_consistent":True,
  "shepherd":shepherd,
}
print(json.dumps(receipt, indent=2))
PY
  ollama ps | grep -F 'qwen3:8b' | grep -F 'Forever'
  exit 0
fi

cargo nextest run \
  --locked \
  --manifest-path adl-runtime-kernel/Cargo.toml \
  --test configuration \
  --test shepherd \
  --test agent_roster \
  --no-tests=fail \
  -E 'test(resident_shepherd) | test(shepherd_provider) | test(shepherd_model_health) | test(shepherd_readiness_consistency)'

cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check
git diff --check
