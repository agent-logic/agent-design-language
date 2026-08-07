#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/run_nessus_remote_validation.sh"
TMP="$(mktemp -d "${TMPDIR:?TMPDIR must name a bounded scratch root}/adl-nessus-test.XXXXXX")"
trap 'rm -rf "$TMP"' EXIT

assert_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "expected file to exist: $path" >&2
    exit 1
  fi
}

origin_src="$TMP/origin-src"
origin_bare="$TMP/origin.git"
mkdir -p "$origin_src"
git -C "$origin_src" init -q
git -C "$origin_src" branch -M main
cat >"$origin_src/README.md" <<'EOF'
# remote validation fixture
EOF
git -C "$origin_src" add README.md
git -C "$origin_src" -c user.name=Codex -c user.email=codex@example.com commit -q -m "fixture"
git clone -q --bare "$origin_src" "$origin_bare"

fake_bin="$TMP/fake-bin"
mkdir -p "$fake_bin"
cat >"$fake_bin/rustc" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "--version" ]]; then
  echo "rustc 1.96.0 (fixture)"
  exit 0
fi
echo "unexpected rustc invocation: $*" >&2
exit 1
EOF
cat >"$fake_bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == "--version" ]]; then
  echo "cargo 1.96.0 (fixture)"
  exit 0
fi
echo "unexpected cargo invocation: $*" >&2
exit 1
EOF
cat >"$fake_bin/sccache" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
  --version)
    echo "sccache 0.16.0"
    ;;
  --zero-stats)
    exit 0
    ;;
  --show-stats)
    cat <<'STATS'
Compile requests                      3
Compile requests executed             1
Cache hits                            2
Cache misses                          1
STATS
    ;;
  *)
    echo "unexpected sccache invocation: $*" >&2
    exit 1
    ;;
esac
EOF
cat >"$fake_bin/apt-get" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${FAIL_APT:-0}" == "1" ]]; then
  echo "apt-get fixture failure" >&2
  exit 1
fi
echo "apt-get update fixture ok"
EOF
cat >"$fake_bin/timeout" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
while [[ "${1:-}" == --* || "${1:-}" == *s ]]; do shift; done
exec "$@"
EOF
chmod +x "$fake_bin/"*

sources_list="$TMP/sources.list"
kubernetes_list="$TMP/kubernetes.list"
cat >"$sources_list" <<'EOF'
deb https://apt.releases.hashicorp.com focal main
EOF
cat >"$kubernetes_list" <<'EOF'
deb https://apt.kubernetes.io/ kubernetes-xenial main
EOF

PATH="$fake_bin:$PATH" \
ADL_NESSUS_APT_SOURCES_LIST="$sources_list" \
ADL_NESSUS_APT_KUBERNETES_LIST="$kubernetes_list" \
bash "$SCRIPT" \
  --executor local \
  --repo-url "$origin_bare" \
  --git-ref origin/main \
  --remote-root "$TMP/remote-root-pass" \
  --run-id fixture-pass \
  --command "printf remote-ok" \
  --local-artifact-dir "$TMP/artifacts-pass" \
  >"$TMP/pass.json"

assert_file "$TMP/artifacts-pass/summary.json"
assert_file "$TMP/artifacts-pass/run-logs.tar.gz"
python3 - <<'PY' "$TMP/artifacts-pass/summary.json"
import json
import sys

summary = json.load(open(sys.argv[1], encoding="utf-8"))
assert summary["schema_version"] == "adl.remote_validation_run.v1"
assert summary["runner"] == "nessus"
assert summary["status"] == "passed"
assert summary["resolved_commit"] != "unknown"
assert summary["command"] == "printf remote-ok"
assert summary["logs"]["command"].endswith("command.log")
PY

grep -F "apt.releases.hashicorp.com" "$sources_list" >/dev/null
assert_file "$kubernetes_list"

fixture_token="ghp_012345678901234567890123456789012345"
PATH="$fake_bin:$PATH" \
ADL_NESSUS_APT_SOURCES_LIST="$sources_list" \
ADL_NESSUS_APT_KUBERNETES_LIST="$kubernetes_list" \
bash "$SCRIPT" \
  --executor local \
  --repo-url "$origin_bare" \
  --git-ref origin/main \
  --remote-root "$TMP/remote-root-redaction" \
  --run-id fixture-redaction \
  --command "printf '$fixture_token /home/runner/work/repo /var/folders/fixture/repo'" \
  --local-artifact-dir "$TMP/artifacts-redaction" \
  >"$TMP/redaction.json"

assert_file "$TMP/artifacts-redaction/summary.json"
assert_file "$TMP/artifacts-redaction/run-logs.tar.gz"
mkdir -p "$TMP/artifacts-redaction-expanded"
tar -xzf "$TMP/artifacts-redaction/run-logs.tar.gz" -C "$TMP/artifacts-redaction-expanded"
if grep -R -F "$fixture_token" "$TMP/artifacts-redaction" "$TMP/artifacts-redaction-expanded" >/dev/null; then
  echo "expected credential-shaped fixture value to be redacted from retained evidence" >&2
  exit 1
fi
if grep -R -E '/home/runner|/var/folders/' "$TMP/artifacts-redaction" "$TMP/artifacts-redaction-expanded" >/dev/null; then
  echo "expected machine-path fixture values to be redacted from retained evidence" >&2
  exit 1
fi
grep -F "<github-token-redacted>" "$TMP/artifacts-redaction/summary.json" >/dev/null
grep -F "<machine-path-redacted>" "$TMP/artifacts-redaction/summary.json" >/dev/null

cat >"$fake_bin/docker" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "${1:-}" in
  image)
    [[ "${2:-}" == "inspect" ]]
    if [[ "${DOCKER_INSPECT_REQUIRES_PULL:-0}" == "1" && ! -f "${DOCKER_PULL_MARKER:?}" ]]; then
      exit 1
    fi
    exit 0
    ;;
  pull)
    if [[ "${FAIL_DOCKER_PULL:-0}" == "1" ]]; then
      echo "docker pull fixture failure" >&2
      exit 1
    fi
    printf 'pull %s\n' "${2:-}" >>"${DOCKER_PULL_LOG:-/dev/null}"
    if [[ -n "${DOCKER_PULL_MARKER:-}" ]]; then
      touch "$DOCKER_PULL_MARKER"
    fi
    exit 0
    ;;
  rm)
    [[ "${2:-}" == "-f" ]]
    printf 'rm %s\n' "${3:-}" >>"${DOCKER_RM_LOG:-/dev/null}"
    if [[ -s "${DOCKER_CONTAINER_PID_FILE:-}" ]]; then
      kill "$(cat "$DOCKER_CONTAINER_PID_FILE")" 2>/dev/null || true
    fi
    exit 0
    ;;
  run)
    shift
    container_name=""
    while [[ $# -gt 0 ]]; do
      case "$1" in
        --rm)
          shift
          ;;
        --name)
          container_name="${2:-}"
          shift 2
          ;;
        -v|-e|-w)
          shift 2
          ;;
        *)
          break
          ;;
      esac
    done
    image="${1:-}"
    command="${2:-}"
    if [[ "$image" != "example.invalid/adl-builder:test" ]]; then
      echo "unexpected image: $image" >&2
      exit 1
    fi
    case "$command" in
      "rustc --version")
        echo "rustc 1.96.0 (builder fixture)"
        ;;
      "cargo --version")
        echo "cargo 1.96.0 (builder fixture)"
        ;;
      "cargo nextest --version")
        echo "cargo-nextest 0.9.99 (builder fixture)"
        ;;
      "sccache --version")
        echo "sccache 0.16.0"
        ;;
      "sccache --zero-stats"* )
        exit 0
        ;;
      "sccache --show-stats")
        cat <<'STATS'
Compile requests                      5
Compile requests executed             1
Cache hits                            4
Cache misses                          1
STATS
        ;;
      "printf builder-ok")
        printf builder-ok
        ;;
      "cd -- '.' && env -i PATH=\"\${PATH-}\" 'sleep' '5'")
        printf '%s\n' "$$" >"${DOCKER_CONTAINER_PID_FILE:?}"
        trap 'rm -f "$DOCKER_CONTAINER_PID_FILE"' EXIT
        sleep 5
        ;;
      *)
        echo "unexpected builder command: $command" >&2
        exit 1
        ;;
    esac
    ;;
  *)
    echo "unexpected docker invocation: $*" >&2
    exit 1
    ;;
esac
EOF
chmod +x "$fake_bin/docker"

PATH="$fake_bin:$PATH" \
ADL_NESSUS_APT_SOURCES_LIST="$sources_list" \
ADL_NESSUS_APT_KUBERNETES_LIST="$kubernetes_list" \
DOCKER_PULL_LOG="$TMP/docker-pull.log" \
bash "$SCRIPT" \
  --executor local \
  --repo-url "$origin_bare" \
  --git-ref origin/main \
  --remote-root "$TMP/remote-root-builder" \
  --run-id fixture-builder \
  --command "printf builder-ok" \
  --builder-image "example.invalid/adl-builder:test" \
  --builder-pull-policy never \
  --local-artifact-dir "$TMP/artifacts-builder" \
  >"$TMP/builder.json"

assert_file "$TMP/artifacts-builder/summary.json"
python3 - <<'PY' "$TMP/artifacts-builder/summary.json"
import json
import sys

summary = json.load(open(sys.argv[1], encoding="utf-8"))
assert summary["status"] == "passed"
assert summary["builder_image"] == "example.invalid/adl-builder:test"
assert summary["builder_runtime"] == "auto"
assert summary["builder_pull_policy"] == "never"
assert summary["builder_image_local_present"] is True
assert summary["builder_image_pull_attempted"] is False
assert summary["resolved_builder_runtime"] == "docker"
assert summary["cache_status"]["cache_hits"] == "4"
PY
if [[ -s "$TMP/docker-pull.log" ]]; then
  echo "expected preloaded builder image run not to pull" >&2
  exit 1
fi

PATH="$fake_bin:$PATH" \
ADL_NESSUS_APT_SOURCES_LIST="$sources_list" \
ADL_NESSUS_APT_KUBERNETES_LIST="$kubernetes_list" \
DOCKER_PULL_LOG="$TMP/docker-pull-missing.log" \
DOCKER_INSPECT_REQUIRES_PULL=1 \
DOCKER_PULL_MARKER="$TMP/docker-pull-marker" \
bash "$SCRIPT" \
  --executor local \
  --repo-url "$origin_bare" \
  --git-ref origin/main \
  --remote-root "$TMP/remote-root-builder-pull" \
  --run-id fixture-builder-pull \
  --command "printf builder-ok" \
  --builder-image "example.invalid/adl-builder:test" \
  --builder-pull-policy missing \
  --local-artifact-dir "$TMP/artifacts-builder-pull" \
  >"$TMP/builder-pull.json"

assert_file "$TMP/artifacts-builder-pull/summary.json"
python3 - <<'PY' "$TMP/artifacts-builder-pull/summary.json"
import json
import sys

summary = json.load(open(sys.argv[1], encoding="utf-8"))
assert summary["status"] == "passed"
assert summary["builder_pull_policy"] == "missing"
assert summary["builder_image_local_present"] is True
assert summary["builder_image_pull_attempted"] is True
PY
grep -F "pull example.invalid/adl-builder:test" "$TMP/docker-pull-missing.log" >/dev/null

PATH="$fake_bin:$PATH" \
FAIL_APT=1 \
ADL_NESSUS_APT_SOURCES_LIST="$sources_list" \
ADL_NESSUS_APT_KUBERNETES_LIST="$kubernetes_list" \
bash "$SCRIPT" \
  --executor local \
  --repo-url "$origin_bare" \
  --git-ref origin/main \
  --remote-root "$TMP/remote-root-builder-apt-skip" \
  --run-id fixture-builder-apt-skip \
  --command "printf builder-ok" \
  --builder-image "example.invalid/adl-builder:test" \
  --builder-pull-policy never \
  --local-artifact-dir "$TMP/artifacts-builder-apt-skip" \
  >"$TMP/builder-apt-skip.json"

assert_file "$TMP/artifacts-builder-apt-skip/summary.json"
tar -xzf "$TMP/artifacts-builder-apt-skip/run-logs.tar.gz" -C "$TMP/artifacts-builder-apt-skip"
grep -F "skipped: builder image mode uses container toolchain" "$TMP/artifacts-builder-apt-skip/apt-update.log" >/dev/null

if PATH="$fake_bin:$PATH" \
  ADL_NESSUS_APT_SOURCES_LIST="$sources_list" \
  ADL_NESSUS_APT_KUBERNETES_LIST="$kubernetes_list" \
  bash "$SCRIPT" \
    --executor local \
    --repo-url "$origin_bare" \
    --git-ref origin/main \
    --remote-root "$TMP/remote-root-nextest-preflight" \
    --run-id fixture-nextest-preflight \
    --command "cargo nextest run --manifest-path adl/Cargo.toml" \
    --local-artifact-dir "$TMP/artifacts-nextest-preflight" \
    >"$TMP/nextest-preflight.json" 2>"$TMP/nextest-preflight.err"; then
  echo "expected raw-host missing cargo-nextest preflight to fail closed" >&2
  exit 1
fi

assert_file "$TMP/artifacts-nextest-preflight/summary.json"
tar -xzf "$TMP/artifacts-nextest-preflight/run-logs.tar.gz" -C "$TMP/artifacts-nextest-preflight"
grep -F "command requires cargo nextest but raw host lacks cargo-nextest" "$TMP/nextest-preflight.err" >/dev/null
grep -F "command requires cargo nextest but raw host lacks cargo-nextest" "$TMP/artifacts-nextest-preflight/preflight.log" >/dev/null

if PATH="$fake_bin:$PATH" \
  FAIL_APT=1 \
  ADL_NESSUS_APT_SOURCES_LIST="$sources_list" \
  ADL_NESSUS_APT_KUBERNETES_LIST="$kubernetes_list" \
  bash "$SCRIPT" \
    --executor local \
    --repo-url "$origin_bare" \
    --git-ref origin/main \
    --remote-root "$TMP/remote-root-fail" \
    --run-id fixture-fail \
    --command "printf should-not-run" \
    --local-artifact-dir "$TMP/artifacts-fail" \
    >"$TMP/fail.json" 2>"$TMP/fail.err"; then
  echo "expected apt failure path to fail closed" >&2
  exit 1
fi

assert_file "$TMP/artifacts-fail/summary.json"
python3 - <<'PY' "$TMP/artifacts-fail/summary.json"
import json
import sys

summary = json.load(open(sys.argv[1], encoding="utf-8"))
assert summary["status"] == "failed"
assert summary["exit_code"] != 0
assert summary["command"] == "printf should-not-run"
PY

cat >"$fake_bin/ssh-fail" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
echo "ssh fixture failure" >&2
exit 255
EOF
chmod +x "$fake_bin/ssh-fail"

if SSH_BIN="$fake_bin/ssh-fail" \
  bash "$SCRIPT" \
    --executor ssh \
    --host fixture.invalid \
    --ssh-user fixture \
    --run-id fixture-transport-fail \
    --command "printf no-transport" \
    --local-artifact-dir "$TMP/artifacts-transport-fail" \
    >"$TMP/transport-fail.json" 2>"$TMP/transport-fail.err"; then
  echo "expected transport failure path to fail closed" >&2
  exit 1
fi

assert_file "$TMP/artifacts-transport-fail/summary.json"
python3 - <<'PY' "$TMP/artifacts-transport-fail/summary.json"
import json
import sys

summary = json.load(open(sys.argv[1], encoding="utf-8"))
assert summary["status"] == "failed"
assert summary["transport_failure"]["summary_fetch_failed"] is True
assert summary["transport_failure"]["executor"] == "ssh"
assert summary["command"] == "printf no-transport"
assert summary["cleanup"]["attempted"] is False
assert summary["cleanup"]["complete"] is False
assert summary["failure_class"] == "provider_availability"
PY
grep -F "fallback summary written locally" "$TMP/transport-fail.err" >/dev/null

portable_runner="${ADL_REMOTE_VALIDATION_BIN:?ADL_REMOTE_VALIDATION_BIN is required for portable adapter proof}"
portable_request="$TMP/portable-nessus-request.json"
fixture_revision="$(git -C "$origin_src" rev-parse HEAD)"
python3 - "$portable_request" "$fixture_revision" <<'PY'
import hashlib
import json
import sys

path, revision = sys.argv[1:]
profile = {
    "argv": ["printf", "portable-ok"],
    "working_directory": ".",
    "environment_allowlist": ["PATH"],
}
digest = hashlib.sha256(json.dumps(profile, separators=(",", ":")).encode()).hexdigest()
payload = {
    "schema": "adl.remote_validation.request.v1",
    "request_id": "wp-5823-nessus-shell-adapter",
    "checkout": ".",
    "revision": revision,
    "source_ref": "refs/heads/main",
    "command_profile": profile,
    "command_profile_digest": digest,
    "adapter": "nessus",
    "requested_platform": "windows",
    "resource_budget": {"cpu_cores": 4, "memory_mib": 4096, "timeout_seconds": 60, "estimated_max_cost_microusd": None},
    "artifact_policy": {"paths": ["summary.json"], "required": True, "max_total_bytes": 1048576},
    "cancellation_file": None,
    "fallback": "offer_local",
}
with open(path, "w", encoding="utf-8") as handle:
    json.dump(payload, handle, separators=(",", ":"))
PY

PATH="$fake_bin:$PATH" \
  bash "$SCRIPT" \
    --portable-request "$portable_request" \
    --portable-runner "$portable_runner" \
    --executor local \
    --remote-root "$TMP/portable-root" \
    --repo-url "$origin_bare" \
    --run-id portable-nessus \
    --local-artifact-dir "$TMP/artifacts-portable" >"$TMP/portable-nessus.out"
python3 - "$TMP/artifacts-portable/summary.json" "$TMP/artifacts-portable/portable-result.json" "$fixture_revision" <<'PY'
import json
import sys

summary = json.load(open(sys.argv[1], encoding="utf-8"))
result = json.load(open(sys.argv[2], encoding="utf-8"))
assert summary["status"] == "passed"
assert summary["resolved_commit"] == sys.argv[3]
assert summary["command"] == "cd -- '.' && env -i PATH=\"${PATH-}\" 'printf' 'portable-ok'"
assert result["revision"] == sys.argv[3]
assert result["platform"] == {"os": "windows", "architecture": "x86_64", "native": False, "qualification": "fixture"}
assert result["resource_budget"]["cpu_cores"] == 4
assert result["artifact_policy"]["paths"] == ["summary.json"]
assert result["fallback"]["policy"] == "offer_local"
assert result["artifact_digests"][0]["path"] == "summary.json"
PY

if SSH_BIN="$fake_bin/ssh-fail" bash "$SCRIPT" \
  --portable-request "$portable_request" --portable-runner "$portable_runner" \
  --executor ssh --host fixture.invalid --ssh-user fixture \
  --run-id portable-transport-fail \
  --local-artifact-dir "$TMP/artifacts-portable-transport" \
  >"$TMP/portable-transport.out" 2>"$TMP/portable-transport.err"; then
  echo "expected portable transport failure with unknown cleanup to fail closed" >&2
  exit 1
fi
python3 - "$TMP/artifacts-portable-transport/portable-execution.json" <<'PY'
import json
import sys
receipt = json.load(open(sys.argv[1], encoding="utf-8"))
assert receipt["outcome"] == "provider_unavailable"
assert receipt["cleanup"] == {
    "attempted": False,
    "complete": False,
    "detail": "transport failed before remote process cleanup could be observed",
}
assert receipt["fallback"]["offered"] is False
PY

validation_request="$TMP/portable-nessus-validation-request.json"
python3 - "$portable_request" "$validation_request" <<'PY'
import hashlib
import json
import sys
payload = json.load(open(sys.argv[1], encoding="utf-8"))
payload["command_profile"]["argv"] = ["false"]
payload["command_profile_digest"] = hashlib.sha256(
    json.dumps(payload["command_profile"], separators=(",", ":")).encode()
).hexdigest()
json.dump(payload, open(sys.argv[2], "w", encoding="utf-8"), separators=(",", ":"))
PY
if PATH="$fake_bin:$PATH" bash "$SCRIPT" \
  --portable-request "$validation_request" --portable-runner "$portable_runner" \
  --executor local --remote-root "$TMP/portable-validation-root" \
  --repo-url "$origin_bare" --run-id portable-validation-fail \
  --local-artifact-dir "$TMP/artifacts-portable-validation" \
  >"$TMP/portable-validation.out" 2>"$TMP/portable-validation.err"; then
  echo "expected portable validation failure" >&2
  exit 1
fi
python3 - "$TMP/artifacts-portable-validation/portable-execution.json" <<'PY'
import json
import sys
receipt = json.load(open(sys.argv[1], encoding="utf-8"))
assert receipt["outcome"] == "failed"
assert receipt["cleanup"]["complete"] is True
assert receipt["fallback"]["offered"] is False
PY

timeout_request="$TMP/portable-nessus-timeout-request.json"
python3 - "$portable_request" "$timeout_request" <<'PY'
import hashlib
import json
import sys
payload = json.load(open(sys.argv[1], encoding="utf-8"))
payload["command_profile"]["argv"] = ["sleep", "5"]
payload["command_profile_digest"] = hashlib.sha256(
    json.dumps(payload["command_profile"], separators=(",", ":")).encode()
).hexdigest()
payload["resource_budget"]["timeout_seconds"] = 1
json.dump(payload, open(sys.argv[2], "w", encoding="utf-8"), separators=(",", ":"))
PY
if PATH="$fake_bin:$PATH" bash "$SCRIPT" \
  --portable-request "$timeout_request" --portable-runner "$portable_runner" \
  --executor local --remote-root "$TMP/portable-timeout-root" \
  --repo-url "$origin_bare" --run-id portable-timeout \
  --local-artifact-dir "$TMP/artifacts-portable-timeout" \
  >"$TMP/portable-timeout.out" 2>"$TMP/portable-timeout.err"; then
  echo "expected portable Nessus total-run timeout" >&2
  exit 1
fi
python3 - "$TMP/artifacts-portable-timeout/portable-execution.json" <<'PY'
import json
import sys
receipt = json.load(open(sys.argv[1], encoding="utf-8"))
assert receipt["outcome"] == "timed_out"
assert receipt["cleanup"]["complete"] is True
assert receipt["fallback"]["offered"] is False
PY

cancel_after_start_request="$TMP/portable-nessus-cancel-after-start.json"
python3 - "$portable_request" "$cancel_after_start_request" <<'PY'
import hashlib
import json
import sys
payload = json.load(open(sys.argv[1], encoding="utf-8"))
payload["command_profile"]["argv"] = ["sleep", "5"]
payload["command_profile_digest"] = hashlib.sha256(
    json.dumps(payload["command_profile"], separators=(",", ":")).encode()
).hexdigest()
payload["cancellation_file"] = "wp5823-nessus-cancel-after-start.signal"
json.dump(payload, open(sys.argv[2], "w", encoding="utf-8"), separators=(",", ":"))
PY
rm -f "$ROOT/wp5823-nessus-cancel-after-start.signal"
set +e
PATH="$fake_bin:$PATH" bash "$SCRIPT" \
  --portable-request "$cancel_after_start_request" --portable-runner "$portable_runner" \
  --executor local --remote-root "$TMP/portable-cancel-root" \
  --repo-url "$origin_bare" --run-id portable-cancel-after-start \
  --local-artifact-dir "$TMP/artifacts-portable-cancel" \
  >"$TMP/portable-cancel-after.out" 2>"$TMP/portable-cancel-after.err" &
cancel_pid=$!
for _ in {1..50}; do
  [[ -d "$TMP/portable-cancel-root/transient/portable-cancel-after-start" ]] && break
  sleep 0.1
done
if [[ ! -d "$TMP/portable-cancel-root/transient/portable-cancel-after-start" ]]; then
  echo "Nessus cancellation fixture did not reach remote execution" >&2
  kill "$cancel_pid" 2>/dev/null || true
  wait "$cancel_pid" 2>/dev/null || true
  exit 1
fi
touch "$ROOT/wp5823-nessus-cancel-after-start.signal"
wait "$cancel_pid"
cancel_rc=$?
set -e
rm -f "$ROOT/wp5823-nessus-cancel-after-start.signal"
if [[ "$cancel_rc" -eq 0 ]]; then
  echo "expected cancellation arriving after Nessus start to fail closed" >&2
  exit 1
fi
python3 - "$TMP/artifacts-portable-cancel/portable-execution.json" <<'PY'
import json
import sys
receipt = json.load(open(sys.argv[1], encoding="utf-8"))
assert receipt["outcome"] == "cancelled"
assert receipt["cleanup"]["attempted"] is True
assert receipt["cleanup"]["complete"] is True
assert receipt["fallback"]["offered"] is False
PY

rm -f "$ROOT/wp5823-nessus-cancel-after-start.signal" "$TMP/docker-container.pid" "$TMP/docker-rm.log"
set +e
PATH="$fake_bin:$PATH" \
DOCKER_CONTAINER_PID_FILE="$TMP/docker-container.pid" \
DOCKER_RM_LOG="$TMP/docker-rm.log" \
bash "$SCRIPT" \
  --portable-request "$cancel_after_start_request" --portable-runner "$portable_runner" \
  --executor local --remote-root "$TMP/portable-builder-cancel-root" \
  --repo-url "$origin_bare" --run-id portable-builder-cancel-after-start \
  --builder-image "example.invalid/adl-builder:test" --builder-pull-policy never \
  --local-artifact-dir "$TMP/artifacts-portable-builder-cancel" \
  >"$TMP/portable-builder-cancel.out" 2>"$TMP/portable-builder-cancel.err" &
builder_cancel_pid=$!
for _ in {1..50}; do
  [[ -s "$TMP/docker-container.pid" ]] && break
  sleep 0.1
done
if [[ ! -s "$TMP/docker-container.pid" ]]; then
  echo "containerized cancellation fixture did not start the builder" >&2
  kill "$builder_cancel_pid" 2>/dev/null || true
  wait "$builder_cancel_pid" 2>/dev/null || true
  exit 1
fi
touch "$ROOT/wp5823-nessus-cancel-after-start.signal"
wait "$builder_cancel_pid"
builder_cancel_rc=$?
set -e
rm -f "$ROOT/wp5823-nessus-cancel-after-start.signal"
if [[ "$builder_cancel_rc" -eq 0 ]]; then
  echo "expected containerized cancellation to fail closed" >&2
  exit 1
fi
python3 - "$TMP/artifacts-portable-builder-cancel/portable-execution.json" <<'PY'
import json
import sys
receipt = json.load(open(sys.argv[1], encoding="utf-8"))
assert receipt["outcome"] == "cancelled"
assert receipt["cleanup"]["attempted"] is True
assert receipt["cleanup"]["complete"] is True
assert receipt["fallback"]["offered"] is False
PY
grep -F "rm adl-nessus-portable-builder-cancel-after-start" "$TMP/docker-rm.log" >/dev/null

cancel_request="$TMP/portable-nessus-cancel-request.json"
python3 - "$portable_request" "$cancel_request" <<'PY'
import json
import sys
payload = json.load(open(sys.argv[1], encoding="utf-8"))
payload["cancellation_file"] = "wp5823-nessus-cancel.signal"
json.dump(payload, open(sys.argv[2], "w", encoding="utf-8"), separators=(",", ":"))
PY
touch "$ROOT/wp5823-nessus-cancel.signal"
if PATH="$fake_bin:$PATH" bash "$SCRIPT" --portable-request "$cancel_request" --portable-runner "$portable_runner" --executor local >"$TMP/portable-cancel.out" 2>"$TMP/portable-cancel.err"; then
  echo "expected portable cancellation file to stop Nessus execution" >&2
  exit 1
fi
rm "$ROOT/wp5823-nessus-cancel.signal"
grep -F "cancellation requested before remote execution" "$TMP/portable-cancel.err" >/dev/null

if bash "$SCRIPT" --portable-request "$portable_request" --portable-runner "$portable_runner" --command "true" >"$TMP/portable-conflict.out" 2>"$TMP/portable-conflict.err"; then
  echo "expected portable/manual Nessus ambiguity to fail closed" >&2
  exit 1
fi
grep -F "mutually exclusive" "$TMP/portable-conflict.err" >/dev/null

echo "PASS test_run_nessus_remote_validation"
