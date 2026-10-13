#!/usr/bin/env bash
set -Eeuo pipefail

# Standalone relay input: no ADL helper layout is assumed before checkout.
ISSUE268_RUNTIME_SOURCE_ROOT=$(python3 - <<'PY_RESOURCE'
import hashlib, os, pathlib
expected = {'adl/tools/issue268_runtime_qualification.cloudformation.yaml': 'e6fa89e5f1d8a18ec22de86ae036835692578c9de7c5972ebdb169dc8185db23', 'adl/tools/issue268_runtime_uts_task_panel.json': 'd0959ef79168d6e8fc05ba02cab7d98b267280511eb5884684e14150ded3617e', 'adl/tools/issue268_six_resident_uts_plan.json': '7fa623819342d86bcfd5b0d16c5a502c5a7950141a7292e5689f8d257ea75f8b', 'adl/tools/materialize_issue268_ollama_plan.py': 'b12738bffa746b9b0b0c7a35215ddced7771b9d7adaf627523db1c0e3369a4f3', 'adl/tools/run_issue268_continuity_uts_qualification.py': '580027455b08064b591ea69077b97e56f27048a0b414e4240f64cf898e98bedf', 'adl/tools/run_issue268_six_resident_uts_cycle.py': 'd104c2a4c3f5294e83587c9e30c422f2dba9d19543bd8740f702714d75f375cd', 'adl/tools/test_issue268_runtime_qualification_cloudformation.py': '93576398e544942e95313bf30103cae0cac487ea8198a5960fc7c849fb1db4d6', 'adl/tools/test_materialize_issue268_ollama_plan.py': '5b677e6899be9ef1b31cf491d152aacddcee5af926291485159d605992b3a60e', 'adl/tools/test_run_issue268_continuity_uts_qualification.py': '29386d1cdc2a8e1faa28948bd2d6dfb06daa2f37c9f016c5240c4f2794f9fab5', 'adl/tools/test_run_issue268_six_resident_uts_cycle.py': 'e5369948db2acf1e6bce02cd6a31f85d93dc0bf0851773ae5baa18cb5f09beff', 'adl/tools/test_warm_issue268_ollama_models.py': '1f404c1960abfe635d5d65ef8d398a35322576d906a4640c2ccbe7c7bf1f4da8', 'adl/tools/validate_issue268_six_resident_uts_plan.py': '6383833aee37ff75ea239294e3d5b5c29e28e92730683bc3a3c8ca32850a23da', 'adl/tools/warm_issue268_ollama_models.py': 'fef747c8e948eae40acd6aee96eaf7240a91c39b16ba03352843c7b6668d3c76'}
value = os.environ.get("ADL_ISSUE268_RUNTIME_SOURCE_ROOT", "")
if not value or not pathlib.Path(value).is_absolute(): raise SystemExit("issue268: explicit absolute ADL_ISSUE268_RUNTIME_SOURCE_ROOT required")
root = pathlib.Path(value).resolve()
for name, digest in expected.items():
 path=root/name
 if not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest()!=digest: raise SystemExit("issue268: qualified Runtime resource missing or changed: "+name)
print(root)
PY_RESOURCE
)

bucket="$1"
git_ref="$2"
runtime_object="$3"

repo_url="${ADL_RELAY_REPO_URL:-https://github.com/agent-logic/agent-design-language.git}"
work_root="${ADL_RELAY_WORK_ROOT:-/var/lib/adl/issue509-runtime-build}"
repo_root="$work_root/repo"
bundle_root="$work_root/runtime-bundle"
bundle_archive="$work_root/runtime-bundle.tar.gz"
log="${ADL_RELAY_BUILD_LOG:-/var/log/adl/issue509-runtime-build.log}"

install -d -m 0755 "$work_root" "$(dirname "$log")"
exec > >(tee -a "$log") 2>&1

date -u '+started_at=%Y-%m-%dT%H:%M:%SZ'
echo "git_ref=$git_ref"
echo "runtime_object=$runtime_object"

export DEBIAN_FRONTEND=noninteractive
for command in git curl gcloud jq sha256sum tar cargo; do
  if ! command -v "$command" >/dev/null 2>&1; then
    apt-get update
    apt-get install -y build-essential ca-certificates curl git jq tar coreutils pkg-config libssl-dev
    break
  fi
done

if ! command -v cargo >/dev/null 2>&1; then
  curl -fsSL https://sh.rustup.rs | sh -s -- -y --profile minimal
fi

if [[ -f "$HOME/.cargo/env" ]]; then
  # shellcheck disable=SC1091
  source "$HOME/.cargo/env"
fi

command -v cargo
command -v git
command -v gcloud

if [[ ! -d "$repo_root/.git" ]]; then
  rm -rf "$repo_root"
  git clone --filter=blob:none "$repo_url" "$repo_root"
fi

git -C "$repo_root" fetch --filter=blob:none origin
if [[ "$git_ref" == origin/* ]]; then
  git_ref="refs/remotes/$git_ref"
fi
git -C "$repo_root" checkout --detach "$git_ref"
git -C "$repo_root" rev-parse HEAD

rm -rf "$bundle_root"
install -d -m 0755 "$bundle_root/bin" "$bundle_root/config"

export CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR="$work_root/cargo-target"
export CARGO_BUILD_JOBS="${ADL_RELAY_CARGO_BUILD_JOBS:-1}"
cargo build --locked --release --manifest-path "$repo_root/adl/Cargo.toml" --bin adl --bin csm

install -m 0755 "$CARGO_TARGET_DIR/release/adl" "$bundle_root/bin/adl"
install -m 0755 "$CARGO_TARGET_DIR/release/csm" "$bundle_root/bin/csm"
install -m 0644 "$ISSUE268_RUNTIME_SOURCE_ROOT/adl/tools/issue268_six_resident_uts_plan.json" "$bundle_root/config/issue268_six_resident_uts_plan.json"
install -m 0644 "$ISSUE268_RUNTIME_SOURCE_ROOT/adl/tools/issue268_runtime_uts_task_panel.json" "$bundle_root/config/issue268_runtime_uts_task_panel.json"
install -m 0644 "$ISSUE268_RUNTIME_SOURCE_ROOT/adl/tools/run_issue268_six_resident_uts_cycle.py" "$bundle_root/config/run_issue268_six_resident_uts_cycle.py"

tar -C "$bundle_root" -czf "$bundle_archive" .
sha256sum "$bundle_archive" "$bundle_root/bin/adl" "$bundle_root/bin/csm"
gcloud storage cp "$bundle_archive" "gs://${bucket}/${runtime_object}"
date -u '+finished_at=%Y-%m-%dT%H:%M:%SZ'
