#!/usr/bin/env python3
import json
import sys
import wave
import xml.etree.ElementTree as ET
from pathlib import Path


FORBIDDEN_PUBLIC_TEXT = [
    "Packet status",
    "proof boundary",
    "truth boundary",
    "render_status",
    "C-SDLC",
    "manifest only",
    "not live-proven",
]


def fail(message: str) -> None:
    raise SystemExit(f"podcast launch validation failed: {message}")


def main() -> None:
    if len(sys.argv) != 3:
        fail("usage: validate_podcast_launch_packet.py <podcast-root> <episodes-json>")
    root = Path(sys.argv[1])
    episodes_path = Path(sys.argv[2])
    packet = json.loads(episodes_path.read_text(encoding="utf-8"))
    episodes = packet.get("episodes") or []
    if len(episodes) != 10:
        fail("expected exactly 10 episode records")

    required = [
        root / "index.html",
        root / "episodes" / "meet-the-ai-coworkers" / "index.html",
        root / "feed.xml",
        root / "audio" / "meet-the-ai-coworkers.wav",
    ]
    for path in required:
        if not path.is_file() or path.stat().st_size == 0:
            fail(f"missing required launch artifact: {path}")

    for html_path in [root / "index.html", root / "episodes" / "meet-the-ai-coworkers" / "index.html"]:
        text = html_path.read_text(encoding="utf-8")
        for forbidden in FORBIDDEN_PUBLIC_TEXT:
            if forbidden.lower() in text.lower():
                fail(f"public page contains internal/non-claim wording {forbidden!r}: {html_path}")
        if "<audio controls" not in text:
            fail(f"missing playable audio control: {html_path}")

    with wave.open(str(root / "audio" / "meet-the-ai-coworkers.wav"), "rb") as wav:
        duration = wav.getnframes() / wav.getframerate()
        if duration <= 1.0:
            fail("episode audio is too short to prove playable output")

    feed = ET.parse(root / "feed.xml").getroot()
    enclosure = feed.find("./channel/item/enclosure")
    if enclosure is None:
        fail("RSS item is missing enclosure")
    if enclosure.attrib.get("type") != "audio/wav":
        fail("RSS enclosure must be audio/wav for the local launch proof")
    length = int(enclosure.attrib.get("length", "0"))
    actual = (root / "audio" / "meet-the-ai-coworkers.wav").stat().st_size
    if length != actual:
        fail(f"RSS enclosure length {length} does not match audio size {actual}")

    print("podcast_launch_packet: PASS")


if __name__ == "__main__":
    main()
