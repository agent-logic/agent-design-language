#!/usr/bin/env python3
"""Fail-closed classifier for mechanical changed-source coverage fallout."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import subprocess
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
            current = {"id": f"{path}:new-{match.group(3)}", "header": line, "added": [], "removed": [], "body": []}
            hunks.append(current)
            continue
        if current is None or line.startswith(("+++", "---")):
            continue
        if line.startswith("+"):
            current["added"].append(line[1:])
            current["body"].append(("+", line[1:]))
        elif line.startswith("-"):
            current["removed"].append(line[1:])
            current["body"].append(("-", line[1:]))
        elif line.startswith(" "):
            current["body"].append((" ", line[1:]))
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

def pass_through_only(hunk: dict[str, object], token: str, callee: str) -> bool:
    meaningful = [line.strip() for line in hunk["added"] if line.strip()]
    if meaningful != [f"&{token},"] or hunk["removed"]:
        return False
    body = hunk["body"]
    added_index = next((index for index, (prefix, line) in enumerate(body) if prefix == "+" and line.strip()), None)
    if added_index is None:
        return False
    # The governed invocation must be the immediately enclosing syntactic
    # surface, not merely another call mentioned somewhere in the same hunk.
    previous = next((line.strip() for prefix, line in reversed(body[:added_index]) if prefix == " " and line.strip()), "")
    return re.search(rf"(?:\.|\b){re.escape(callee)}\($", previous) is not None

def run_verified_result(
    repo_root: Path,
    evidence_dir: Path,
    command: object,
    *,
    kind: str,
    subject: str,
    base: str,
    head: str,
    diff_digest: str,
) -> tuple[dict[str, object], str]:
    if not isinstance(command, list) or not command or not all(isinstance(part, str) and part for part in command):
        raise Rejected(f"governed {kind} command is invalid for {subject}")
    evidence_dir.mkdir(parents=True, exist_ok=True)
    stem = f"{kind}-{sha256(subject.encode())[:16]}"
    log_path, artifact_path = evidence_dir / f"{stem}.log", evidence_dir / f"{stem}.json"
    completed = subprocess.run(command, cwd=repo_root, capture_output=True, check=False)
    log_path.write_bytes(completed.stdout + completed.stderr)
    artifact = {"schema":"adl.mechanical_proof_result.v2","producer":"mechanical_coverage_fallout.py:subprocess","kind":kind,"subject":subject,"base_revision":base,"head_revision":head,"diff_sha256":diff_digest,"command":command,"exit_code":completed.returncode,"evidence":log_path.name,"evidence_sha256":file_sha256(log_path)}
    artifact_path.write_text(json.dumps(artifact, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    result_digest = file_sha256(artifact_path)
    if completed.returncode != 0:
        raise Rejected(f"governed {kind} command failed for {subject}; evidence {log_path}")
    return artifact, result_digest

def classify(diff: str, mapping: dict[str, object], mapping_path: Path, repo_root: Path, evidence_dir: Path, base: str, head: str) -> dict[str, object]:
    diff_digest = sha256(diff.encode())
    mapping_digest = file_sha256(mapping_path)
    path, hunks = parse_diff(diff)
    entries = mapping.get("mappings")
    if not isinstance(entries, list):
        raise Rejected("mapping has no mappings array")
    candidates = [entry for entry in entries if isinstance(entry, dict) and entry.get("file") == path]
    if len(candidates) != 1:
        raise Rejected(f"file is not mapped exactly once: {path}")
    entry = candidates[0]
    token, owners, rationale, callee = entry.get("token"), entry.get("owners"), entry.get("rationale"), entry.get("callee")
    if not isinstance(token, str) or not TOKEN.fullmatch(token):
        raise Rejected("mapping token is invalid")
    if not isinstance(owners, list) or not owners or not all(isinstance(owner, str) and owner for owner in owners):
        raise Rejected("mapping owners are incomplete")
    if not isinstance(rationale, str) or not rationale.strip():
        raise Rejected("mapping rationale is missing")
    compile_command, behavioral = entry.get("compile_command"), entry.get("behavior_commands")
    if not isinstance(callee, str) or not re.fullmatch(r"[a-z_][a-z0-9_]*", callee) or not isinstance(behavioral, dict):
        raise Rejected("mapping callee or governed commands are incomplete")
    receipt_hunks = []
    for hunk in hunks:
        added, removed = hunk["added"], hunk["removed"]
        kind = "import_only" if import_only(added, removed, token) else "argument_pass_through" if pass_through_only(hunk, token, callee) else None
        if kind is None:
            raise Rejected(f"non-mechanical addition in {hunk['id']}")
        hunk_proof, result_digest = run_verified_result(repo_root, evidence_dir, compile_command, kind="compile", subject=hunk["id"], base=base, head=head, diff_digest=diff_digest)
        content = {"header": hunk["header"], "removed": removed, "added": added}
        receipt_hunks.append({"hunk": hunk["id"], "header": hunk["header"], "kind": kind, "content": content, "content_sha256": sha256(json.dumps(content, sort_keys=True, separators=(",", ":")).encode()), "compile_result_sha256": result_digest, "compile_evidence_sha256": hunk_proof["evidence_sha256"]})
    tests: dict[str, list[str]] = {}
    for owner in owners:
        result, result_digest = run_verified_result(repo_root, evidence_dir, behavioral.get(owner), kind="behavior", subject=owner, base=base, head=head, diff_digest=diff_digest)
        tests[owner] = {"command": result["command"], "result_sha256": result_digest, "evidence_sha256": result["evidence_sha256"]}
    results_digest = sha256(json.dumps({"hunks":receipt_hunks,"tests":tests}, sort_keys=True, separators=(",", ":")).encode())
    return {"schema":"adl.mechanical_coverage_fallout.v2","classification":"mechanical_compile_fallout","execution_provenance":"classifier_executed_governed_commands","base_revision":base,"head_revision":head,"diff_sha256":diff_digest,"mapping_sha256":mapping_digest,"execution_results_sha256":results_digest,"file":path,"hunks":receipt_hunks,"token":token,"callee":callee,"owner":owners,"tests":tests,"rationale":rationale,"coverage_authority":"pr_fast_non_authoritative"}

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--diff", required=True, type=Path)
    parser.add_argument("--mapping", required=True, type=Path)
    parser.add_argument("--receipt", required=True, type=Path)
    parser.add_argument("--repo-root", required=True, type=Path)
    parser.add_argument("--evidence-dir", required=True, type=Path)
    parser.add_argument("--base-revision", required=True)
    parser.add_argument("--head-revision", required=True)
    args = parser.parse_args()
    try:
        mapping = load_json(args.mapping)
        if not isinstance(mapping, dict):
            raise Rejected("mapping must be a JSON object")
        receipt = classify(args.diff.read_text(encoding="utf-8"), mapping, args.mapping, args.repo_root, args.evidence_dir, args.base_revision, args.head_revision)
        args.receipt.parent.mkdir(parents=True, exist_ok=True)
        args.receipt.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    except (OSError, Rejected) as exc:
        print(f"mechanical-coverage-fallout: rejected: {exc}", file=sys.stderr)
        return 1
    print(json.dumps(receipt, sort_keys=True))
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
