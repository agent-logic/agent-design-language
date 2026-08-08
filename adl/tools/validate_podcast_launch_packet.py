#!/usr/bin/env python3
import json
import hashlib
import struct
import sys
import wave
import xml.etree.ElementTree as ET
import urllib.request
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import unquote, urlsplit


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
ITUNES_NS = "http://www.itunes.com/dtds/podcast-1.0.dtd"


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


class LocalReferenceParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.refs: list[tuple[str, str, str]] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]) -> None:
        attr_map = {key.lower(): value for key, value in attrs if value is not None}
        for attr in ("href", "src"):
            value = attr_map.get(attr)
            if value:
                self.refs.append((tag.lower(), attr, value))
        if tag.lower() == "meta" and attr_map.get("http-equiv", "").lower() == "refresh":
            content = attr_map.get("content", "")
            marker = "url="
            index = content.lower().find(marker)
            if index >= 0:
                self.refs.append(("meta", "content", content[index + len(marker) :].strip()))


def is_skipped_reference(ref: str) -> bool:
    stripped = ref.strip()
    if not stripped or stripped.startswith("#") or "{{" in stripped or "}}" in stripped:
        return True
    split = urlsplit(stripped)
    if split.scheme in {"http", "https", "mailto", "tel", "javascript", "data"}:
        return True
    return False


def resolve_local_reference(html_path: Path, ref: str) -> Path:
    split = urlsplit(ref.strip())
    raw_path = unquote(split.path)
    if not raw_path:
        return html_path
    return (html_path.parent / raw_path).resolve()


def validate_local_references(html_path: Path) -> None:
    parser = LocalReferenceParser()
    parser.feed(html_path.read_text(encoding="utf-8"))
    for tag, attr, ref in parser.refs:
        if is_skipped_reference(ref):
            continue
        target = resolve_local_reference(html_path, ref)
        if not target.exists():
            fail(f"{html_path} has broken local {tag} {attr} reference {ref!r}")


def validate_html_public_text(html_path: Path, require_audio: bool = False) -> None:
    text = html_path.read_text(encoding="utf-8")
    for forbidden in FORBIDDEN_PUBLIC_TEXT:
        if forbidden.lower() in text.lower():
            fail(f"public page contains internal/non-claim wording {forbidden!r}: {html_path}")
    if require_audio and "<audio controls" not in text:
        fail(f"missing playable audio control: {html_path}")
    validate_local_references(html_path)


def validate_feed(root: Path) -> None:
    feed = ET.parse(root / "feed.xml").getroot()
    channel = feed.find("./channel")
    if channel is None:
        fail("RSS feed is missing channel")
    title = channel.findtext("title", "")
    link = channel.findtext("link", "")
    if "Podcast" not in title:
        fail("RSS feed title does not identify a podcast")
    if link.rstrip("/") != "https://agent-logic.ai/podcast":
        fail("RSS feed link does not target the podcast route")
    enclosure = feed.find("./channel/item/enclosure")
    if enclosure is None:
        fail("RSS item is missing enclosure")
    enclosure_type = enclosure.attrib.get("type")
    if enclosure_type not in {"audio/wav", "audio/mpeg"}:
        fail("RSS enclosure must be audio/wav or audio/mpeg")
    length = int(enclosure.attrib.get("length", "0"))
    audio_name = "meet-the-ai-coworkers.mp3" if enclosure_type == "audio/mpeg" else "meet-the-ai-coworkers.wav"
    actual = (root / "audio" / audio_name).stat().st_size
    if length != actual:
        fail(f"RSS enclosure length {length} does not match audio size {actual}")
    if enclosure_type == "audio/mpeg":
        expected_url = f"https://agent-logic.ai/podcast/audio/{audio_name}"
        if enclosure.attrib.get("url") != expected_url:
            fail("production RSS enclosure URL is not the stable MP3 route")
        required_channel_tags = ["image", "category"]
        for name in required_channel_tags:
            if channel.find(f"{{{ITUNES_NS}}}{name}") is None:
                fail(f"production RSS channel is missing itunes:{name}")
        item = channel.find("item")
        if item is None:
            fail("production RSS feed is missing its episode item")
        expected = {
            "episode": "1",
            "episodeType": "full",
            "explicit": "false",
            "duration": "00:18:32",
        }
        for name, value in expected.items():
            if item.findtext(f"{{{ITUNES_NS}}}{name}", "") != value:
                fail(f"production RSS item has invalid itunes:{name}")


def validate_png_artwork(path: Path) -> None:
    data = path.read_bytes()[:33]
    if len(data) < 33 or data[:8] != b"\x89PNG\r\n\x1a\n" or data[12:16] != b"IHDR":
        fail("show artwork is not a valid PNG")
    width, height, bit_depth, color_type = struct.unpack(">IIBB", data[16:26])
    if (width, height) != (3000, 3000):
        fail(f"show artwork must be 3000 x 3000, found {width} x {height}")
    if bit_depth != 8 or color_type not in {2, 3}:
        fail("show artwork must be 8-bit RGB or indexed-color PNG without alpha")


def validate_storage_manifest(root: Path, package_root: Path, metadata: dict) -> None:
    manifest_path = package_root / metadata.get("storage_manifest", "")
    inventory_path = package_root / "s3-object-inventory.json"
    runbook_path = root / "S3_CLOUDFRONT_RUNBOOK.md"
    for path in (manifest_path, inventory_path, runbook_path):
        if not path.is_file() or path.stat().st_size == 0:
            fail(f"production storage evidence is missing {path}")

    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    inventory = json.loads(inventory_path.read_text(encoding="utf-8"))
    if manifest.get("publication_status") != "held_for_human_review":
        fail("storage manifest must remain held for human review")
    if manifest.get("bucket") != "agent-logic-podcast-archive-agentlogic":
        fail("storage manifest names an unexpected podcast bucket")
    if manifest.get("archive_object_count") != inventory.get("object_count"):
        fail("storage manifest and S3 inventory object counts differ")
    if manifest.get("archive_total_bytes") != inventory.get("total_bytes"):
        fail("storage manifest and S3 inventory byte totals differ")

    cloudfront = manifest.get("cloudfront") or {}
    if cloudfront.get("origin_path") != "/public" or cloudfront.get("public_object_count") != 0:
        fail("CloudFront must remain bound to an empty public prefix before approval")
    if cloudfront.get("archive_access_probe_status") != 403:
        fail("storage manifest does not prove the private archive boundary")

    local_by_key = {
        "archive/cognitive-spacetime/episodes/001/package/artwork-source.png": package_root / "artwork-source.png",
        "archive/cognitive-spacetime/episodes/001/package/artwork.png": package_root / "artwork.png",
        "archive/cognitive-spacetime/episodes/001/package/script.md": package_root / "script.md",
        "archive/cognitive-spacetime/episodes/001/package/transcript.md": package_root / "transcript.md",
        "archive/cognitive-spacetime/episodes/001/package/audio-manifest.json": package_root / "audio-manifest.json",
        "archive/cognitive-spacetime/episodes/001/media/meet-the-ai-coworkers.mp3": root / "audio" / "meet-the-ai-coworkers.mp3",
        "archive/cognitive-spacetime/episodes/001/media/meet-the-ai-coworkers.wav": root / "audio" / "meet-the-ai-coworkers.wav",
    }
    critical = {entry.get("key"): entry for entry in manifest.get("critical_objects") or []}
    if set(critical) != set(local_by_key):
        fail("storage manifest critical-object set is incomplete")
    for key, local_path in local_by_key.items():
        entry = critical[key]
        if entry.get("bytes") != local_path.stat().st_size:
            fail(f"storage manifest byte count differs for {key}")
        if entry.get("sha256") != hashlib.sha256(local_path.read_bytes()).hexdigest():
            fail(f"storage manifest digest differs for {key}")
        if not entry.get("version_id") or not entry.get("s3_checksum_sha256"):
            fail(f"storage manifest lacks retained S3 identity for {key}")

    runbook = runbook_path.read_text(encoding="utf-8")
    for value in (manifest["bucket"], cloudfront.get("distribution_id"), cloudfront.get("origin_access_control_id")):
        if not value or value not in runbook:
            fail(f"storage runbook does not retain infrastructure identity {value!r}")


def validate_production_episode(root: Path) -> None:
    package_root = root / "episodes" / "001-meet-the-ai-coworkers"
    metadata_path = package_root / "episode.json"
    required = [
        metadata_path,
        package_root / "script.md",
        package_root / "transcript.md",
        package_root / "show-notes.md",
        package_root / "CREATOR_WORKFLOW.md",
        package_root / "audio-manifest.json",
        package_root / "artwork-source.png",
        package_root / "artwork.png",
        root / "artwork.png",
        root / "audio" / "meet-the-ai-coworkers.mp3",
        root / "audio" / "meet-the-ai-coworkers.wav",
    ]
    for path in required:
        if not path.is_file() or path.stat().st_size == 0:
            fail(f"production episode is missing {path}")
    metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
    audio_manifest = json.loads((package_root / "audio-manifest.json").read_text(encoding="utf-8"))
    if metadata.get("publication_status") != "held_for_human_review":
        fail("production episode must remain held for human review")
    mp3 = root / "audio" / "meet-the-ai-coworkers.mp3"
    wav = root / "audio" / "meet-the-ai-coworkers.wav"
    if metadata.get("audio_bytes") != mp3.stat().st_size:
        fail("episode metadata MP3 byte count does not match")
    if metadata.get("archive_audio_bytes") != wav.stat().st_size:
        fail("episode metadata WAV byte count does not match")
    if hashlib.sha256(mp3.read_bytes()).hexdigest() != metadata.get("audio_sha256"):
        fail("episode metadata MP3 digest does not match")
    if hashlib.sha256(wav.read_bytes()).hexdigest() != metadata.get("archive_audio_sha256"):
        fail("episode metadata WAV digest does not match")
    if audio_manifest.get("credential_retention") != "none" or audio_manifest.get("publication_status") != "held_for_human_review":
        fail("audio manifest has invalid credential or publication truth")
    renderers = audio_manifest.get("voice_renderers") or {}
    if renderers.get("Claude", {}).get("voice") != "aura-2-pluto-en" or renderers.get("Claude", {}).get("surrogate") is not True:
        fail("audio manifest does not retain the approved Claude voice boundary")
    if renderers.get("Gemini", {}).get("surrogate") is not True or renderers.get("ChatGPT", {}).get("surrogate") is not False:
        fail("audio manifest does not distinguish native and surrogate rendering")
    outputs = audio_manifest.get("outputs") or {}
    if outputs.get("distribution_mp3", {}).get("sha256") != metadata.get("audio_sha256"):
        fail("audio manifest MP3 digest does not match episode metadata")
    if outputs.get("archive_wav", {}).get("sha256") != metadata.get("archive_audio_sha256"):
        fail("audio manifest WAV digest does not match episode metadata")
    with wave.open(str(wav), "rb") as source:
        duration = source.getnframes() / source.getframerate()
        if duration < 600:
            fail("production episode must contain at least ten minutes of audio")
        if source.getnchannels() != 1 or source.getframerate() != 24000 or source.getsampwidth() != 2:
            fail("production WAV must be 24 kHz mono 16-bit PCM")
    transcript = (package_root / "transcript.md").read_text(encoding="utf-8")
    if transcript.count("## Act ") != 4 or transcript.count("### ChatGPT") != 8 or transcript.count("### Gemini") != 8 or transcript.count("### Claude") != 8:
        fail("production transcript must contain the complete four-act, three-speaker dialogue")
    validate_png_artwork(root / "artwork.png")
    validate_png_artwork(package_root / "artwork.png")
    if hashlib.sha256((root / "artwork.png").read_bytes()).hexdigest() != metadata.get("artwork_sha256"):
        fail("episode metadata artwork digest does not match")
    if hashlib.sha256((package_root / "artwork-source.png").read_bytes()).hexdigest() != metadata.get("artwork_source_sha256"):
        fail("episode metadata artwork-source digest does not match")
    validate_storage_manifest(root, package_root, metadata)


def validate_http_route(http_base: str, route: str, contains: str) -> None:
    url = http_base.rstrip("/") + "/" + route.lstrip("/")
    with urllib.request.urlopen(url, timeout=5) as response:
        status = getattr(response, "status", response.getcode())
        body = response.read().decode("utf-8", errors="replace")
    if status != 200:
        fail(f"HTTP route {url} returned {status}")
    if contains not in body:
        fail(f"HTTP route {url} did not contain expected text {contains!r}")


def main() -> None:
    args = sys.argv[1:]
    if len(args) < 2:
        fail("usage: validate_podcast_launch_packet.py <podcast-root> <episodes-json> [--preview-root <path>] [--http-base <url>]")
    root = Path(args[0])
    episodes_path = Path(args[1])
    preview_root: Path | None = None
    http_base: str | None = None
    index = 2
    while index < len(args):
        if args[index] == "--preview-root" and index + 1 < len(args):
            preview_root = Path(args[index + 1])
            index += 2
        elif args[index] == "--http-base" and index + 1 < len(args):
            http_base = args[index + 1]
            index += 2
        else:
            fail(f"unknown argument: {args[index]}")
    packet = json.loads(episodes_path.read_text(encoding="utf-8"))
    episodes = packet.get("episodes") or []
    if len(episodes) != 10:
        fail("expected exactly 10 episode records")

    production = (root / "audio" / "meet-the-ai-coworkers.mp3").is_file()
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
    if production:
        validate_production_episode(root)

    for html_path in [root / "index.html", root / "episodes" / "meet-the-ai-coworkers" / "index.html"]:
        validate_html_public_text(html_path, require_audio=True)
    if preview_root is not None:
        validate_html_public_text(preview_root / "index.html", require_audio=True)

    studio_html = root / "studio" / STUDIO_HTML
    studio_text = studio_html.read_text(encoding="utf-8")
    if "Cognitive Spacetime" not in studio_text or "{{ latest.title }}" not in studio_text:
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

    validate_feed(root)
    if http_base is not None:
        validate_http_route(http_base, "/podcast/", "Cognitive Spacetime Podcast")
        validate_http_route(http_base, "/podcast/feed.xml", "Cognitive Spacetime Podcast")
        validate_http_route(http_base, "/podcast/studio/podcast-studio.html", "Cognitive Spacetime Podcast")
        validate_http_route(http_base, "/_preview/podcast/", "Cognitive Spacetime Podcast")

    print("podcast_launch_packet: PASS")


if __name__ == "__main__":
    main()
