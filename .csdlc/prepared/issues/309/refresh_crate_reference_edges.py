#!/usr/bin/env python3
"""Refresh deterministic Rust crate-path edges in the #309 evidence manifests."""

from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import re
import subprocess

BASE = "e926e3bca0ab1981d77b4658d2feb4059bdf33a6"
CRATE_PATH = re.compile(rb"(?:crate|adl)::([A-Za-z_][A-Za-z0-9_]*)")


def git_bytes(root: pathlib.Path, *argv: str) -> bytes:
    return subprocess.check_output(["git", "-C", str(root), *argv])


def load(path: pathlib.Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def write(path: pathlib.Path, value: dict) -> None:
    path.write_text(json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n", encoding="utf-8")


def edge_id(edge: dict) -> str:
    identity = [edge["source"], edge["target"], edge["reference_class"], edge["disposition"]]
    return hashlib.sha256(json.dumps(identity, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=".")
    parser.add_argument("--evidence-root", default=".csdlc/evidence/309")
    args = parser.parse_args()
    root = pathlib.Path(args.root).resolve()
    evidence = root / args.evidence_root
    baseline = load(evidence / "baseline-manifest.json")
    reference = load(evidence / "reference-edge-manifest.json")
    dispositions = load(evidence / "disposition-manifest.json")

    baseline_rows = {row["path"]: row for row in baseline["files"]}
    disposition_rows = {row["path"]: row for row in dispositions["files"]}
    module_targets: dict[str, str] = {}
    for path in baseline_rows:
        relative = path.removeprefix("adl/src/")
        if "/" not in relative and relative.endswith(".rs"):
            module_targets[relative[:-3]] = path
        elif relative.endswith("/mod.rs") and relative.count("/") == 1:
            module_targets[relative.split("/", 1)[0]] = path

    raw_tree = git_bytes(root, "ls-tree", "-r", BASE).decode().splitlines()
    blobs = {line.split("\t", 1)[1]: line.split()[2] for line in raw_tree}
    existing = {edge["edge_id"]: edge for edge in reference["edges"]}
    added = 0
    crate_pairs: set[tuple[str, str]] = set()
    for source_path in sorted(path for path in blobs if path.endswith(".rs")):
        content = git_bytes(root, "show", f"{BASE}:{source_path}")
        for module in sorted(set(match.decode() for match in CRATE_PATH.findall(content))):
            target = module_targets.get(module)
            if not target or target == source_path:
                continue
            crate_pairs.add((source_path, target))
            source_disposition = disposition_rows.get(source_path, {}).get("disposition")
            target_disposition = disposition_rows[target]["disposition"]
            edge = {
                "source": {"path": source_path, "blob": blobs[source_path]},
                "target": target,
                "reference_class": "module",
                "owner": "#309",
                "disposition": "remove"
                if source_disposition in {"delete_dead", "delete_superseded"}
                or target_disposition in {"delete_dead", "delete_superseded"}
                else "retain",
                "evidence": "Rust crate-path reachability",
            }
            edge["edge_id"] = edge_id(edge)
            if edge["edge_id"] in existing:
                continue
            existing[edge["edge_id"]] = edge
            added += 1

    edges = sorted(existing.values(), key=lambda edge: (edge["target"], edge["source"].get("path", ""), edge["edge_id"]))
    by_target: dict[str, list[str]] = {path: [] for path in baseline_rows}
    for edge in edges:
        by_target[edge["target"]].append(edge["edge_id"])
    for path, row in disposition_rows.items():
        row["reference_edge_ids"] = sorted(by_target[path])
        if row["disposition"] == "retain_active" and row.get("owner") == "adl crate module/build graph":
            active = sum(existing[edge_id]["disposition"] == "retain" for edge_id in by_target[path])
            row["evidence"] = f"{active} normalized active incoming edges plus exact baseline blob"

    reference["edges"] = edges
    reference["scan_denominator"]["rust_crate_path_edges"] = len(crate_pairs)
    reference["scan_denominator"]["reference_algorithm"] = "module-declaration + exact-path + crate/adl top-level path v2"
    write(evidence / "reference-edge-manifest.json", reference)
    dispositions["files"] = [disposition_rows[path] for path in sorted(disposition_rows)]
    write(evidence / "disposition-manifest.json", dispositions)
    print(json.dumps({"status": "pass", "added_edges": added, "total_edges": len(edges)}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
