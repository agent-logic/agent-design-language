#!/usr/bin/env python3
"""Fail-closed classifier for mechanical changed-source coverage fallout."""

from __future__ import annotations

import argparse
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

def import_only(added: list[str], removed: list[str], token: str) -> bool:
    added_text, removed_text = " ".join(added), " ".join(removed)
    if token not in added_text or "use " not in added_text + removed_text:
        return False
    forbidden = re.compile(r"\b(fn|if|else|match|return|let|self\.|Err|Ok)\b|[=!<>]=|&&|\|\|")
    if forbidden.search(added_text) or forbidden.search(removed_text):
        return False
    identifiers = lambda text: {
        value for value in re.findall(r"[A-Za-z_][A-Za-z0-9_]*", text)
        if value not in {"use", "pub", "super", "self", "crate"}
    }
    return identifiers(added_text) - identifiers(removed_text) == {token}

def pass_through_only(lines: list[str], token: str) -> bool:
    meaningful = [line.strip() for line in lines if line.strip()]
    return bool(meaningful) and all(line == f"&{token}," for line in meaningful)

def classify(diff: str, mapping: dict[str, object], proof: dict[str, object]) -> dict[str, object]:
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
    compile_hunks, behavioral = proof.get("compile_hunks"), proof.get("behavioral_tests")
    if not isinstance(compile_hunks, dict) or not isinstance(behavioral, dict):
        raise Rejected("proof must contain compile_hunks and behavioral_tests")
    receipt_hunks = []
    for hunk in hunks:
        added, removed = hunk["added"], hunk["removed"]
        kind = "import_only" if import_only(added, removed, token) else "argument_pass_through" if not removed and pass_through_only(added, token) else None
        if kind is None:
            raise Rejected(f"non-mechanical addition in {hunk['id']}")
        hunk_proof = compile_hunks.get(hunk["id"])
        if not isinstance(hunk_proof, dict) or hunk_proof.get("outcome") != "passed" or not isinstance(hunk_proof.get("command"), list) or not hunk_proof["command"]:
            raise Rejected(f"missing compile proof for {hunk['id']}")
        receipt_hunks.append({"hunk": hunk["id"], "header": hunk["header"], "kind": kind, "compile": hunk_proof})
    tests: dict[str, list[str]] = {}
    for owner in owners:
        owner_tests = behavioral.get(owner)
        if not isinstance(owner_tests, list) or not owner_tests or not all(isinstance(test, str) and test for test in owner_tests):
            raise Rejected(f"missing behavioral proof for owner {owner}")
        tests[owner] = sorted(set(owner_tests))
    return {"schema":"adl.mechanical_coverage_fallout.v1","classification":"mechanical_compile_fallout","file":path,"hunks":receipt_hunks,"token":token,"owner":owners,"tests":tests,"rationale":rationale,"coverage_authority":"pr_fast_non_authoritative"}

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--diff", required=True, type=Path)
    parser.add_argument("--mapping", required=True, type=Path)
    parser.add_argument("--proof", required=True, type=Path)
    parser.add_argument("--receipt", required=True, type=Path)
    args = parser.parse_args()
    try:
        mapping, proof = load_json(args.mapping), load_json(args.proof)
        if not isinstance(mapping, dict) or not isinstance(proof, dict):
            raise Rejected("mapping and proof must be JSON objects")
        receipt = classify(args.diff.read_text(encoding="utf-8"), mapping, proof)
        args.receipt.parent.mkdir(parents=True, exist_ok=True)
        args.receipt.write_text(json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    except (OSError, Rejected) as exc:
        print(f"mechanical-coverage-fallout: rejected: {exc}", file=sys.stderr)
        return 1
    print(json.dumps(receipt, sort_keys=True))
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
