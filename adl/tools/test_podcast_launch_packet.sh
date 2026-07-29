#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/adl-podcast-launch.XXXXXX")"
trap 'rm -rf "$TMP_DIR"' EXIT

env -u OPENAI_API_KEY -u GEMINI_API_KEY \
ADL_OPENAI_KEY_FILE="$TMP_DIR/missing-openai.key" \
ADL_GEMINI_KEY_FILE="$TMP_DIR/missing-gemini.key" \
ADL_PODCAST_AUDIO_TEST_TONES=1 \
ADL_PODCAST_LAUNCH_WORK_DIR="$TMP_DIR/adl-podcast-launch-work" \
  bash "$ROOT_DIR/adl/tools/demo_v0918_podcast_launch.sh" "$TMP_DIR/site" >/dev/null

python3 "$ROOT_DIR/adl/tools/validate_podcast_launch_packet.py" \
  "$TMP_DIR/site" \
  "$ROOT_DIR/docs/milestones/v0.91.8/review/podcast_launch_5711/episodes.json"

echo "test_podcast_launch_packet: PASS"
