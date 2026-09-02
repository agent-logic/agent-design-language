#!/usr/bin/env bash
set -euo pipefail

repo_root="$(git rev-parse --show-toplevel)"
cd "$repo_root"

if [[ "${1:-}" == "--live-wuji" ]]; then
  observe_existing_restart=false
  if [[ "${2:-}" == "--observe-existing-guardian-restart" ]]; then
    observe_existing_restart=true
  elif [[ -n "${2:-}" ]]; then
    echo "unsupported live Wuji acceptance option: $2" >&2
    exit 64
  fi
  primary_root="$(dirname "$(git rev-parse --git-common-dir)")"
  init="$primary_root/.adl/runtime-v3/live/runtime-init.toml"
  csm="$primary_root/.adl/runtime-v3/current/bin/csm"
  evidence_dir="$repo_root/.adl/runs/issue-640"
  mkdir -p "$evidence_dir"
  status="$evidence_dir/wuji-service-status.json"
  feed="$evidence_dir/wuji-observatory-feed.json"
  ready="$evidence_dir/wuji-readiness.json"
  before="$evidence_dir/wuji-service-before-restart.json"
  receipt="$evidence_dir/wuji-restart-receipt.json"
  source_kernel="$repo_root/adl-runtime-kernel/target/release/adl-runtime-kernel"
  installed_kernel="$primary_root/.adl/runtime-v3/current/bin/adl-runtime-kernel"
  cargo build --locked --release --manifest-path adl-runtime-kernel/Cargo.toml --bin adl-runtime-kernel
  source_kernel_sha256="$(shasum -a 256 "$source_kernel" | awk '{print $1}')"
  installed_kernel_sha256="$(shasum -a 256 "$installed_kernel" | awk '{print $1}')"
  if [[ "$source_kernel_sha256" != "$installed_kernel_sha256" ]]; then
    echo "installed Runtime kernel does not match the exact checked-out source build" >&2
    exit 1
  fi
  if [[ "$observe_existing_restart" == false ]]; then
    "$csm" runtime-v3 status --init "$init" --json >"$before" || true
    "$csm" runtime-v3 stop --init "$init" --json >/dev/null
    for _ in $(seq 1 120); do
      if python3 - <<'PY'
import socket
listeners = []
try:
    for port in (20997, 20998):
        listener = socket.socket()
        listener.bind(("127.0.0.1", port))
        listeners.append(listener)
except OSError:
    raise SystemExit(1)
finally:
    for listener in listeners:
        listener.close()
PY
      then
        break
      fi
      sleep 1
    done
    "$csm" runtime-v3 start --init "$init" --json >/dev/null
  elif [[ ! -s "$before" ]]; then
    echo "existing Guardian restart observation requires a captured before status" >&2
    exit 1
  fi
  "$csm" runtime-v3 status --init "$init" --json >"$status"
  public_base_url="$(python3 -c 'import sys,tomllib; print(tomllib.load(open(sys.argv[1], "rb"))["api"]["public_base_url"])' "$init")"
  for _ in $(seq 1 180); do
    curl -fsSk "$public_base_url:20997/v1/observatory" >"$feed"
    if jq -e '.agents.sample[] | select(.id == "shepherd" and .state == "ready" and .communication_eligible == true)' "$feed" >/dev/null; then
      break
    fi
    sleep 5
  done
  curl -fsSk "$public_base_url:20997/v1/ready" >"$ready"
  python3 - "$status" "$feed" "$ready" "$source_kernel_sha256" "$observe_existing_restart" <<'PY'
import json, os, socket, subprocess, sys
status = json.load(open(sys.argv[1]))
feed = json.load(open(sys.argv[2]))
ready = json.load(open(sys.argv[3]))
installed_kernel_sha256 = sys.argv[4]
observed_existing_restart = sys.argv[5] == "true"
assert status["service_loaded"] and status["listener_ready"] and status["observability_ready"]
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
  "command":["bash", ".csdlc/prepared/issues/640/validate-model-backed-shepherd.sh", "--live-wuji"] + (["--observe-existing-guardian-restart"] if observed_existing_restart else []),
  "restart_mechanism":"guardian_child_recovery" if observed_existing_restart else "csm_service_restart",
  "installed_kernel_sha256":installed_kernel_sha256,
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
