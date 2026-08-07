#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

usage() {
  cat <<'EOF'
Usage:
  adl/tools/run_nessus_remote_validation.sh --command <shell-command> [options]

Options:
  --command <shell-command>        Command to run inside the remote ADL checkout. Required.
  --portable-request <path>       Portable request JSON; mutually exclusive with --command.
  --portable-runner <path>        adl-remote-validation binary for portable requests.
  --executor <ssh|local>           Execution transport. Defaults to ssh. local is for bounded contract tests.
  --host <host>                    Remote host. Defaults to nessus.local.
  --ssh-user <user>                SSH login user. Defaults to danie.
  --wsl-user <user>                WSL Linux user. Defaults to root.
  --remote-root <path>             Remote workspace root. Defaults to /root/adl-remote-runner.
  --repo-url <url>                 Git remote used to materialize/refresh the checkout.
  --git-ref <ref>                  Git ref to fetch and check out. Defaults to origin/main.
  --run-id <id>                    Deterministic run id. Defaults to UTC timestamp.
  --local-artifact-dir <path>      Optional local artifact directory for fetched summary + log tarball.
  --summary-name <name>            Remote summary filename. Defaults to summary.json.
  --builder-image <image>          Optional ADL builder image used to execute the command.
  --builder-runtime <auto|docker|podman>
                                   Container runtime for --builder-image. Defaults to auto.
  --builder-pull-policy <missing|always|never>
                                   Pull behavior for --builder-image. Defaults to missing.
                                   Use never for preloaded local tags.
  --help                           Show this help.

Environment overrides:
  ADL_NESSUS_REMOTE_EXECUTOR
  ADL_NESSUS_REMOTE_HOST
  ADL_NESSUS_REMOTE_SSH_USER
  ADL_NESSUS_REMOTE_WSL_USER
  ADL_NESSUS_REMOTE_ROOT
  ADL_NESSUS_REMOTE_REPO_URL
  ADL_NESSUS_REMOTE_GIT_REF
  ADL_NESSUS_REMOTE_ARTIFACT_DIR
  ADL_NESSUS_BUILDER_IMAGE
  ADL_NESSUS_BUILDER_RUNTIME
  ADL_NESSUS_BUILDER_PULL_POLICY
  ADL_NESSUS_APT_SOURCES_LIST
  ADL_NESSUS_APT_KUBERNETES_LIST
  SSH_BIN
EOF
}

timestamp_id() {
  date -u +"%Y%m%d-%H%M%S"
}

quote_remote_single() {
  printf "%s" "$1" | sed "s/'/'\"'\"'/g"
}

COMMAND_STRING=""
COMMAND_EXPLICIT=false
PORTABLE_REQUEST=""
PORTABLE_RUNNER="${ADL_REMOTE_VALIDATION_BIN:-}"
PORTABLE_EXPECTED_REVISION=""
PORTABLE_CPU_CORES=""
PORTABLE_MEMORY_MIB=""
PORTABLE_TIMEOUT_SECONDS=""
PORTABLE_CANCELLATION_FILE=""
PORTABLE_FALLBACK="disabled"
EXECUTOR="${ADL_NESSUS_REMOTE_EXECUTOR:-ssh}"
HOST="${ADL_NESSUS_REMOTE_HOST:-nessus.local}"
SSH_USER="${ADL_NESSUS_REMOTE_SSH_USER:-danie}"
WSL_USER="${ADL_NESSUS_REMOTE_WSL_USER:-root}"
REMOTE_ROOT="${ADL_NESSUS_REMOTE_ROOT:-/root/adl-remote-runner}"
REPO_URL="${ADL_NESSUS_REMOTE_REPO_URL:-https://github.com/agent-logic/agent-design-language.git}"
GIT_REF="${ADL_NESSUS_REMOTE_GIT_REF:-origin/main}"
RUN_ID=""
LOCAL_ARTIFACT_DIR="${ADL_NESSUS_REMOTE_ARTIFACT_DIR:-}"
SUMMARY_NAME="summary.json"
BUILDER_IMAGE="${ADL_NESSUS_BUILDER_IMAGE:-}"
BUILDER_RUNTIME="${ADL_NESSUS_BUILDER_RUNTIME:-auto}"
BUILDER_PULL_POLICY="${ADL_NESSUS_BUILDER_PULL_POLICY:-missing}"
SSH_BIN="${SSH_BIN:-ssh}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --command)
      COMMAND_STRING="${2:-}"
      COMMAND_EXPLICIT=true
      shift 2
      ;;
    --portable-request)
      PORTABLE_REQUEST="${2:-}"
      shift 2
      ;;
    --portable-runner)
      PORTABLE_RUNNER="${2:-}"
      shift 2
      ;;
    --executor)
      EXECUTOR="${2:-}"
      shift 2
      ;;
    --host)
      HOST="${2:-}"
      shift 2
      ;;
    --ssh-user)
      SSH_USER="${2:-}"
      shift 2
      ;;
    --wsl-user)
      WSL_USER="${2:-}"
      shift 2
      ;;
    --remote-root)
      REMOTE_ROOT="${2:-}"
      shift 2
      ;;
    --repo-url)
      REPO_URL="${2:-}"
      shift 2
      ;;
    --git-ref)
      GIT_REF="${2:-}"
      shift 2
      ;;
    --run-id)
      RUN_ID="${2:-}"
      shift 2
      ;;
    --local-artifact-dir)
      LOCAL_ARTIFACT_DIR="${2:-}"
      shift 2
      ;;
    --summary-name)
      SUMMARY_NAME="${2:-}"
      shift 2
      ;;
    --builder-image)
      BUILDER_IMAGE="${2:-}"
      shift 2
      ;;
    --builder-runtime)
      BUILDER_RUNTIME="${2:-}"
      shift 2
      ;;
    --builder-pull-policy)
      BUILDER_PULL_POLICY="${2:-}"
      shift 2
      ;;
    --help)
      usage
      exit 0
      ;;
    *)
      echo "run_nessus_remote_validation: unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -n "$PORTABLE_REQUEST" ]]; then
  if [[ "$COMMAND_EXPLICIT" == true ]]; then
    echo "run_nessus_remote_validation: --portable-request and --command are mutually exclusive" >&2
    exit 2
  fi
  if [[ ! -x "$PORTABLE_RUNNER" ]]; then
    echo "run_nessus_remote_validation: portable runner is missing or not executable" >&2
    exit 2
  fi
  PORTABLE_PLAN="$($PORTABLE_RUNNER adapter-plan nessus "$PORTABLE_REQUEST")" || {
    echo "run_nessus_remote_validation: portable request was rejected" >&2
    exit 2
  }
  COMMAND_STRING="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["shell_command"])')"
  GIT_REF="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["source_ref"])')"
  PORTABLE_EXPECTED_REVISION="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["revision"])')"
  PORTABLE_CPU_CORES="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["resource_budget"]["cpu_cores"])')"
  PORTABLE_MEMORY_MIB="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["resource_budget"]["memory_mib"])')"
  PORTABLE_TIMEOUT_SECONDS="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["resource_budget"]["timeout_seconds"])')"
  PORTABLE_CANCELLATION_FILE="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin).get("cancellation_file") or "")')"
  PORTABLE_FALLBACK="$(printf '%s' "$PORTABLE_PLAN" | python3 -c 'import json,sys; print(json.load(sys.stdin)["fallback"])')"
fi

if [[ -z "$COMMAND_STRING" ]]; then
  echo "run_nessus_remote_validation: --command or --portable-request is required" >&2
  usage >&2
  exit 2
fi

if [[ "$EXECUTOR" != "ssh" && "$EXECUTOR" != "local" ]]; then
  echo "run_nessus_remote_validation: unsupported --executor '$EXECUTOR' (expected ssh or local)" >&2
  exit 2
fi

if [[ "$BUILDER_RUNTIME" != "auto" && "$BUILDER_RUNTIME" != "docker" && "$BUILDER_RUNTIME" != "podman" ]]; then
  echo "run_nessus_remote_validation: unsupported --builder-runtime '$BUILDER_RUNTIME' (expected auto, docker, or podman)" >&2
  exit 2
fi

if [[ "$BUILDER_PULL_POLICY" != "missing" && "$BUILDER_PULL_POLICY" != "always" && "$BUILDER_PULL_POLICY" != "never" ]]; then
  echo "run_nessus_remote_validation: unsupported --builder-pull-policy '$BUILDER_PULL_POLICY' (expected missing, always, or never)" >&2
  exit 2
fi

if [[ -z "$RUN_ID" ]]; then
  RUN_ID="$(timestamp_id)"
fi

if [[ -n "$PORTABLE_CANCELLATION_FILE" && -e "$ROOT_DIR/$PORTABLE_CANCELLATION_FILE" ]]; then
  echo "run_nessus_remote_validation: cancellation requested before remote execution" >&2
  exit 130
fi

COMMAND_B64="$(printf "%s" "$COMMAND_STRING" | base64 | tr -d '\n')"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/adl-nessus-remote-validation.XXXXXX")"
REMOTE_SCRIPT="$TMP_DIR/remote_runner.sh"
trap 'rm -rf "$TMP_DIR"' EXIT

cat >"$REMOTE_SCRIPT" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail

REMOTE_ROOT="$1"
REPO_URL="$2"
GIT_REF="$3"
RUN_ID="$4"
COMMAND_B64="$5"
SUMMARY_NAME="$6"
BUILDER_IMAGE="$7"
BUILDER_RUNTIME="$8"
BUILDER_PULL_POLICY="$9"
REQUIRED_CPU_CORES="${10}"
REQUIRED_MEMORY_MIB="${11}"
TIMEOUT_SECONDS="${12}"
EXPECTED_REVISION="${13}"
APT_SOURCES_LIST="${ADL_NESSUS_APT_SOURCES_LIST:-/etc/apt/sources.list}"
APT_KUBERNETES_LIST="${ADL_NESSUS_APT_KUBERNETES_LIST:-/etc/apt/sources.list.d/kubernetes.list}"
COMMAND_STRING="$(printf '%s' "$COMMAND_B64" | base64 -d)"
RUN_ROOT="$REMOTE_ROOT/transient/$RUN_ID"
PUBLISH_ROOT="$REMOTE_ROOT/logs/$RUN_ID"
REPO_DIR="$REMOTE_ROOT/agent-design-language"
CACHE_ROOT="$REMOTE_ROOT/cache"
TARGET_DIR="$CACHE_ROOT/target"
SCCACHE_DIR="$CACHE_ROOT/sccache"
SUMMARY_PATH="$PUBLISH_ROOT/$SUMMARY_NAME"
WINDOWS_IDENTITY_FILE="$RUN_ROOT/windows-identity.txt"
WSL_IDENTITY_FILE="$RUN_ROOT/wsl-identity.txt"
STATUS="failed"
FAILURE_CLASS="validation"
RESOLVED_COMMIT="unknown"
COMMAND_EXIT=1
COMMAND_PID=""
ACTIVE_CONTAINER_NAME=""
COMMAND_CLEANUP_ATTEMPTED=false
COMMAND_CLEANUP_COMPLETE=false
RESOLVED_BUILDER_RUNTIME="none"
BUILDER_IMAGE_LOCAL_PRESENT=false
BUILDER_IMAGE_PULL_ATTEMPTED=false
APT_MASKED=false
HASHICORP_MASKED=false
KUBERNETES_BACKUP=""
SOURCES_BACKUP=""
START_EPOCH="$(date +%s)"
START_ISO="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
CANCELLATION_MARKER="$RUN_ROOT/cancel.request"
TIMEOUT_MARKER="$RUN_ROOT/timeout.request"
RUNNER_PID_FILE="$RUN_ROOT/runner.pid"

mkdir -p "$RUN_ROOT" "$CACHE_ROOT" "$TARGET_DIR" "$SCCACHE_DIR"
printf '%s\n' "$$" >"$RUNNER_PID_FILE"

if [[ -n "$REQUIRED_CPU_CORES" ]]; then
  if command -v nproc >/dev/null 2>&1; then
    AVAILABLE_CPU="$(nproc)"
  else
    AVAILABLE_CPU="$(getconf _NPROCESSORS_ONLN)"
  fi
  if [[ -r /proc/meminfo ]]; then
    AVAILABLE_MEMORY_MIB="$(awk '/MemTotal:/ { print int($2 / 1024) }' /proc/meminfo)"
  else
    AVAILABLE_MEMORY_MIB="$(python3 -c 'import os; print(os.sysconf("SC_PAGE_SIZE") * os.sysconf("SC_PHYS_PAGES") // 1024 // 1024)')"
  fi
  if (( AVAILABLE_CPU < REQUIRED_CPU_CORES || AVAILABLE_MEMORY_MIB < REQUIRED_MEMORY_MIB )); then
    echo "remote machine does not satisfy portable CPU/memory request" >&2
    exit 2
  fi
fi

restore_apt_sources() {
  if [[ -n "$KUBERNETES_BACKUP" && -f "$KUBERNETES_BACKUP" ]]; then
    mv "$KUBERNETES_BACKUP" "$APT_KUBERNETES_LIST"
  fi
  if [[ -n "$SOURCES_BACKUP" && -f "$SOURCES_BACKUP" ]]; then
    mv "$SOURCES_BACKUP" "$APT_SOURCES_LIST"
  fi
}

write_summary() {
  local exit_code="$1"
  local end_epoch elapsed end_iso
  end_epoch="$(date +%s)"
  elapsed="$((end_epoch - START_EPOCH))"
  end_iso="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  python3 - <<'PY' \
    "$SUMMARY_PATH" "$RUN_ID" "$STATUS" "$exit_code" "$START_ISO" "$end_iso" "$elapsed" \
    "$REPO_URL" "$GIT_REF" "$RESOLVED_COMMIT" "$COMMAND_STRING" "$REMOTE_ROOT" "$PUBLISH_ROOT" \
    "$REPO_DIR" "$TARGET_DIR" "$SCCACHE_DIR" "$BUILDER_IMAGE" "$BUILDER_RUNTIME" "$RESOLVED_BUILDER_RUNTIME" \
    "$BUILDER_PULL_POLICY" "$BUILDER_IMAGE_LOCAL_PRESENT" "$BUILDER_IMAGE_PULL_ATTEMPTED" \
    "$APT_SOURCES_LIST" "$APT_KUBERNETES_LIST" "$FAILURE_CLASS" \
    "$COMMAND_CLEANUP_ATTEMPTED" "$COMMAND_CLEANUP_COMPLETE"
import json
import os
import sys
from pathlib import Path

(
    summary_path,
    run_id,
    status,
    exit_code,
    started_at,
    finished_at,
    elapsed_seconds,
    repo_url,
    git_ref,
    resolved_commit,
    command,
    remote_root,
    run_root,
    repo_dir,
    target_dir,
    sccache_dir,
    builder_image,
    builder_runtime,
    resolved_builder_runtime,
    builder_pull_policy,
    builder_image_local_present,
    builder_image_pull_attempted,
    apt_sources_list,
    apt_kubernetes_list,
    failure_class,
    cleanup_attempted,
    cleanup_complete,
) = sys.argv[1:]

run_root_path = Path(run_root)
sccache_stats_path = run_root_path / "sccache-stats.log"
cache_status = {
    "target_dir": target_dir,
    "sccache_dir": sccache_dir,
    "sccache_stats_log": str(sccache_stats_path),
}
if sccache_stats_path.exists():
    for raw_line in sccache_stats_path.read_text(encoding="utf-8", errors="replace").splitlines():
        normalized = " ".join(raw_line.split())
        if normalized.startswith("Compile requests "):
            cache_status["compile_requests"] = normalized.split()[-1]
        elif normalized.startswith("Compile requests executed "):
            cache_status["compile_requests_executed"] = normalized.split()[-1]
        elif normalized.startswith("Cache hits ") and not normalized.startswith("Cache hits rate "):
            cache_status["cache_hits"] = normalized.split()[-1]
        elif normalized.startswith("Cache misses ") and not normalized.startswith("Cache misses ("):
            cache_status["cache_misses"] = normalized.split()[-1]

def redact(value):
    import re
    if isinstance(value, dict):
        return {key: redact(item) for key, item in value.items()}
    if isinstance(value, list):
        return [redact(item) for item in value]
    if isinstance(value, str):
        value = re.sub(
            r"/(?:Users|Volumes|private|tmp|root)/[^\s,\"]*",
            lambda match: f"<machine-path-redacted>/{Path(match.group(0)).name}",
            value,
        )
        value = re.sub(r"arn:aws:[^\s,\"]*", "<aws-arn-redacted>", value)
        value = re.sub(r"\b\d{12}\b", "<account-id-redacted>", value)
        value = re.sub(r"\bi-[0-9a-f]{8,17}\b", "<instance-id-redacted>", value)
        value = re.sub(r"\b(?:\d{1,3}\.){3}\d{1,3}\b", "<ip-address-redacted>", value)
        value = re.sub(r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b", "<aws-access-key-redacted>", value)
        value = re.sub(r"(?i)\b(Bearer|Basic)\s+[A-Za-z0-9._~+/=-]+", r"\1 <credential-redacted>", value)
        value = re.sub(r"\b(?:gh[pousr]_[A-Za-z0-9_]{20,}|github_pat_[A-Za-z0-9_]{20,})\b", "<github-token-redacted>", value)
        value = re.sub(r"(?i)\b(?:AWS_SECRET_ACCESS_KEY|GITHUB_TOKEN|OPENAI_API_KEY|ANTHROPIC_API_KEY|GEMINI_API_KEY|GOOGLE_API_KEY)\s*[:=]\s*[^\s,\"']+", "<secret-assignment-redacted>", value)
        value = re.sub(r"-----BEGIN [A-Z ]*PRIVATE KEY-----.*?-----END [A-Z ]*PRIVATE KEY-----", "<private-key-redacted>", value, flags=re.DOTALL)
        return value
    return value

payload = redact({
    "schema_version": "adl.remote_validation_run.v1",
    "runner": "nessus",
    "run_id": run_id,
    "status": status,
    "exit_code": int(exit_code),
    "failure_class": failure_class,
    "cleanup": {
        "attempted": cleanup_attempted == "true",
        "complete": cleanup_complete == "true",
        "detail": None if cleanup_complete == "true" else "remote command termination was not observed",
    },
    "started_at": started_at,
    "finished_at": finished_at,
    "elapsed_seconds": int(elapsed_seconds),
    "repo_url": repo_url,
    "git_ref": git_ref,
    "resolved_commit": resolved_commit,
    "command": command,
    "remote_root": remote_root,
    "run_root": run_root,
    "repo_dir": repo_dir,
    "target_dir": target_dir,
    "sccache_dir": sccache_dir,
    "builder_image": builder_image,
    "builder_runtime": builder_runtime,
    "resolved_builder_runtime": resolved_builder_runtime,
    "builder_pull_policy": builder_pull_policy,
    "builder_image_local_present": builder_image_local_present == "true",
    "builder_image_pull_attempted": builder_image_pull_attempted == "true",
    "cache_status": cache_status,
    "apt_sources_list": apt_sources_list,
    "apt_kubernetes_list": apt_kubernetes_list,
    "logs": {
        "host_facts": os.path.join(run_root, "host-facts.log"),
        "rustc_version": os.path.join(run_root, "rustc-version.log"),
        "cargo_version": os.path.join(run_root, "cargo-version.log"),
        "sccache_version": os.path.join(run_root, "sccache-version.log"),
        "apt_update": os.path.join(run_root, "apt-update.log"),
        "git_fetch": os.path.join(run_root, "git-fetch.log"),
        "git_checkout": os.path.join(run_root, "git-checkout.log"),
        "command": os.path.join(run_root, "command.log"),
        "sccache_stats": os.path.join(run_root, "sccache-stats.log"),
        "windows_identity": os.path.join(run_root, "windows-identity.txt"),
        "wsl_identity": os.path.join(run_root, "wsl-identity.txt"),
    },
})

with open(summary_path, "w", encoding="utf-8") as handle:
    json.dump(payload, handle, indent=2, sort_keys=True)
    handle.write("\n")
PY
}

terminate_active_command() {
  COMMAND_CLEANUP_ATTEMPTED=true
  local container_cleanup_complete=true
  if [[ -n "$ACTIVE_CONTAINER_NAME" ]]; then
    "$RESOLVED_BUILDER_RUNTIME" rm -f "$ACTIVE_CONTAINER_NAME" >/dev/null 2>&1 || container_cleanup_complete=false
  fi
  if [[ -n "$COMMAND_PID" ]] && kill -0 "$COMMAND_PID" 2>/dev/null; then
    kill -TERM -- "-$COMMAND_PID" 2>/dev/null || kill -TERM "$COMMAND_PID" 2>/dev/null || true
    for _ in $(seq 1 50); do
      kill -0 "$COMMAND_PID" 2>/dev/null || break
      sleep 0.1
    done
    if kill -0 "$COMMAND_PID" 2>/dev/null; then
      kill -KILL -- "-$COMMAND_PID" 2>/dev/null || kill -KILL "$COMMAND_PID" 2>/dev/null || true
    fi
    wait "$COMMAND_PID" 2>/dev/null || true
  fi
  if { [[ -z "$COMMAND_PID" ]] || ! kill -0 "$COMMAND_PID" 2>/dev/null; } && \
    [[ "$container_cleanup_complete" == true ]]; then
    COMMAND_CLEANUP_COMPLETE=true
  fi
}

handle_control_signal() {
  if [[ -e "$TIMEOUT_MARKER" ]]; then
    STATUS="timed_out"
    FAILURE_CLASS="timeout"
    COMMAND_EXIT=124
  else
    STATUS="cancelled"
    FAILURE_CLASS="cancelled"
    COMMAND_EXIT=130
  fi
  terminate_active_command
  exit "$COMMAND_EXIT"
}

redact_retained_logs() {
  python3 - "$RUN_ROOT" <<'PY'
import re
import sys
from pathlib import Path

root = Path(sys.argv[1])
patterns = (
    (re.compile(r"/(?:Users|Volumes|private|tmp|root)/[^\s,\"]*"), "<machine-path-redacted>"),
    (re.compile(r"arn:aws:[^\s,\"]*"), "<aws-arn-redacted>"),
    (re.compile(r"\b\d{12}\b"), "<account-id-redacted>"),
    (re.compile(r"\bi-[0-9a-f]{8,17}\b"), "<instance-id-redacted>"),
    (re.compile(r"\b(?:\d{1,3}\.){3}\d{1,3}\b"), "<ip-address-redacted>"),
    (re.compile(r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b"), "<aws-access-key-redacted>"),
    (re.compile(r"(?i)\b(Bearer|Basic)\s+[A-Za-z0-9._~+/=-]+"), r"\1 <credential-redacted>"),
    (re.compile(r"\b(?:gh[pousr]_[A-Za-z0-9_]{20,}|github_pat_[A-Za-z0-9_]{20,})\b"), "<github-token-redacted>"),
    (re.compile(r"(?i)\b(?:AWS_SECRET_ACCESS_KEY|GITHUB_TOKEN|OPENAI_API_KEY|ANTHROPIC_API_KEY|GEMINI_API_KEY|GOOGLE_API_KEY)\s*[:=]\s*[^\s,\"']+"), "<secret-assignment-redacted>"),
    (re.compile(r"-----BEGIN [A-Z ]*PRIVATE KEY-----.*?-----END [A-Z ]*PRIVATE KEY-----", re.DOTALL), "<private-key-redacted>"),
)
for path in root.glob("*"):
    if not path.is_file() or path.name in {"runner.pid", "cancel.request", "timeout.request"}:
        continue
    text = path.read_text(encoding="utf-8", errors="replace")
    for pattern, replacement in patterns:
        if replacement == "<machine-path-redacted>":
            text = pattern.sub(
                lambda match: f"<machine-path-redacted>/{Path(match.group(0)).name}",
                text,
            )
        else:
            text = pattern.sub(replacement, text)
    path.write_text(text, encoding="utf-8")
PY
}

finish() {
  local exit_code="$1"
  set +e
  if [[ -z "$BUILDER_IMAGE" ]] && command -v sccache >/dev/null 2>&1; then
    sccache --show-stats >"$RUN_ROOT/sccache-stats.log" 2>&1
  fi
  restore_apt_sources
  redact_retained_logs
  mkdir -p "$PUBLISH_ROOT"
  python3 - "$RUN_ROOT" "$PUBLISH_ROOT" <<'PY'
import shutil
import sys
from pathlib import Path

source, destination = map(Path, sys.argv[1:])
for path in source.glob("*"):
    if path.is_file() and path.name not in {"runner.pid", "cancel.request", "timeout.request"}:
        shutil.copyfile(path, destination / path.name)
PY
  write_summary "$exit_code"
  rm -rf "$RUN_ROOT"
  exit "$exit_code"
}

trap 'finish $?' EXIT
trap 'handle_control_signal' TERM INT HUP

printf '%s\n' "${ADL_NESSUS_WINDOWS_IDENTITY:-unknown}" >"$WINDOWS_IDENTITY_FILE"
whoami >"$WSL_IDENTITY_FILE"
{
  echo "whoami=$(whoami)"
  echo "uname=$(uname -s)"
  echo "kernel=$(uname -r)"
  if command -v nproc >/dev/null 2>&1; then
    echo "cpus=$(nproc)"
  fi
  if command -v free >/dev/null 2>&1; then
    free -h
  fi
} >"$RUN_ROOT/host-facts.log" 2>&1

if [[ -f "$HOME/.cargo/env" ]]; then
  # Non-login WSL shells need the Cargo toolchain path restored explicitly.
  # The proven interactive path already had rust/cargo/sccache available.
  # The wrapper preserves that truth for automation without assuming systemwide
  # installation.
  # shellcheck disable=SC1090
  . "$HOME/.cargo/env"
fi

command -v git >/dev/null 2>&1

resolve_builder_runtime() {
  if [[ -z "$BUILDER_IMAGE" ]]; then
    RESOLVED_BUILDER_RUNTIME="none"
    return 0
  fi
  if [[ "$BUILDER_RUNTIME" == "docker" || "$BUILDER_RUNTIME" == "podman" ]]; then
    command -v "$BUILDER_RUNTIME" >/dev/null 2>&1
    RESOLVED_BUILDER_RUNTIME="$BUILDER_RUNTIME"
    return 0
  fi
  if command -v docker >/dev/null 2>&1; then
    RESOLVED_BUILDER_RUNTIME="docker"
    return 0
  fi
  if command -v podman >/dev/null 2>&1; then
    RESOLVED_BUILDER_RUNTIME="podman"
    return 0
  fi
  echo "ADL builder image requested but neither docker nor podman is available" >&2
  return 2
}

run_in_builder_image() {
  local command="$1"
  if [[ -n "$ACTIVE_CONTAINER_NAME" ]]; then
    "$RESOLVED_BUILDER_RUNTIME" run --rm --name "$ACTIVE_CONTAINER_NAME" \
      -v "$REPO_DIR:/workspace" \
      -v "$TARGET_DIR:/workspace/target" \
      -v "$SCCACHE_DIR:/cache/sccache" \
      -e CARGO_TARGET_DIR=/workspace/target \
      -e SCCACHE_DIR=/cache/sccache \
      -e RUSTC_WRAPPER=sccache \
      -w /workspace \
      "$BUILDER_IMAGE" \
      "$command"
    return
  fi
  "$RESOLVED_BUILDER_RUNTIME" run --rm \
    -v "$REPO_DIR:/workspace" \
    -v "$TARGET_DIR:/workspace/target" \
    -v "$SCCACHE_DIR:/cache/sccache" \
    -e CARGO_TARGET_DIR=/workspace/target \
    -e SCCACHE_DIR=/cache/sccache \
    -e RUSTC_WRAPPER=sccache \
    -w /workspace \
    "$BUILDER_IMAGE" \
    "$command"
}

ensure_builder_image_available() {
  if [[ -z "$BUILDER_IMAGE" ]]; then
    return 0
  fi

  if "$RESOLVED_BUILDER_RUNTIME" image inspect "$BUILDER_IMAGE" >/dev/null 2>&1; then
    BUILDER_IMAGE_LOCAL_PRESENT=true
  fi

  case "$BUILDER_PULL_POLICY" in
    never)
      if [[ "$BUILDER_IMAGE_LOCAL_PRESENT" == true ]]; then
        printf 'skipped: builder pull policy never and image already present\n' >"$RUN_ROOT/builder-image-pull.log"
        return 0
      fi
      printf 'failed: builder pull policy never and image not present locally: %s\n' "$BUILDER_IMAGE" >"$RUN_ROOT/builder-image-pull.log"
      echo "ADL builder image '$BUILDER_IMAGE' is not present locally and builder pull policy is never" >&2
      return 2
      ;;
    always)
      BUILDER_IMAGE_PULL_ATTEMPTED=true
      "$RESOLVED_BUILDER_RUNTIME" pull "$BUILDER_IMAGE" >"$RUN_ROOT/builder-image-pull.log" 2>&1
      ;;
    missing)
      if [[ "$BUILDER_IMAGE_LOCAL_PRESENT" == true ]]; then
        printf 'skipped: builder image already present locally\n' >"$RUN_ROOT/builder-image-pull.log"
      else
        BUILDER_IMAGE_PULL_ATTEMPTED=true
        "$RESOLVED_BUILDER_RUNTIME" pull "$BUILDER_IMAGE" >"$RUN_ROOT/builder-image-pull.log" 2>&1
      fi
      ;;
  esac

  if "$RESOLVED_BUILDER_RUNTIME" image inspect "$BUILDER_IMAGE" >/dev/null 2>&1; then
    BUILDER_IMAGE_LOCAL_PRESENT=true
  fi
}

preflight_raw_host_command() {
  if [[ -n "$BUILDER_IMAGE" ]]; then
    return 0
  fi
  if [[ ( "$COMMAND_STRING" == *"nextest"* || "$COMMAND_STRING" == *"run_pr_fast_test_lane.sh"* ) ]] \
    && ! cargo nextest --version >/dev/null 2>&1; then
    printf 'failed: command requires cargo nextest but raw host lacks cargo-nextest; set ADL_NESSUS_BUILDER_IMAGE or install cargo-nextest\n' >"$RUN_ROOT/preflight.log"
    echo "run_nessus_remote_validation: command requires cargo nextest but raw host lacks cargo-nextest; set ADL_NESSUS_BUILDER_IMAGE or install cargo-nextest" >&2
    return 2
  fi
}

resolve_builder_runtime
if [[ -n "$BUILDER_IMAGE" ]]; then
  ensure_builder_image_available
  run_in_builder_image "rustc --version" >"$RUN_ROOT/rustc-version.log" 2>&1
  run_in_builder_image "cargo --version" >"$RUN_ROOT/cargo-version.log" 2>&1
  run_in_builder_image "cargo nextest --version" >"$RUN_ROOT/nextest-version.log" 2>&1
  run_in_builder_image "sccache --version" >"$RUN_ROOT/sccache-version.log" 2>&1
  run_in_builder_image "sccache --zero-stats >/dev/null 2>&1 || true" >/dev/null 2>&1 || true
else
  command -v rustc >/dev/null 2>&1
  command -v cargo >/dev/null 2>&1
  command -v sccache >/dev/null 2>&1

  rustc --version >"$RUN_ROOT/rustc-version.log" 2>&1
  cargo --version >"$RUN_ROOT/cargo-version.log" 2>&1
  preflight_raw_host_command
  sccache --version >"$RUN_ROOT/sccache-version.log" 2>&1
  sccache --zero-stats >/dev/null 2>&1 || true
fi

if [[ -z "$BUILDER_IMAGE" ]]; then
  if [[ -f "$APT_KUBERNETES_LIST" ]]; then
    KUBERNETES_BACKUP="$RUN_ROOT/kubernetes.list.backup"
    mv "$APT_KUBERNETES_LIST" "$KUBERNETES_BACKUP"
    APT_MASKED=true
  fi
  if [[ -f "$APT_SOURCES_LIST" ]] && grep -q 'apt.releases.hashicorp.com' "$APT_SOURCES_LIST"; then
    SOURCES_BACKUP="$RUN_ROOT/sources.list.backup"
    cp "$APT_SOURCES_LIST" "$SOURCES_BACKUP"
    python3 - <<'PY' "$APT_SOURCES_LIST"
from pathlib import Path
import sys

path = Path(sys.argv[1])
lines = path.read_text(encoding="utf-8").splitlines()
rewritten = []
for line in lines:
    if "apt.releases.hashicorp.com" in line and not line.lstrip().startswith("#"):
        rewritten.append(f"# ADL-temporarily-masked {line}")
    else:
        rewritten.append(line)
path.write_text("\n".join(rewritten) + "\n", encoding="utf-8")
PY
    HASHICORP_MASKED=true
  fi
  apt-get update >"$RUN_ROOT/apt-update.log" 2>&1
else
  printf 'skipped: builder image mode uses container toolchain\n' >"$RUN_ROOT/apt-update.log"
fi

mkdir -p "$REMOTE_ROOT"
if [[ ! -d "$REPO_DIR/.git" ]]; then
  git clone "$REPO_URL" "$REPO_DIR" >"$RUN_ROOT/git-clone.log" 2>&1
fi

git -C "$REPO_DIR" reset --hard HEAD >"$RUN_ROOT/git-reset.log" 2>&1
git -C "$REPO_DIR" clean -fd >"$RUN_ROOT/git-clean.log" 2>&1
git -C "$REPO_DIR" fetch origin --prune >"$RUN_ROOT/git-fetch.log" 2>&1
CHECKOUT_REF="$GIT_REF"
REMOTE_REF="${GIT_REF#refs/heads/}"
if git -C "$REPO_DIR" show-ref --verify --quiet "refs/remotes/origin/$REMOTE_REF"; then
  CHECKOUT_REF="origin/$REMOTE_REF"
fi
git -C "$REPO_DIR" checkout --detach "$CHECKOUT_REF" >"$RUN_ROOT/git-checkout.log" 2>&1
RESOLVED_COMMIT="$(git -C "$REPO_DIR" rev-parse HEAD)"
if [[ -n "$EXPECTED_REVISION" && "$RESOLVED_COMMIT" != "$EXPECTED_REVISION" ]]; then
  echo "resolved revision does not match portable request" >&2
  exit 2
fi

export CARGO_TARGET_DIR="$TARGET_DIR"
export SCCACHE_DIR
export RUSTC_WRAPPER="sccache"
if [[ -z "$BUILDER_IMAGE" ]]; then
  if [[ -x "$REPO_DIR/adl/tools/rust_cache_env.sh" ]]; then
    ADL_RUST_CACHE_TARGET_DIR="$TARGET_DIR" \
    ADL_RUST_CACHE_SCCACHE_DIR="$SCCACHE_DIR" \
    ADL_RUST_CACHE_REQUIRE_SCCACHE=1 \
    ADL_RUST_CACHE_USE_LLD=auto \
      bash "$REPO_DIR/adl/tools/rust_cache_env.sh" write-shell-env "$RUN_ROOT/rust-cache-env.sh"
    # shellcheck disable=SC1090
    . "$RUN_ROOT/rust-cache-env.sh"
  else
    printf 'skipped: cloned ref does not contain adl/tools/rust_cache_env.sh\n' >"$RUN_ROOT/rust-cache-env.log"
  fi
fi

RUN_COMMAND="$COMMAND_STRING"
if [[ -n "$BUILDER_IMAGE" ]]; then
  ACTIVE_CONTAINER_NAME="adl-nessus-${RUN_ID//[^a-zA-Z0-9_.-]/-}"
  ACTIVE_CONTAINER_NAME="${ACTIVE_CONTAINER_NAME:0:63}"
  run_in_builder_image "$RUN_COMMAND" >"$RUN_ROOT/command.log" 2>&1 &
else
  if command -v setsid >/dev/null 2>&1; then
    setsid bash -lc "cd '$REPO_DIR' && $RUN_COMMAND" >"$RUN_ROOT/command.log" 2>&1 &
  else
    bash -lc "cd '$REPO_DIR' && $RUN_COMMAND" >"$RUN_ROOT/command.log" 2>&1 &
  fi
fi
COMMAND_PID=$!
while kill -0 "$COMMAND_PID" 2>/dev/null; do
  if [[ -e "$CANCELLATION_MARKER" ]]; then
    STATUS="cancelled"
    FAILURE_CLASS="cancelled"
    COMMAND_EXIT=130
    terminate_active_command
    break
  fi
  if [[ -n "$TIMEOUT_SECONDS" ]] && (( $(date +%s) - START_EPOCH >= TIMEOUT_SECONDS )); then
    STATUS="timed_out"
    FAILURE_CLASS="timeout"
    COMMAND_EXIT=124
    terminate_active_command
    break
  fi
  sleep 0.1
done
if [[ "$COMMAND_EXIT" -ne 124 && "$COMMAND_EXIT" -ne 130 ]]; then
  set +e
  wait "$COMMAND_PID"
  COMMAND_EXIT=$?
  set -e
  COMMAND_CLEANUP_ATTEMPTED=true
  COMMAND_CLEANUP_COMPLETE=true
  if [[ "$COMMAND_EXIT" -eq 0 ]]; then
    STATUS="passed"
    FAILURE_CLASS="none"
  else
    STATUS="failed"
    FAILURE_CLASS="validation"
  fi
fi
if [[ -n "$BUILDER_IMAGE" ]]; then
  run_in_builder_image "sccache --show-stats" >"$RUN_ROOT/sccache-stats.log" 2>&1 || true
fi

exit "$COMMAND_EXIT"
EOF
chmod +x "$REMOTE_SCRIPT"

REMOTE_SUMMARY_PATH="$REMOTE_ROOT/logs/$RUN_ID/$SUMMARY_NAME"
REMOTE_RUN_ROOT="$REMOTE_ROOT/transient/$RUN_ID"
REMOTE_PUBLISH_ROOT="$REMOTE_ROOT/logs/$RUN_ID"
LOCAL_SUMMARY_PATH=""

run_remote() {
  if [[ "$EXECUTOR" == "ssh" ]]; then
    local remote_cmd
    remote_cmd="wsl.exe -u $WSL_USER -- bash -s -- '$(quote_remote_single "$REMOTE_ROOT")' '$(quote_remote_single "$REPO_URL")' '$(quote_remote_single "$GIT_REF")' '$(quote_remote_single "$RUN_ID")' '$(quote_remote_single "$COMMAND_B64")' '$(quote_remote_single "$SUMMARY_NAME")' '$(quote_remote_single "$BUILDER_IMAGE")' '$(quote_remote_single "$BUILDER_RUNTIME")' '$(quote_remote_single "$BUILDER_PULL_POLICY")' '$(quote_remote_single "$PORTABLE_CPU_CORES")' '$(quote_remote_single "$PORTABLE_MEMORY_MIB")' '$(quote_remote_single "$PORTABLE_TIMEOUT_SECONDS")' '$(quote_remote_single "$PORTABLE_EXPECTED_REVISION")'"
    "$SSH_BIN" -o BatchMode=yes -o ConnectTimeout=15 "${SSH_USER}@${HOST}" "$remote_cmd" <"$REMOTE_SCRIPT"
  else
    ADL_NESSUS_WINDOWS_IDENTITY="local-executor" bash "$REMOTE_SCRIPT" \
      "$REMOTE_ROOT" "$REPO_URL" "$GIT_REF" "$RUN_ID" "$COMMAND_B64" "$SUMMARY_NAME" "$BUILDER_IMAGE" "$BUILDER_RUNTIME" "$BUILDER_PULL_POLICY" "$PORTABLE_CPU_CORES" "$PORTABLE_MEMORY_MIB" "$PORTABLE_TIMEOUT_SECONDS" "$PORTABLE_EXPECTED_REVISION"
  fi
}

fetch_summary() {
  local destination="$1"
  if [[ "$EXECUTOR" == "ssh" ]]; then
    local remote_cmd
    remote_cmd="wsl.exe -u $WSL_USER -- bash -lc 'cat '\''$(quote_remote_single "$REMOTE_SUMMARY_PATH")'\'''"
    "$SSH_BIN" -o BatchMode=yes -o ConnectTimeout=15 "${SSH_USER}@${HOST}" "$remote_cmd" >"$destination"
  else
    cp "$REMOTE_SUMMARY_PATH" "$destination"
  fi
}

fetch_logs_tarball() {
  local destination="$1"
  if [[ "$EXECUTOR" == "ssh" ]]; then
    local remote_cmd
    remote_cmd="wsl.exe -u $WSL_USER -- bash -lc 'tar -C '\''$(quote_remote_single "$REMOTE_PUBLISH_ROOT")'\'' -czf - .'"
    "$SSH_BIN" -o BatchMode=yes -o ConnectTimeout=15 "${SSH_USER}@${HOST}" "$remote_cmd" >"$destination"
  else
    tar -C "$REMOTE_PUBLISH_ROOT" -czf "$destination" .
  fi
}

write_transport_failure_summary() {
  local destination="$1"
  python3 - <<'PY' \
    "$destination" "$RUN_ID" "$RUN_EXIT" "$COMMAND_STRING" "$EXECUTOR" "$HOST" "$SSH_USER" \
    "$REMOTE_ROOT" "$REMOTE_SUMMARY_PATH" "$REMOTE_RUN_ROOT" "$REPO_URL" "$GIT_REF"
import json
import sys

(
    destination,
    run_id,
    exit_code,
    command,
    executor,
    host,
    ssh_user,
    remote_root,
    remote_summary_path,
    remote_run_root,
    repo_url,
    git_ref,
) = sys.argv[1:]

payload = {
    "schema_version": "adl.remote_validation_run.v1",
    "runner": "nessus",
    "run_id": run_id,
    "status": "failed",
    "exit_code": int(exit_code),
    "failure_class": "provider_availability",
    "cleanup": {
        "attempted": False,
        "complete": False,
        "detail": "transport failed before remote process cleanup could be observed",
    },
    "command": command,
    "repo_url": repo_url,
    "git_ref": git_ref,
    "resolved_commit": "unknown",
    "remote_root": remote_root,
    "run_root": remote_run_root,
    "repo_dir": f"{remote_root}/agent-design-language",
    "target_dir": f"{remote_root}/cache/target",
    "sccache_dir": f"{remote_root}/cache/sccache",
    "started_at": "unknown",
    "finished_at": "unknown",
    "elapsed_seconds": 0,
    "transport_failure": {
        "executor": executor,
        "host": host,
        "ssh_user": ssh_user,
        "summary_fetch_failed": True,
        "expected_remote_summary_path": remote_summary_path,
    },
    "logs": {
        "host_facts": "not_available",
        "rustc_version": "not_available",
        "cargo_version": "not_available",
        "sccache_version": "not_available",
        "apt_update": "not_available",
        "git_fetch": "not_available",
        "git_checkout": "not_available",
        "command": "not_available",
        "sccache_stats": "not_available",
        "windows_identity": "not_available",
        "wsl_identity": "not_available",
    },
    "cache_status": {
        "target_dir": f"{remote_root}/cache/target",
        "sccache_dir": f"{remote_root}/cache/sccache",
        "sccache_stats_log": "not_available",
    },
    "apt_sources_list": "unknown",
    "apt_kubernetes_list": "unknown",
}

def redact(value):
    import re
    if isinstance(value, dict):
        return {key: redact(item) for key, item in value.items()}
    if isinstance(value, list):
        return [redact(item) for item in value]
    if isinstance(value, str):
        value = re.sub(r"/(?:Users|Volumes|private|tmp|root)/[^\s,\"]*", "<machine-path-redacted>", value)
        value = re.sub(r"arn:aws:[^\s,\"]*", "<aws-arn-redacted>", value)
        value = re.sub(r"\b\d{12}\b", "<account-id-redacted>", value)
        value = re.sub(r"\bi-[0-9a-f]{8,17}\b", "<instance-id-redacted>", value)
        value = re.sub(r"\b(?:\d{1,3}\.){3}\d{1,3}\b", "<ip-address-redacted>", value)
        value = re.sub(r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b", "<aws-access-key-redacted>", value)
        value = re.sub(r"(?i)\b(Bearer|Basic)\s+[A-Za-z0-9._~+/=-]+", r"\1 <credential-redacted>", value)
        value = re.sub(r"\b(?:gh[pousr]_[A-Za-z0-9_]{20,}|github_pat_[A-Za-z0-9_]{20,})\b", "<github-token-redacted>", value)
        value = re.sub(r"(?i)\b(?:AWS_SECRET_ACCESS_KEY|GITHUB_TOKEN|OPENAI_API_KEY|ANTHROPIC_API_KEY|GEMINI_API_KEY|GOOGLE_API_KEY)\s*[:=]\s*[^\s,\"']+", "<secret-assignment-redacted>", value)
        value = re.sub(r"-----BEGIN [A-Z ]*PRIVATE KEY-----.*?-----END [A-Z ]*PRIVATE KEY-----", "<private-key-redacted>", value, flags=re.DOTALL)
    return value

with open(destination, "w", encoding="utf-8") as handle:
    json.dump(redact(payload), handle, indent=2, sort_keys=True)
    handle.write("\n")
PY
}

STARTED_UNIX_MS="$(python3 -c 'import time; print(time.time_ns() // 1000000)')"
signal_remote_stop() {
  local reason="$1"
  local marker="$REMOTE_RUN_ROOT/${reason}.request"
  local pid_file="$REMOTE_RUN_ROOT/runner.pid"
  if [[ "$EXECUTOR" == "ssh" ]]; then
    local remote_cmd
    remote_cmd="wsl.exe -u $WSL_USER -- bash -lc 'mkdir -p '\''$(quote_remote_single "$REMOTE_RUN_ROOT")'\''; touch '\''$(quote_remote_single "$marker")'\''; if [[ -s '\''$(quote_remote_single "$pid_file")'\'' ]]; then kill -TERM \$(cat '\''$(quote_remote_single "$pid_file")'\'') 2>/dev/null || true; fi'"
    "$SSH_BIN" -o BatchMode=yes -o ConnectTimeout=15 "${SSH_USER}@${HOST}" "$remote_cmd" >/dev/null 2>&1 || return 1
  else
    mkdir -p "$REMOTE_RUN_ROOT"
    touch "$marker"
    if [[ -s "$pid_file" ]]; then
      kill -TERM "$(cat "$pid_file")" 2>/dev/null || true
    fi
  fi
}

set +e
run_remote &
TRANSPORT_PID=$!
set -e
CONTROL_EXIT=""
while kill -0 "$TRANSPORT_PID" 2>/dev/null; do
  if [[ -n "$PORTABLE_CANCELLATION_FILE" && -e "$ROOT_DIR/$PORTABLE_CANCELLATION_FILE" ]]; then
    CONTROL_EXIT=130
    signal_remote_stop cancel || true
    break
  fi
  if [[ -n "$PORTABLE_TIMEOUT_SECONDS" ]] && \
    (( $(python3 -c 'import time; print(time.time_ns() // 1000000)') - STARTED_UNIX_MS >= PORTABLE_TIMEOUT_SECONDS * 1000 )); then
    CONTROL_EXIT=124
    signal_remote_stop timeout || true
    break
  fi
  sleep 0.1
done
if [[ -n "$CONTROL_EXIT" ]]; then
  for _ in $(seq 1 50); do
    kill -0 "$TRANSPORT_PID" 2>/dev/null || break
    sleep 0.1
  done
  if kill -0 "$TRANSPORT_PID" 2>/dev/null; then
    kill -TERM "$TRANSPORT_PID" 2>/dev/null || true
  fi
fi
set +e
wait "$TRANSPORT_PID"
RUN_EXIT=$?
set -e
if [[ -n "$CONTROL_EXIT" ]]; then
  RUN_EXIT="$CONTROL_EXIT"
fi
FINISHED_UNIX_MS="$(python3 -c 'import time; print(time.time_ns() // 1000000)')"

SUMMARY_FETCH_OK=true
if [[ -n "$LOCAL_ARTIFACT_DIR" ]]; then
  mkdir -p "$LOCAL_ARTIFACT_DIR"
  LOCAL_SUMMARY_PATH="$LOCAL_ARTIFACT_DIR/$SUMMARY_NAME"
else
  LOCAL_SUMMARY_PATH="$TMP_DIR/$SUMMARY_NAME"
fi

set +e
fetch_summary "$LOCAL_SUMMARY_PATH"
SUMMARY_FETCH_RC=$?
set -e
if [[ "$SUMMARY_FETCH_RC" -ne 0 ]]; then
  SUMMARY_FETCH_OK=false
  if [[ "$RUN_EXIT" -ne 0 ]]; then
    write_transport_failure_summary "$LOCAL_SUMMARY_PATH"
  else
    echo "run_nessus_remote_validation: failed to fetch remote summary from $REMOTE_SUMMARY_PATH" >&2
    exit "$SUMMARY_FETCH_RC"
  fi
fi

if [[ -n "$LOCAL_ARTIFACT_DIR" ]]; then
  set +e
  fetch_logs_tarball "$LOCAL_ARTIFACT_DIR/run-logs.tar.gz"
  LOG_FETCH_RC=$?
  set -e
  if [[ "$LOG_FETCH_RC" -ne 0 && "$RUN_EXIT" -eq 0 ]]; then
    echo "run_nessus_remote_validation: failed to fetch remote log tarball from $REMOTE_RUN_ROOT" >&2
    exit "$LOG_FETCH_RC"
  fi
fi

if [[ -n "$PORTABLE_REQUEST" ]]; then
  PORTABLE_ARTIFACT_ROOT="${LOCAL_ARTIFACT_DIR:-$TMP_DIR}/portable-artifacts"
  PORTABLE_EXECUTION="${LOCAL_ARTIFACT_DIR:-$TMP_DIR}/portable-execution.json"
  PORTABLE_RESULT="${LOCAL_ARTIFACT_DIR:-$TMP_DIR}/portable-result.json"
  mkdir -p "$PORTABLE_ARTIFACT_ROOT"
  python3 - "$PORTABLE_REQUEST" "$LOCAL_SUMMARY_PATH" "$PORTABLE_ARTIFACT_ROOT" <<'PY'
import json
import re
import sys
from pathlib import Path

request_path, summary_path, artifact_root = map(Path, sys.argv[1:])
request = json.loads(request_path.read_text(encoding="utf-8"))
summary = json.loads(summary_path.read_text(encoding="utf-8"))

def redact(value):
    if isinstance(value, dict):
        return {key: redact(item) for key, item in value.items()}
    if isinstance(value, list):
        return [redact(item) for item in value]
    if isinstance(value, str):
        value = re.sub(r"/(?:Users|Volumes|private|tmp|root)/[^\s,\"]*", "<machine-path-redacted>", value)
        value = re.sub(r"\b(?:\d{1,3}\.){3}\d{1,3}\b", "<ip-address-redacted>", value)
        value = re.sub(r"arn:aws:[^\s,\"]*", "<aws-arn-redacted>", value)
        value = re.sub(r"\b\d{12}\b", "<account-id-redacted>", value)
        value = re.sub(r"\bi-[0-9a-f]{8,17}\b", "<instance-id-redacted>", value)
        value = re.sub(r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b", "<aws-access-key-redacted>", value)
        value = re.sub(r"(?i)\b(Bearer|Basic)\s+[A-Za-z0-9._~+/=-]+", r"\1 <credential-redacted>", value)
        value = re.sub(r"\b(?:gh[pousr]_[A-Za-z0-9_]{20,}|github_pat_[A-Za-z0-9_]{20,})\b", "<github-token-redacted>", value)
        value = re.sub(r"(?i)\b(?:AWS_SECRET_ACCESS_KEY|GITHUB_TOKEN|OPENAI_API_KEY|ANTHROPIC_API_KEY|GEMINI_API_KEY|GOOGLE_API_KEY)\s*[:=]\s*[^\s,\"']+", "<secret-assignment-redacted>", value)
        value = re.sub(r"-----BEGIN [A-Z ]*PRIVATE KEY-----.*?-----END [A-Z ]*PRIVATE KEY-----", "<private-key-redacted>", value, flags=re.DOTALL)
        return value
    return value

summary = redact(summary)
paths = request["artifact_policy"]["paths"]
for index, relative in enumerate(paths):
    destination = artifact_root / relative
    destination.parent.mkdir(parents=True, exist_ok=True)
    if index == 0:
        destination.write_text(json.dumps(summary, sort_keys=True, separators=(",", ":")) + "\n", encoding="utf-8")
PY
  python3 - "$PORTABLE_EXECUTION" "$PORTABLE_REQUEST" "$PORTABLE_EXPECTED_REVISION" \
    "$STARTED_UNIX_MS" "$FINISHED_UNIX_MS" "$RUN_EXIT" "$SUMMARY_FETCH_OK" "$EXECUTOR" \
    "$LOCAL_SUMMARY_PATH" "${LOCAL_ARTIFACT_DIR:-$TMP_DIR}/run-logs.tar.gz" <<'PY'
import json
import re
import sys
import tarfile
from pathlib import Path

path, request_path, revision, started, finished, exit_code, summary_fetch, executor, summary_path, archive_path = sys.argv[1:]
request = json.load(open(request_path, encoding="utf-8"))
summary = json.load(open(summary_path, encoding="utf-8"))
exit_code = int(exit_code)
passed = exit_code == 0 and summary_fetch == "true"
failure_class = summary.get("failure_class", "unknown")
cleanup = summary.get("cleanup") or {
    "attempted": False,
    "complete": False,
    "detail": "provider cleanup was not observed",
}
if passed:
    outcome = "passed"
elif exit_code == 130 or failure_class == "cancelled":
    outcome = "cancelled"
elif exit_code == 124 or failure_class == "timeout":
    outcome = "timed_out"
elif failure_class in {"provider_availability", "authentication", "capacity"}:
    outcome = "provider_unavailable"
else:
    outcome = "failed"
fallback_allowed = (
    outcome == "provider_unavailable"
    and cleanup.get("complete") is True
    and request["fallback"] != "disabled"
)
sensitive_patterns = tuple(re.compile(pattern) for pattern in (
    r"/(?:Users|Volumes|private|tmp|root)/",
    r"arn:aws:",
    r"\b\d{12}\b",
    r"\bi-[0-9a-f]{8,17}\b",
    r"\b(?:\d{1,3}\.){3}\d{1,3}\b",
    r"\b(?:AKIA|ASIA)[0-9A-Z]{16}\b",
    r"(?i)\b(?:Bearer|Basic)\s+[A-Za-z0-9._~+/=-]+",
    r"\b(?:gh[pousr]_[A-Za-z0-9_]{20,}|github_pat_[A-Za-z0-9_]{20,})\b",
    r"(?i)\b(?:AWS_SECRET_ACCESS_KEY|GITHUB_TOKEN|OPENAI_API_KEY|ANTHROPIC_API_KEY|GEMINI_API_KEY|GOOGLE_API_KEY)\s*[:=]\s*[^\s,\"']+",
    r"-----BEGIN [A-Z ]*PRIVATE KEY-----",
))
retained_text = [json.dumps(summary, sort_keys=True)]
archive = Path(archive_path)
if archive.is_file() and tarfile.is_tarfile(archive):
    with tarfile.open(archive, "r:gz") as handle:
        for member in handle.getmembers():
            if not member.isfile():
                continue
            extracted = handle.extractfile(member)
            if extracted is not None:
                retained_text.append(extracted.read().decode("utf-8", errors="replace"))
redaction_passed = not any(
    pattern.search(text)
    for text in retained_text
    for pattern in sensitive_patterns
)
payload = {
    "schema": "adl.remote_validation.adapter_execution.v1",
    "adapter": "nessus",
    "platform": {
        "os": request["requested_platform"],
        "architecture": "x86_64",
        "native": executor == "ssh",
        "qualification": "live" if executor == "ssh" else "fixture",
    },
    "revision": revision,
    "started_unix_ms": int(started),
    "finished_unix_ms": int(finished),
    "exit_code": exit_code,
    "outcome": outcome,
    "redaction_passed": redaction_passed,
    "cleanup": cleanup,
    "fallback": {
        "policy": request["fallback"],
        "offered": fallback_allowed,
        "ran": False,
        "local_profile_digest": None,
    },
}
with open(path, "w", encoding="utf-8") as handle:
    json.dump(payload, handle, separators=(",", ":"))
PY
  "$PORTABLE_RUNNER" canonical-result "$PORTABLE_REQUEST" "$PORTABLE_EXECUTION" "$PORTABLE_ARTIFACT_ROOT" >"$PORTABLE_RESULT"
  cat "$PORTABLE_RESULT"
else
  python3 - <<'PY' "$LOCAL_SUMMARY_PATH"
import json
import sys

summary = json.load(open(sys.argv[1], encoding="utf-8"))
print(json.dumps(summary, indent=2, sort_keys=True))
PY
fi

if [[ "$RUN_EXIT" -ne 0 ]]; then
  if [[ "$SUMMARY_FETCH_OK" == true ]]; then
    echo "run_nessus_remote_validation: remote command failed (see summary above)" >&2
  else
    echo "run_nessus_remote_validation: transport failed before remote summary was available; fallback summary written locally" >&2
  fi
fi

exit "$RUN_EXIT"
