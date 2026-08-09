#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/adl-podcast-launch.XXXXXX")"
server_pid=""
cleanup() {
  if [ -n "$server_pid" ]; then
    kill "$server_pid" 2>/dev/null || true
  fi
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

env -u OPENAI_API_KEY -u GEMINI_API_KEY \
ADL_OPENAI_KEY_FILE="$TMP_DIR/missing-openai.key" \
ADL_GEMINI_KEY_FILE="$TMP_DIR/missing-gemini.key" \
ADL_PODCAST_AUDIO_TEST_TONES=1 \
ADL_PODCAST_LAUNCH_WORK_DIR="$TMP_DIR/adl-podcast-launch-work" \
  bash "$ROOT_DIR/adl/tools/demo_v0918_podcast_launch.sh" "$TMP_DIR/site" >/dev/null

python3 "$ROOT_DIR/adl/tools/validate_podcast_launch_packet.py" \
  "$TMP_DIR/site" \
  "$ROOT_DIR/docs/milestones/v0.91.8/review/podcast_launch_5711/episodes.json"

port="$(python3 - <<'PY'
import socket

with socket.socket() as sock:
    sock.bind(("127.0.0.1", 0))
    print(sock.getsockname()[1])
PY
)"
python3 -m http.server "$port" --bind 127.0.0.1 --directory "$ROOT_DIR/demos" >"$TMP_DIR/http.log" 2>&1 &
server_pid="$!"
python3 - <<'PY' "$port"
import sys
import time
import urllib.request

port = sys.argv[1]
url = f"http://127.0.0.1:{port}/podcast/"
deadline = time.time() + 5
while True:
    try:
        with urllib.request.urlopen(url, timeout=1) as response:
            if response.status == 200:
                break
    except Exception:
        pass
    if time.time() > deadline:
        raise SystemExit(f"podcast test HTTP server did not become ready: {url}")
    time.sleep(0.1)
PY

python3 "$ROOT_DIR/adl/tools/validate_podcast_launch_packet.py" \
  "$ROOT_DIR/demos/podcast" \
  "$ROOT_DIR/docs/milestones/v0.91.8/review/podcast_launch_5711/episodes.json" \
  --preview-root "$ROOT_DIR/demos/_preview/podcast" \
  --http-base "http://127.0.0.1:$port"

python3 - "$ROOT_DIR" "$TMP_DIR" <<'PY'
import importlib.util
import json
import sys
from pathlib import Path

root = Path(sys.argv[1])
tmp = Path(sys.argv[2])
spec = importlib.util.spec_from_file_location(
    "podcast_validator", root / "adl/tools/validate_podcast_launch_packet.py"
)
validator = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(validator)


def expect_failure(label, operation):
    try:
        operation()
    except SystemExit:
        return
    raise SystemExit(f"negative podcast validation did not reject {label}")


false_page = tmp / "false-public-provenance.html"
false_page.write_text(
    f"<html><body>{validator.SHOW_TITLE}: live, unscripted<a href='#'>x</a></body></html>",
    encoding="utf-8",
)
expect_failure("false public provenance", lambda: validator.validate_html_public_text(false_page))

package = root / "demos/podcast/episodes/001-meet-the-ai-coworkers"
episode = json.loads((package / "episode.json").read_text(encoding="utf-8"))
guest = json.loads((package / "guest-metadata.json").read_text(encoding="utf-8"))
guest["guest_acceptance_claimed"] = True
expect_failure("guest acceptance overclaim", lambda: validator.validate_guest_packet(guest))

enclosure = json.loads((package / "rss-enclosure.json").read_text(encoding="utf-8"))
enclosure["bytes"] += 1
expect_failure("RSS enclosure mismatch", lambda: validator.validate_enclosure_packet(enclosure, episode))
PY

episode_source="$ROOT_DIR/demos/podcast/episodes/001-meet-the-ai-coworkers"
ADL_PODCAST_AUDIO_TEST_TONES=1 \
ADL_PODCAST_AUDIO_SOURCE_DIR="$episode_source" \
  bash "$ROOT_DIR/adl/tools/demo_v0911_multiagent_podcast_audio.sh" \
    "$TMP_DIR/episode-001-reproduction" >/dev/null
python3 - "$TMP_DIR/episode-001-reproduction/audio_manifest.json" <<'PY'
import json
import sys

manifest = json.load(open(sys.argv[1], encoding="utf-8"))
segments = manifest.get("segments") or []
if len(segments) != 24:
    raise SystemExit(f"Episode 001 reproduction expected 24 turns, found {len(segments)}")
if not all(segment["source_text_file"].startswith("script.md#turn-") for segment in segments):
    raise SystemExit("Episode 001 reproduction did not retain script turn provenance")
PY

echo "test_podcast_launch_packet: PASS"
