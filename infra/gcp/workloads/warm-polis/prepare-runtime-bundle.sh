#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "$0")/../../../../" && pwd)"
base_bundle="${ADL_GCP_RUNTIME_BASE_BUNDLE:?}"
certificate_chain="${ADL_GCP_RUNTIME_CERTIFICATE_CHAIN:?}"
private_key="${ADL_GCP_RUNTIME_PRIVATE_KEY:?}"
trust_roots="${ADL_GCP_RUNTIME_TRUST_ROOTS:?}"
output="${ADL_GCP_RUNTIME_BUNDLE_OUTPUT:?}"
work_root="${ADL_GCP_RUNTIME_BUNDLE_WORK_ROOT:?}"
lifecycle_soak="${ADL_RUNTIME_LIFECYCLE_SOAK:?}"
vector_archive="${ADL_GCP_VECTOR_ARCHIVE:?}"
source_revision="${ADL_GCP_RUNTIME_SOURCE_REVISION:?}"
elf_strip="${ADL_GCP_ELF_STRIP:-}"

case "$work_root" in
  "$root"/.csdlc/local/*) ;;
  *) echo "work root must stay under the issue worktree .csdlc/local" >&2; exit 64 ;;
esac
for file in "$base_bundle" "$certificate_chain" "$private_key" "$trust_roots" "$lifecycle_soak" "$vector_archive"; do
  [ -f "$file" ] || { echo "missing input: $file" >&2; exit 66; }
done
[ "$(shasum -a 256 "$vector_archive" | awk '{print $1}')" = "8c114c5e9fd9646516f014d5d837690447cf0d4f43ba4a3746713bc0612b039b" ]

[ ! -e "$work_root" ] || { echo "work root already exists: $work_root" >&2; exit 73; }
install -d -m 0700 "$work_root/stage/install/bin" "$work_root/state" "$work_root/tls-input"
tar -xzf "$base_bundle" -C "$work_root/stage/install"

cat "$root"/.csdlc/evidence/5820/native/linux/adl-runtime-guardian.gz.part-* >"$work_root/guardian.gz"
cat "$root"/.csdlc/evidence/5820/native/linux/adl-runtime-kernel.gz.part-* >"$work_root/kernel.gz"
[ "$(shasum -a 256 "$work_root/guardian.gz" | awk '{print $1}')" = "9ae7b7747c38d068f81287b4309eeaeb0b7b950f40d9dc6f13c7a82a78edc16e" ]
[ "$(shasum -a 256 "$work_root/kernel.gz" | awk '{print $1}')" = "de637e13ea10175c1e70f7683ee76c17d08f61a01a56d11986baf6877dca6527" ]
gzip -dc "$work_root/guardian.gz" >"$work_root/stage/install/bin/adl-runtime-guardian"
gzip -dc "$work_root/kernel.gz" >"$work_root/stage/install/bin/adl-runtime-kernel"
[ "$(shasum -a 256 "$work_root/stage/install/bin/adl-runtime-guardian" | awk '{print $1}')" = "1b0bfdc5d5fecf3e456fd9db1f3b3be63e7683f3fc23e5c1fc2096567559cef1" ]
[ "$(shasum -a 256 "$work_root/stage/install/bin/adl-runtime-kernel" | awk '{print $1}')" = "4833d3701c0b3afd0aa5bafed5d884fd7881df7425622b51fa5347adceaa15b0" ]
chmod 0755 "$work_root/stage/install/bin/adl-runtime-guardian" "$work_root/stage/install/bin/adl-runtime-kernel"
if [ -n "$elf_strip" ]; then
  [ -x "$elf_strip" ] || { echo "ELF strip tool is not executable: $elf_strip" >&2; exit 66; }
  "$elf_strip" --strip-debug \
    "$work_root/stage/install/bin/adl-runtime-guardian" \
    "$work_root/stage/install/bin/adl-runtime-kernel"
fi

tar -xzf "$vector_archive" -C "$work_root"
cp "$work_root/vector-x86_64-unknown-linux-musl/bin/vector" "$work_root/stage/install/bin/vector"
chmod 0755 "$work_root/stage/install/bin/vector"

cp "$certificate_chain" "$work_root/tls-input/fullchain.pem"
cp "$private_key" "$work_root/tls-input/private-key.pem"
cp "$trust_roots" "$work_root/tls-input/trust-roots.pem"
chmod 0600 "$work_root/tls-input"/*

cp "$root/infra/runtime-v3/runtime-init.toml" "$work_root/runtime-init.toml"
python3 - "$work_root/runtime-init.toml" "$work_root/tls-input" <<'PY'
import pathlib
import re
import sys

path = pathlib.Path(sys.argv[1])
tls = pathlib.Path(sys.argv[2])
text = path.read_text()
text = text.replace('public_base_url = "https://runtime.dev.agent-logic.ai:20997"', 'public_base_url = "https://wuji.dev.csm.agent-logic.ai:20997"')
text = text.replace('server_name = "runtime.dev.agent-logic.ai"', 'server_name = "wuji.dev.csm.agent-logic.ai"')
text = text.replace('public_domain = "runtime.dev.agent-logic.ai"', 'public_domain = "wuji.dev.csm.agent-logic.ai"')
text = text.replace('/var/lib/adl/runtime-v3/tls/fullchain.pem', str(tls / 'fullchain.pem'))
text = text.replace('/var/lib/adl/runtime-v3/tls/private-key.pem', str(tls / 'private-key.pem'))
text = text.replace('/var/lib/adl/runtime-v3/tls/trust-roots.pem', str(tls / 'trust-roots.pem'))
section = ""
kept = []
blocked_sections = (
    "continuity_control",
    "polis",
    "resident_shepherd",
    "observability_pipeline.cloudwatch",
)
blocked_credentials = {
    "migration_decision_public_key_path",
    "migration_decision_key_id",
    "migration_decision_key_generation",
    "acip_write_token_path",
    "birth_witness_trust_manifest_path",
}
for line in text.splitlines(keepends=True):
    match = re.match(r'^\[([^]]+)\]\s*$', line)
    if match:
        section = match.group(1)
    if any(section == prefix or section.startswith(prefix + ".") for prefix in blocked_sections):
        continue
    key = line.split('=', 1)[0].strip() if '=' in line else ""
    if section == "api.tls" and key in {"server_name", "trust_roots_path"}:
        continue
    if section == "credentials" and key in blocked_credentials:
        continue
    kept.append(line)
path.write_text(''.join(kept))
PY

"$lifecycle_soak" \
  --guardian "$work_root/stage/install/bin/adl-runtime-guardian" \
  --kernel "$work_root/stage/install/bin/adl-runtime-kernel" \
  --vector "$work_root/stage/install/bin/vector" \
  --init-template "$work_root/runtime-init.toml" \
  --state-root "$work_root/state" \
  --report "$work_root/prepared-state.json" \
  --revision "$source_revision" \
  --suite preflight_1x \
  --prepare-only >/dev/null

install -d -m 0700 "$work_root/stage/install/runtime-state"
cp -a "$work_root/state/." "$work_root/stage/install/runtime-state/"
python3 - "$work_root/stage/install/runtime-state/runtime-init.toml" "$work_root" <<'PY'
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
work = sys.argv[2]
text = path.read_text()
text = text.replace(f'{work}/state', '/var/lib/adl/issue663/runtime-state')
text = text.replace(f'{work}/stage/install/bin/adl-runtime-kernel', '/mnt/adl-runtime/install/bin/adl-runtime-kernel')
text = text.replace(f'{work}/stage/install/bin/vector', '/mnt/adl-runtime/install/bin/vector')
path.write_text(text)
if work in text:
    raise SystemExit('prepared state retains local worktree paths')
PY

install -d -m 0755 "$work_root/stage/install/config/tls"
cp "$certificate_chain" "$work_root/stage/install/config/tls/ca.pem"
chmod 0600 "$work_root/stage/install/runtime-state/tls"/* "$work_root/stage/install/runtime-state/credentials"/*
tar -C "$work_root/stage" -czf "$output" .
shasum -a 256 "$output"
