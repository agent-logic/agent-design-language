#!/usr/bin/env python3
"""Validate #309 per-band Git revert/reapply proof receipts."""

from __future__ import annotations

import argparse
import json
import pathlib
import re
import subprocess
import sys

HEX40 = re.compile(r"^[0-9a-f]{40}$")


def git(root: pathlib.Path, *argv: str) -> str:
    return subprocess.check_output(["git", "-C", str(root), *argv], text=True).strip()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("receipt", nargs="?", default=".csdlc/evidence/309/rollback-proof.json")
    parser.add_argument("--root", default=".")
    args = parser.parse_args()
    root = pathlib.Path(args.root).resolve()
    path = pathlib.Path(args.receipt)
    if not path.is_absolute():
        path = root / path
    if not path.is_file():
        print(json.dumps({"status": "blocked", "missing": str(path)}))
        return 2
    receipt = json.loads(path.read_text(encoding="utf-8"))
    errors: list[str] = []
    if receipt.get("schema") != "adl.issue309.rollback_proof.v1":
        errors.append("schema mismatch")
    if receipt.get("baseline_commit") != "e926e3bca0ab1981d77b4658d2feb4059bdf33a6":
        errors.append("baseline mismatch")
    bands = receipt.get("bands")
    if not isinstance(bands, list) or not bands:
        errors.append("bands missing")
        bands = []
    seen: set[str] = set()
    for band in bands:
        if not isinstance(band, dict):
            errors.append("band is not object")
            continue
        name = band.get("band")
        if name not in {"A", "B", "C"} or name in seen:
            errors.append(f"invalid/duplicate band {name}")
        seen.add(str(name))
        for key in ("commit", "tree_before", "tree_after", "reverted_tree", "reapplied_tree"):
            if not HEX40.fullmatch(str(band.get(key, ""))):
                errors.append(f"{name}: invalid {key}")
        if band.get("reverted_tree") != band.get("tree_before"):
            errors.append(f"{name}: revert did not restore exact tree")
        if band.get("reapplied_tree") != band.get("tree_after"):
            errors.append(f"{name}: reapply did not restore exact tree")
        if band.get("unrelated_paths_changed") not in ([], None):
            errors.append(f"{name}: unrelated paths changed")
        if not band.get("focused_validation_passed"):
            errors.append(f"{name}: focused validation not passed")
        commit = str(band.get("commit", ""))
        revert = str(band.get("revert_commit", ""))
        reapply = str(band.get("reapply_commit", ""))
        if not HEX40.fullmatch(revert) or not HEX40.fullmatch(reapply):
            errors.append(f"{name}: invalid revert/reapply commit")
            continue
        try:
            for oid in (commit, revert, reapply):
                git(root, "cat-file", "-e", f"{oid}^{{commit}}")
            derived = {
                "tree_before": git(root, "rev-parse", f"{commit}^^{{tree}}"),
                "tree_after": git(root, "rev-parse", f"{commit}^{{tree}}"),
                "reverted_tree": git(root, "rev-parse", f"{revert}^{{tree}}"),
                "reapplied_tree": git(root, "rev-parse", f"{reapply}^{{tree}}"),
            }
            for key, value in derived.items():
                if band.get(key) != value:
                    errors.append(f"{name}: recorded {key} differs from Git")
            if git(root, "rev-parse", f"{revert}^") != commit:
                errors.append(f"{name}: revert parent is not band commit")
            if git(root, "rev-parse", f"{reapply}^") != revert:
                errors.append(f"{name}: reapply parent is not revert commit")
            path_sets = [
                set(git(root, "diff", "--name-only", f"{oid}^", oid).splitlines())
                for oid in (commit, revert, reapply)
            ]
            if not path_sets[0] or path_sets[0] != path_sets[1] or path_sets[0] != path_sets[2]:
                errors.append(f"{name}: band/revert/reapply changed-path sets differ")
        except subprocess.CalledProcessError:
            errors.append(f"{name}: Git object/topology verification failed")
    print(json.dumps({"status": "pass" if not errors else "fail", "bands": len(bands), "errors": errors}, sort_keys=True))
    return 0 if not errors else 1


if __name__ == "__main__":
    sys.exit(main())
