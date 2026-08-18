#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SCRIPT="$ROOT/adl/tools/run_aws_spot_builder_image_validation.sh"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

FAKE_BIN="$TMP/fake-bin"
RUN_ROOT="$TMP/run"
CACHE_MOUNT="$TMP/cache"
mkdir -p "$FAKE_BIN" "$RUN_ROOT" "$CACHE_MOUNT"
mkdir -p "$TMP/raw"
export ADL_REAL_PYTHON3="$(command -v python3)"

cat >"$FAKE_BIN/mountpoint" <<'EOF'
#!/usr/bin/env bash
[[ "${ADL_FAKE_MOUNT_OK:-1}" == "1" ]]
EOF

cat >"$FAKE_BIN/findmnt" <<'EOF'
#!/usr/bin/env bash
case "${*: -1}" in
  /) echo /dev/root ;;
  *) echo /dev/fake-retained-cache ;;
esac
EOF

cat >"$FAKE_BIN/df" <<'EOF'
#!/usr/bin/env bash
printf 'Filesystem 1-blocks Used Available Capacity Mounted on\n'
printf '/dev/fake 100000000000 1 %s 1%% /cache\n' "${ADL_FAKE_CACHE_FREE_BYTES:-90000000000}"
EOF

cat >"$FAKE_BIN/systemctl" <<'EOF'
#!/usr/bin/env bash
exit 0
EOF

cat >"$FAKE_BIN/sudo" <<'EOF'
#!/usr/bin/env bash
exec "$@"
EOF

cat >"$FAKE_BIN/aws" <<'EOF'
#!/usr/bin/env bash
if [[ "$1 $2" == "ecr get-login-password" ]]; then
  echo fake-password
  exit 0
fi
echo "unexpected aws command: $*" >&2
exit 1
EOF

cat >"$FAKE_BIN/docker" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
case "$1" in
  login) cat >/dev/null; exit 0 ;;
  pull) exit 0 ;;
  image)
    if [[ "$2" != "inspect" ]]; then exit 2; fi
    case "$4" in
      *Architecture*) echo "${ADL_FAKE_IMAGE_ARCH:-amd64}" ;;
      *) echo sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb ;;
    esac
    exit 0
    ;;
  run)
    args="$*"
    if [[ "$args" == *"ADL_BUILDER_CHECK="* ]]; then
      echo toolchain >>"$ADL_FAKE_DOCKER_CALLS"
      check="${args#*ADL_BUILDER_CHECK=}"
      check="${check%% *}"
      if [[ "${ADL_FAKE_MISSING_CHECK:-}" == "$check" ]]; then
        [[ -z "${ADL_FAKE_SENSITIVE_OUTPUT:-}" ]] || printf '%s\n' "$ADL_FAKE_SENSITIVE_OUTPUT"
        echo "/bin/bash: ${ADL_FAKE_MISSING_EXECUTABLE:-cargo-nextest}: command not found" >&2
        exit 127
      fi
      case "$check" in
        architecture) ;;
        rustc) echo 'rustc 1.96.0' ;;
        cargo) echo 'cargo 1.96.0' ;;
        nextest) echo 'cargo-nextest 0.9.140' ;;
        sccache) echo 'sccache 0.16.0' ;;
        linker) echo 'Ubuntu LLD 18.1.3' ;;
        aws_cli) echo 'aws-cli/2.35.15' ;;
        ruby) echo 'ruby 3.3.6' ;;
        ruby_smoke) [[ "${ADL_FAKE_RUBY_OK:-1}" == "1" ]] && echo 'ruby-smoke-ok' ;;
        receipt_validator) echo 'PASS: finalization allowlist rejects Runtime product drift' ;;
        *) echo "unknown fake builder check: $check" >&2; exit 2 ;;
      esac
      exit 0
    fi
    echo validation >>"$ADL_FAKE_DOCKER_CALLS"
    [[ "$args" == *"--env RUSTFLAGS= --env CARGO_INCREMENTAL=0"* ]] || {
      echo "validation container did not preserve the known-good Rust flags" >&2
      exit 2
    }
    [[ "$args" == *"--user "* && "$args" == *"AWS_EC2_METADATA_DISABLED=true"* ]] || {
      echo "validation container did not isolate root permissions and EC2 role discovery" >&2
      exit 2
    }
    [[ "$args" == *":/cache-root"* && "$args" == *"CARGO_TARGET_DIR=/cache-root/target"* && "$args" == *"SCCACHE_DIR=/cache-root/sccache"* && "$args" == *"CARGO_HOME=/cache-root/cargo-home"* ]] || {
      echo "validation container did not preserve the known-good cache layout" >&2
      exit 2
    }
    [[ "$args" == *"/adl-aws-remote-validation/shared/tmp:/tmp"* && "$args" == *"TMPDIR=/tmp"* ]] || {
      echo "validation container did not mount EBS-backed temp space" >&2
      exit 2
    }
    run_root=""
    previous=""
    for arg in "$@"; do
      if [[ "$previous" == "--volume" && "$arg" == *:/run-output ]]; then
        run_root="${arg%:/run-output}"
      fi
      previous="$arg"
    done
    [[ -n "$run_root" ]]
    echo 'Compile requests                     10' >"$run_root/sccache-stats.log"
    exit "${ADL_FAKE_VALIDATION_EXIT:-0}"
    ;;
esac
echo "unexpected docker command: $*" >&2
exit 1
EOF

cat >"$FAKE_BIN/python3" <<'EOF'
#!/usr/bin/env bash
if [[ "${ADL_FAKE_SUMMARY_EXIT:-0}" != "0" && "${2:-}" == */spot-builder-summary.json.tmp.* ]]; then
  printf '{' >"$2"
  exit "$ADL_FAKE_SUMMARY_EXIT"
fi
exec "$ADL_REAL_PYTHON3" "$@"
EOF
chmod +x "$FAKE_BIN"/*

commit="$(git -C "$ROOT" rev-parse HEAD)"
digest="sha256:$(printf 'a%.0s' {1..64})"
image="123456789012.dkr.ecr.us-west-2.amazonaws.com/adl-builder@$digest"

run_fixture() {
  local command="${1:-cargo nextest run --workspace}"
  PATH="$FAKE_BIN:$PATH" \
  TMPDIR="$TMP/raw" \
  ADL_REMOTE_REPO_DIR="$ROOT" \
  ADL_RUN_ROOT="$RUN_ROOT" \
  ADL_CACHE_VOLUME_MOUNT_PATH="$CACHE_MOUNT" \
  ADL_REGION=us-west-2 \
  ADL_FAKE_DOCKER_CALLS="$TMP/docker-calls.log" \
  bash "$SCRIPT" \
    --image "$image" \
    --expected-ref "$commit" \
    --command "$command"
}

run_fixture >"$TMP/pass.out" 2>"$TMP/pass.err"
grep -F 'ADL_SPOT_BUILDER_PROOF=' "$TMP/pass.out" >/dev/null
python3 - "$RUN_ROOT/spot-builder-summary.json" "$commit" <<'PY'
import json
import sys
payload = json.load(open(sys.argv[1], encoding="utf-8"))
assert payload["status"] == "passed"
assert payload["validation_exit_code"] == 0
assert payload["source_commit"] == sys.argv[2]
assert payload["source_commit_verified"] is True
assert payload["builder_image_immutable"] is True
assert payload["toolchain_verified"] is True
assert payload["cache_mount_verified"] is True
assert payload["host_validation_tools_installed"] is False
assert payload["cache_mount_source_sha256"]
PY

if PATH="$FAKE_BIN:$PATH" ADL_REMOTE_REPO_DIR="$ROOT" ADL_RUN_ROOT="$RUN_ROOT" \
  ADL_CACHE_VOLUME_MOUNT_PATH="$CACHE_MOUNT" bash "$SCRIPT" \
  --image 'example.invalid/adl-builder:mutable' --expected-ref "$commit" --command true \
  >"$TMP/mutable.out" 2>"$TMP/mutable.err"; then
  echo "expected mutable image to fail" >&2
  exit 1
fi
grep -F 'immutable sha256 digest' "$TMP/mutable.err" >/dev/null

wrong_ref="$(printf 'f%.0s' {1..40})"
if PATH="$FAKE_BIN:$PATH" ADL_REMOTE_REPO_DIR="$ROOT" ADL_RUN_ROOT="$RUN_ROOT" \
  ADL_CACHE_VOLUME_MOUNT_PATH="$CACHE_MOUNT" bash "$SCRIPT" \
  --image "$image" --expected-ref "$wrong_ref" --command true \
  >"$TMP/ref.out" 2>"$TMP/ref.err"; then
  echo "expected wrong source ref to fail" >&2
  exit 1
fi
grep -F 'resolved source commit does not match' "$TMP/ref.err" >/dev/null

if ADL_FAKE_MOUNT_OK=0 run_fixture >"$TMP/mount.out" 2>"$TMP/mount.err"; then
  echo "expected missing cache mount to fail" >&2
  exit 1
fi
grep -F 'not a mountpoint' "$TMP/mount.err" >/dev/null

if ADL_FAKE_CACHE_FREE_BYTES=1024 run_fixture >"$TMP/space.out" 2>"$TMP/space.err"; then
  echo "expected insufficient cache headroom to fail" >&2
  exit 1
fi
grep -F 'insufficient free space' "$TMP/space.err" >/dev/null

ADL_FAKE_CACHE_FREE_BYTES=1024 run_fixture \
  'cargo clean --manifest-path adl/Cargo.toml' \
  >"$TMP/clean.out" 2>"$TMP/clean.err"
grep -F 'low-space target cleanup recovery authorized' "$TMP/clean.err" >/dev/null

rm -f "$RUN_ROOT/builder-toolchain.log"
: >"$TMP/docker-calls.log"
if ADL_FAKE_MISSING_CHECK=nextest ADL_FAKE_MISSING_EXECUTABLE=cargo-nextest \
  ADL_FAKE_SENSITIVE_OUTPUT='account=123456789012 key=AKIAABCDEFGHIJKLMNOP ip=10.2.3.4 path=/Users/example/private.log' \
  run_fixture >"$TMP/tool.out" 2>"$TMP/tool.err"; then
  echo "expected missing builder tool to fail" >&2
  exit 1
fi
grep -F 'builder preflight failed check=nextest executable=cargo-nextest exit_status=127' "$TMP/tool.err" >/dev/null
grep -F 'ADL_BUILDER_CHECK_END label=nextest executable=cargo-nextest exit_status=127' \
  "$RUN_ROOT/builder-toolchain.log" >/dev/null
grep -F 'cargo-nextest: command not found' "$RUN_ROOT/builder-toolchain.log" >/dev/null
grep -F '<aws-account-id-redacted>' "$RUN_ROOT/builder-toolchain.log" >/dev/null
grep -F '<aws-access-key-redacted>' "$RUN_ROOT/builder-toolchain.log" >/dev/null
grep -F '<ip-address-redacted>' "$RUN_ROOT/builder-toolchain.log" >/dev/null
grep -F '<machine-path-redacted>' "$RUN_ROOT/builder-toolchain.log" >/dev/null
if grep -E '123456789012|AKIAABCDEFGHIJKLMNOP|10[.]2[.]3[.]4|/Users/example' \
  "$RUN_ROOT/builder-toolchain.log" >/dev/null; then
  echo "missing-tool diagnostic retained unredacted identifiers" >&2
  exit 1
fi
if compgen -G "$TMP/raw/adl-builder-toolchain-raw.*" >/dev/null; then
  echo "missing-tool failure retained a raw toolchain capture" >&2
  exit 1
fi
if grep -F validation "$TMP/docker-calls.log" >/dev/null; then
  echo "missing tool reached the requested validation command" >&2
  exit 1
fi

: >"$TMP/docker-calls.log"
if ADL_FAKE_RUBY_OK=0 run_fixture >"$TMP/ruby.out" 2>"$TMP/ruby.err"; then
  echo "expected missing Ruby to fail builder preflight" >&2
  exit 1
fi
grep -F 'builder toolchain verification missing ruby-smoke-ok' "$TMP/ruby.err" >/dev/null
if grep -F validation "$TMP/docker-calls.log" >/dev/null; then
  echo "missing Ruby reached the requested validation command" >&2
  exit 1
fi

if ADL_FAKE_IMAGE_ARCH=arm64 run_fixture >"$TMP/arch.out" 2>"$TMP/arch.err"; then
  echo "expected wrong image architecture to fail" >&2
  exit 1
fi
grep -F 'builder image architecture mismatch' "$TMP/arch.err" >/dev/null

printf '{"status":"passed"}\n' >"$RUN_ROOT/spot-builder-summary.json"
set +e
ADL_FAKE_SUMMARY_EXIT=23 run_fixture >"$TMP/summary-failure.out" 2>"$TMP/summary-failure.err"
summary_status=$?
set -e
[[ "$summary_status" -eq 23 ]]
grep -F 'retained summary generation failed with status 23' "$TMP/summary-failure.err" >/dev/null
[[ ! -e "$RUN_ROOT/spot-builder-summary.json" ]]
if compgen -G "$RUN_ROOT/spot-builder-summary.json.tmp.*" >/dev/null; then
  echo "summary failure retained a partial temporary file" >&2
  exit 1
fi
if grep -F 'ADL_SPOT_BUILDER_PROOF=' "$TMP/summary-failure.out" >/dev/null; then
  echo "summary failure emitted a proof marker" >&2
  exit 1
fi

# Exercise the real remote runner's captured nonzero-command path. The command
# starts with the governed builder entrypoint, creates a retained diagnostic
# after its expected argument failure, and returns 127. The runner must emit the
# retained log on its normal path even though ERR is suppressed by set +e.
runner="$ROOT/tools/aws_remote_validation/scripts/remote_validation_runner.sh"
runner_root="$TMP/runner"
runner_command="bash adl/tools/run_aws_spot_builder_image_validation.sh --invalid || { printf 'runner-retained-builder-diagnostic\\n' > '$runner_root/builder-toolchain.log'; exit 127; }"
set +e
HOME="$TMP/runner-home" \
ADL_RUN_ID=issue415-runner-fixture \
ADL_RUN_ROOT="$runner_root" \
ADL_REMOTE_REPO_DIR="$ROOT" \
ADL_REMOTE_COMMAND="$runner_command" \
  bash "$runner" >"$TMP/runner.out" 2>"$TMP/runner.err"
runner_status=$?
set -e
[[ "$runner_status" -eq 127 ]]
grep -F 'ADL_REMOTE_LOG_BEGIN:builder_toolchain' "$TMP/runner.err" >/dev/null
grep -F 'runner-retained-builder-diagnostic' "$TMP/runner.err" >/dev/null
grep -F 'ADL_AWS_REMOTE_SUMMARY_BEGIN' "$TMP/runner.out" >/dev/null

# Missing diagnostics are non-authoritative: the same runner path still emits
# its summary and original command result without a builder log.
missing_runner_root="$TMP/runner-missing"
set +e
HOME="$TMP/runner-missing-home" \
ADL_RUN_ID=issue415-runner-missing-fixture \
ADL_RUN_ROOT="$missing_runner_root" \
ADL_REMOTE_REPO_DIR="$ROOT" \
ADL_REMOTE_COMMAND='bash adl/tools/run_aws_spot_builder_image_validation.sh --invalid' \
  bash "$runner" >"$TMP/runner-missing.out" 2>"$TMP/runner-missing.err"
missing_runner_status=$?
set -e
[[ "$missing_runner_status" -eq 2 ]]
grep -F 'ADL_AWS_REMOTE_SUMMARY_BEGIN' "$TMP/runner-missing.out" >/dev/null
if grep -F 'ADL_REMOTE_LOG_BEGIN:builder_toolchain' "$TMP/runner-missing.err" >/dev/null; then
  echo "missing runner diagnostic emitted a false retained log" >&2
  exit 1
fi

bash -n "$SCRIPT" "$runner" "$0"

scope_paths="$TMP/scope-paths.txt"
{
  git -C "$ROOT" diff --name-only HEAD --
  git -C "$ROOT" ls-files --others --exclude-standard
} | LC_ALL=C sort -u >"$scope_paths"
while IFS= read -r path; do
  case "$path" in
    adl/tools/run_aws_spot_builder_image_validation.sh | \
    adl/tools/test_run_aws_spot_builder_image_validation.sh | \
    tools/aws_remote_validation/scripts/remote_validation_runner.sh | \
    .csdlc/issues/415/* | \
    .csdlc/prepared/issues/415/* | \
    .csdlc/locks/415.lock) ;;
    *)
      echo "issue 415 exact-scope violation: $path" >&2
      exit 1
      ;;
  esac
done <"$scope_paths"

rm -f "$RUN_ROOT/spot-builder-summary.json"
set +e
ADL_FAKE_VALIDATION_EXIT=17 run_fixture >"$TMP/validation.out" 2>"$TMP/validation.err"
validation_status=$?
set -e
[[ "$validation_status" -eq 17 ]]
grep -F 'validation command failed with status 17' "$TMP/validation.err" >/dev/null
grep -F 'ADL_SPOT_BUILDER_PROOF=' "$TMP/validation.out" >/dev/null
python3 - "$RUN_ROOT/spot-builder-summary.json" <<'PY'
import json
import sys
payload = json.load(open(sys.argv[1], encoding="utf-8"))
assert payload["status"] == "failed"
assert payload["validation_exit_code"] == 17
PY

rm -f "$RUN_ROOT/spot-builder-summary.json"
set +e
ADL_FAKE_VALIDATION_EXIT=17 ADL_FAKE_SUMMARY_EXIT=23 \
  run_fixture >"$TMP/both-fail.out" 2>"$TMP/both-fail.err"
both_status=$?
set -e
[[ "$both_status" -eq 17 ]]
grep -F 'retained summary generation failed with status 23' "$TMP/both-fail.err" >/dev/null
grep -F 'validation command failed with status 17' "$TMP/both-fail.err" >/dev/null

echo "PASS test_run_aws_spot_builder_image_validation"
