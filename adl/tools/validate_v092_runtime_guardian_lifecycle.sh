#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "$0")/../.." && pwd -P)
qualification_root=${ADL_RUNTIME_GUARDIAN_EVIDENCE_ROOT:-$repo_root/.adl/runtime-v3/qualification}
target_dir=${CARGO_TARGET_DIR:-$repo_root/.adl/target/5820-runtime}

case "$qualification_root" in
  "$repo_root"/.adl/*) ;;
  *) echo "evidence root must stay under the issue worktree .adl directory" >&2; exit 64 ;;
esac
case "$target_dir" in
  /*) ;;
  *) echo "CARGO_TARGET_DIR must be absolute" >&2; exit 64 ;;
esac
mkdir -p "$qualification_root" "$target_dir"
export CARGO_TARGET_DIR="$target_dir"

vector_bin=${ADL_RUNTIME_VECTOR_BIN:-}
if [[ -z "$vector_bin" ]]; then
  vector_bin=$(command -v vector || true)
fi
if [[ -z "$vector_bin" || ! -x "$vector_bin" ]]; then
  echo "ADL_RUNTIME_VECTOR_BIN must name an executable Vector binary" >&2
  exit 69
fi
vector_bin=$(cd "$(dirname "$vector_bin")" && pwd -P)/$(basename "$vector_bin")

cargo build --locked --manifest-path "$repo_root/adl-runtime-kernel/Cargo.toml" \
  --bin adl-runtime-kernel
cargo build --locked --manifest-path "$repo_root/adl-runtime/Cargo.toml" \
  --bin adl-runtime-guardian --bin adl-runtime-lifecycle-soak

revision=$(git -C "$repo_root" rev-parse HEAD)
run_root=$(mktemp -d "$qualification_root/5820-run.XXXXXX")
state_root="$run_root/state"
report="$run_root/report.json"
api_port=$(ruby -rsocket -e 'socket = TCPServer.new("127.0.0.1", 0); puts socket.addr[1]; socket.close')
init_template="$qualification_root/5820-runtime-init-$api_port.toml"
mkdir -p "$(dirname "$init_template")"
ruby -e '
  source, destination, port = ARGV
  text = File.read(source)
  address = %(address = "127.0.0.1:20997")
  public_url = %(public_base_url = "https://localhost:20997")
  abort "canonical API address missing" unless text.scan(address).length == 1
  abort "canonical public URL missing" unless text.scan(public_url).length == 1
  text = text.sub(address, %(address = "127.0.0.1:#{port}"))
  text = text.sub(public_url, %(public_base_url = "https://localhost:#{port}"))
  File.write(destination, text)
' "$repo_root/infra/runtime-v3/runtime-init.toml" "$init_template" "$api_port"

"$target_dir/debug/adl-runtime-lifecycle-soak" \
  --guardian "$target_dir/debug/adl-runtime-guardian" \
  --kernel "$target_dir/debug/adl-runtime-kernel" \
  --vector "$vector_bin" \
  --init-template "$init_template" \
  --state-root "$state_root" \
  --report "$report" \
  --revision "$revision" \
  --suite preflight

ruby -rjson -e '
  report = JSON.parse(File.read(ARGV.fetch(0)))
  abort "wrong lifecycle report schema" unless report["schema"] == "adl.runtime_v3.lifecycle_soak.v1"
  abort "lifecycle preflight failed" unless report["status"] == "pass"
  abort "Guardian was not launched" unless report["guardian_launch_count"].to_i == 1
  abort "kernel start denominator drifted" unless report["runtime_start_count"].to_i == 2
  abort "kernel restart was not exercised" unless report["restart_budget_exercised"] == true
  abort "kernel restart count drifted" unless report["total_restarts"].to_i == 1
  abort "durable continuity was not retained" unless report["continuity_generation"].to_i == 1
  abort "clean log proof is missing" unless report["logging_complete"] == true && report["master_log_status"] == "clean"
' "$report"

printf 'PASS: production Guardian lifecycle report=%s revision=%s\n' "$report" "$revision"
