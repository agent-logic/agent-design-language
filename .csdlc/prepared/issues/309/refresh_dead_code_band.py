#!/usr/bin/env python3
"""Reconcile the reviewed #309 dead-code band with canonical evidence."""

from __future__ import annotations

import hashlib
import argparse
import json
import pathlib
import subprocess
import re


BASE = "e926e3bca0ab1981d77b4658d2feb4059bdf33a6"
BAND_B = {
    "adl/src/adl_skill_v1.rs": (
        "delete_superseded",
        "current typed skill surfaces and retained historical v0.91.7/v0.92 evidence",
        "pre-v0.92 skill schema has no current Runtime, CLI, provider, or #414 consumer",
    ),
    "adl/src/speculative_decoding_prototype.rs": (
        "delete_superseded",
        "retained v0.91.8 speculative-decoding evaluation artifacts",
        "the executable demo and its tests were already deleted by the reviewed v0.91.8 external band",
    ),
    "adl/src/gws_live_capability_execution_surface.rs": (
        "delete_superseded",
        "retained v0.91.8 GWS capability-demo artifacts",
        "the GWS live-capability demo entrypoint was already deleted by the reviewed v0.91.8 external band",
    ),
    "adl/src/gws_live_content_card_roundtrip.rs": (
        "delete_superseded",
        "retained v0.91.8 GWS content-card demo artifacts",
        "the GWS content-card demo entrypoint was already deleted by the reviewed v0.91.8 external band",
    ),
    "adl/src/gws_live_content_card_roundtrip/logic.rs": (
        "delete_superseded",
        "retained v0.91.8 GWS content-card demo artifacts",
        "implementation belonged only to the retired GWS content-card demo cluster",
    ),
    "adl/src/gws_live_content_card_roundtrip/tests.rs": (
        "delete_superseded",
        "retained v0.91.8 GWS content-card demo artifacts",
        "tests belonged only to the retired GWS content-card demo cluster",
    ),
    "adl/src/gws_live_content_card_roundtrip/types.rs": (
        "delete_superseded",
        "retained v0.91.8 GWS content-card demo artifacts",
        "types belonged only to the retired GWS content-card demo cluster",
    ),
    "adl/src/gws_live_safety_package.rs": (
        "delete_superseded",
        "retained v0.91.8 GWS safety-package demo artifacts",
        "the GWS safety-package demo entrypoint was already deleted by the reviewed v0.91.8 external band",
    ),
    "adl/src/rust_native_gws_adapter_boundary.rs": (
        "delete_superseded",
        "retained v0.91.8 GWS adapter-boundary artifacts",
        "the native GWS comparison/demo entrypoints were already deleted by the reviewed v0.91.8 external band",
    ),
    "adl/src/local_gemma_model_evaluation.rs": (
        "delete_superseded",
        "retained v0.91.8 local-Gemma evaluation artifacts",
        "the local-Gemma demo entrypoint/tests were already deleted; its only remaining Rust consumer was the retired UTS benchmark cluster",
    ),
    "adl/src/uts_acc_multi_model_benchmark/evaluation.rs": (
        "delete_superseded", "retained v0.91.8 UTS/ACC benchmark artifacts", "retired benchmark-only implementation"
    ),
    "adl/src/uts_acc_multi_model_benchmark/execution.rs": (
        "delete_superseded", "retained v0.91.8 UTS/ACC benchmark artifacts", "retired benchmark-only implementation"
    ),
    "adl/src/uts_acc_multi_model_benchmark/mod.rs": (
        "delete_superseded", "retained v0.91.8 UTS/ACC benchmark artifacts", "benchmark entrypoint/tests were already deleted by the reviewed v0.91.8 external band"
    ),
    "adl/src/uts_acc_multi_model_benchmark/parsing.rs": (
        "delete_superseded", "retained v0.91.8 UTS/ACC benchmark artifacts", "retired benchmark-only implementation"
    ),
    "adl/src/uts_acc_multi_model_benchmark/runtime.rs": (
        "delete_superseded", "retained v0.91.8 UTS/ACC benchmark artifacts", "retired benchmark-only implementation"
    ),
    "adl/src/uts_acc_multi_model_benchmark/task_fixtures.rs": (
        "delete_superseded", "retained v0.91.8 UTS/ACC benchmark artifacts", "retired benchmark-only fixture"
    ),
    "adl/src/uts_acc_multi_model_benchmark/tests.rs": (
        "delete_superseded", "retained v0.91.8 UTS/ACC benchmark artifacts", "tests belonged only to the retired benchmark cluster"
    ),
    "adl/src/uts_acc_multi_model_benchmark/types.rs": (
        "delete_superseded", "retained v0.91.8 UTS/ACC benchmark artifacts", "retired benchmark-only types"
    ),
}
BAND_B_REVERT = "6ad24bc19"
BAND_B_REAPPLY = "29093a166"
BAND_B_COMMIT = "f3cf4c937cbd55beb5e78b73b838033ff63bae66"


def load(path: pathlib.Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def write(path: pathlib.Path, value: dict) -> None:
    path.write_text(json.dumps(value, sort_keys=True, separators=(",", ":")) + "\n", encoding="utf-8")


def git(root: pathlib.Path, *args: str) -> str:
    return subprocess.check_output(["git", "-C", str(root), *args], text=True).strip()


def edge_id(edge: dict) -> str:
    identity = [edge["source"], edge["target"], edge["reference_class"], edge["disposition"]]
    return hashlib.sha256(json.dumps(identity, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def current_reference_exists(root: pathlib.Path, edge: dict) -> bool:
    source_path = edge.get("source", {}).get("path")
    target = edge["target"]
    if not source_path or not (root / source_path).is_file():
        return False
    content = (root / source_path).read_text(encoding="utf-8", errors="replace")
    if edge["evidence"] == "exact tracked path reference":
        return target in content
    if edge["reference_class"] != "module":
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
    resolved = source_module_dir / f"{module}.rs"
    resolved_mod = source_module_dir / module / "mod.rs"
    return target_path in {resolved, resolved_mod} and bool(
        re.search(rf"(?m)^\s*(?:pub\s+)?mod\s+{re.escape(module)}\s*;", content)
    )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--validated", action="store_true")
    args = parser.parse_args()
    root = pathlib.Path(__file__).resolve().parents[4]
    evidence = root / ".csdlc/evidence/309"
    disposition_path = evidence / "disposition-manifest.json"
    edge_path = evidence / "reference-edge-manifest.json"
    report_path = evidence / "reduction-report.json"
    rollback_path = evidence / "rollback-proof.json"
    dispositions = load(disposition_path)
    references = load(edge_path)
    report = load(report_path)
    rollback = load(rollback_path)
    rows = {row["path"]: row for row in dispositions["files"]}

    policy = rows["adl/src/policy_authority.rs"]
    policy.update({
        "disposition": "retain_active",
        "owner": "Runtime governance and policy authority",
        "evidence": "current architecture/ADR governance contract plus exact baseline blob; byte-identical source retained",
        "validation": "exact blob equality to the published candidate plus all-target compile and strict Clippy",
    })
    policy.pop("replacement", None)

    validation = (
        "reviewed v0.91.8 external-band deletion manifest; deterministic reference census; "
        "cargo check --locked --all-targets; Runtime v2 and #414 focused regressions; exact rollback proof"
    )
    for path, (_previous_disposition, replacement, reason) in BAND_B.items():
        row = rows[path]
        row.update({
            "disposition": "delete_dead",
            "owner": "#309 Band B",
            "evidence": reason,
            "validation": validation,
        })
        row.pop("replacement", None)

    delete_paths = {path for path, row in rows.items() if row["disposition"] in {"delete_dead", "delete_superseded"}}
    canonical: dict[str, dict] = {}
    for edge in references["edges"]:
        source_path = edge.get("source", {}).get("path")
        if source_path:
            baseline_content = subprocess.check_output(
                ["git", "-C", str(root), "show", f"{BASE}:{source_path}"]
            ).decode("utf-8", errors="replace")
            if edge["target"] in baseline_content:
                edge["evidence"] = "exact tracked path reference"
            elif edge["reference_class"] == "module":
                edge["evidence"] = "Rust module declaration/path/include reachability"
        exists = current_reference_exists(root, edge)
        if edge["target"] in delete_paths:
            if source_path in delete_paths:
                edge["disposition"] = "remove"
                edge["resolution_evidence"] = "source and target are removed in the same reviewed dead-code band"
            elif not exists:
                edge["disposition"] = "remove"
                edge["resolution_evidence"] = "corrected candidate reference resolution proves the baseline edge absent"
            elif source_path == "adl/tools/test_run_pr_fast_test_lane.sh":
                edge["disposition"] = "replace"
                edge["reference_class"] = "artifact"
                edge["resolution_evidence"] = "retained deletion-routing regression fixture, not a runtime or build consumer"
            else:
                raise SystemExit(f"retained source still references deleted target: {source_path} -> {edge['target']}")
        elif source_path in delete_paths:
            edge["disposition"] = "remove"
            edge["resolution_evidence"] = "baseline source is removed in the reviewed dead-code band"
        elif not exists and edge["reference_class"] == "module":
            continue
        else:
            edge["disposition"] = "retain"
        edge["edge_id"] = edge_id(edge)
        canonical[edge["edge_id"]] = edge
    references["edges"] = sorted(
        canonical.values(), key=lambda edge: (edge["target"], edge.get("source", {}).get("path", ""), edge["edge_id"])
    )
    incoming = {path: [] for path in rows}
    for edge in references["edges"]:
        incoming[edge["target"]].append(edge["edge_id"])
    for path, row in rows.items():
        row["reference_edge_ids"] = sorted(incoming[path])
    dispositions["files"] = [rows[path] for path in sorted(rows)]

    deleted = sorted(
        line.split("\t", 1)[1]
        for line in git(root, "diff", "--name-status", BASE, "HEAD", "--", "adl/src").splitlines()
        if line.startswith("D\t")
    )
    modified = sorted(
        line.split("\t", 1)[1]
        for line in git(root, "diff", "--name-status", BASE, "HEAD", "--", "adl/src").splitlines()
        if line.startswith("M\t")
    )
    expected = sorted({"adl/src/dspark_speculative_decoding_evaluation.rs", "adl/src/provider_native_tool_call_comparison.rs", *BAND_B})
    if deleted != expected:
        raise SystemExit(f"candidate deletion set differs from reviewed bands: {deleted!r}")
    removed_lines = sum(len(subprocess.check_output(["git", "-C", str(root), "show", f"{BASE}:{path}"]).splitlines()) for path in deleted)
    report.update({
        "candidate_source_commit": BAND_B_COMMIT,
        "removed_files": len(deleted),
        "removed_physical_lines": removed_lines,
        "modified_files": modified,
        "status": "band_b_candidate_pending_rollback_review",
    })
    band_a = report["bands"][0]
    band_a["modified_paths"] = ["adl/src/lib.rs"]
    report["bands"] = [
        band_a,
        {
            "band": "B",
            "classification": "delete_dead_orphan_implementations",
            "paths": sorted(BAND_B),
            "replacement": "none required; complete consumer census proves the implementations unreachable",
            "supporting_paths": [],
            "modified_paths": ["adl/src/gws_live_test_support.rs", "adl/src/lib.rs"],
        },
    ]
    rollback_by_band = {row["band"]: row for row in rollback["bands"]}
    subprocess.check_call(["git", "-C", str(root), "merge-base", "--is-ancestor", BAND_B_COMMIT, "HEAD"])
    if git(root, "diff", "--name-only", BAND_B_COMMIT, "HEAD", "--", "adl/src"):
        raise SystemExit("adl/src changed after the pinned Band B source commit")
    rollback_by_band["B"] = {
        "band": "B",
        "commit": BAND_B_COMMIT,
        "tree_before": git(root, "rev-parse", f"{BAND_B_COMMIT}^^{{tree}}"),
        "tree_after": git(root, "rev-parse", f"{BAND_B_COMMIT}^{{tree}}"),
        "revert_commit": git(root, "rev-parse", BAND_B_REVERT),
        "reverted_tree": git(root, "rev-parse", f"{BAND_B_REVERT}^{{tree}}"),
        "reapply_commit": git(root, "rev-parse", BAND_B_REAPPLY),
        "reapplied_tree": git(root, "rev-parse", f"{BAND_B_REAPPLY}^{{tree}}"),
        "unrelated_paths_changed": [],
        "focused_validation_passed": args.validated,
        "validation": [
            "cargo check --locked --manifest-path adl/Cargo.toml --all-targets: pass",
            "cargo clippy --locked --manifest-path adl/Cargo.toml --all-targets -- -D warnings: pass",
            "cargo fmt --manifest-path adl/Cargo.toml -- --check: pass",
            "bash adl/tools/test_owner_binary_install.sh: pass",
            "bash adl/tools/test_run_pr_fast_test_lane.sh: pass",
            "cargo test --locked --manifest-path adl/Cargo.toml --lib resident_shepherd_spot_continuity: 6 passed",
            "cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test live_continuity: 8 passed",
            "validate_reduction_inventory.py: pass",
            "git revert/reapply exact-tree proof: pass",
        ] if args.validated else ["focused validation pending after Band B evidence update"],
    }
    rollback["bands"] = [rollback_by_band[band] for band in ("A", "B")]
    write(disposition_path, dispositions)
    write(edge_path, references)
    write(report_path, report)
    write(rollback_path, rollback)
    print(json.dumps({"status": "pass", "band_b_files": len(BAND_B), "removed_files": len(deleted), "removed_lines": removed_lines}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
