#!/usr/bin/env python3
"""Read-only validator for the retained #868 generation-9 fixture."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import struct
import subprocess
import sys
from pathlib import Path
from typing import Any


SCHEMA = "csdlc.v3.issue872_generation9_fixture.v1"
EXPECTED = {
    "issue": 868,
    "phase": "bound",
    "generation": 9,
    "repository": "agent-logic/agent-design-language",
    "branch": "codex/868-v0922-installed-command-contract",
    "template_registry_version": "1.0.5",
    "lifecycle_digest": "db09a36738970942ae88401ee79508503967cbc39ae6d03e5d57f2dbd26928c8",
    "file_count": 14,
}
# Pins the reviewed normalization map as well as portable bytes; hashes in an
# untrusted replacement manifest cannot authorize a new fixture.
MANIFEST_SHA256 = "438f0dafbb2a4b4b674e770bea1a748dadafbbf0d1f4d78c3ee495e7ddf07a52"
SOURCE_INDEX_SHA256 = "9a3d2206485a06c04fb8926b58b61e089e8de54de937e724af7f7afd291bcbaf"
SOURCE_INVENTORY_SHA256 = "5072f429f2737ac0fd81e990ab748aab894d84f89f263069ad4312e8e676051c"
BINARY_SHA256 = "c1c1a7a9a9928c25139e1d5d12cbbb275f8c595f69b65b2ff8feb43b9299aeaf"
BINARY_SIZE = 10_721_624
SOURCE_REVISION = "6425ba9bbce4cc1f46789c2e3ac019adf86238b8"
LOGICAL_WORKTREE = b"adl://worktree/issue/868"
HISTORICAL_WORKTREE_SHA256 = "ecc55462e0fd847c7367e371ed39c82f0d85eb371fb7e02b4acae68f2a4bc123"
HOST_PATH = re.compile(rb"(?:/(?:Users|Volumes|private|home|tmp)/|[A-Za-z]:[\\/]+(?:Users|Documents and Settings)[\\/]+)", re.IGNORECASE)
CREDENTIAL = re.compile(rb"(?:gh[pousr]_[A-Za-z0-9]{20,}|github_pat_[A-Za-z0-9_]{20,}|Bearer\s+[A-Za-z0-9._~+/=-]{16,})")
HEX256 = re.compile(r"^[0-9a-f]{64}$")
EXPECTED_BLAKE3 = "e7fa9aedc8f992151dcfc82ff9bd0da7f5415d69798ac67956f2977f88416f64"
EXPECTED_TEST = "copied_record_conversion_rehearsal_release_gate_executes_complete_isolated_denominator"

# Minimal unkeyed BLAKE3 implementation used to authenticate retained executable
# bytes without executing them or depending on a machine-local helper.
_IV = (0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
       0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19)
_PERM = (2, 6, 3, 10, 7, 0, 4, 13, 1, 11, 12, 5, 9, 14, 15, 8)
_CHUNK_START, _CHUNK_END, _PARENT, _ROOT = 1, 2, 4, 8


def _rotr32(value: int, shift: int) -> int:
    return ((value >> shift) | (value << (32 - shift))) & 0xFFFFFFFF


def _g(state: list[int], a: int, b: int, c: int, d: int, x: int, y: int) -> None:
    state[a] = (state[a] + state[b] + x) & 0xFFFFFFFF
    state[d] = _rotr32(state[d] ^ state[a], 16)
    state[c] = (state[c] + state[d]) & 0xFFFFFFFF
    state[b] = _rotr32(state[b] ^ state[c], 12)
    state[a] = (state[a] + state[b] + y) & 0xFFFFFFFF
    state[d] = _rotr32(state[d] ^ state[a], 8)
    state[c] = (state[c] + state[d]) & 0xFFFFFFFF
    state[b] = _rotr32(state[b] ^ state[c], 7)


def _compress(cv: tuple[int, ...], words: tuple[int, ...], counter: int,
              block_len: int, flags: int) -> tuple[int, ...]:
    state = list(cv) + list(_IV[:4]) + [counter & 0xFFFFFFFF, counter >> 32, block_len, flags]
    message = list(words)
    for round_number in range(7):
        _g(state, 0, 4, 8, 12, message[0], message[1])
        _g(state, 1, 5, 9, 13, message[2], message[3])
        _g(state, 2, 6, 10, 14, message[4], message[5])
        _g(state, 3, 7, 11, 15, message[6], message[7])
        _g(state, 0, 5, 10, 15, message[8], message[9])
        _g(state, 1, 6, 11, 12, message[10], message[11])
        _g(state, 2, 7, 8, 13, message[12], message[13])
        _g(state, 3, 4, 9, 14, message[14], message[15])
        if round_number != 6:
            message = [message[index] for index in _PERM]
    return tuple(state[i] ^ state[i + 8] for i in range(8)) + tuple(
        state[i + 8] ^ cv[i] for i in range(8))


def _words(block: bytes) -> tuple[int, ...]:
    return struct.unpack("<16I", block.ljust(64, b"\0"))


def _chunk_output(chunk: bytes, counter: int) -> tuple[tuple[int, ...], tuple[int, ...], int, int, int]:
    cv = _IV
    blocks = [chunk[offset:offset + 64] for offset in range(0, len(chunk), 64)] or [b""]
    for index, block in enumerate(blocks[:-1]):
        flags = _CHUNK_START if index == 0 else 0
        cv = _compress(cv, _words(block), counter, len(block), flags)[:8]
    last = blocks[-1]
    flags = _CHUNK_END | (_CHUNK_START if len(blocks) == 1 else 0)
    return cv, _words(last), counter, len(last), flags


def _output_cv(output: tuple[tuple[int, ...], tuple[int, ...], int, int, int]) -> tuple[int, ...]:
    return _compress(*output)[:8]


def _parent_output(left: tuple[int, ...], right: tuple[int, ...]):
    return _IV, tuple(left + right), 0, 64, _PARENT


def blake3_bytes(data: bytes) -> str:
    chunks = [data[offset:offset + 1024] for offset in range(0, len(data), 1024)] or [b""]
    stack: list[tuple[int, ...]] = []
    for index, chunk in enumerate(chunks[:-1]):
        cv = _output_cv(_chunk_output(chunk, index))
        total = index + 1
        while total & 1 == 0:
            cv = _output_cv(_parent_output(stack.pop(), cv))
            total >>= 1
        stack.append(cv)
    output = _chunk_output(chunks[-1], len(chunks) - 1)
    while stack:
        output = _parent_output(stack.pop(), _output_cv(output))
    root_words = _compress(output[0], output[1], 0, output[3], output[4] | _ROOT)
    return b"".join(word.to_bytes(4, "little") for word in root_words).hex()[:64]


def blake3(path: Path) -> str:
    return blake3_bytes(path.read_bytes())


class InvalidFixture(Exception):
    def __init__(self, reason: str, detail: str):
        super().__init__(detail)
        self.reason = reason
        self.detail = detail


def fail(reason: str, detail: str) -> None:
    raise InvalidFixture(reason, detail)


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def load_json(path: Path, reason: str) -> Any:
    reject_symlinks(path)
    if not path.is_file():
        fail(reason, "JSON input must be a regular file")
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        fail(reason, f"cannot read {path}: {exc}")


def safe_relative(value: Any, field: str) -> str:
    if not isinstance(value, str) or not value:
        fail("invalid_manifest_path", f"{field} must be a non-empty string")
    candidate = Path(value)
    if candidate.is_absolute() or value != candidate.as_posix() or not candidate.parts or ".." in candidate.parts or "\\" in value:
        fail("path_escape", f"{field} is not a normalized relative path: {value}")
    return value


def reject_symlinks(path: Path) -> None:
    for component in (path, *path.absolute().parents):
        if component.is_symlink():
            fail("symlink_rejected", "symlink input or ancestor is forbidden")


def regular_files(root: Path) -> dict[str, Path]:
    reject_symlinks(root)
    result: dict[str, Path] = {}
    if root.is_symlink():
        fail("symlink_rejected", f"root is a symlink: {root}")
    for path in root.rglob("*"):
        if path.is_symlink():
            fail("symlink_rejected", f"symlink is forbidden: {path}")
        if path.is_file():
            result[path.relative_to(root).as_posix()] = path
        elif not path.is_dir():
            fail("special_file_rejected", "non-regular fixture entry")
    return result


def registered_worktrees() -> list[Path]:
    try:
        result = subprocess.run(
            ["git", "-C", str(Path(__file__).resolve().parents[2]), "worktree", "list", "--porcelain"], check=True,
            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    except (OSError, subprocess.CalledProcessError):
        fail("worktree_census_unavailable", "cannot establish registered worktree boundaries")
    if not any(line.startswith("worktree ") for line in result.stdout.splitlines()):
        fail("worktree_census_unavailable", "empty worktree census")
    return [Path(line[9:]).resolve() for line in result.stdout.splitlines()
            if line.startswith("worktree ")]


def reject_live_root(path: Path, field: str) -> None:
    reject_symlinks(path)
    resolved = path.resolve()
    text = resolved.as_posix()
    if "/.csdlc/" in text or "/.git/csdlc-v3/local/" in text or "/.adl/bin/" in text:
        fail("live_root_rejected", f"{field} uses a live or shared root")
    for worktree in registered_worktrees():
        try:
            resolved.relative_to(worktree)
        except ValueError:
            continue
        fail("registered_worktree_rejected", f"{field} is beneath registered worktree {worktree.name}")


def validate_manifest(root: Path) -> tuple[dict[str, Any], list[dict[str, Any]]]:
    manifest_path = root / "manifest.json"
    if not manifest_path.is_file():
        fail("manifest_missing", "fixture manifest is missing; direct archive execution is forbidden")
    manifest = load_json(manifest_path, "manifest_invalid")
    if not isinstance(manifest, dict) or manifest.get("schema") != SCHEMA:
        fail("manifest_schema_mismatch", "unexpected fixture manifest schema")
    for key, expected in EXPECTED.items():
        if manifest.get(key) != expected:
            fail("identity_mismatch", f"{key}: expected {expected!r}, got {manifest.get(key)!r}")
    lineage = manifest.get("lineage")
    if not isinstance(lineage, dict):
        fail("lineage_missing", "lineage object is required")
    if lineage.get("source_index_sha256") != SOURCE_INDEX_SHA256:
        fail("source_index_mismatch", "retained source index digest differs")
    if lineage.get("retained_sorted_inventory_sha256") != SOURCE_INVENTORY_SHA256:
        fail("source_inventory_mismatch", "retained source inventory digest differs")
    copies = lineage.get("copies")
    if not isinstance(copies, list) or [entry.get("id") for entry in copies if isinstance(entry, dict)] != ["relocated-original", "verified-copy"]:
        fail("archive_lineage_mismatch", "both ordered retained archive identities are required")
    if lineage.get("archive_id") != "868-20260912-closeout" or lineage.get("copies_byte_identical") is not True:
        fail("archive_lineage_mismatch", "archive identity or byte-equality claim differs")
    logical_sources = [entry.get("logical_source") for entry in copies]
    if logical_sources != [
        "archive:868-20260912-closeout/relocated-original-.csdlc/issues/868",
        "archive:868-20260912-closeout/verified-copy/.csdlc/issues/868",
    ]:
        fail("archive_lineage_mismatch", "logical archive sources differ")
    binary = manifest.get("accepted_binary")
    if not isinstance(binary, dict) or binary.get("sha256") != BINARY_SHA256 or binary.get("size") != BINARY_SIZE or binary.get("source_revision") != SOURCE_REVISION:
        fail("binary_identity_mismatch", "accepted #868 binary identity differs")
    policy = manifest.get("proof_policy")
    if not isinstance(policy, dict) or policy.get("direct_archive_execution") is not False or policy.get("minimum_test_denominator") != 1 or policy.get("marker_only_proof") is not False or policy.get("live_roots_allowed") is not False:
        fail("proof_policy_mismatch", "fail-closed proof policy is incomplete")
    materialization = manifest.get("portable_materialization")
    if not isinstance(materialization, dict) or materialization.get("root") != "portable" or materialization.get("logical_worktree") != LOGICAL_WORKTREE.decode() or materialization.get("authentic_bytes_tracked") is not False or materialization.get("allowed_transformation_kinds") != ["exact_json_string_substitution"]:
        fail("portable_contract_mismatch", "portable materialization contract differs")
    rebinding = materialization.get("execution_rebinding")
    if not isinstance(rebinding, dict) or rebinding != {
            "from": LOGICAL_WORKTREE.decode(),
            "target_class": "disposable_authenticated_worktree",
            "substitution_count": 8,
            "recompute_lifecycle_digest": True,
    }:
        fail("portable_contract_mismatch", "disposable execution rebinding contract differs")
    entries = manifest.get("files")
    if not isinstance(entries, list) or len(entries) != 14:
        fail("file_denominator_mismatch", "manifest must contain exactly 14 files")
    paths = [safe_relative(entry.get("path") if isinstance(entry, dict) else None, "files[].path") for entry in entries]
    if paths != sorted(paths) or len(set(paths)) != 14:
        fail("inventory_order_mismatch", "manifest paths must be unique and sorted")
    canonical = (json.dumps(entries, sort_keys=True, separators=(",", ":")) + "\n").encode()
    if lineage.get("manifest_inventory_sha256") != hashlib.sha256(canonical).hexdigest():
        fail("inventory_mutation", "canonical manifest inventory digest differs")
    authentic_inventory = "".join(f"{entry.get('authentic_sha256')}  ./{entry['path']}\n" for entry in entries).encode()
    if hashlib.sha256(authentic_inventory).hexdigest() != SOURCE_INVENTORY_SHA256:
        fail("source_inventory_mismatch", "manifest authentic inventory does not match retained source")
    if sha256(manifest_path) != MANIFEST_SHA256:
        fail("reviewed_manifest_mismatch", "manifest differs from the reviewed authentic-to-portable mapping")
    return manifest, entries


def validate_materialization(root: Path, entries: list[dict[str, Any]]) -> int:
    portable_root = root / "portable"
    portable = regular_files(portable_root)
    expected = {entry["path"] for entry in entries}
    if set(portable) != expected:
        fail("inventory_coverage_mismatch", f"portable: missing={sorted(expected-set(portable))} extra={sorted(set(portable)-expected)}")
    transformations = 0
    for entry in entries:
        rel = entry["path"]
        materialized = portable[rel]
        payload = materialized.read_bytes()
        if HOST_PATH.search(payload):
            fail("absolute_host_path", rel)
        if CREDENTIAL.search(payload):
            fail("credential_like_content", rel)
        if materialized.stat().st_size != entry.get("portable_size") or sha256(materialized) != entry.get("portable_sha256"):
            fail("portable_hash_mismatch", rel)
        rules = entry.get("transformations")
        if not isinstance(rules, list):
            fail("normalization_map_invalid", rel)
        for rule in rules:
            if not isinstance(rule, dict) or rule.get("kind") != "exact_json_string_substitution" or rule.get("replacement") != LOGICAL_WORKTREE.decode() or rule.get("source_sha256") != HISTORICAL_WORKTREE_SHA256 or rule.get("count") != 1:
                fail("unrecorded_normalization", rel)
            transformations += 1
    index = load_json(portable_root / "index.json", "portable_index_invalid")
    for key, value in EXPECTED.items():
        if key != "file_count" and index.get(key if key != "lifecycle_digest" else "digest") != value:
            fail("portable_identity_mismatch", key)
    if index.get("worktree") != LOGICAL_WORKTREE.decode():
        fail("topology_mismatch", "portable index does not use the declared logical worktree")
    if transformations != 8:
        fail("normalization_count_mismatch", "exactly eight path substitutions are required")
    return transformations


def normalize_json_strings(value: Any, source_sha256: str, replacement: str) -> tuple[Any, int]:
    if isinstance(value, str):
        if hashlib.sha256(value.encode()).hexdigest() == source_sha256:
            return replacement, 1
        return value, 0
    if isinstance(value, list):
        values, count = [], 0
        for item in value:
            normalized, occurrences = normalize_json_strings(item, source_sha256, replacement)
            values.append(normalized)
            count += occurrences
        return values, count
    if isinstance(value, dict):
        result, count = {}, 0
        for key, item in value.items():
            normalized, occurrences = normalize_json_strings(item, source_sha256, replacement)
            result[key] = normalized
            count += occurrences
        return result, count
    return value, 0


def validate_archives(roots: list[Path], fixture_root: Path, entries: list[dict[str, Any]]) -> None:
    if roots and len(roots) != 2:
        fail("archive_denominator_mismatch", "supply both retained archive copies or neither")
    if not roots:
        return
    expected = {entry["path"]: entry for entry in entries}
    for number, root in enumerate(roots):
        resolved = root.resolve()
        allowed_tail = ("relocated-original-.csdlc/issues/868" if number == 0 else "verified-copy/.csdlc/issues/868")
        if not resolved.as_posix().endswith(f"868-20260912-closeout/{allowed_tail}"):
            fail("archive_root_rejected", str(root))
        actual = regular_files(root)
        if set(actual) != set(expected):
            fail("archive_inventory_mismatch", str(root))
        inventory_lines = b"".join(
            f"{sha256(actual[rel])}  ./{rel}\n".encode() for rel in sorted(actual))
        if hashlib.sha256(inventory_lines).hexdigest() != SOURCE_INVENTORY_SHA256:
            fail("source_inventory_mismatch", f"archive {number} inventory digest differs")
        if sha256(actual["index.json"]) != SOURCE_INDEX_SHA256:
            fail("source_index_mismatch", f"archive {number} index digest differs")
        for rel, path in actual.items():
            entry = expected[rel]
            if path.stat().st_size != entry.get("size") or sha256(path) != entry.get("authentic_sha256"):
                fail("archive_byte_mismatch", f"{number}:{rel}")
            portable = fixture_root / "portable" / rel
            rules = entry.get("transformations", [])
            if rules:
                source_json = load_json(path, "authentic_json_invalid")
                portable_json = load_json(portable, "portable_json_invalid")
                normalized = source_json
                for rule in rules:
                    normalized, count = normalize_json_strings(
                        normalized, rule["source_sha256"], rule["replacement"])
                    if count != rule["count"]:
                        fail("normalization_count_mismatch", f"{number}:{rel}")
                if normalized != portable_json:
                    fail("semantic_change", f"{number}:{rel}")
            elif path.read_bytes() != portable.read_bytes():
                fail("semantic_change", f"{number}:{rel}")


def scan_tracked_fixture(root: Path) -> None:
    files = regular_files(root)
    expected = {"manifest.json"} | {f"portable/{entry}" for entry in regular_files(root / "portable")}
    if set(files) != expected:
        fail("tracked_fixture_coverage_mismatch", f"unexpected tracked fixture paths: {sorted(set(files)-expected)}")
    for rel, path in files.items():
        payload = path.read_bytes()
        if HOST_PATH.search(payload):
            fail("absolute_host_path", rel)
        if CREDENTIAL.search(payload):
            fail("credential_like_content", rel)


def validate_binary(path: Path) -> None:
    reject_symlinks(path)
    if not path.is_file():
        fail("old_binary_missing", str(path))
    resolved = path.resolve().as_posix()
    if "/.adl/bin/" in resolved:
        fail("shared_binary_rejected", resolved)
    reject_live_root(path, "old binary")
    if path.stat().st_size != BINARY_SIZE or sha256(path) != BINARY_SHA256 or blake3(path) != EXPECTED_BLAKE3:
        fail("old_binary_mismatch", "old binary size, SHA-256 or BLAKE3 differs")


def validate_execution_report(path: Path | None) -> None:
    if path is None:
        return
    reject_live_root(path, "execution report")
    report = load_json(path, "execution_report_invalid")
    if not isinstance(report, dict) or report.get("schema") != "csdlc.v3.issue872_generation9_release_gate_report.v1" or report.get("evidence_kind") != "observed_process_streams" or report.get("status") != "passed" or report.get("test_name") != EXPECTED_TEST or report.get("process_exit") != 0:
        fail("marker_only_proof", "execution report lacks exact observed gate identity and process result")
    denominator = report.get("proof_denominator", {}).get("tests") if isinstance(report.get("proof_denominator"), dict) else None
    if not isinstance(denominator, int) or isinstance(denominator, bool) or denominator != 1:
        fail("zero_test_denominator", "execution report has no executed proof")
    if report.get("candidate_behavior_reached") is not True:
        fail("candidate_behavior_not_reached", "release gate did not reach candidate behavior")
    streams = report.get("streams")
    if not isinstance(streams, dict):
        fail("stream_evidence_missing", "raw process streams are required")
    for name in ("stdout", "stderr"):
        item = streams.get(name)
        if not isinstance(item, dict) or not isinstance(item.get("path"), str) or not HEX256.fullmatch(str(item.get("sha256", ""))):
            fail("stream_evidence_missing", name)
        stream_path = path.parent / safe_relative(item["path"], f"streams.{name}.path")
        reject_symlinks(stream_path)
        stream_path = stream_path.resolve()
        try:
            stream_path.relative_to(path.parent.resolve())
        except ValueError:
            fail("path_escape", f"stream {name} escapes report directory")
        reject_live_root(stream_path, f"{name} stream")
        if not stream_path.is_file() or sha256(stream_path) != item["sha256"]:
            fail("stream_digest_mismatch", name)
    stdout = (path.parent / streams["stdout"]["path"]).read_text(encoding="utf-8")
    if f"test {EXPECTED_TEST} ... ok" not in stdout or "test result: ok. 1 passed; 0 failed; 0 ignored" not in stdout:
        fail("cargo_gate_output_mismatch", "stdout lacks exact one-test Cargo result")
    provenance = report.get("provenance")
    if not isinstance(provenance, dict) or provenance.get("old_binary_sha256") != BINARY_SHA256 or provenance.get("fixture_manifest_sha256") != MANIFEST_SHA256:
        fail("execution_provenance_mismatch", "fixture/binary provenance is incomplete")
    packet = report.get("conversion_packet")
    if not isinstance(packet, dict):
        fail("conversion_packet_missing", "the complete retained 7-role/12-scenario/30-fault packet is required")
    packet_root = path.parent / safe_relative(packet.get("path"), "conversion_packet.path")
    reject_live_root(packet_root, "conversion packet")
    regular_files(packet_root)
    packet_manifest = packet_root / "manifest.json"
    if not packet_manifest.is_file() or packet.get("manifest_sha256") != sha256(packet_manifest):
        fail("conversion_packet_digest_mismatch", "conversion packet manifest differs")
    validator = Path(__file__).with_name("validate_issue872_conversion_rehearsal.py")
    result = subprocess.run([sys.executable, "-B", str(validator), "--evidence-root", str(packet_root)],
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if result.returncode != 0:
        fail("conversion_packet_invalid", "retained conversion validator rejected the packet")
    try:
        decision = json.loads(result.stdout)
    except json.JSONDecodeError:
        fail("conversion_packet_invalid", "retained validator did not produce JSON")
    if decision.get("status") != "passed":
        fail("conversion_packet_invalid", "retained validator did not pass")
    binaries = load_json(packet_root / "binary-provenance.json", "conversion_packet_invalid")
    if binaries.get("old", {}).get("sha256") != BINARY_SHA256:
        fail("execution_provenance_mismatch", "packet was not produced with the accepted old binary")
    candidate = binaries.get("candidate", {})
    if provenance.get("candidate_sha256") != candidate.get("sha256") or provenance.get("source_revision") != candidate.get("source_revision"):
        fail("execution_provenance_mismatch", "report candidate/source differs from retained conversion proof")


def validate_effect_evidence(path: Path | None) -> None:
    if path is None:
        return
    reject_live_root(path, "effect evidence")
    evidence = load_json(path, "effect_evidence_invalid")
    if not isinstance(evidence, dict) or evidence.get("schema") != "csdlc.v3.issue872_generation9_no_effect_evidence.v1" or evidence.get("transport_invocations") != 0:
        fail("effect_evidence_invalid", "effect evidence schema or transport count differs")
    inventories = evidence.get("inventories")
    required = {"retained_archives", "portable_fixture", "live_roots", "shared_binary"}
    if not isinstance(inventories, dict) or set(inventories) != required:
        fail("effect_evidence_invalid", "all four before/after inventories are required")
    for name, pair in inventories.items():
        if not isinstance(pair, dict) or not HEX256.fullmatch(str(pair.get("before_sha256", ""))) or pair.get("after_sha256") != pair.get("before_sha256"):
            fail("effect_detected", name)


def emit(status: str, reason: str, detail: str, denominators: dict[str, int]) -> None:
    print(json.dumps({
        "schema": "csdlc.v3.issue872_generation9_fixture_validation.v1",
        "status": status,
        "reason_code": reason,
        "detail": detail,
        "proof_denominator": denominators,
        "effect_claim": "not_proven_without_hash_bound_before_after_evidence",
    }, sort_keys=True))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--fixture-root", type=Path)
    parser.add_argument("--old-binary", type=Path)
    parser.add_argument("--archive-root", action="append", default=[], type=Path)
    parser.add_argument("--execution-report", type=Path)
    parser.add_argument("--effect-evidence", type=Path)
    args = parser.parse_args()
    if args.fixture_root is None or args.old_binary is None:
        emit("not-proven", "missing_inputs", "fixture root and exact old binary are required", {})
        return 2
    denominators = {"archive_copies": len(args.archive_root), "authentic_files": 0, "portable_files": 0, "transformations": 0, "old_binaries": 0}
    try:
        reject_live_root(args.fixture_root, "fixture root")
        _, entries = validate_manifest(args.fixture_root)
        scan_tracked_fixture(args.fixture_root)
        denominators["authentic_files"] = len(entries) if args.archive_root else 0
        denominators["portable_files"] = len(entries)
        denominators["transformations"] = validate_materialization(args.fixture_root, entries)
        validate_archives(args.archive_root, args.fixture_root, entries)
        validate_binary(args.old_binary)
        denominators["old_binaries"] = 1
        validate_execution_report(args.execution_report)
        validate_effect_evidence(args.effect_evidence)
    except InvalidFixture as exc:
        emit("fail", exc.reason, exc.detail, denominators)
        return 1
    except (OSError, ValueError, TypeError) as exc:
        emit("not-proven", "input_unavailable", str(exc), denominators)
        return 2
    emit("pass", "fixture_validated", "generation-9 fixture and exact old binary validated read-only", denominators)
    return 0


if __name__ == "__main__":
    sys.exit(main())
