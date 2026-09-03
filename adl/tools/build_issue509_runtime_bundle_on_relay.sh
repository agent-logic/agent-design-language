#!/usr/bin/env bash
set -Eeuo pipefail

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
install -m 0644 "$repo_root/adl/tools/issue268_six_resident_uts_plan.json" "$bundle_root/config/issue268_six_resident_uts_plan.json"
install -m 0644 "$repo_root/adl/tools/issue268_runtime_uts_task_panel.json" "$bundle_root/config/issue268_runtime_uts_task_panel.json"
install -m 0644 "$repo_root/adl/tools/run_issue268_six_resident_uts_cycle.py" "$bundle_root/config/run_issue268_six_resident_uts_cycle.py"

tar -C "$bundle_root" -czf "$bundle_archive" .
sha256sum "$bundle_archive" "$bundle_root/bin/adl" "$bundle_root/bin/csm"
gcloud storage cp "$bundle_archive" "gs://${bucket}/${runtime_object}"
date -u '+finished_at=%Y-%m-%dT%H:%M:%SZ'
