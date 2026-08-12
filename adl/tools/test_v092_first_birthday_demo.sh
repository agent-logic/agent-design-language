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
    ;;
  *)
    echo "usage: $0 --positive|--negative|--native-platform macos|linux" >&2
    exit 64
    ;;
esac
