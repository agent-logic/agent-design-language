#!/usr/bin/env bash
set -euo pipefail

: "${ADL_RUN_ID:?ADL_RUN_ID is required}"
: "${ADL_REMOTE_REPO_DIR:?ADL_REMOTE_REPO_DIR is required}"
: "${ADL_REMOTE_COMMAND:?ADL_REMOTE_COMMAND is required}"

RUN_ROOT="${ADL_RUN_ROOT:-/tmp/adl-aws-remote-validation/${ADL_RUN_ID}}"
PROGRESS_ROOT="${ADL_PROGRESS_ROOT:-$RUN_ROOT}"
WORK_ROOT="$RUN_ROOT"
TOOLCHAIN_ROOT=""
TARGET_DIR="$WORK_ROOT/target"
SCCACHE_DIR="$WORK_ROOT/sccache"
CARGO_HOME_DIR="${HOME:-/root}/.cargo"
RUSTUP_HOME_DIR="${HOME:-/root}/.rustup"
CARGO_BIN_DIR=""

if [ "${ADL_CACHE_VOLUME_ENABLED:-0}" = "1" ]; then
  CACHE_VOLUME_DEVICE_NAME="${ADL_CACHE_VOLUME_DEVICE_NAME:?ADL_CACHE_VOLUME_DEVICE_NAME is required}"
  CACHE_VOLUME_MOUNT_PATH="${ADL_CACHE_VOLUME_MOUNT_PATH:?ADL_CACHE_VOLUME_MOUNT_PATH is required}"
  CURRENT_STAGE="prepare_cache_volume"
  mkdir -p "$PROGRESS_ROOT"
  ROOT_SOURCE="$(findmnt -n -o SOURCE / || true)"
  ROOT_DISK="$(lsblk -no PKNAME "$ROOT_SOURCE" 2>/dev/null | head -n 1 || true)"
  resolve_cache_device() {
    local attempt candidate basename
    for attempt in $(seq 1 60); do
      for candidate in "$CACHE_VOLUME_DEVICE_NAME" /dev/nvme1n1 /dev/nvme2n1 /dev/xvdf /dev/xvdg; do
        [ -b "$candidate" ] || continue
        basename="$(basename "$candidate")"
        if [ -n "$ROOT_DISK" ] && [ "$basename" = "$ROOT_DISK" ]; then
          continue
        fi
        readlink -f "$candidate" 2>/dev/null || printf '%s\n' "$candidate"
        return 0
      done
      sleep 2
    done
    return 1
  }
  CACHE_DEVICE="$(resolve_cache_device)"
  sudo mkdir -p "$CACHE_VOLUME_MOUNT_PATH"
  if ! sudo blkid "$CACHE_DEVICE" >/dev/null 2>&1; then
    sudo mkfs.ext4 -F "$CACHE_DEVICE" >/tmp/adl-cache-volume-format.log 2>&1
  fi
  CACHE_UUID="$(sudo blkid -s UUID -o value "$CACHE_DEVICE")"
  if ! grep -q "$CACHE_UUID" /etc/fstab 2>/dev/null; then
    echo "UUID=$CACHE_UUID $CACHE_VOLUME_MOUNT_PATH ext4 defaults,nofail 0 2" | sudo tee -a /etc/fstab >/dev/null
  fi
  sudo mountpoint -q "$CACHE_VOLUME_MOUNT_PATH" || sudo mount "$CACHE_VOLUME_MOUNT_PATH"
  sudo resize2fs "$CACHE_DEVICE" >/tmp/adl-cache-volume-resize.log 2>&1
  CACHE_OWNER_USER="$(id -un)"
  CACHE_OWNER_GROUP="$(id -gn)"
  sudo chown "$CACHE_OWNER_USER":"$CACHE_OWNER_GROUP" "$CACHE_VOLUME_MOUNT_PATH"
  if [ "${ADL_RETAINED_VOLUME_ROLE:-build_cache}" = "runtime_continuity" ]; then
    ADL_RUNTIME_CONTINUITY_ROOT="$CACHE_VOLUME_MOUNT_PATH/runtime"
    mkdir -p "$ADL_RUNTIME_CONTINUITY_ROOT"
    printf '%s\n' "runtime_continuity" > "$CACHE_VOLUME_MOUNT_PATH/.adl-volume-role"
    export ADL_RUNTIME_CONTINUITY_ROOT
    ADL_CACHE_VOLUME_MOUNT_PATH="$WORK_ROOT/build-cache"
    mkdir -p "$ADL_CACHE_VOLUME_MOUNT_PATH"
    export ADL_CACHE_VOLUME_MOUNT_PATH
  else
    TOOLCHAIN_ROOT="$CACHE_VOLUME_MOUNT_PATH/adl-aws-remote-validation/shared"
    WORK_ROOT="$CACHE_VOLUME_MOUNT_PATH/adl-aws-remote-validation/runs/${ADL_RUN_ID}"
    TARGET_DIR="$TOOLCHAIN_ROOT/target"
    SCCACHE_DIR="$TOOLCHAIN_ROOT/sccache"
    CARGO_HOME_DIR="$TOOLCHAIN_ROOT/cargo-home"
    RUSTUP_HOME_DIR="$TOOLCHAIN_ROOT/rustup-home"

    EPHEMERAL_CHECKOUT="$ADL_REMOTE_REPO_DIR"
    SOURCE_COMMIT="$(git -C "$EPHEMERAL_CHECKOUT" rev-parse HEAD)"
    PERSISTENT_CHECKOUT="$TOOLCHAIN_ROOT/source/agent-design-language"
    mkdir -p "$(dirname "$PERSISTENT_CHECKOUT")"
    if [ ! -d "$PERSISTENT_CHECKOUT/.git" ]; then
      git clone "$EPHEMERAL_CHECKOUT" "$PERSISTENT_CHECKOUT" >/tmp/adl-persistent-clone.log 2>&1
    fi
    CURRENT_PERSISTENT_COMMIT="$(git -C "$PERSISTENT_CHECKOUT" rev-parse HEAD 2>/dev/null || true)"
    if [ "$CURRENT_PERSISTENT_COMMIT" != "$SOURCE_COMMIT" ]; then
      git -C "$PERSISTENT_CHECKOUT" fetch "$EPHEMERAL_CHECKOUT" "$SOURCE_COMMIT" \
        >/tmp/adl-persistent-fetch.log 2>&1
      git -C "$PERSISTENT_CHECKOUT" checkout --detach --force "$SOURCE_COMMIT" \
        >/tmp/adl-persistent-checkout.log 2>&1
    fi
    git -C "$PERSISTENT_CHECKOUT" clean -ffd >/tmp/adl-persistent-clean.log 2>&1
    ADL_REMOTE_REPO_DIR="$PERSISTENT_CHECKOUT"
    export ADL_REMOTE_REPO_DIR
  fi
fi

if [ "${ADL_RETAINED_VOLUME_ROLE:-build_cache}" = "runtime_continuity" ] \
    && [ "${ADL_ISSUE268_RUNTIME_QUALIFICATION:-0}" = "1" ]; then
  # Generate the ephemeral #414 signing key on the remote host before either
  # the qualification command or interruption watcher starts. The key is
  # process-scoped: it is never placed in user data or retained evidence/logs.
  if [ -z "${ADL_ISSUE414_SIGNING_KEY_HEX:-}" ]; then
    ADL_ISSUE414_SIGNING_KEY_HEX="$(od -An -N32 -tx1 /dev/urandom | tr -d ' \n')"
  fi
  case "$ADL_ISSUE414_SIGNING_KEY_HEX" in
    (*[!0-9a-f]*|'') echo "issue268: failed to generate the #414 signing key" >&2; exit 70 ;;
  esac
  [ "${#ADL_ISSUE414_SIGNING_KEY_HEX}" -eq 64 ] || {
    echo "issue268: invalid #414 signing key length" >&2
    exit 70
  }
  export ADL_ISSUE414_SIGNING_KEY_HEX
  # Generate a process-scoped P-256 custody keypair for #414 capsule signing.
  # Private material is removed from disk before qualification starts.
  custody_key_file="$(mktemp /tmp/adl-issue268-custody-key.XXXXXX)"
  chmod 600 "$custody_key_file"
  openssl genpkey -algorithm EC -pkeyopt ec_paramgen_curve:P-256 \
    -out "$custody_key_file" >/dev/null 2>&1
  custody_key_text="$(openssl pkey -in "$custody_key_file" -text -noout)"
  custody_private_hex="$(printf '%s\n' "$custody_key_text" | awk '/^priv:/{capture=1;next}/^pub:/{capture=0}capture{gsub(/[^0-9a-fA-F]/,"");printf "%s",$0}')"
  custody_public_hex="$(printf '%s\n' "$custody_key_text" | awk '/^pub:/{capture=1;next}/^ASN1 OID:/{capture=0}capture{gsub(/[^0-9a-fA-F]/,"");printf "%s",$0}')"
  ADL_CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64="$(printf '%s' "$custody_private_hex" | python3 -c 'import base64,sys; print(base64.b64encode(bytes.fromhex(sys.stdin.read())).decode())')"
  ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64="$(printf '%s' "$custody_public_hex" | python3 -c 'import base64,sys; print(base64.b64encode(bytes.fromhex(sys.stdin.read())).decode())')"
  rm -f "$custody_key_file"
  unset custody_key_file custody_key_text custody_private_hex custody_public_hex
  [ "${#ADL_CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64}" -eq 44 ] || {
    echo "issue268: invalid ephemeral custody private key" >&2
    exit 70
  }
  [ "${#ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64}" -eq 88 ] || {
    echo "issue268: invalid ephemeral custody public key" >&2
    exit 70
  }
  custody_public_fingerprint="$(printf '%s' "$ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64" | sha256sum | awk '{print substr($1,1,12)}')"
  ADL_CSM_CUSTODY_SIGNING_KEY_ID="issue268-ephemeral-${ADL_RUN_ID}-${custody_public_fingerprint}"
  unset custody_public_fingerprint
  ADL_ISSUE268_CUSTODY_ENV_FILE="$(mktemp /tmp/adl-issue268-custody-env.XXXXXX)"
  chmod 600 "$ADL_ISSUE268_CUSTODY_ENV_FILE"
  printf 'ADL_CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64=%s\nADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64=%s\nADL_CSM_CUSTODY_SIGNING_KEY_ID=%s\n' \
    "$ADL_CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64" \
    "$ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64" \
    "$ADL_CSM_CUSTODY_SIGNING_KEY_ID" >"$ADL_ISSUE268_CUSTODY_ENV_FILE"
  export ADL_CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64
  export ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64
  export ADL_CSM_CUSTODY_SIGNING_KEY_ID
  export ADL_ISSUE268_CUSTODY_ENV_FILE
  export ADL_ISSUE268_REMOTE_EVIDENCE_ROOT="$RUN_ROOT/issue268"
  export ADL_SPOT_DEHYDRATE_CALLBACK="$ADL_REMOTE_REPO_DIR/adl/tools/issue414_spot_dehydrate_callback.sh"
  export ADL_ISSUE414_CONTINUITY_BIN="$ADL_RUNTIME_CONTINUITY_ROOT/install/current/bin/adl_resident_shepherd_continuity"
  export ADL_SPOT_RESIDENT_INPUT="$ADL_ISSUE268_REMOTE_EVIDENCE_ROOT/continuity-uts/dehydration-input.json"
  export ADL_SPOT_DEHYDRATE_READY="$ADL_ISSUE268_REMOTE_EVIDENCE_ROOT/continuity-ready"
  export ADL_SPOT_RETAINED_RUNTIME_ROOT="$ADL_RUNTIME_CONTINUITY_ROOT/state/$ADL_RUN_ID"
  export ADL_SPOT_RUNTIME_VOLUME_ID_SHA256="${ADL_RUNTIME_CONTINUITY_VOLUME_ID_SHA256:?runtime volume identity is required}"
fi

mkdir -p "$RUN_ROOT" "$PROGRESS_ROOT" "$WORK_ROOT" "$TARGET_DIR" "$SCCACHE_DIR" "$CARGO_HOME_DIR" "$RUSTUP_HOME_DIR"
CARGO_BIN_DIR="$CARGO_HOME_DIR/bin"
mkdir -p "$CARGO_BIN_DIR"

BOOTSTRAP_START="$(date +%s)"
CURRENT_STAGE="bootstrap"

emit_debug_log() {
  local label="$1"
  local path="$2"
  if [ -f "$path" ]; then
    local line_count
    line_count="$(wc -l < "$path" 2>/dev/null || echo 0)"
    echo "ADL_REMOTE_LOG_BEGIN:$label" >&2
    sed -n '1,160p' "$path" >&2 || true
    if [ "$line_count" -gt 160 ]; then
      echo "ADL_REMOTE_LOG_MIDDLE_ELIDED:$label:$line_count" >&2
      tail -n 160 "$path" >&2 || true
    fi
    echo "ADL_REMOTE_LOG_END:$label" >&2
  fi
}

log_progress() {
  local message="$1"
  local timestamp
  timestamp="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  mkdir -p "$PROGRESS_ROOT"
  printf '%s %s\n' "$timestamp" "$message" | tee -a "$PROGRESS_ROOT/progress.log" >&2
}

on_error() {
  local exit_code="$?"
  echo "ADL_REMOTE_FAILURE_STAGE=$CURRENT_STAGE" >&2
  emit_debug_log rustup /tmp/adl-rustup.log
  emit_debug_log build_toolchain /tmp/adl-build-toolchain.log
  emit_debug_log sccache_install /tmp/adl-sccache-install.log
  emit_debug_log nextest_install /tmp/adl-nextest-install.log
  emit_debug_log command_stdout "$RUN_ROOT/command.log"
  emit_debug_log command_stderr "$RUN_ROOT/command.err"
  emit_debug_log builder_toolchain "$RUN_ROOT/builder-toolchain.log"
  emit_debug_log sccache_stats "$RUN_ROOT/sccache-stats.log"
  exit "$exit_code"
}
trap on_error ERR

TOOL_INSTALL_POLICY="package_manager_or_prebuilt_only"
CONTAINERIZED_VALIDATION=0
ISSUE268_RUNTIME_QUALIFICATION="${ADL_ISSUE268_RUNTIME_QUALIFICATION:-0}"
case "$ADL_REMOTE_COMMAND" in
  "bash adl/tools/run_aws_spot_builder_image_validation.sh "*)
    CONTAINERIZED_VALIDATION=1
    TOOL_INSTALL_POLICY="immutable_builder_image_only"
    ;;
  *)
    if [ "$ISSUE268_RUNTIME_QUALIFICATION" = "1" ]; then
      TOOL_INSTALL_POLICY="amazon_linux_packages_and_pinned_runtime_components"
    fi
    ;;
esac

release_target_triple() {
  local arch
  arch="$(uname -m)"
  case "$arch" in
    x86_64) printf '%s\n' "x86_64-unknown-linux-musl" ;;
    aarch64|arm64) printf '%s\n' "aarch64-unknown-linux-musl" ;;
    *) return 1 ;;
  esac
}

install_github_release_binary() {
  local repo_name binary_name target api_url asset_url archive_path extract_dir release_bin
  repo_name="$1"
  binary_name="$2"
  if [ -n "${3:-}" ]; then
    target="$3"
  else
    target="$(release_target_triple)" || return 1
  fi
  api_url="https://api.github.com/repos/$repo_name/releases/latest"
  asset_url="$(curl -fsSL "$api_url" | python3 -c 'import json, sys
repo = sys.argv[1]
binary = sys.argv[2]
target = sys.argv[3]
data = json.load(sys.stdin)
for asset in data.get("assets", []):
    url = asset.get("browser_download_url", "")
    if binary in url and target in url and url.endswith(".tar.gz"):
        print(url)
        break
' "$repo_name" "$binary_name" "$target")"
  [ -n "$asset_url" ] || return 1
  archive_path="/tmp/adl-$binary_name-release.tar.gz"
  extract_dir="/tmp/adl-$binary_name-release"
  curl -fsSL "$asset_url" -o "$archive_path"
  rm -rf "$extract_dir"
  mkdir -p "$extract_dir"
  tar -xzf "$archive_path" -C "$extract_dir"
  release_bin="$(find "$extract_dir" -type f -name "$binary_name" | head -n 1)"
  [ -n "$release_bin" ] || return 1
  install -m 0755 "$release_bin" "$CARGO_BIN_DIR/$binary_name"
}

install_sccache_release() {
  local target
  target="$(release_target_triple)" || return 1
  case "$target" in
    x86_64-unknown-linux-musl) target="x86_64-unknown-linux-gnu" ;;
    aarch64-unknown-linux-musl) target="aarch64-unknown-linux-gnu" ;;
    *) return 1 ;;
  esac
  install_github_release_binary "mozilla/sccache" "sccache" "$target"
}

ensure_aws_cli() {
  if command -v aws >/dev/null 2>&1; then
    return 0
  fi
  sudo dnf install -y awscli-2 >/tmp/adl-awscli-install.log 2>&1 \
    || sudo yum install -y awscli >/tmp/adl-awscli-install.log 2>&1
}

install_package_manager_binary() {
  local package_name
  package_name="$1"
  sudo dnf install -y "$package_name" >/tmp/adl-"$package_name"-pkg-install.log 2>&1 \
    || sudo yum install -y "$package_name" >/tmp/adl-"$package_name"-pkg-install.log 2>&1
}

archive_installed_binary() {
  local binary_name archive_path package_dir
  binary_name="$1"
  archive_path="$2"
  package_dir="/tmp/adl-$binary_name-package"
  rm -rf "$package_dir"
  mkdir -p "$package_dir"
  cp "$CARGO_BIN_DIR/$binary_name" "$package_dir/$binary_name"
  tar -czf "$archive_path" -C "$package_dir" "$binary_name"
}

install_binary_from_tarball_url() {
  local binary_name tarball_url archive_path
  binary_name="$1"
  tarball_url="$2"
  [ -n "$tarball_url" ] || return 1
  archive_path="/tmp/adl-$binary_name-cache.tar.gz"
  curl -fsSL "$tarball_url" -o "$archive_path"
  install_binary_from_archive_path "$binary_name" "$archive_path"
}

install_binary_from_archive_path() {
  local binary_name archive_path extract_dir release_bin
  binary_name="$1"
  archive_path="$2"
  [ -f "$archive_path" ] || return 1
  extract_dir="/tmp/adl-$binary_name-cache"
  rm -rf "$extract_dir"
  mkdir -p "$extract_dir"
  tar -xzf "$archive_path" -C "$extract_dir"
  release_bin="$(find "$extract_dir" -type f -name "$binary_name" | head -n 1)"
  [ -n "$release_bin" ] || return 1
  install -m 0755 "$release_bin" "$CARGO_BIN_DIR/$binary_name"
}

install_binary_from_s3_cache() {
  local binary_name bucket prefix object_uri archive_path tool_prefix
  binary_name="$1"
  bucket="$2"
  prefix="$3"
  [ -n "$bucket" ] || return 1
  ensure_aws_cli || return 1
  archive_path="/tmp/adl-$binary_name-cache.tar.gz"
  tool_prefix="$prefix/tools"
  object_uri="s3://$bucket/$tool_prefix/$binary_name.tar.gz"
  aws s3 cp "$object_uri" "$archive_path" >/tmp/adl-$binary_name-s3-download.log 2>&1 || return 1
  install_binary_from_archive_path "$binary_name" "$archive_path"
}

upload_binary_to_s3_cache() {
  local binary_name bucket prefix archive_path object_uri tool_prefix
  binary_name="$1"
  bucket="$2"
  prefix="$3"
  [ -n "$bucket" ] || return 0
  ensure_aws_cli || return 1
  archive_path="/tmp/adl-$binary_name-upload.tar.gz"
  tool_prefix="$prefix/tools"
  object_uri="s3://$bucket/$tool_prefix/$binary_name.tar.gz"
  archive_installed_binary "$binary_name" "$archive_path" || return 1
  aws s3 cp "$archive_path" "$object_uri"
}

verify_sccache_binary() {
  command -v sccache >/dev/null 2>&1 || return 1
  sccache --version >/dev/null 2>&1 || return 1
  sccache --start-server >/dev/null 2>&1 || return 1
  sccache --zero-stats >/dev/null 2>&1 || return 1
}

remove_installed_binary() {
  local binary_name
  binary_name="$1"
  rm -f "$CARGO_BIN_DIR/$binary_name"
}

verify_nextest_binary() {
  cargo nextest --version >/dev/null 2>&1
}

install_nextest_release() {
  local target
  target="$(release_target_triple)" || return 1
  case "$target" in
    x86_64-unknown-linux-musl) target="x86_64-unknown-linux-gnu" ;;
    aarch64-unknown-linux-musl) target="aarch64-unknown-linux-gnu" ;;
    *) return 1 ;;
  esac
  install_github_release_binary "nextest-rs/nextest" "cargo-nextest" "$target"
}

export HOME="${HOME:-/root}"
export CARGO_HOME="$CARGO_HOME_DIR"
export RUSTUP_HOME="$RUSTUP_HOME_DIR"
CACHE_BUCKET="${ADL_CACHE_BUCKET:-}"
CACHE_PREFIX="${ADL_CACHE_PREFIX:-}"
SCCACHE_TARBALL_URL="${ADL_SCCACHE_TARBALL_URL:-}"
NEXTEST_TARBALL_URL="${ADL_NEXTEST_TARBALL_URL:-}"
NEEDS_NEXTEST="${ADL_NEEDS_NEXTEST:-0}"
REGION="${ADL_REGION:-us-west-2}"

CURRENT_STAGE="ensure_build_toolchain"
log_progress "stage=ensure_build_toolchain"
if [ "$ISSUE268_RUNTIME_QUALIFICATION" = "1" ]; then
  cloud_init_status=0
  cloud-init status --wait >/tmp/adl-cloud-init.log 2>&1 || cloud_init_status=$?
  runtime_ready=false
  for _ in $(seq 1 450); do
    if [ -f /var/lib/adl/issue268-bootstrap-failed ]; then
      break
    fi
    if [ -f /var/lib/adl/issue268-bootstrap-ready ] \
        && mountpoint -q /opt/adl-runtime \
        && [ -d /opt/adl-runtime/install ]; then
      runtime_ready=true
      break
    fi
    sleep 2
  done
  if [ "$runtime_ready" != true ]; then
    printf '%s\n' "issue268 retained Runtime mount did not become ready" >&2
    sudo systemctl status adl-issue268-runtime-volume.service --no-pager 2>/dev/null >&2 || true
    sudo journalctl -u adl-issue268-runtime-volume.service -n 200 --no-pager 2>/dev/null >&2 || true
    sudo tail -n 200 /var/log/cloud-init-output.log 2>/dev/null >&2 || true
    exit 1
  fi
  if [ "$cloud_init_status" -ne 0 ]; then
    log_progress "stage=ensure_build_toolchain source=user_data_ready cloud_init_status=$cloud_init_status"
  fi
elif [ "$CONTAINERIZED_VALIDATION" = "0" ] && ! command -v cc >/dev/null 2>&1; then
  sudo dnf install -y gcc gcc-c++ make pkgconf-pkg-config openssl-devel >/tmp/adl-build-toolchain.log 2>&1 \
    || sudo yum install -y gcc gcc-c++ make pkgconfig openssl-devel >/tmp/adl-build-toolchain.log 2>&1
fi

CURRENT_STAGE="ensure_rustup"
log_progress "stage=ensure_rustup"
if [ "$CONTAINERIZED_VALIDATION" = "0" ] && [ "$ISSUE268_RUNTIME_QUALIFICATION" = "0" ] && ! command -v cargo >/dev/null 2>&1; then
  curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal >/tmp/adl-rustup.log 2>&1
fi
if [ "$CONTAINERIZED_VALIDATION" = "0" ] && [ -f "$HOME/.cargo/env" ]; then
  . "$HOME/.cargo/env"
fi
export PATH="$CARGO_HOME/bin:$HOME/.cargo/bin:$PATH"
export CARGO_TARGET_DIR="$TARGET_DIR"
export SCCACHE_DIR="$SCCACHE_DIR"
if [ -n "$CACHE_BUCKET" ]; then
  export SCCACHE_BUCKET="$CACHE_BUCKET"
  export SCCACHE_REGION="$REGION"
  export SCCACHE_S3_KEY_PREFIX="$CACHE_PREFIX/sccache"
fi

CURRENT_STAGE="ensure_sccache"
log_progress "stage=ensure_sccache"
log_progress "tool_install_policy=$TOOL_INSTALL_POLICY tool=sccache"
if [ "$CONTAINERIZED_VALIDATION" = "0" ] && [ "$ISSUE268_RUNTIME_QUALIFICATION" = "0" ] && ! command -v sccache >/dev/null 2>&1; then
  SCCACHE_CACHE_HIT=0
  if install_package_manager_binary sccache >>/tmp/adl-sccache-install.log 2>&1 && verify_sccache_binary >>/tmp/adl-sccache-install.log 2>&1; then
    SCCACHE_CACHE_HIT=1
  elif install_binary_from_s3_cache sccache "$CACHE_BUCKET" "$CACHE_PREFIX" >/tmp/adl-sccache-install.log 2>&1 && verify_sccache_binary >>/tmp/adl-sccache-install.log 2>&1; then
    SCCACHE_CACHE_HIT=1
  elif install_binary_from_tarball_url sccache "$SCCACHE_TARBALL_URL" >>/tmp/adl-sccache-install.log 2>&1 && verify_sccache_binary >>/tmp/adl-sccache-install.log 2>&1; then
    SCCACHE_CACHE_HIT=1
  elif install_sccache_release >>/tmp/adl-sccache-install.log 2>&1 && verify_sccache_binary >>/tmp/adl-sccache-install.log 2>&1; then
    :
  else
    remove_installed_binary sccache
    echo "failed to install sccache via package manager or prebuilt artifact paths; source compilation is disabled" >>/tmp/adl-sccache-install.log
    exit 1
  fi
  if [ "$SCCACHE_CACHE_HIT" -eq 0 ]; then
    upload_binary_to_s3_cache sccache "$CACHE_BUCKET" "$CACHE_PREFIX" >>/tmp/adl-sccache-install.log 2>&1 || true
  fi
fi

CURRENT_STAGE="ensure_nextest"
log_progress "stage=ensure_nextest"
log_progress "tool_install_policy=$TOOL_INSTALL_POLICY tool=cargo-nextest"
if [ "$CONTAINERIZED_VALIDATION" = "0" ] && [ "$ISSUE268_RUNTIME_QUALIFICATION" = "0" ] && [ "$NEEDS_NEXTEST" = "1" ] && ! cargo nextest --version >/dev/null 2>&1; then
  NEXTEST_CACHE_HIT=0
  if install_package_manager_binary cargo-nextest >>/tmp/adl-nextest-install.log 2>&1 && verify_nextest_binary >>/tmp/adl-nextest-install.log 2>&1; then
    NEXTEST_CACHE_HIT=1
  elif install_binary_from_s3_cache cargo-nextest "$CACHE_BUCKET" "$CACHE_PREFIX" >/tmp/adl-nextest-install.log 2>&1 && verify_nextest_binary >>/tmp/adl-nextest-install.log 2>&1; then
    NEXTEST_CACHE_HIT=1
  elif install_binary_from_tarball_url cargo-nextest "$NEXTEST_TARBALL_URL" >>/tmp/adl-nextest-install.log 2>&1 && verify_nextest_binary >>/tmp/adl-nextest-install.log 2>&1; then
    NEXTEST_CACHE_HIT=1
  elif install_nextest_release >>/tmp/adl-nextest-install.log 2>&1 && verify_nextest_binary >>/tmp/adl-nextest-install.log 2>&1; then
    :
  else
    remove_installed_binary cargo-nextest
    echo "failed to install cargo-nextest via package manager or prebuilt artifact paths; source compilation is disabled" >>/tmp/adl-nextest-install.log
    exit 1
  fi
  if [ "$NEXTEST_CACHE_HIT" -eq 0 ]; then
    upload_binary_to_s3_cache cargo-nextest "$CACHE_BUCKET" "$CACHE_PREFIX" >>/tmp/adl-nextest-install.log 2>&1 || true
  fi
fi
if [ "$CONTAINERIZED_VALIDATION" = "0" ] && [ "$ISSUE268_RUNTIME_QUALIFICATION" = "0" ]; then
  export RUSTC_WRAPPER="sccache"
else
  unset RUSTC_WRAPPER
fi

RESOLVED_COMMIT="$(git -C "$ADL_REMOTE_REPO_DIR" rev-parse HEAD)"
RUSTC_VERSION="$(rustc --version 2>/dev/null || true)"
CARGO_VERSION="$(cargo --version 2>/dev/null || true)"
SCCACHE_VERSION="$(sccache --version 2>/dev/null || true)"
sccache --start-server >/dev/null 2>&1 || true
sccache --zero-stats >/dev/null 2>&1 || true
SCCACHE_DEGRADED=0
SCCACHE_DEGRADED_REASON=""

watch_sccache_health() {
  while true; do
    if ! sccache --show-stats >/dev/null 2>&1; then
      printf '%s sccache_watch_restart\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$RUN_ROOT/sccache-watch.log"
      sccache --start-server >/dev/null 2>&1 || true
    fi
    sleep 5
  done
}
SCCACHE_WATCH_PID=""
if [ "$CONTAINERIZED_VALIDATION" = "0" ] && [ "$ISSUE268_RUNTIME_QUALIFICATION" = "0" ]; then
  watch_sccache_health >/tmp/adl-sccache-watch.log 2>&1 &
  SCCACHE_WATCH_PID="$!"
fi

INTERRUPTION_NOTICE=""
watch_spot_notice() {
  while true; do
    [ -f "$RUN_ROOT/spot-watcher-stop-requested" ] && break
    TOKEN="$(curl -fsS -X PUT http://169.254.169.254/latest/api/token -H 'X-aws-ec2-metadata-token-ttl-seconds: 60' 2>/dev/null || true)"
    if [ -z "$TOKEN" ]; then
      sleep 5
      continue
    fi
    NOTICE_FILE="$RUN_ROOT/spot-interruption.pending.json"
    HTTP_CODE="$(curl -sS -o "$NOTICE_FILE" -w '%{http_code}' -H "X-aws-ec2-metadata-token: $TOKEN" http://169.254.169.254/latest/meta-data/spot/instance-action || printf '000')"
    if [ "$HTTP_CODE" = "200" ]; then
      if ! jq -e '(.action == "terminate" or .action == "stop") and (.time | type == "string" and length > 0)' "$NOTICE_FILE" >/dev/null; then
        printf '%s\n' "invalid IMDSv2 Spot interruption payload" > "$RUN_ROOT/spot-dehydration.failed"
        break
      fi
      # Establish deterministic ordering with normal-command completion: an
      # idle-stop request observed before acceptance wins over an in-flight
      # IMDS response.
      if [ -f "$RUN_ROOT/spot-watcher-stop-requested" ]; then
        rm -f "$NOTICE_FILE"
        break
      fi
      mv "$NOTICE_FILE" "$RUN_ROOT/spot-interruption.log"
      printf '%s\n' "accepted" > "$RUN_ROOT/spot-dehydration.active"
      DEADLINE_UTC="$(jq -r '.time' "$RUN_ROOT/spot-interruption.log")"
      CALLBACK="${ADL_SPOT_DEHYDRATE_CALLBACK:-}"
      READY="${ADL_SPOT_DEHYDRATE_READY:-}"
      if [ -z "$READY" ] || [ ! -f "$READY" ]; then
        printf '%s\n' "Spot interruption arrived before resident continuity was ready" > "$RUN_ROOT/spot-interruption-before-ready"
        rm -f "$RUN_ROOT/spot-dehydration.active"
        printf '%s\n' "terminal" > "$RUN_ROOT/spot-dehydration.done"
        break
      fi
      if [ -z "$CALLBACK" ] || [ ! -x "$CALLBACK" ]; then
        printf '%s\n' "Spot dehydration callback is missing or not executable" > "$RUN_ROOT/spot-dehydration.failed"
        rm -f "$RUN_ROOT/spot-dehydration.active"
        printf '%s\n' "terminal" > "$RUN_ROOT/spot-dehydration.done"
        break
      fi
      if timeout "${ADL_SPOT_DEHYDRATE_TIMEOUT_SECONDS:-90}" "$CALLBACK" \
        --notice-file "$RUN_ROOT/spot-interruption.log" \
        --deadline-utc "$DEADLINE_UTC" \
        --run-root "$RUN_ROOT" \
        >"$RUN_ROOT/spot-dehydration-receipt.json" \
        2>"$RUN_ROOT/spot-dehydration.err"; then
        jq -e '.admission_open == false and .generation > 0' "$RUN_ROOT/spot-dehydration-receipt.json" >/dev/null || {
          printf '%s\n' "Spot dehydration callback emitted an invalid receipt" > "$RUN_ROOT/spot-dehydration.failed"
        }
      else
        printf '%s\n' "Spot dehydration callback failed or exceeded its deadline" > "$RUN_ROOT/spot-dehydration.failed"
      fi
      rm -f "$RUN_ROOT/spot-dehydration.active"
      printf '%s\n' "terminal" > "$RUN_ROOT/spot-dehydration.done"
      break
    fi
    rm -f "$NOTICE_FILE"
    sleep 5
  done
}
watch_spot_notice >/tmp/adl-spot-watch.log 2>&1 &
WATCH_PID="$!"

BOOTSTRAP_END="$(date +%s)"
COMMAND_START="$(date +%s)"
CURRENT_STAGE="validation_command"
log_progress "stage=validation_command command=${ADL_REMOTE_COMMAND}"
set +e
( cd "$ADL_REMOTE_REPO_DIR" && bash -lc "$ADL_REMOTE_COMMAND" ) >"$RUN_ROOT/command.log" 2>"$RUN_ROOT/command.err"
COMMAND_EXIT="$?"
set -e
COMMAND_END="$(date +%s)"
# Builder failures are captured under set +e, so ERR does not run. Emit the
# retained redacted diagnostic on the normal path without changing command,
# summary, or cleanup authority.
emit_debug_log builder_toolchain "$RUN_ROOT/builder-toolchain.log" || true
# Request idle watcher shutdown, then wait. A notice already accepted takes
# precedence and its bounded callback reaches a terminal receipt/failure before
# this wait returns; normal validation completion can never cancel it.
printf '%s\n' "validation-command-terminal" > "$RUN_ROOT/spot-watcher-stop-requested"
WATCH_EXIT=0
wait "$WATCH_PID" >/dev/null 2>&1 || WATCH_EXIT="$?"
if [ "$WATCH_EXIT" != "0" ]; then
  printf '%s\n' "Spot watcher exited unexpectedly" > "$RUN_ROOT/spot-dehydration.failed"
fi
if [ -n "$SCCACHE_WATCH_PID" ]; then
  kill "$SCCACHE_WATCH_PID" >/dev/null 2>&1 || true
  wait "$SCCACHE_WATCH_PID" >/dev/null 2>&1 || true
  sccache --show-stats >"$RUN_ROOT/sccache-stats.log" 2>&1 || true
fi
[ -f "$RUN_ROOT/spot-interruption.log" ] && INTERRUPTION_NOTICE="$(cat "$RUN_ROOT/spot-interruption.log")"
if [ -f "$RUN_ROOT/spot-interruption.log" ] && { [ ! -f "$RUN_ROOT/spot-dehydration.done" ] || [ -f "$RUN_ROOT/spot-dehydration.active" ]; }; then
  printf '%s\n' "Accepted Spot transaction lacks exact terminal state" > "$RUN_ROOT/spot-dehydration.failed"
fi
if [ -f "$RUN_ROOT/spot-dehydration.failed" ]; then
  COMMAND_EXIT=70
  cat "$RUN_ROOT/spot-dehydration.failed" >> "$RUN_ROOT/command.err"
fi
if [ -f "$RUN_ROOT/spot-interruption-before-ready" ]; then
  COMMAND_EXIT=75
  cat "$RUN_ROOT/spot-interruption-before-ready" >> "$RUN_ROOT/command.err"
fi
if grep -Fq "sccache: warning: The server looks like it shut down unexpectedly" "$RUN_ROOT/command.err"; then
  SCCACHE_DEGRADED=1
  SCCACHE_DEGRADED_REASON="server_shut_down_unexpectedly"
elif grep -Fq "sccache: error:" "$RUN_ROOT/command.err"; then
  SCCACHE_DEGRADED=1
  SCCACHE_DEGRADED_REASON="client_or_server_error"
fi
if [ "$CONTAINERIZED_VALIDATION" = "0" ] && [ "$ISSUE268_RUNTIME_QUALIFICATION" = "0" ] && [ ! -s "$RUN_ROOT/sccache-stats.log" ]; then
  SCCACHE_DEGRADED=1
  if [ -z "$SCCACHE_DEGRADED_REASON" ]; then
    SCCACHE_DEGRADED_REASON="missing_stats"
  fi
fi

export ADL_RUN_ROOT="$RUN_ROOT"
export COMMAND_EXIT BOOTSTRAP_START BOOTSTRAP_END COMMAND_START COMMAND_END
export INTERRUPTION_NOTICE RESOLVED_COMMIT RUSTC_VERSION CARGO_VERSION SCCACHE_VERSION
export SCCACHE_DEGRADED SCCACHE_DEGRADED_REASON
export CONTAINERIZED_VALIDATION
export ISSUE268_RUNTIME_QUALIFICATION
python3 - <<'PY'
import json
import os
from pathlib import Path
run_root = Path(os.environ["ADL_RUN_ROOT"])
payload = {
  "status": "passed" if int(os.environ["COMMAND_EXIT"]) == 0 else "failed",
  "bootstrap_seconds": int(os.environ["BOOTSTRAP_END"]) - int(os.environ["BOOTSTRAP_START"]),
  "command_seconds": int(os.environ["COMMAND_END"]) - int(os.environ["COMMAND_START"]),
  "interruption_detected": bool(os.environ.get("INTERRUPTION_NOTICE", "")),
  "interruption_notice": os.environ.get("INTERRUPTION_NOTICE") or None,
  "resolved_commit": os.environ.get("RESOLVED_COMMIT") or None,
  "rustc_version": os.environ.get("RUSTC_VERSION") or None,
  "cargo_version": os.environ.get("CARGO_VERSION") or None,
  "sccache_version": os.environ.get("SCCACHE_VERSION") or None,
  "sccache_degraded": os.environ.get("SCCACHE_DEGRADED") == "1",
  "sccache_degraded_reason": os.environ.get("SCCACHE_DEGRADED_REASON") or None,
  "sccache_stats": {"raw_excerpt": run_root.joinpath("sccache-stats.log").read_text(errors="replace").splitlines()[:16] if run_root.joinpath("sccache-stats.log").exists() else []}
}
builder_summary = run_root.joinpath("spot-builder-summary.json")
if builder_summary.exists():
  payload["builder_proof"] = json.loads(builder_summary.read_text(encoding="utf-8"))
payload["host_validation_tools_installed"] = os.environ.get("CONTAINERIZED_VALIDATION") != "1"
payload["validation_environment"] = "direct_host_runtime" if os.environ.get("ISSUE268_RUNTIME_QUALIFICATION") == "1" else ("immutable_builder" if os.environ.get("CONTAINERIZED_VALIDATION") == "1" else "direct_host")
payload["runtime_toolchain_verified"] = bool(os.environ.get("ISSUE268_RUNTIME_QUALIFICATION") == "1" and payload["rustc_version"] and payload["cargo_version"])
print("ADL_AWS_REMOTE_SUMMARY_BEGIN")
print(json.dumps(payload))
print("ADL_AWS_REMOTE_SUMMARY_END")
PY
cat "$RUN_ROOT/command.log"
cat "$RUN_ROOT/command.err" >&2
exit "$COMMAND_EXIT"
