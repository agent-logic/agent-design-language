#!/usr/bin/env python3
"""Fail-closed classifier for mechanical changed-source coverage fallout."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path

HUNK = re.compile(r"^@@ -(\d+)(?:,(\d+))? \+(\d+)(?:,(\d+))? @@")
TOKEN = re.compile(r"^[A-Z][A-Z0-9_]+$")

class Rejected(ValueError):
    pass

def load_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise Rejected(f"cannot read valid JSON from {path}: {exc}") from exc

def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()

def file_sha256(path: Path) -> str:
    try:
        return sha256(path.read_bytes())
    except OSError as exc:
        raise Rejected(f"cannot hash evidence artifact {path}: {exc}") from exc

def parse_diff(text: str) -> tuple[str, list[dict[str, object]]]:
    lines = text.splitlines()
    new_files = [line[6:] for line in lines if line.startswith("+++ b/")]
    if len(new_files) != 1 or any(line.startswith("+++ /dev/null") for line in lines):
        raise Rejected("receipt classification requires exactly one modified file")
    path = new_files[0]
    hunks: list[dict[str, object]] = []
    current: dict[str, object] | None = None
    for line in lines:
        match = HUNK.match(line)
        if match:
            current = {"id": f"{path}:new-{match.group(3)}", "header": line, "added": [], "removed": []}
            hunks.append(current)
            continue
        if current is None or line.startswith(("+++", "---")):
            continue
        if line.startswith("+"):
            current["added"].append(line[1:])
        elif line.startswith("-"):
            current["removed"].append(line[1:])
    if not hunks:
        raise Rejected("diff contains no hunks")
    return path, hunks

def parse_simple_use(lines: list[str]) -> tuple[str, list[str]] | None:
    text = " ".join(line.strip() for line in lines if line.strip())
    text = re.sub(r"\s+", " ", text)
    match = re.fullmatch(r"(pub )?use ([A-Za-z_][A-Za-z0-9_:]*)::\{([^{}]+)\};", text)
    if not match or match.group(1):
        return None
    prefix = match.group(2)
    members = [member.strip() for member in match.group(3).split(",") if member.strip()]
    if not members or any(not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", member) for member in members):
        return None
    return prefix, members

def import_only(added: list[str], removed: list[str], token: str) -> bool:
    old, new = parse_simple_use(removed), parse_simple_use(added)
    if old is None or new is None or old[0] != new[0]:
        return False
    old_members, new_members = old[1], new[1]
    if token in old_members or new_members.count(token) != 1 or len(new_members) != len(old_members) + 1:
        return False
    return [member for member in new_members if member != token] == old_members

def pass_through_only(lines: list[str], token: str) -> bool:
    meaningful = [line.strip() for line in lines if line.strip()]
    return bool(meaningful) and all(line == f"&{token}," for line in meaningful)

def verified_result(
    proof_dir: Path,
    binding: object,
    *,
    kind: str,
    subject: str,
    base: str,
    head: str,
    diff_digest: str,
) -> tuple[dict[str, object], str]:
    if not isinstance(binding, dict) or set(binding) != {"artifact", "sha256"}:
        raise Rejected(f"{kind} proof binding for {subject} is incomplete")
    relative, expected = binding.get("artifact"), binding.get("sha256")
    if not isinstance(relative, str) or not isinstance(expected, str) or not re.fullmatch(r"[0-9a-f]{64}", expected):
        raise Rejected(f"{kind} proof binding for {subject} is invalid")
    artifact_path = proof_dir / relative
    if artifact_path.resolve().parent != proof_dir.resolve() or file_sha256(artifact_path) != expected:
        raise Rejected(f"{kind} result artifact digest mismatch for {subject}")
    artifact = load_json(artifact_path)
    if not isinstance(artifact, dict) or artifact.get("schema") != "adl.mechanical_proof_result.v1":
        raise Rejected(f"{kind} result artifact schema mismatch for {subject}")
    required = {"schema", "kind", "subject", "base_revision", "head_revision", "diff_sha256", "command", "exit_code", "evidence", "evidence_sha256"}
    if set(artifact) != required or artifact.get("kind") != kind or artifact.get("subject") != subject:
        raise Rejected(f"{kind} result artifact identity mismatch for {subject}")
    if artifact.get("base_revision") != base or artifact.get("head_revision") != head or artifact.get("diff_sha256") != diff_digest:
        raise Rejected(f"{kind} result artifact revision mismatch for {subject}")
    if artifact.get("exit_code") != 0 or not isinstance(artifact.get("command"), list) or not artifact["command"]:
        raise Rejected(f"{kind} result did not pass for {subject}")
    evidence, evidence_digest = artifact.get("evidence"), artifact.get("evidence_sha256")
    if not isinstance(evidence, str) or not isinstance(evidence_digest, str):
        raise Rejected(f"{kind} evidence binding is missing for {subject}")
    evidence_path = proof_dir / evidence
    if evidence_path.resolve().parent != proof_dir.resolve() or file_sha256(evidence_path) != evidence_digest:
        raise Rejected(f"{kind} evidence digest mismatch for {subject}")
    return artifact, expected

def classify(diff: str, mapping: dict[str, object], proof: dict[str, object], proof_path: Path, mapping_path: Path, base: str, head: str) -> dict[str, object]:
    diff_digest = sha256(diff.encode())
    mapping_digest = file_sha256(mapping_path)
    proof_digest = file_sha256(proof_path)
    if proof.get("schema") != "adl.mechanical_coverage_proof.v1" or proof.get("base_revision") != base or proof.get("head_revision") != head or proof.get("diff_sha256") != diff_digest or proof.get("mapping_sha256") != mapping_digest:
        raise Rejected("proof manifest does not bind the exact base, head, diff, and mapping")
    path, hunks = parse_diff(diff)
    entries = mapping.get("mappings")
    if not isinstance(entries, list):
        raise Rejected("mapping has no mappings array")
    candidates = [entry for entry in entries if isinstance(entry, dict) and entry.get("file") == path]
    if len(candidates) != 1:
        raise Rejected(f"file is not mapped exactly once: {path}")
    entry = candidates[0]
    token, owners, rationale = entry.get("token"), entry.get("owners"), entry.get("rationale")
    if not isinstance(token, str) or not TOKEN.fullmatch(token):
        raise Rejected("mapping token is invalid")
    if not isinstance(owners, list) or not owners or not all(isinstance(owner, str) and owner for owner in owners):
        raise Rejected("mapping owners are incomplete")
    if not isinstance(rationale, str) or not rationale.strip():
        raise Rejected("mapping rationale is missing")
    compile_hunks, behavioral = proof.get("compile_results"), proof.get("behavioral_results")
    if not isinstance(compile_hunks, dict) or not isinstance(behavioral, dict):
        raise Rejected("proof must contain compile_hunks and behavioral_tests")
    receipt_hunks = []
    for hunk in hunks:
        added, removed = hunk["added"], hunk["removed"]
        kind = "import_only" if import_only(added, removed, token) else "argument_pass_through" if not removed and pass_through_only(added, token) else None
        if kind is None:
            raise Rejected(f"non-mechanical addition in {hunk['id']}")
        hunk_proof, result_digest = verified_result(proof_path.parent, compile_hunks.get(hunk["id"]), kind="compile", subject=hunk["id"], base=base, head=head, diff_digest=diff_digest)
        content = {"header": hunk["header"], "removed": removed, "added": added}
        receipt_hunks.append({"hunk": hunk["id"], "header": hunk["header"], "kind": kind, "content": content, "content_sha256": sha256(json.dumps(content, sort_keys=True, separators=(",", ":")).encode()), "compile_result_sha256": result_digest, "compile_evidence_sha256": hunk_proof["evidence_sha256"]})
    tests: dict[str, list[str]] = {}
    for owner in owners:
        result, result_digest = verified_result(proof_path.parent, behavioral.get(owner), kind="behavior", subject=owner, base=base, head=head, diff_digest=diff_digest)
        tests[owner] = {"command": result["command"], "result_sha256": result_digest, "evidence_sha256": result["evidence_sha256"]}
    return {"schema":"adl.mechanical_coverage_fallout.v1","classification":"mechanical_compile_fallout","base_revision":base,"head_revision":head,"diff_sha256":diff_digest,"mapping_sha256":mapping_digest,"proof_manifest_sha256":proof_digest,"file":path,"hunks":receipt_hunks,"token":token,"owner":owners,"tests":tests,"rationale":rationale,"coverage_authority":"pr_fast_non_authoritative"}

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--diff", required=True, type=Path)
    parser.add_argument("--mapping", required=True, type=Path)
    parser.add_argument("--proof", required=True, type=Path)
    parser.add_argument("--receipt", required=True, type=Path)
    parser.add_argument("--base-revision", required=True)
    parser.add_argument("--head-revision", required=True)
    args = parser.parse_args()
    try:
        mapping, proof = load_json(args.mapping), load_json(args.proof)
        if not isinstance(mapping, dict) or not isinstance(proof, dict):
            raise Rejected("mapping and proof must be JSON objects")
        receipt = classify(args.diff.read_text(encoding="utf-8"), mapping, proof, args.proof, args.mapping, args.base_revision, args.head_revision)
        args.receipt.parent.mkdir(parents=True, exist_ok=True)
        args.receipt.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    except (OSError, Rejected) as exc:
        print(f"mechanical-coverage-fallout: rejected: {exc}", file=sys.stderr)
        return 1
    print(json.dumps(receipt, sort_keys=True))
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
