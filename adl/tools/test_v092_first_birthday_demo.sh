#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
case "${1:-}" in
  --positive)
    packet="demos/v0.92/first-birthday/positive.json"
    cargo run --quiet --manifest-path "$repo_root/adl-runtime-kernel/Cargo.toml" --locked --bin adl-runtime-birthday-demo -- --case positive --output "$packet"
    python3 "$repo_root/adl/tools/validate_v092_first_birthday_packet.py" "$repo_root/$packet" --expect complete
    ;;
  --negative)
    for case_name in startup wake restore snapshot admission copied_state simulation named_fixture missing_identity_root missing_continuity_head missing_memory_grounding missing_capability_envelope missing_cognitive_profile missing_witness_set missing_receipt missing_reviewer_validation; do
      packet="demos/v0.92/first-birthday/negative-${case_name}.json"
      cargo run --quiet --manifest-path "$repo_root/adl-runtime-kernel/Cargo.toml" --locked --bin adl-runtime-birthday-demo -- --case "$case_name" --output "$packet"
      python3 "$repo_root/adl/tools/validate_v092_first_birthday_packet.py" "$repo_root/$packet" --expect rejected
    done
    packet="demos/v0.92/first-birthday/interrupted.json"
    cargo run --quiet --manifest-path "$repo_root/adl-runtime-kernel/Cargo.toml" --locked --bin adl-runtime-birthday-demo -- --case interrupted --output "$packet"
    python3 "$repo_root/adl/tools/validate_v092_first_birthday_packet.py" "$repo_root/$packet" --expect incomplete
    ;;
  --native-platform)
    platform="${2:?native platform required}"
    actual="$(uname -s | tr '[:upper:]' '[:lower:]')"
    case "$platform:$actual" in macos:darwin|linux:linux) ;; *) echo "requested native $platform proof on $actual" >&2; exit 65 ;; esac
    "$0" --positive
    architecture="$(uname -m)"
    source_revision="$(git -C "$repo_root" rev-parse HEAD)"
    packet_sha256="$(shasum -a 256 "$repo_root/demos/v0.92/first-birthday/positive.json" | awk '{print $1}')"
    receipt="$repo_root/.csdlc/evidence/5836/native-${platform}-receipt.json"
    python3 - "$receipt" "$platform" "$actual" "$architecture" "$source_revision" "$packet_sha256" <<'PY'
import json
import pathlib
import sys

receipt, platform, kernel, architecture, revision, packet_sha256 = sys.argv[1:]
payload = {
    "schema": "adl.first_birthday.native_receipt.v1",
    "platform": platform,
    "host_class": f"native-{kernel}-{architecture}",
    "source_revision": revision,
    "argv": [
        "bash",
        "adl/tools/test_v092_first_birthday_demo.sh",
        "--native-platform",
        platform,
    ],
    "result": "passed",
    "packet": "demos/v0.92/first-birthday/positive.json",
    "packet_sha256": packet_sha256,
    "allowed_nondeterminism": [
        "transient_runtime_directory",
        "native_host_class_recorded_outside_semantic_packet",
    ],
}
path = pathlib.Path(receipt)
path.parent.mkdir(parents=True, exist_ok=True)
path.write_text(json.dumps(payload, indent=2) + "\n")
print(json.dumps(payload, separators=(",", ":")))
PY
    ;;
  *)
    echo "usage: $0 --positive|--negative|--native-platform macos|linux" >&2
    exit 64
    ;;
esac
