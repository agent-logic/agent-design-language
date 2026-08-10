#!/usr/bin/env bash
set -euo pipefail

repo_root=$(git rev-parse --show-toplevel)
cd "$repo_root"

case "$(uname -s)" in
  Darwin) platform=macos ;;
  Linux) platform=linux ;;
  MINGW*|MSYS*|CYGWIN*) platform=windows ;;
  *) echo "unsupported native platform" >&2; exit 69 ;;
esac

arch=$(uname -m | tr '[:upper:]' '[:lower:]')
revision=$(git rev-parse HEAD)
github_run_id=${GITHUB_RUN_ID:-}
run_attempt=${GITHUB_RUN_ATTEMPT:-1}
run_id=${github_run_id:+"${github_run_id}-${run_attempt}-${platform}"}
run_id=${run_id:-"local-$(date -u +%Y%m%dT%H%M%SZ)-$$"}
provider=${GITHUB_ACTIONS:+github_actions}
provider=${provider:-local_native}
python_bin=$(command -v python3 || command -v python || true)
if [[ -z "$python_bin" ]]; then
  echo "Python is required to produce a canonical receipt" >&2
  exit 69
fi
evidence_root=${ADL_DISTRIBUTED_EVIDENCE_ROOT:-"$repo_root/.csdlc/evidence/5878/native/$platform"}
evidence_root=$("$python_bin" - "$repo_root" "$evidence_root" <<'PY'
import os
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
candidate = pathlib.Path(sys.argv[2])
if not root.is_absolute() or not candidate.is_absolute():
    raise SystemExit("repository and evidence roots must be absolute")
if root.is_symlink():
    raise SystemExit("repository root must not be a symlink")
root_real = pathlib.Path(os.path.realpath(root))
candidate_absolute = pathlib.Path(os.path.abspath(candidate))
required = root_real / ".csdlc" / "evidence" / "5878"
try:
    relative = candidate_absolute.relative_to(required)
except ValueError as error:
    raise SystemExit("evidence root must remain issue-local") from error
if not relative.parts:
    raise SystemExit("evidence root must be below the issue root")

current = root_real
for part in candidate_absolute.relative_to(root_real).parts:
    current = current / part
    if current.is_symlink():
        raise SystemExit("evidence root traverses a symlink")
    if current.exists() and not current.is_dir():
        raise SystemExit("evidence root traverses a non-directory")
print(candidate_absolute)
PY
) || exit 64
mkdir -p "$evidence_root"
if [[ "$(cd "$evidence_root" && pwd -P)" != "$evidence_root" ]]; then
  echo "evidence root canonicalization changed after creation" >&2
  exit 64
fi

protected=(
  adl-runtime/src/distributed/mod.rs
  adl-runtime/src/lib.rs
  adl-runtime/tests/distributed_guardian.rs
  adl/tools/validate_v092_distributed_guardian.sh
  adl/tools/validate_v092_distributed_native_receipts.rb
  .github/workflows/wp04-native-distributed.yml
)
if [[ -n "$(git status --porcelain -- "${protected[@]}")" ]]; then
  echo "protected source must be committed before native proof" >&2
  exit 65
fi

stdout="$evidence_root/distributed-guardian.stdout.log"
stderr="$evidence_root/distributed-guardian.stderr.log"
started_at=$(date -u +%Y-%m-%dT%H:%M:%SZ)
set +e
NO_COLOR=1 cargo nextest run --manifest-path adl-runtime/Cargo.toml \
  --test distributed_guardian --no-tests=fail --no-capture >"$stdout" 2>"$stderr"
exit_code=$?
set -e
finished_at=$(date -u +%Y-%m-%dT%H:%M:%SZ)
if (( exit_code != 0 )); then
  echo "distributed Guardian integration target failed" >&2
  exit "$exit_code"
fi

max_log_bytes=$((4 * 1024 * 1024))
for log in "$stdout" "$stderr"; do
  bytes=$(wc -c <"$log" | tr -d ' ')
  if (( bytes > max_log_bytes )); then
    echo "integration output exceeds hard bound" >&2
    exit 70
  fi
done

"$python_bin" - "$repo_root" "$revision" "$platform" "$arch" "$provider" "$run_id" \
  "$started_at" "$finished_at" "$stdout" "$stderr" "$evidence_root/runner-provenance.json" \
  "$evidence_root/receipt.json" <<'PY'
import hashlib
import json
import os
import pathlib
import re
import subprocess
import sys

(root, revision, platform, arch, provider, run_id, started, finished,
 stdout_path, stderr_path, provenance_path, receipt_path) = sys.argv[1:]

def digest(path):
    return hashlib.sha256(pathlib.Path(path).read_bytes()).hexdigest()

def relative(path):
    resolved = pathlib.Path(path).resolve()
    base = pathlib.Path(root).resolve()
    try:
        value = resolved.relative_to(base)
    except ValueError as error:
        raise SystemExit("evidence path escaped repository") from error
    if resolved.is_symlink():
        raise SystemExit("evidence path is a symlink")
    return value.as_posix()

text = pathlib.Path(stdout_path).read_text(encoding="utf-8", errors="replace")
summary_text = text + pathlib.Path(stderr_path).read_text(encoding="utf-8", errors="replace")
summary = re.search(r"(?m)^\s*Summary\s+\[[^]]+\]\s+(\d+) tests? run:", summary_text)
if summary is None:
    summary = re.search(r"(?m)(\d+) tests? run", summary_text)
if summary is None or int(summary.group(1)) < 1:
    raise SystemExit("could not prove a nonzero selected test denominator")
selected = int(summary.group(1))
negative = sorted(set(re.findall(r"ADL_ISSUE_5878_NEGATIVE_CASE_V1\s+([a-z0-9_]+)", text)))
if not negative:
    raise SystemExit("machine-derived negative cases missing")

rustc_verbose = subprocess.run(
    ["rustc", "-vV"], check=True, capture_output=True, text=True
).stdout
rustc_host = next(
    (line.removeprefix("host: ") for line in rustc_verbose.splitlines() if line.startswith("host: ")),
    None,
)
if not rustc_host:
    raise SystemExit("rustc host identity missing")
provenance = {
    "schema": "adl.distributed_guardian.runner_provenance.v1",
    "provider": provider,
    "run_id": run_id,
    "os": platform,
    "arch": arch,
    "rustc_host": rustc_host,
    "source_revision": revision,
}
pathlib.Path(provenance_path).write_text(
    json.dumps(provenance, sort_keys=True, separators=(",", ":")) + "\n", encoding="utf-8"
)
runner = {
    "provider": provider,
    "run_id": run_id,
    "os": platform,
    "arch": arch,
    "repository": os.environ.get("GITHUB_REPOSITORY"),
    "workflow_ref": os.environ.get("GITHUB_WORKFLOW_REF"),
    "run_attempt": os.environ.get("GITHUB_RUN_ATTEMPT"),
    "github_run_id": os.environ.get("GITHUB_RUN_ID"),
    "commit": revision,
    "provenance_path": relative(provenance_path),
    "provenance_sha256": digest(provenance_path),
}
runner_bytes = json.dumps(runner, sort_keys=True, separators=(",", ":")).encode()
runner["identity_sha256"] = hashlib.sha256(runner_bytes).hexdigest()
receipt = {
    "schema": "adl.distributed_guardian.native_receipt.v1",
    "issue": 5878,
    "platform": platform,
    "source_revision": revision,
    "command": {
        "argv": ["bash", "adl/tools/validate_v092_distributed_guardian.sh"],
        "exit_code": 0,
        "selected_tests": selected,
        "started_at": started,
        "finished_at": finished,
        "runner": runner,
        "stdout_path": relative(stdout_path),
        "stdout_sha256": digest(stdout_path),
        "stderr_path": relative(stderr_path),
        "stderr_sha256": digest(stderr_path),
    },
    "negative_cases": negative,
    "artifacts": [
        {"role": "integration_stdout", "path": relative(stdout_path), "sha256": digest(stdout_path)},
        {"role": "integration_stderr", "path": relative(stderr_path), "sha256": digest(stderr_path)},
        {"role": "runner_provenance", "path": relative(provenance_path), "sha256": digest(provenance_path)},
    ],
}
temporary = receipt_path + ".tmp"
pathlib.Path(temporary).write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
os.replace(temporary, receipt_path)
print(json.dumps({"schema": "adl.distributed_guardian.native_run.v1", "platform": platform,
                  "selected_tests": selected, "receipt": relative(receipt_path)}, sort_keys=True))
PY
