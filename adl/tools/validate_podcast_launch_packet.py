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
    "live, unscripted",
    "no human-written dialogue",
    "generated live",
    "zero rehearsal",
    "never for content",
]

STUDIO_HTML = "podcast-studio.html"
ITUNES_NS = "http://www.itunes.com/dtds/podcast-1.0.dtd"
SHOW_TITLE = "The Cognitive Stack"


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
    if SHOW_TITLE not in text:
        fail(f"public page does not use the approved show identity: {html_path}")
    validate_local_references(html_path)


def validate_feed(root: Path) -> None:
    feed = ET.parse(root / "feed.xml").getroot()
    channel = feed.find("./channel")
    if channel is None:
        fail("RSS feed is missing channel")
    title = channel.findtext("title", "")
    link = channel.findtext("link", "")
    if title != SHOW_TITLE:
        fail("RSS feed does not use the approved show identity")
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


def synchsafe(value: bytes) -> int:
    if len(value) != 4 or any(byte & 0x80 for byte in value):
        fail("MP3 contains an invalid synchsafe ID3 size")
    return (value[0] << 21) | (value[1] << 14) | (value[2] << 7) | value[3]


def decode_id3_text(payload: bytes) -> str:
    if not payload:
        return ""
    encodings = {0: "latin-1", 1: "utf-16", 2: "utf-16-be", 3: "utf-8"}
    encoding = encodings.get(payload[0])
    if encoding is None:
        fail(f"MP3 contains unsupported ID3 text encoding {payload[0]}")
    return payload[1:].decode(encoding).rstrip("\x00")


def validate_mp3_id3(mp3: Path, metadata: dict) -> None:
    data = mp3.read_bytes()
    if len(data) < 10 or data[:3] != b"ID3" or data[3] not in {3, 4}:
        fail("distribution MP3 must contain an ID3v2.3 or ID3v2.4 tag")
    version = data[3]
    tag_end = min(len(data), 10 + synchsafe(data[6:10]))
    offset = 10
    frames: dict[str, bytes] = {}
    while offset + 10 <= tag_end:
        frame_id = data[offset : offset + 4].decode("ascii", errors="ignore")
        if not frame_id.strip("\x00"):
            break
        raw_size = data[offset + 4 : offset + 8]
        size = int.from_bytes(raw_size, "big") if version == 3 else synchsafe(raw_size)
        offset += 10
        if size <= 0 or offset + size > tag_end:
            fail(f"MP3 contains invalid ID3 frame size for {frame_id}")
        frames[frame_id] = data[offset : offset + size]
        offset += size

    expected = {
        "TIT2": "Meet the AI Coworkers",
        "TPE1": "The Cognitive Stack",
        "TALB": "The Cognitive Stack",
        "TPE2": "Agent Logic",
        "TRCK": "1",
    }
    for frame_id, value in expected.items():
        if decode_id3_text(frames.get(frame_id, b"")) != value:
            fail(f"MP3 ID3 {frame_id} does not match {value!r}")
    year = decode_id3_text(frames.get("TYER", frames.get("TDRC", b"")))
    if year != "2026":
        fail("MP3 ID3 year does not match 2026")

    apic = frames.get("APIC")
    if not apic:
        fail("MP3 ID3 tag is missing embedded artwork")
    encoding = apic[0]
    mime_end = apic.find(b"\x00", 1)
    if mime_end < 0 or mime_end + 2 > len(apic):
        fail("MP3 ID3 APIC frame is malformed")
    mime_type = apic[1:mime_end].decode("ascii", errors="strict")
    description_start = mime_end + 2
    terminator = b"\x00\x00" if encoding in {1, 2} else b"\x00"
    description_end = apic.find(terminator, description_start)
    if description_end < 0:
        fail("MP3 ID3 APIC description is malformed")
    image = apic[description_end + len(terminator) :]
    if mime_type != metadata.get("embedded_artwork_type"):
        fail("MP3 embedded artwork MIME type does not match episode metadata")
    if len(image) != metadata.get("embedded_artwork_bytes"):
        fail("MP3 embedded artwork byte count does not match episode metadata")
    if hashlib.sha256(image).hexdigest() != metadata.get("embedded_artwork_sha256"):
        fail("MP3 embedded artwork digest does not match episode metadata")


def validate_guest_packet(packet: dict) -> None:
    if packet.get("external_guests") != [] or packet.get("guest_acceptance_claimed") is not False:
        fail("guest metadata must not claim an external guest or guest acceptance")
    if packet.get("human_guest_consent_required") is not False:
        fail("guest metadata has an invalid consent boundary for an episode with no human guests")
    hosts = {host.get("name"): host for host in packet.get("regular_model_hosts") or []}
    if set(hosts) != {"ChatGPT", "Gemini", "Claude"}:
        fail("guest metadata must identify all three regular model hosts")
    if hosts["ChatGPT"].get("surrogate_voice") is not False or hosts["Gemini"].get("surrogate_voice") is not True or hosts["Claude"].get("surrogate_voice") is not True:
        fail("guest metadata does not retain native and surrogate voice truth")


def validate_enclosure_packet(packet: dict, metadata: dict) -> None:
    expected = {
        "guid": metadata.get("guid"),
        "url": metadata.get("audio_url"),
        "mime_type": metadata.get("audio_type"),
        "bytes": metadata.get("audio_bytes"),
        "duration": metadata.get("audio_duration"),
        "duration_seconds": metadata.get("audio_duration_seconds"),
        "sha256": metadata.get("audio_sha256"),
        "publication_status": "held_for_human_review",
    }
    for field, value in expected.items():
        if packet.get(field) != value:
            fail(f"RSS enclosure packet {field} does not match episode metadata")


def resolve_package_child(package_root: Path, field: str, relative: object) -> Path:
    if not isinstance(relative, str) or not relative:
        fail(f"episode metadata is missing required package path {field}")
    if Path(relative).name != relative:
        fail(f"episode metadata {field} must name a direct package child")
    path = package_root / relative
    if path.is_symlink() or path.resolve().parent != package_root.resolve():
        fail(f"episode metadata {field} escapes the episode package")
    return path


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
        "archive/the-cognitive-stack/episodes/001/package/CREATOR_WORKFLOW.md": package_root / "CREATOR_WORKFLOW.md",
        "archive/the-cognitive-stack/episodes/001/package/artwork-source.png": package_root / "artwork-source.png",
        "archive/the-cognitive-stack/episodes/001/package/artwork.png": package_root / "artwork.png",
        "archive/the-cognitive-stack/episodes/001/package/episode.json": package_root / "episode.json",
        "archive/the-cognitive-stack/episodes/001/package/source-packet.md": package_root / "source-packet.md",
        "archive/the-cognitive-stack/episodes/001/package/script.md": package_root / "script.md",
        "archive/the-cognitive-stack/episodes/001/package/transcript.md": package_root / "transcript.md",
        "archive/the-cognitive-stack/episodes/001/package/show-notes.md": package_root / "show-notes.md",
        "archive/the-cognitive-stack/episodes/001/package/audio-manifest.json": package_root / "audio-manifest.json",
        "archive/the-cognitive-stack/episodes/001/package/qa-report.md": package_root / "qa-report.md",
        "archive/the-cognitive-stack/episodes/001/package/guest-metadata.json": package_root / "guest-metadata.json",
        "archive/the-cognitive-stack/episodes/001/package/rss-enclosure.json": package_root / "rss-enclosure.json",
        "archive/the-cognitive-stack/episodes/001/package/redaction-report.md": package_root / "redaction-report.md",
        "archive/the-cognitive-stack/episodes/001/package/review.md": package_root / "review.md",
        "archive/the-cognitive-stack/episodes/001/media/meet-the-ai-coworkers.mp3": root / "audio" / "meet-the-ai-coworkers.mp3",
        "archive/the-cognitive-stack/episodes/001/media/meet-the-ai-coworkers.wav": root / "audio" / "meet-the-ai-coworkers.wav",
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
    required_metadata_paths = [
        "source_packet",
        "qa_report",
        "guest_metadata",
        "rss_enclosure",
        "redaction_report",
        "review",
    ]
    for path in required:
        if not path.is_file() or path.stat().st_size == 0:
            fail(f"production episode is missing {path}")
    metadata = json.loads(metadata_path.read_text(encoding="utf-8"))
    if metadata.get("show_title") != SHOW_TITLE:
        fail("episode metadata does not use the approved show identity")
    for field in required_metadata_paths:
        relative = metadata.get(field)
        path = resolve_package_child(package_root, field, relative)
        if not path.is_file() or path.stat().st_size == 0:
            fail(f"production episode is missing declared {field}: {path}")
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

    guest_metadata = json.loads((package_root / metadata["guest_metadata"]).read_text(encoding="utf-8"))
    validate_guest_packet(guest_metadata)

    enclosure = json.loads((package_root / metadata["rss_enclosure"]).read_text(encoding="utf-8"))
    validate_enclosure_packet(enclosure, metadata)
    feed_item = ET.parse(root / "feed.xml").getroot().find("./channel/item")
    if feed_item is None:
        fail("production RSS feed is missing its episode item")
    feed_enclosure = feed_item.find("enclosure")
    if feed_item.findtext("guid", "") != enclosure["guid"] or feed_enclosure is None:
        fail("production RSS item does not match enclosure packet identity")
    feed_values = {
        "url": feed_enclosure.attrib.get("url"),
        "mime_type": feed_enclosure.attrib.get("type"),
        "bytes": int(feed_enclosure.attrib.get("length", "0")),
        "duration": feed_item.findtext(f"{{{ITUNES_NS}}}duration", ""),
    }
    for field, value in feed_values.items():
        if value != enclosure[field]:
            fail(f"production RSS item {field} does not match enclosure packet")

    source_packet = (package_root / metadata["source_packet"]).read_text(encoding="utf-8")
    for marker in (
        "not recorded live",
        "surrogate",
        "Publication and directory submission remain separate human-controlled launch",
        "directory-specific account-side verification mail remains part",
    ):
        if marker not in source_packet:
            fail(f"source packet is missing required provenance boundary {marker!r}")
    redaction = (package_root / metadata["redaction_report"]).read_text(encoding="utf-8")
    for marker in ("Provider credentials retained: none", "External guest acceptance claimed: no", "Publication claimed: no"):
        if marker not in redaction:
            fail(f"redaction report is missing required result {marker!r}")
    review = (package_root / metadata["review"]).read_text(encoding="utf-8")
    if "Status: pass" not in review or "Exact reviewed revision:" not in review or "Reviewer:" not in review:
        fail("episode review record is not an exact-head passing review")

    qa_report = (package_root / metadata["qa_report"]).read_text(encoding="utf-8")
    for marker in (
        metadata["audio_sha256"],
        metadata["archive_audio_sha256"],
        "ID3 version: 2.3",
        "Integrated loudness: -15.9 LUFS",
        "True peak: -1.4 dBTP",
    ):
        if marker not in qa_report:
            fail(f"QA report is missing exact media proof {marker!r}")
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
    validate_mp3_id3(mp3, metadata)
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
    if SHOW_TITLE not in studio_text or "{{ latest.title }}" not in studio_text:
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
        validate_http_route(http_base, "/podcast/", SHOW_TITLE)
        validate_http_route(http_base, "/podcast/feed.xml", SHOW_TITLE)
        validate_http_route(http_base, "/podcast/studio/podcast-studio.html", SHOW_TITLE)
        validate_http_route(http_base, "/_preview/podcast/", SHOW_TITLE)

    print("podcast_launch_packet: PASS")


if __name__ == "__main__":
    main()
