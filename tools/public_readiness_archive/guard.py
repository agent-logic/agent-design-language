"""Exact #1325 successor archive; original retirement guards remain unchanged."""
import hashlib
import json
import subprocess
from pathlib import Path

TRANSITION_SHA256 = "4b1cc0ef6b60d7cc0ec8b2b9d3354489d0036328dcfa9f1d1e43543ffc7bf560"


def normalize_candidate(root, candidate, tree):
    root = Path(root)
    raw = (root / "tools/public_readiness_archive/transition.json").read_bytes()
    if hashlib.sha256(raw).hexdigest() != TRANSITION_SHA256:
        raise ValueError("public readiness archive transition changed")
    transition = json.loads(raw)
    baseline = tree(transition["source_commit"])
    result = dict(candidate)
    seen = set()
    for entry in transition["entries"]:
        path = entry["path"]
        identity = [entry["mode"], entry["git_blob"]]
        if path in seen or baseline.get(path) != identity or path in candidate:
            raise ValueError("archive removal identity mismatch: " + path)
        seen.add(path)
        data = subprocess.check_output(["git", "cat-file", "blob", entry["git_blob"]], cwd=root)
        if len(data) != entry["bytes"] or hashlib.sha256(data).hexdigest() != entry["sha256"]:
            raise ValueError("archive preimage custody mismatch: " + path)
        result[path] = identity
    caller = transition["caller_change"]
    if baseline.get(caller["path"]) != caller["before"] or candidate.get(caller["path"]) != caller["after"]:
        raise ValueError("archive guard caller identity mismatch")
    result[caller["path"]] = caller["before"]
    return result
