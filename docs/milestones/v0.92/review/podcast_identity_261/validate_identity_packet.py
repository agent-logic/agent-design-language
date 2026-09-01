#!/usr/bin/env python3
"""Deterministic validator for the #261 podcast identity packet."""

from __future__ import annotations

import argparse
import datetime
import hashlib
import json
import pathlib
import re
import struct
import subprocess
import sys


ROOT = pathlib.Path(__file__).resolve().parents[5]
PACKET = pathlib.Path(__file__).resolve().parent
HEX64 = re.compile(r"^[0-9a-f]{64}$")
HEX40 = re.compile(r"^[0-9a-f]{40}$")
RFC3339_UTC = re.compile(
    r"^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}(?:\.[0-9]{1,6})?Z$"
)
PROHIBITED = re.compile(
    r"(?i)(bearer\s+[a-z0-9._-]{16,}|"
    r"(?:password|api[_ -]?key|oauth[_ -]?token|recovery[_ -]?code|"
    r"verification[_ -]?code|session[_ -]?cookie|magic[_ -]?link)"
    r"\s*[:=]\s*[\"']?[^\"'\s,}]{6,}|"
    r"-----BEGIN [A-Z ]*PRIVATE KEY-----)"
)
BASE = "44ec88dbafb405edf28ba41275149c54d03f83ab"
RIGHTS_NOTE = "Repository history proves retained bytes and technical derivation, but does not independently prove creation ownership or publication license. Operator confirmation is required."
MAILBOX_REDACTION = "No mailbox credential, token, recovery code, verification code, message body, header, sender address, magic link, screenshot, or original message is retained."
RETAINED_SOURCE_PATH = "demos/podcast/episodes/001-meet-the-ai-coworkers/artwork-source.png"
EXPECTED_PROVENANCE = {
    "repository_statement": "Episode 001 records operator-selected artwork and a proportional 3000 x 3000 technical derivative.",
    "source_packet": "demos/podcast/episodes/001-meet-the-ai-coworkers/source-packet.md",
    "runbook": "demos/podcast/S3_CLOUDFRONT_RUNBOOK.md",
    "first_distribution_commit": "4e18ec45fa0a812bbc9e25f5172b69dfeeb94bba",
    "first_source_commit": "fe6f4cae4602cda099c063c4b52633425b90fc2f",
}
ALLOWED_PATHS = {
    "demos/podcast/artwork.png",
    ".csdlc/locks/261.lock",
}
ALLOWED_PREFIXES = (
    ".csdlc/issues/261/",
    ".csdlc/prepared/issues/261/",
    ".csdlc/evidence/261/",
    ".csdlc/evidence/.csdlc-finalize-261-",
    "docs/milestones/v0.92/review/podcast_identity_261/",
)


def fail(reason: str) -> None:
    print(json.dumps({"schema": "agent_logic.podcast.identity_validation.v1", "status": "failed", "reason": reason}, sort_keys=True))
    raise SystemExit(1)


def load(name: str) -> dict:
    path = PACKET / name
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        fail(f"invalid {name}: {exc}")
    if not isinstance(value, dict):
        fail(f"{name} must contain one JSON object")
    return value


def exact_keys(value: dict, expected: set[str], label: str) -> None:
    if not isinstance(value, dict):
        fail(f"{label} must be an object")
    actual = set(value)
    if actual != expected:
        fail(f"{label} schema keys mismatch: missing={sorted(expected - actual)} unknown={sorted(actual - expected)}")


def git_lines(*argv: str) -> set[str]:
    result = subprocess.run(
        ["git", *argv], cwd=ROOT, text=True, stdout=subprocess.PIPE,
        stderr=subprocess.PIPE, check=False,
    )
    if result.returncode:
        fail(f"git {' '.join(argv)} failed: {result.stderr.strip()}")
    return {line for line in result.stdout.splitlines() if line}


def validate_scope() -> list[str]:
    paths = set()
    paths |= git_lines("diff", "--name-only", f"{BASE}...HEAD")
    paths |= git_lines("diff", "--name-only")
    paths |= git_lines("diff", "--cached", "--name-only")
    paths |= git_lines("ls-files", "--others", "--exclude-standard")
    disallowed = sorted(
        path for path in paths
        if path not in ALLOWED_PATHS and not path.startswith(ALLOWED_PREFIXES)
    )
    if disallowed:
        fail(f"actual git scope contains undeclared paths: {disallowed}")
    return sorted(paths)


def require_hex64(value: object, label: str) -> None:
    if not isinstance(value, str) or not HEX64.fullmatch(value):
        fail(f"{label} must be a lowercase SHA-256 digest")


def require_git_path(commit: str, path: str, label: str) -> None:
    result = subprocess.run(
        ["git", "cat-file", "-e", f"{commit}:{path}"], cwd=ROOT,
        stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, check=False,
    )
    if result.returncode:
        fail(f"{label} does not resolve the declared repository path")


def require_nonempty(value: object, label: str) -> None:
    if not isinstance(value, str) or not value.strip():
        fail(f"{label} must be a non-empty string")


def require_bounded(value: object, label: str, maximum: int, pattern: str | None = None) -> None:
    require_nonempty(value, label)
    assert isinstance(value, str)
    if len(value) > maximum or (pattern is not None and re.fullmatch(pattern, value) is None):
        fail(f"{label} has invalid grammar or length")


def utc_timestamp(value: object, label: str) -> datetime.datetime:
    require_nonempty(value, label)
    assert isinstance(value, str)
    if RFC3339_UTC.fullmatch(value) is None:
        fail(f"{label} must use exact RFC3339 full-date T full-time UTC Z form")
    try:
        parsed = datetime.datetime.fromisoformat(value[:-1] + "+00:00")
    except ValueError:
        fail(f"{label} is not RFC3339")
    if parsed.utcoffset() != datetime.timedelta(0):
        fail(f"{label} is not UTC")
    return parsed


def png_properties(path: pathlib.Path) -> tuple[int, int, int, str]:
    data = path.read_bytes()
    if data[:8] != b"\x89PNG\r\n\x1a\n" or data[12:16] != b"IHDR":
        fail(f"not a PNG with IHDR: {path.relative_to(ROOT)}")
    width, height, bit_depth, color_type = struct.unpack(">IIBB", data[16:26])
    if color_type != 2:
        fail(f"artwork is not RGB: {path.relative_to(ROOT)}")
    return width, height, bit_depth, "RGB"


parser = argparse.ArgumentParser()
parser.add_argument("--release", action="store_true")
parser.add_argument("--redaction-only", action="store_true")
args = parser.parse_args()

all_text = "\n".join(
    path.read_text(encoding="utf-8")
    for path in sorted(PACKET.iterdir())
    if path.is_file() and path.suffix in {".json", ".md", ".py"} and path.name != pathlib.Path(__file__).name
)
for match in PROHIBITED.finditer(all_text):
    fail(f"prohibited secret/private-mailbox term found: {match.group(1)}")

identity = load("show-identity.json")
rights = load("artwork-rights.json")
mailbox = load("mailbox-readiness.json")
name_decision = load("name-decision.json")

exact_keys(identity, {"schema", "version", "approval_status", "show", "artwork", "decision_records", "ownership", "publication_claimed"}, "show identity")
exact_keys(identity["show"], {"title", "subtitle", "description", "author", "category", "language", "explicit", "copyright", "cadence", "website_url", "public_contact"}, "show")
exact_keys(identity["artwork"], {"path", "bytes", "sha256", "format", "width", "height", "bit_depth", "color_space", "rights_record"}, "artwork")
exact_keys(identity["decision_records"], {"operator_decision", "name_conflict_review", "mailbox_readiness"}, "decision records")
exact_keys(identity["ownership"], {"issue_261", "issue_342", "issue_262"}, "ownership")
exact_keys(rights, {"schema", "status", "distribution_artwork", "retained_source", "observed_provenance", "rights_basis", "license_identifier", "creator_or_source_owner", "operator_confirmation", "operator_confirmation_sha256", "operator_confirmation_timestamp_utc", "publication_authorized", "note"}, "artwork rights")
exact_keys(rights["distribution_artwork"], {"path", "bytes", "sha256", "format", "width", "height", "bit_depth", "color_space"}, "rights distribution artwork")
exact_keys(rights["retained_source"], {"path", "bytes", "sha256", "format", "width", "height", "bit_depth", "color_space"}, "rights retained source")
exact_keys(rights["observed_provenance"], {"repository_statement", "source_packet", "runbook", "first_distribution_commit", "first_source_commit"}, "observed provenance")
exact_keys(mailbox, {"schema", "status", "mailbox", "control_class", "control_evidence_sha256", "test_timestamp_utc", "sender_class", "receive_outcome", "provider_class", "source_evidence_sha256", "redaction_statement", "operator_retention_approval", "publication_authorized"}, "mailbox readiness")
exact_keys(name_decision, {"schema", "version", "status", "candidate_title", "decision", "approved_title", "decided_at_utc", "operator_confirmation_sha256", "research_record", "research_sha256"}, "name decision")

if identity["schema"] != "agent_logic.podcast.show_identity.v1" or identity["version"] != "v0.92.1-261-candidate.1":
    fail("show identity schema/version mismatch")
if rights["schema"] != "agent_logic.podcast.artwork_rights.v1":
    fail("artwork rights schema mismatch")
if mailbox["schema"] != "agent_logic.podcast.mailbox_readiness.v1":
    fail("mailbox readiness schema mismatch")
if name_decision["schema"] != "agent_logic.podcast.name_decision.v1" or name_decision["version"] != identity["version"]:
    fail("name decision schema/version mismatch")
for label, value in (("explicit", identity["show"]["explicit"]), ("publication claimed", identity["publication_claimed"]), ("rights publication authorized", rights["publication_authorized"]), ("mailbox publication authorized", mailbox["publication_authorized"])):
    if not isinstance(value, bool):
        fail(f"{label} must be boolean")
for label, value in (("artwork bytes", identity["artwork"]["bytes"]), ("artwork width", identity["artwork"]["width"]), ("artwork height", identity["artwork"]["height"]), ("artwork bit depth", identity["artwork"]["bit_depth"]), ("rights artwork bytes", rights["distribution_artwork"]["bytes"]), ("rights artwork width", rights["distribution_artwork"]["width"]), ("rights artwork height", rights["distribution_artwork"]["height"]), ("rights artwork bit depth", rights["distribution_artwork"]["bit_depth"]), ("source bytes", rights["retained_source"]["bytes"]), ("source width", rights["retained_source"]["width"]), ("source height", rights["retained_source"]["height"]), ("source bit depth", rights["retained_source"]["bit_depth"])):
    if not isinstance(value, int) or isinstance(value, bool) or value < 0:
        fail(f"{label} must be a nonnegative integer")
if identity["artwork"]["format"] != "PNG" or identity["artwork"]["rights_record"] != "docs/milestones/v0.92/review/podcast_identity_261/artwork-rights.json":
    fail("artwork format/rights record path mismatch")
if rights["distribution_artwork"]["format"] != "PNG" or rights["retained_source"]["format"] != "PNG":
    fail("rights artwork formats must be PNG")
for label, value in (
    ("artwork path", identity["artwork"]["path"]), ("artwork sha256", identity["artwork"]["sha256"]),
    ("artwork color space", identity["artwork"]["color_space"]),
    ("rights artwork path", rights["distribution_artwork"]["path"]), ("rights artwork sha256", rights["distribution_artwork"]["sha256"]),
    ("rights artwork color space", rights["distribution_artwork"]["color_space"]),
    ("source path", rights["retained_source"]["path"]), ("source sha256", rights["retained_source"]["sha256"]),
    ("source color space", rights["retained_source"]["color_space"]),
):
    require_bounded(value, label, 256)
if rights["observed_provenance"] != EXPECTED_PROVENANCE:
    fail("observed provenance does not match the exact declared source paths and commits")
for key in ("first_distribution_commit", "first_source_commit"):
    if HEX40.fullmatch(rights["observed_provenance"][key]) is None:
        fail(f"observed provenance {key} must be a full lowercase Git object id")
for key in ("source_packet", "runbook"):
    provenance_path = ROOT / rights["observed_provenance"][key]
    if not provenance_path.is_file():
        fail(f"observed provenance {key} path is missing")
require_git_path(
    rights["observed_provenance"]["first_distribution_commit"],
    "demos/podcast/artwork.png",
    "first distribution commit",
)
require_git_path(
    rights["observed_provenance"]["first_source_commit"],
    RETAINED_SOURCE_PATH,
    "first source commit",
)
if identity["approval_status"] not in {"pending_operator_decision", "operator_approved"}:
    fail("invalid identity approval status")
if rights["status"] not in {"pending_operator_rights_confirmation", "operator_confirmed"}:
    fail("invalid artwork rights status")
if mailbox["status"] not in {"pending_external_verification", "verified_received"}:
    fail("invalid mailbox readiness status")
expected_records = {
    "operator_decision": "docs/milestones/v0.92/review/podcast_identity_261/name-decision.json",
    "name_conflict_review": "docs/milestones/v0.92/review/podcast_identity_261/name-conflict-review.md",
    "mailbox_readiness": "docs/milestones/v0.92/review/podcast_identity_261/mailbox-readiness.json",
}
if identity["decision_records"] != expected_records:
    fail("decision record paths mismatch")
if name_decision["research_record"] != expected_records["name_conflict_review"]:
    fail("name decision research record path mismatch")
research_bytes = (ROOT / name_decision["research_record"]).read_bytes()
if hashlib.sha256(research_bytes).hexdigest() != name_decision["research_sha256"]:
    fail("name decision research digest mismatch")
if name_decision["candidate_title"] != identity["show"]["title"] or name_decision["status"] != identity["approval_status"]:
    fail("name decision does not bind identity candidate/status")
if rights["note"] != RIGHTS_NOTE or mailbox["redaction_statement"] != MAILBOX_REDACTION:
    fail("bounded rights note or mailbox redaction statement mismatch")
for markdown_name in ("README.md", "name-conflict-review.md"):
    markdown = (PACKET / markdown_name).read_text(encoding="utf-8")
    if len(markdown) > 12000 or "\x00" in markdown:
        fail(f"{markdown_name} exceeds bounded public-text contract")
for key in ("title", "subtitle", "description", "author", "category", "language", "copyright", "cadence", "website_url", "public_contact"):
    require_bounded(identity["show"][key], f"show {key}", 500 if key == "description" else 160)
if identity["show"]["public_contact"] != "podcast@agent-logic.ai":
    fail("public contact mismatch")

actual_scope = validate_scope()

if identity["approval_status"] == "pending_operator_decision":
    for key in ("decision", "approved_title", "decided_at_utc", "operator_confirmation_sha256"):
        if name_decision[key] is not None:
            fail(f"pending name decision must retain null {key}")
if rights["status"] == "pending_operator_rights_confirmation":
    if rights["rights_basis"] != "pending_operator_confirmation" or rights["publication_authorized"] is not False:
        fail("pending rights classification/authority mismatch")
    for key in ("license_identifier", "creator_or_source_owner", "operator_confirmation", "operator_confirmation_sha256", "operator_confirmation_timestamp_utc"):
        if rights[key] is not None:
            fail(f"pending rights must retain null {key}")
if mailbox["status"] == "pending_external_verification":
    if mailbox["control_class"] != "company_controlled_claim_pending_receive_proof" or mailbox["publication_authorized"] is not False:
        fail("pending mailbox control/authority mismatch")
    for key in ("control_evidence_sha256", "test_timestamp_utc", "sender_class", "receive_outcome", "provider_class", "source_evidence_sha256", "operator_retention_approval"):
        if mailbox[key] is not None:
            fail(f"pending mailbox must retain null {key}")

if args.redaction_only:
    # Exact schemas above reject hidden/unknown retained content. Pending fields must
    # remain exactly null; release-mode values are constrained below.
    print(json.dumps({"schema": "agent_logic.podcast.identity_validation.v1", "status": "passed", "mode": "redaction-only", "actual_scope": actual_scope}, sort_keys=True))
    raise SystemExit(0)

art = identity.get("artwork", {})
art_path = ROOT / str(art.get("path", ""))
if not art_path.is_file():
    fail("distribution artwork is missing")
data = art_path.read_bytes()
digest = hashlib.sha256(data).hexdigest()
width, height, bit_depth, color_space = png_properties(art_path)
expected = (art.get("bytes"), art.get("sha256"), art.get("width"), art.get("height"), art.get("bit_depth"), art.get("color_space"))
observed = (len(data), digest, width, height, bit_depth, color_space)
if expected != observed:
    fail(f"distribution artwork metadata mismatch: expected={expected!r} observed={observed!r}")
if (width, height, bit_depth, color_space) != (3000, 3000, 8, "RGB"):
    fail("distribution artwork does not satisfy 3000x3000 RGB PNG contract")

rights_art = rights.get("distribution_artwork", {})
if tuple(rights_art.get(key) for key in ("path", "bytes", "sha256", "format", "width", "height", "bit_depth", "color_space")) != (
    art.get("path"), len(data), digest, art.get("format"), width, height, bit_depth, color_space
):
    fail("rights record does not bind exact distribution artwork")
source = rights.get("retained_source", {})
if source.get("path") != RETAINED_SOURCE_PATH:
    fail("retained source path is not the exact declared artwork source")
source_path = ROOT / str(source.get("path", ""))
if not source_path.is_file():
    fail("retained source artwork is missing")
source_data = source_path.read_bytes()
source_props = png_properties(source_path)
if source.get("bytes") != len(source_data) or source.get("sha256") != hashlib.sha256(source_data).hexdigest():
    fail("retained source artwork digest/byte count mismatch")
if tuple(source.get(key) for key in ("width", "height", "bit_depth", "color_space")) != source_props:
    fail("retained source artwork properties mismatch")

ownership = identity.get("ownership", {})
if ownership.get("issue_261") != ["demos/podcast/artwork.png", "docs/milestones/v0.92/review/podcast_identity_261/**"]:
    fail("#261 ownership allocation drift")
if ownership.get("issue_342") != ["demos/podcast/episode-packages/**", "demos/podcast/episode-packages/feed-fragment.xml"]:
    fail("#342 ownership allocation drift")
if ownership.get("issue_262") != [
    "Production feed at https://agent-logic.ai/podcast/feed.xml using the approved 51.a identity packet.",
    "Stable HTTPS episode enclosure URLs from terminal review-ready episode packages.",
    "RSS 2.0 and required podcast metadata validation.",
    "Exact MIME type, byte length, duration, GUID, publication date, artwork, and digest reconciliation.",
    "HEAD/GET and 206 Partial Content proof for audio.",
    "Representative desktop and mobile subscription, metadata, and playback proof.",
    "Hosting/feed correction and rollback limited to this owned publication surface.",
]:
    fail("#262 production feed ownership drift")
if identity.get("publication_claimed") is not False:
    fail("packet overclaims publication")
for label, field in (("artwork sha256", art.get("sha256")), ("source sha256", source.get("sha256"))):
    require_hex64(field, label)

pending = {
    "identity": identity.get("approval_status"),
    "rights": rights.get("status"),
    "mailbox": mailbox.get("status"),
}
if args.release:
    required = {
        "identity": "operator_approved",
        "rights": "operator_confirmed",
        "mailbox": "verified_received",
    }
    if pending != required:
        fail(f"release gates not satisfied: {pending}")
    if name_decision["decision"] != "approved" or name_decision["approved_title"] != identity["show"]["title"]:
        fail("operator title decision does not approve the exact candidate title")
    title_time = utc_timestamp(name_decision["decided_at_utc"], "title decision timestamp")
    require_hex64(name_decision["operator_confirmation_sha256"], "title confirmation digest")
    if rights["rights_basis"] not in {"owned_original", "licensed", "commissioned_work_for_hire"}:
        fail("rights basis remains pending")
    require_bounded(rights["license_identifier"], "license identifier", 120, r"[A-Za-z0-9][A-Za-z0-9._:/ -]{0,119}")
    if rights["creator_or_source_owner"] not in {"agent_logic_inc", "named_creator_retained_privately"}:
        fail("creator/source owner must use an allowed redacted classification")
    if rights["operator_confirmation"] != "operator_confirmed_rights_basis":
        fail("rights operator confirmation classification mismatch")
    require_hex64(rights["operator_confirmation_sha256"], "rights confirmation digest")
    rights_time = utc_timestamp(rights["operator_confirmation_timestamp_utc"], "rights confirmation timestamp")
    if rights["publication_authorized"] is not True:
        fail("rights publication authority missing")
    if mailbox["mailbox"] != identity["show"]["public_contact"]:
        fail("mailbox proof does not bind the exact public contact")
    if mailbox["control_class"] != "company_controlled_verified":
        fail("mailbox control class is not verified")
    require_hex64(mailbox["control_evidence_sha256"], "mailbox control evidence digest")
    mailbox_time = utc_timestamp(mailbox["test_timestamp_utc"], "mailbox test timestamp")
    if mailbox["sender_class"] not in {"operator_external_account", "provider_verification_service"}:
        fail("mailbox sender class is not an allowed redacted classification")
    if mailbox["receive_outcome"] != "received":
        fail("mailbox receive outcome is not received")
    if mailbox["provider_class"] not in {"company_mail_provider", "domain_verification_provider"}:
        fail("mailbox provider class is not an allowed redacted classification")
    require_hex64(mailbox["source_evidence_sha256"], "mailbox source evidence digest")
    if mailbox["operator_retention_approval"] is not True or mailbox["publication_authorized"] is not True:
        fail("mailbox operator retention/publication authority missing")
    if rights_time < title_time or mailbox_time < title_time:
        fail("rights/mailbox evidence predates the operator title decision")

print(
    json.dumps(
        {
            "schema": "agent_logic.podcast.identity_validation.v1",
            "status": "passed",
            "mode": "release" if args.release else "candidate",
            "artwork_sha256": digest,
            "artwork": {"width": width, "height": height, "bit_depth": bit_depth, "color_space": color_space},
            "external_gates": pending,
            "publication_claimed": False,
            "actual_scope": actual_scope,
        },
        sort_keys=True,
    )
)
