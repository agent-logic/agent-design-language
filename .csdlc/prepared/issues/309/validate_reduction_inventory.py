#!/usr/bin/env python3
"""Validate the complete #309 baseline, reference-edge, disposition, and reduction evidence."""

from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import re
import subprocess
import sys

BASE = "e926e3bca0ab1981d77b4658d2feb4059bdf33a6"
BASE_TREE = "c57bae97083b42125d7308047595ec2e96033240"
DISPOSITIONS = {
    "retain_active",
    "delete_dead",
    "delete_superseded",
    "migrate_then_delete",
    "temporary_exception",
}
REFERENCE_CLASSES = {
    "module",
    "build",
    "cli",
    "test",
    "documentation",
    "artifact",
    "workflow",
    "external_contract",
}
HEX40 = re.compile(r"^[0-9a-f]{40}$")
HEX64 = re.compile(r"^[0-9a-f]{64}$")


def load(path: pathlib.Path) -> object:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def git(root: pathlib.Path, *argv: str) -> str:
    return subprocess.check_output(["git", "-C", str(root), *argv], text=True).strip()


def git_bytes(root: pathlib.Path, *argv: str) -> bytes:
    return subprocess.check_output(["git", "-C", str(root), *argv])


def canonical_edge_id(edge: dict) -> str:
    identity = [edge.get("source"), edge.get("target"), edge.get("reference_class"), edge.get("disposition")]
    encoded = json.dumps(identity, sort_keys=True, separators=(",", ":")).encode()
    return hashlib.sha256(encoded).hexdigest()


def fail(errors: list[str], message: str) -> None:
    errors.append(message)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=".")
    parser.add_argument("--evidence-root", default=".csdlc/evidence/309")
    args = parser.parse_args()
    root = pathlib.Path(args.root).resolve()
    evidence = root / args.evidence_root
    baseline_path = evidence / "baseline-manifest.json"
    edges_path = evidence / "reference-edge-manifest.json"
    dispositions_path = evidence / "disposition-manifest.json"
    report_path = evidence / "reduction-report.json"
    required = [baseline_path, edges_path, dispositions_path, report_path]
    missing = [str(path.relative_to(root)) for path in required if not path.is_file()]
    if missing:
        print(json.dumps({"status": "blocked", "missing": missing}, sort_keys=True))
        return 2

    errors: list[str] = []
    baseline = load(baseline_path)
    edges = load(edges_path)
    dispositions = load(dispositions_path)
    report = load(report_path)
    if not all(isinstance(value, dict) for value in (baseline, edges, dispositions, report)):
        print(json.dumps({"status": "fail", "errors": ["top-level evidence must be objects"]}))
        return 1

    if git(root, "rev-parse", f"{BASE}^{{tree}}") != BASE_TREE:
        fail(errors, "baseline tree mismatch")
    rows = baseline.get("files")
    if baseline.get("schema") != "adl.issue309.baseline.v1" or not isinstance(rows, list):
        fail(errors, "baseline schema/files invalid")
        rows = []
    observed = {}
    for row in rows:
        if not isinstance(row, dict):
            fail(errors, "baseline row is not an object")
            continue
        path = row.get("path")
        if not isinstance(path, str) or not path.startswith("adl/src/") or not path.endswith(".rs"):
            fail(errors, f"invalid baseline path: {path!r}")
            continue
        if path in observed:
            fail(errors, f"duplicate baseline path: {path}")
        observed[path] = row
        if not HEX40.fullmatch(str(row.get("blob", ""))):
            fail(errors, f"invalid baseline blob: {path}")
        if not isinstance(row.get("physical_lines"), int) or row["physical_lines"] < 0:
            fail(errors, f"invalid physical line count: {path}")
        for key in ("owner", "consumers", "rollback_source"):
            if key not in row:
                fail(errors, f"missing {key}: {path}")

    raw_tree = git_bytes(root, "ls-tree", "-r", BASE)
    tree_rows: dict[str, str] = {}
    for raw in raw_tree.decode().splitlines():
        meta, path = raw.split("\t", 1)
        _mode, _kind, blob = meta.split()
        tree_rows[path] = blob
    tracked = [path for path in tree_rows if path.startswith("adl/src/")]
    tracked_rs = sorted(path for path in tracked if path.endswith(".rs"))
    if sorted(observed) != tracked_rs:
        fail(errors, "baseline path denominator differs from Git tree")
    if len(tracked_rs) != 485 or sum(int(row.get("physical_lines", 0)) for row in observed.values()) != 265633:
        fail(errors, "baseline 485-file/265633-line denominator mismatch")
    for path, row in observed.items():
        if row.get("blob") != tree_rows.get(path):
            fail(errors, f"baseline blob differs from Git tree: {path}")
        actual_lines = len(git_bytes(root, "show", f"{BASE}:{path}").splitlines())
        if row.get("physical_lines") != actual_lines:
            fail(errors, f"baseline line count differs from Git blob: {path}")

    disposition_rows = dispositions.get("files")
    if dispositions.get("schema") != "adl.issue309.dispositions.v1" or not isinstance(disposition_rows, list):
        fail(errors, "disposition schema/files invalid")
        disposition_rows = []
    by_path: dict[str, dict] = {}
    for row in disposition_rows:
        if not isinstance(row, dict) or not isinstance(row.get("path"), str):
            fail(errors, "invalid disposition row")
            continue
        path = row["path"]
        if path in by_path:
            fail(errors, f"duplicate disposition: {path}")
        by_path[path] = row
        disposition = row.get("disposition")
        if disposition not in DISPOSITIONS:
            fail(errors, f"invalid disposition: {path}")
        if not row.get("owner") or not row.get("evidence") or not row.get("validation"):
            fail(errors, f"incomplete accountable disposition: {path}")
        if disposition == "temporary_exception" and not all(row.get(key) for key in ("reason", "expiry")):
            fail(errors, f"incomplete temporary exception: {path}")
        if disposition in {"delete_superseded", "migrate_then_delete"} and not row.get("replacement"):
            fail(errors, f"missing replacement: {path}")
    if set(by_path) != set(observed):
        fail(errors, "disposition denominator differs from baseline")

    edge_rows = edges.get("edges")
    scan = edges.get("scan_denominator")
    if edges.get("schema") != "adl.issue309.reference_edges.v1" or not isinstance(edge_rows, list) or not isinstance(scan, dict):
        fail(errors, "reference-edge schema/denominator invalid")
        edge_rows, scan = [], {}
    if not HEX40.fullmatch(str(scan.get("candidate_commit", ""))) or not HEX64.fullmatch(str(scan.get("tracked_path_blob_digest_sha256", ""))):
        fail(errors, "reference scan identity invalid")
    if not isinstance(scan.get("tracked_paths"), int) or scan.get("tracked_paths", 0) <= 0:
        fail(errors, "reference tracked-path denominator missing")
    if scan.get("scan_revision") != BASE:
        fail(errors, "reference scan revision differs from pinned baseline")
    if scan.get("tracked_paths") != len(tree_rows):
        fail(errors, "reference tracked-path count differs from Git tree")
    if scan.get("rust_targets") != len(tracked_rs):
        fail(errors, "reference Rust-target count differs from baseline")
    if scan.get("tracked_path_blob_digest_sha256") != hashlib.sha256(raw_tree).hexdigest():
        fail(errors, "reference tracked-path/blob digest differs from Git tree")
    edge_ids: set[str] = set()
    incoming: dict[str, list[dict]] = {}
    for edge in edge_rows:
        if not isinstance(edge, dict):
            fail(errors, "reference edge is not an object")
            continue
        edge_id = edge.get("edge_id")
        if not isinstance(edge_id, str) or not HEX64.fullmatch(edge_id) or edge_id in edge_ids:
            fail(errors, f"invalid or duplicate edge identity: {edge_id!r}")
        edge_ids.add(str(edge_id))
        if isinstance(edge_id, str) and edge_id != canonical_edge_id(edge):
            fail(errors, f"edge identity does not bind source/target/class/disposition: {edge_id}")
        if edge.get("reference_class") not in REFERENCE_CLASSES or not edge.get("owner") or not edge.get("evidence"):
            fail(errors, f"unclassified reference edge: {edge_id}")
        source = edge.get("source")
        if not isinstance(source, dict) or not (source.get("path") and HEX40.fullmatch(str(source.get("blob", ""))) or source.get("external_contract")):
            fail(errors, f"invalid edge source: {edge_id}")
        target = edge.get("target")
        if not isinstance(target, str) or target not in observed:
            fail(errors, f"invalid edge target: {edge_id}")
        else:
            incoming.setdefault(target, []).append(edge)
        if isinstance(source, dict) and source.get("path"):
            source_path = source["path"]
            if source_path not in tree_rows or source.get("blob") != tree_rows[source_path]:
                fail(errors, f"edge source does not bind pinned Git blob: {edge_id}")
    for path, row in by_path.items():
        expected_ids = set(row.get("reference_edge_ids", [])) if isinstance(row.get("reference_edge_ids"), list) else set()
        actual_ids = {str(edge.get("edge_id")) for edge in incoming.get(path, [])}
        if not actual_ids:
            fail(errors, f"baseline target has no normalized incoming edge: {path}")
        if expected_ids != actual_ids:
            fail(errors, f"disposition/reference edge denominator mismatch: {path}")
        if row.get("disposition") in {"delete_dead", "delete_superseded"}:
            for edge in incoming.get(path, []):
                if edge.get("disposition") not in {"remove", "replace"}:
                    fail(errors, f"active incoming edge blocks deletion: {path}")

    referenced_ids = {
        edge_id
        for row in by_path.values()
        if isinstance(row.get("reference_edge_ids"), list)
        for edge_id in row["reference_edge_ids"]
    }
    if referenced_ids != edge_ids:
        fail(errors, "orphan or unreferenced normalized edge identity")

    # Recompute the active exact-path reference denominator independently from
    # the retained edge rows. Missing active consumers must fail closed.
    active_prefixes = (
        ".github/",
        "adl/tools/",
        "docs/planning/",
        "docs/milestones/v0.92/",
        "docs/architecture/",
        "docs/templates/",
    )
    active_paths = [
        path for path in tree_rows
        if path.endswith(".rs") or path == "adl/Cargo.toml" or path.startswith(active_prefixes)
    ]
    exact_reference = re.compile(rb"adl/src/[A-Za-z0-9_./-]+\.rs")
    expected_pairs: set[tuple[str, str]] = set()
    for source_path in active_paths:
        content = git_bytes(root, "show", f"{BASE}:{source_path}")
        for match in exact_reference.findall(content):
            target = match.decode()
            if target in observed and target != source_path:
                expected_pairs.add((source_path, target))
    actual_pairs = {
        (str(edge["source"]["path"]), str(edge["target"]))
        for edge in edge_rows
        if isinstance(edge, dict)
        and edge.get("evidence") == "exact tracked path reference"
        and isinstance(edge.get("source"), dict)
        and edge["source"].get("path")
    }
    if expected_pairs != actual_pairs:
        missing_pairs = sorted(expected_pairs - actual_pairs)[:5]
        extra_pairs = sorted(actual_pairs - expected_pairs)[:5]
        fail(errors, f"exact tracked-path reference census differs from pinned Git scan: missing={missing_pairs!r} extra={extra_pairs!r}")

    if report.get("schema") != "adl.issue309.reduction.v1" or report.get("baseline_commit") != BASE:
        fail(errors, "reduction report identity invalid")
    if report.get("baseline_files") != 485 or report.get("baseline_physical_lines") != 265633:
        fail(errors, "reduction report baseline mismatch")
    if report.get("removed_files", -1) < 0 or report.get("removed_physical_lines", -1) < 0:
        fail(errors, "reduction report values invalid")
    source_commit = str(report.get("candidate_source_commit", ""))
    if not HEX40.fullmatch(source_commit):
        fail(errors, "reduction source commit identity invalid")
    else:
        try:
            git(root, "merge-base", "--is-ancestor", source_commit, "HEAD")
            if git(root, "diff", "--name-only", source_commit, "HEAD", "--", "adl/src"):
                fail(errors, "candidate source changed after the pinned reduction commit")
        except subprocess.CalledProcessError:
            fail(errors, "pinned reduction commit is not an ancestor of candidate HEAD")

    diff_rows = git(root, "diff", "--name-status", BASE, "HEAD", "--", "adl/src").splitlines()
    deleted_paths: set[str] = set()
    modified_paths: set[str] = set()
    for line in diff_rows:
        fields = line.split("\t")
        if len(fields) != 2:
            fail(errors, f"unsupported candidate diff row: {line}")
            continue
        status, path = fields
        if status == "D":
            deleted_paths.add(path)
        elif status == "M":
            modified_paths.add(path)
        else:
            fail(errors, f"unclassified candidate source change: {line}")
    declared_deleted = {
        path
        for band in report.get("bands", []) if isinstance(band, dict)
        for path in band.get("paths", []) if isinstance(path, str)
    }
    declared_modified = set(report.get("modified_files", [])) if isinstance(report.get("modified_files"), list) else set()
    if deleted_paths != declared_deleted or modified_paths != declared_modified:
        fail(errors, "candidate Git diff differs from declared deleted/modified paths")
    removed_lines = sum(len(git_bytes(root, "show", f"{BASE}:{path}").splitlines()) for path in deleted_paths)
    if report.get("removed_files") != len(deleted_paths) or report.get("removed_physical_lines") != removed_lines:
        fail(errors, "reduction counts differ from candidate Git diff")
    if any((root / path).exists() for path in deleted_paths):
        fail(errors, "declared deleted source path remains in candidate worktree")

    result = {
        "status": "pass" if not errors else "fail",
        "baseline_files": len(observed),
        "reference_edges": len(edge_ids),
        "errors": errors,
        "evidence_sha256": {
            path.name: hashlib.sha256(path.read_bytes()).hexdigest() for path in required
        },
    }
    print(json.dumps(result, sort_keys=True))
    return 0 if not errors else 1


if __name__ == "__main__":
    sys.exit(main())
