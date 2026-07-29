#!/usr/bin/env python3
import json
import hashlib
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

STUDIO_HTML = "podcast-studio.html"


def fail(message: str) -> None:
    raise SystemExit(f"podcast launch validation failed: {message}")


def reference_digest(path: Path) -> str:
    if not path.is_file():
        fail(f"missing studio reference digest manifest: {path}")
    for line in path.read_text(encoding="utf-8").splitlines():
        parts = line.split(maxsplit=1)
        if len(parts) == 2 and parts[1] == STUDIO_HTML:
            return parts[0]
    fail(f"studio reference digest manifest is missing {STUDIO_HTML}")


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
        root / "studio" / "index.html",
        root / "studio" / STUDIO_HTML,
        root / "studio" / "support.js",
        root / "studio" / "image-slot.js",
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
    index_text = (root / "index.html").read_text(encoding="utf-8")
    if 'href="studio/"' not in index_text:
        fail("podcast landing page does not link to the integrated studio route")

    studio_html = root / "studio" / STUDIO_HTML
    studio_text = studio_html.read_text(encoding="utf-8")
    if "Synthetic Minds" not in studio_text or "{{ latest.title }}" not in studio_text:
        fail("studio reference HTML no longer looks like the operator-provided export")
    if '<script src="./support.js"></script>' not in studio_text:
        fail("studio reference HTML is not wired to its local support.js asset")
    digest_file = root / "studio" / "reference.sha256"
    if not digest_file.is_file():
        fail("studio route is missing reference.sha256")
    expected_digest = reference_digest(root / "studio" / "REFERENCE_DIGESTS.txt")
    generated_digest = digest_file.read_text(encoding="utf-8").split()[0]
    if generated_digest != expected_digest:
        fail("studio route reference.sha256 does not match source reference digest")
    actual_digest = hashlib.sha256(studio_html.read_bytes()).hexdigest()
    if expected_digest != actual_digest:
        fail("studio reference HTML digest does not match source reference digest")
    source_reference = root / "studio-reference" / STUDIO_HTML
    if source_reference.is_file():
        source_digest = hashlib.sha256(source_reference.read_bytes()).hexdigest()
        source_expected = reference_digest(root / "studio-reference" / "REFERENCE_DIGESTS.txt")
        if source_digest != source_expected or source_digest != actual_digest:
            fail("generated studio HTML is not byte-identical to the tracked studio reference")

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
