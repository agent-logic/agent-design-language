#!/usr/bin/env python3
"""Merge isolated ADL coverage summaries by source-tree ownership."""

from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import posixpath
import tempfile
from typing import Any


REQUIRED_METRICS = (
    "branches",
    "functions",
    "instantiations",
    "lines",
    "regions",
)
METRICS = ("branches", "mcdc", "functions", "instantiations", "lines", "regions")
NOT_COVERED_METRICS = {"branches", "mcdc", "regions"}


class SummaryError(ValueError):
    pass


def fail(message: str) -> SummaryError:
    return SummaryError(message)


def read_document(path: Path, label: str) -> dict[str, Any]:
    try:
        with path.open(encoding="utf-8") as stream:
            document = json.load(stream)
    except FileNotFoundError as error:
        raise fail(f"{label} summary is missing: {path}") from error
    except (OSError, json.JSONDecodeError) as error:
        raise fail(f"{label} summary is not valid JSON: {path}: {error}") from error

    if not isinstance(document, dict):
        raise fail(f"{label} summary root must be an object")
    data = document.get("data")
    if not isinstance(data, list) or len(data) != 1 or not isinstance(data[0], dict):
        raise fail(f"{label} summary must contain exactly one data document")
    files = data[0].get("files")
    if not isinstance(files, list) or not files:
        raise fail(f"{label} summary files must be a non-empty array")
    if not isinstance(data[0].get("totals"), dict):
        raise fail(f"{label} summary totals must be an object")
    return document


def validate_metric(metric: Any, context: str) -> tuple[int, int]:
    if not isinstance(metric, dict):
        raise fail(f"{context} must be an object")
    count = metric.get("count")
    covered = metric.get("covered")
    if (
        isinstance(count, bool)
        or not isinstance(count, int)
        or isinstance(covered, bool)
        or not isinstance(covered, int)
        or count < 0
        or covered < 0
        or covered > count
    ):
        raise fail(f"{context} must have integer 0 <= covered <= count")
    return count, covered


def canonical_owned_filename(filename: str, ownership_segment: str) -> str | None:
    lexical = filename.replace("\\", "/")
    marker = ownership_segment if ownership_segment.startswith("/") else f"/{ownership_segment}"
    relative_marker = marker.lstrip("/")

    marker_index = lexical.find(marker)
    prefix = lexical[:marker_index] if marker_index >= 0 else lexical
    if ".." in prefix.split("/"):
        raise fail(f"coverage filename escapes repository root: {filename}")
    if marker_index >= 0:
        depth = 0
        for component in lexical[marker_index + len(marker) :].split("/"):
            if component in ("", "."):
                continue
            if component == "..":
                if depth == 0:
                    raise fail(f"coverage filename escapes owned source root: {filename}")
                depth -= 1
            else:
                depth += 1

    normalized = posixpath.normpath(lexical)
    if normalized == ".." or normalized.startswith("../"):
        raise fail(f"coverage filename escapes repository root: {filename}")
    if normalized.startswith(relative_marker):
        return f"/{normalized}"
    normalized_marker_index = normalized.find(marker)
    if normalized_marker_index >= 0:
        return normalized[normalized_marker_index:]
    if marker_index >= 0:
        raise fail(f"coverage filename escapes owned source root: {filename}")
    return None


def owned_files(
    document: dict[str, Any], label: str, ownership_segment: str
) -> list[dict[str, Any]]:
    selected: list[dict[str, Any]] = []
    seen: set[str] = set()
    for index, file_summary in enumerate(document["data"][0]["files"]):
        context = f"{label} file[{index}]"
        if not isinstance(file_summary, dict):
            raise fail(f"{context} must be an object")
        filename = file_summary.get("filename")
        if not isinstance(filename, str) or not filename:
            raise fail(f"{context} filename must be a non-empty string")
        normalized = filename.replace("\\", "/")
        if normalized in seen:
            raise fail(f"{label} summary contains duplicate filename: {filename}")
        seen.add(normalized)
        summary = file_summary.get("summary")
        if not isinstance(summary, dict):
            raise fail(f"{context} summary must be an object")
        for metric_name in REQUIRED_METRICS:
            validate_metric(summary.get(metric_name), f"{context} summary.{metric_name}")
        if "mcdc" in summary:
            validate_metric(summary["mcdc"], f"{context} summary.mcdc")
        canonical_filename = canonical_owned_filename(filename, ownership_segment)
        if canonical_filename is not None:
            canonical_summary = dict(file_summary)
            canonical_summary["filename"] = canonical_filename
            selected.append(canonical_summary)

    if not selected:
        raise fail(f"{label} summary has no files owned by {ownership_segment}")
    return selected


def recompute_totals(files: list[dict[str, Any]]) -> dict[str, dict[str, int | float]]:
    totals: dict[str, dict[str, int | float]] = {}
    for metric_name in METRICS:
        count = 0
        covered = 0
        for file_summary in files:
            metric = file_summary["summary"].get(metric_name)
            if metric_name == "mcdc" and metric is None:
                continue
            metric_count, metric_covered = validate_metric(
                metric,
                f"{file_summary['filename']} summary.{metric_name}",
            )
            count += metric_count
            covered += metric_covered
        metric_total: dict[str, int | float] = {
            "count": count,
            "covered": covered,
            "percent": 0.0 if count == 0 else (covered * 100.0) / count,
        }
        if metric_name in NOT_COVERED_METRICS:
            metric_total["notcovered"] = count - covered
        totals[metric_name] = metric_total
    return totals


def atomic_write_json(path: Path, document: dict[str, Any]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = json.dumps(document, indent=2, sort_keys=True, allow_nan=False) + "\n"
    temp_path: Path | None = None
    try:
        with tempfile.NamedTemporaryFile(
            mode="w", encoding="utf-8", dir=path.parent, prefix=f".{path.name}.", delete=False
        ) as stream:
            temp_path = Path(stream.name)
            stream.write(payload)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temp_path, path)
    finally:
        if temp_path is not None and temp_path.exists():
            temp_path.unlink()


def merge(workspace_path: Path, runtime_path: Path, output_path: Path) -> None:
    workspace = read_document(workspace_path, "workspace")
    runtime = read_document(runtime_path, "adl-runtime")
    files = owned_files(workspace, "workspace", "/adl/src/")
    files.extend(owned_files(runtime, "adl-runtime", "/adl-runtime/src/"))

    filenames = [file_summary["filename"].replace("\\", "/") for file_summary in files]
    if len(filenames) != len(set(filenames)):
        raise fail("owned coverage summaries contain duplicate filenames")

    files.sort(key=lambda file_summary: file_summary["filename"].replace("\\", "/"))
    merged = dict(workspace)
    merged_data = dict(workspace["data"][0])
    merged_data["files"] = files
    merged_data["totals"] = recompute_totals(files)
    merged["data"] = [merged_data]
    atomic_write_json(output_path, merged)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--workspace", required=True, type=Path)
    parser.add_argument("--adl-runtime", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    try:
        merge(args.workspace, args.adl_runtime, args.output)
    except (SummaryError, OSError, ValueError) as error:
        print(f"coverage summary merge failed: {error}", file=os.sys.stderr)
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
