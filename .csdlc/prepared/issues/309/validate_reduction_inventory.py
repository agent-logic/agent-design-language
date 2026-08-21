#!/usr/bin/env python3
"""Validate the complete #309 baseline, reference-edge, disposition, and reduction evidence."""

from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import re
import subprocess
import tomllib
import sys

BASE = "e926e3bca0ab1981d77b4658d2feb4059bdf33a6"
BAND_B_COMMIT = "f3cf4c937cbd55beb5e78b73b838033ff63bae66"
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
CRATE_PATH = re.compile(rb"(?:crate|adl)::([A-Za-z_][A-Za-z0-9_]*)")
MOD_DECL = re.compile(rb"(?m)^\s*(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*;")


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


def candidate_reference_exists(root: pathlib.Path, edge: dict) -> bool:
    source_path = edge.get("source", {}).get("path")
    target = edge.get("target")
    if not source_path or not isinstance(target, str) or not (root / source_path).is_file():
        return False
    content = (root / source_path).read_text(encoding="utf-8", errors="replace")
    if edge.get("evidence") == "exact tracked path reference":
        return target in content
    if edge.get("reference_class") != "module":
        return True
    relative = target.removeprefix("adl/src/")
    if "/" not in relative or relative.count("/") == 1 and relative.endswith("/mod.rs"):
        module = relative.removesuffix(".rs").removesuffix("/mod")
        return bool(re.search(rf"\b(?:crate|adl)::\s*{re.escape(module)}\b", content)) or (
            source_path == "adl/src/lib.rs"
            and bool(re.search(rf"(?m)^\s*(?:pub\s+)?mod\s+{re.escape(module)}\s*;", content))
        )
    target_path = pathlib.PurePosixPath(target)
    module = target_path.parent.name if target_path.name == "mod.rs" else target_path.stem
    source_path_obj = pathlib.PurePosixPath(source_path)
    source_module_dir = source_path_obj.parent if source_path_obj.name == "mod.rs" else source_path_obj.with_suffix("")
    return target_path in {
        source_module_dir / f"{module}.rs",
        source_module_dir / module / "mod.rs",
    } and bool(re.search(rf"(?m)^\s*(?:pub\s+)?mod\s+{re.escape(module)}\s*;", content))


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
    policy_row = by_path.get("adl/src/policy_authority.rs", {})
    if policy_row.get("disposition") != "retain_active" or not (root / "adl/src/policy_authority.rs").is_file():
        fail(errors, "current policy authority must remain retained and present")

    historical_path = root / "docs/milestones/v0.91.8/evidence/wp13-external-bands/external-band-deletion-manifest.json"
    if not historical_path.is_file():
        fail(errors, "reviewed v0.91.8 external-band deletion manifest missing")
    else:
        historical = load(historical_path)
        historical_paths = {
            row.get("path") for row in historical.get("deleted_files", []) if isinstance(row, dict)
        }
        required_retired_entrypoints = {
            "adl/src/bin/demo_v0905_local_gemma_model_evaluation.rs",
            "adl/src/bin/demo_v0912_gws_live_capability_execution_surface.rs",
            "adl/src/bin/demo_v0912_gws_live_content_card_roundtrip.rs",
            "adl/src/bin/demo_v0912_gws_live_safety_package.rs",
            "adl/src/bin/demo_v0912_rust_native_gws_adapter_boundary.rs",
            "adl/src/bin/demo_v0912_speculative_decoding_prototype.rs",
            "adl/src/bin/demo_v0912_uts_acc_multi_model_benchmark.rs",
        }
        if not required_retired_entrypoints.issubset(historical_paths):
            fail(errors, "historical deletion authority lacks a required retired demo entrypoint")

    edge_rows = edges.get("edges")
    scan = edges.get("scan_denominator")
    if edges.get("schema") != "adl.issue309.reference_edges.v1" or not isinstance(edge_rows, list) or not isinstance(scan, dict):
        fail(errors, "reference-edge schema/denominator invalid")
        edge_rows, scan = [], {}
    if scan.get("candidate_commit") != BAND_B_COMMIT or not HEX64.fullmatch(str(scan.get("tracked_path_blob_digest_sha256", ""))):
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
    cargo = tomllib.loads((root / "adl/Cargo.toml").read_text(encoding="utf-8"))
    cargo_targets = {f"adl/{row['path']}" for row in cargo.get("bin", []) if isinstance(row, dict) and row.get("path")}
    cargo_targets.add(f"adl/{cargo.get('lib', {}).get('path', 'src/lib.rs')}")
    cargo_targets.update(
        path.relative_to(root).as_posix()
        for path in (root / "adl/src/bin").glob("*.rs")
        if path.is_file()
    )
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
            source_present = (root / source_path).is_file()
            if edge.get("disposition") == "remove" and source_present and candidate_reference_exists(root, edge):
                fail(errors, f"retained candidate source still contains removed edge: {edge_id}")
            if edge.get("disposition") == "replace" and not (
                edge.get("reference_class") == "artifact"
                and source_path == "adl/tools/test_run_pr_fast_test_lane.sh"
            ):
                fail(errors, f"unapproved replacement edge classification: {edge_id}")
        elif isinstance(source, dict) and source.get("external_contract"):
            if (
                source.get("external_contract") != "adl/Cargo.toml target discovery"
                or edge.get("disposition") != "retain"
                or target not in cargo_targets
            ):
                fail(errors, f"unproved external-contract edge: {edge_id}")
    for path, row in by_path.items():
        expected_ids = set(row.get("reference_edge_ids", [])) if isinstance(row.get("reference_edge_ids"), list) else set()
        actual_ids = {str(edge.get("edge_id")) for edge in incoming.get(path, [])}
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
    # A single canonical edge may be discovered by more than one deterministic
    # scanner (for example both an exact path string and a Rust crate path).
    # Edge identity intentionally excludes the evidence label, so prove that
    # every exact-path observation resolves to a retained canonical edge rather
    # than requiring one scanner's label to win deduplication.
    actual_pairs = {
        (str(edge["source"]["path"]), str(edge["target"]))
        for edge in edge_rows
        if isinstance(edge, dict)
        and isinstance(edge.get("source"), dict)
        and edge["source"].get("path")
    }
    if not expected_pairs.issubset(actual_pairs):
        missing_pairs = sorted(expected_pairs - actual_pairs)[:5]
        fail(errors, f"exact tracked-path reference census differs from pinned Git scan: missing={missing_pairs!r}")

    module_targets: dict[str, str] = {}
    for path in observed:
        relative = path.removeprefix("adl/src/")
        if "/" not in relative and relative.endswith(".rs"):
            module_targets[relative[:-3]] = path
        elif relative.endswith("/mod.rs") and relative.count("/") == 1:
            module_targets[relative.split("/", 1)[0]] = path
    expected_crate_pairs: set[tuple[str, str]] = set()
    for source_path in sorted(path for path in tree_rows if path.endswith(".rs")):
        content = git_bytes(root, "show", f"{BASE}:{source_path}")
        source = pathlib.PurePosixPath(source_path)
        module_dir = source.parent if source.name == "mod.rs" else source.with_suffix("")
        for match in MOD_DECL.findall(content):
            module = match.decode()
            target = next(
                (
                    candidate
                    for candidate in (
                        str(module_dir / f"{module}.rs"),
                        str(module_dir / module / "mod.rs"),
                    )
                    if candidate in observed
                ),
                None,
            )
            if target and target != source_path:
                expected_crate_pairs.add((source_path, target))
        for match in CRATE_PATH.findall(content):
            target = module_targets.get(match.decode())
            if target and target != source_path:
                expected_crate_pairs.add((source_path, target))
    actual_crate_pairs = {
        (str(edge["source"]["path"]), str(edge["target"]))
        for edge in edge_rows
        if isinstance(edge, dict)
        and edge.get("reference_class") == "module"
        and isinstance(edge.get("source"), dict)
        and edge["source"].get("path")
    }
    if not expected_crate_pairs.issubset(actual_crate_pairs):
        missing_pairs = sorted(expected_crate_pairs - actual_crate_pairs)[:5]
        fail(errors, f"Rust crate-path reference census differs from pinned Git scan: missing={missing_pairs!r}")
    if scan.get("rust_crate_path_edges") != len(expected_crate_pairs):
        fail(errors, "Rust crate-path edge denominator mismatch")

    if report.get("schema") != "adl.issue309.reduction.v1" or report.get("baseline_commit") != BASE:
        fail(errors, "reduction report identity invalid")
    if report.get("status") != "complete_dead_code_reduction":
        fail(errors, "reduction report status is not complete")
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
    report_bands = report.get("bands")
    if not isinstance(report_bands, list) or not report_bands:
        fail(errors, "reduction report bands missing")
        report_bands = []
    report_band_ids: set[str] = set()
    declared_deleted: set[str] = set()
    declared_band_modified: set[str] = set()
    for band in report_bands:
        if not isinstance(band, dict):
            fail(errors, "reduction report band is not an object")
            continue
        band_id = band.get("band")
        band_paths = band.get("paths")
        if band_id not in {"A", "B", "C"} or band_id in report_band_ids:
            fail(errors, f"invalid or duplicate reduction report band: {band_id}")
        else:
            report_band_ids.add(band_id)
        if not isinstance(band_paths, list) or not band_paths or not all(isinstance(path, str) for path in band_paths):
            fail(errors, f"reduction report band paths missing: {band_id}")
            continue
        overlap = declared_deleted & set(band_paths)
        if overlap:
            fail(errors, f"deleted paths appear in multiple bands: {sorted(overlap)!r}")
        declared_deleted.update(band_paths)
        band_modified = band.get("modified_paths")
        if not isinstance(band_modified, list) or not all(isinstance(path, str) for path in band_modified):
            fail(errors, f"reduction report band modified paths missing: {band_id}")
        else:
            declared_band_modified.update(band_modified)
    declared_modified = set(report.get("modified_files", [])) if isinstance(report.get("modified_files"), list) else set()
    if declared_band_modified != declared_modified:
        fail(errors, "per-band modified path coverage differs from reduction report")
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
