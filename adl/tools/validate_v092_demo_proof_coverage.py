#!/usr/bin/env python3
"""Fail-closed validator for v0.92 WP-20 demo/proof coverage truth."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path


VALID_STATUSES = {
    "accepted",
    "blocked_with_evidence",
    "deferred_non_claim",
    "planned",
}

EXACT_REVISION = re.compile(r"[0-9a-f]{40}")
ACCEPTED_REVIEW_STATES = {"reviewed_pass"}

DEMO = Path("docs/milestones/v0.92/DEMO_MATRIX_v0.92.md")
COVERAGE = Path("docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md")
LEDGER = Path("docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md")
INDEX = Path("docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md")
BASE_REVISIONS = Path(".csdlc/evidence/308/exact-base-revisions.txt")

PREDECESSOR_GATE = {
    "#256": {
        "repository": "agent-logic/agent-design-language",
        "pull_request": "427",
        "canonical_generation": "22",
        "canonical_digest": "f96cf1052c911a104f2a8a228cf83ffb345990ca2160b0b70032f002ba5bc671",
        "merge_sha": "fb4c853bdb9cb140059d2a28af02d70bd36a27a4",
        "retained_evidence_ref": ".csdlc/evidence/308/predecessors/256-derived-terminal.json",
    },
    "#340": {
        "repository": "agent-logic/agent-design-language",
        "pull_request": "430",
        "canonical_generation": "39",
        "canonical_digest": "b59cd71fa9399f166366e42b84503b78e7dfce46f93096f116f2c613f5cc3bd8",
        "merge_sha": "aa36a828793366f92d0d9e16247bd3fb1cce7878",
        "retained_evidence_ref": ".csdlc/evidence/308/predecessors/340-derived-terminal.json",
    },
    "#341": {
        "repository": "agent-logic/agent-design-language",
        "pull_request": "442",
        "canonical_generation": "2",
        "canonical_digest": "31d3e54f171dbd164ef1de1bc4504110e0de1f3de44c206eb667770c4dad1141",
        "merge_sha": "0b5aadebd7cff653c2500106d4a4055f1b9b8818",
        "retained_evidence_ref": ".csdlc/evidence/308/predecessors/341-derived-terminal.json",
    },
    "#5839": {
        "repository": "danielbaustin/agent-design-language",
        "pull_request": "289",
        "canonical_generation": "38",
        "canonical_digest": "93a28ce3d40b0eb0444c1c7d011401856db489075da1cbf6221f45a74546fa7e",
        "merge_sha": "7f88697ce82215188af941e15cf02a6220c9ad63",
        "retained_evidence_ref": ".csdlc/evidence/308/predecessors/5839-derived-terminal.json",
    },
}


def fail(message: str) -> None:
    raise SystemExit(f"v0.92 demo proof coverage validation failed: {message}")


def read(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except FileNotFoundError as exc:
        fail(f"missing required file: {path}")
        raise AssertionError from exc


def split_row(line: str) -> list[str]:
    return [cell.strip() for cell in line.strip().strip("|").split("|")]


def markdown_tables(text: str) -> list[list[dict[str, str]]]:
    tables: list[list[dict[str, str]]] = []
    lines = text.splitlines()
    index = 0
    while index + 1 < len(lines):
        if not lines[index].lstrip().startswith("|"):
            index += 1
            continue
        header = split_row(lines[index])
        separator = split_row(lines[index + 1])
        if not all(re.fullmatch(r":?-{3,}:?", cell) for cell in separator):
            index += 1
            continue
        index += 2
        rows: list[dict[str, str]] = []
        while index < len(lines) and lines[index].lstrip().startswith("|"):
            cells = split_row(lines[index])
            if len(cells) != len(header):
                fail(f"table row has {len(cells)} cells but header has {len(header)}: {lines[index]}")
            rows.append(dict(zip(header, cells)))
            index += 1
        tables.append(rows)
    return tables


def table_with_columns(path: Path, required: set[str]) -> list[dict[str, str]]:
    text = read(path)
    for table in markdown_tables(text):
        if table and required.issubset(table[0].keys()):
            return table
    fail(f"missing table with columns {sorted(required)} in {path}")


def normalize_status(value: str, context: str) -> str:
    status = value.strip("` ").lower()
    if status not in VALID_STATUSES:
        fail(f"{context} has unsupported status {value!r}")
    return status


def ensure_no_forbidden_claims(path: Path) -> None:
    text = read(path).lower()
    forbidden_patterns = [
        "synthetic proof",
        "synthetic success",
        "provider substitution",
        "unsupported platform claim",
        "unsupported platform claims",
        "planned as passed",
    ]
    if path == INDEX:
        # The index names these phrases only as rejected validator conditions.
        text = re.sub(r"the validator must fail closed when:.*", "", text, flags=re.DOTALL)
    for phrase in forbidden_patterns:
        start = text.find(phrase)
        if start == -1:
            continue
        context = text[max(0, start - 160): start]
        guarded = any(
            marker in context
            for marker in (
                "reject",
                "fail",
                "must not",
                "non-claim",
                "non-accepted",
                "without",
                "lacks",
                "relies on",
            )
        )
        if not guarded:
            fail(f"{path} contains unguarded forbidden claim phrase: {phrase}")


def artifact_exists(root: Path, artifact_cell: str, context: str) -> None:
    for raw in artifact_cell.split(";"):
        value = raw.strip().strip("`")
        if not value or value.startswith("pending-"):
            fail(f"{context} has non-concrete artifact value {value!r}")
        if value.startswith("http://") or value.startswith("https://"):
            continue
        if not value.startswith(".csdlc/evidence/"):
            fail(f"{context} must cite retained evidence, not source/docs path {value!r}")
        candidate = root / value
        if not candidate.exists():
            fail(f"{context} points at missing artifact {value}")


def validate_accepted_revision(value: str, context: str) -> None:
    revision = value.strip().strip("`")
    if not EXACT_REVISION.fullmatch(revision):
        fail(f"{context} exact revision must be an immutable 40-hex commit, got {value!r}")


def validate_accepted_review_state(value: str, context: str) -> None:
    state = value.strip("` ").lower()
    if state not in ACCEPTED_REVIEW_STATES:
        fail(f"{context} review state must be one of {sorted(ACCEPTED_REVIEW_STATES)}, got {value!r}")


def validate_predecessor_gate(root: Path) -> None:
    evidence = read(root / BASE_REVISIONS)
    for issue, required in PREDECESSOR_GATE.items():
        header = f"issue: {issue}"
        if header not in evidence:
            fail(f"predecessor gate evidence does not label {issue}")
        issue_section = evidence.split(header, 1)[1].split("\n\n", 1)[0]
        for field, expected in required.items():
            needle = f"{field}: {expected}"
            if needle not in issue_section:
                fail(f"predecessor gate evidence for {issue} lacks {needle}")
        for needle in (
            "disposition: merged",
            "terminal_state: closed_by_merged_pr",
            "reconciled_state: typed_derived_terminal_reconciled",
            "ancestor_of_308_base: true",
        ):
            if needle not in issue_section:
                fail(f"predecessor gate evidence for {issue} lacks {needle}")
        retained_ref = required["retained_evidence_ref"]
        retained_text = read(root / retained_ref)
        for needle in (
            '"schema": "csdlc.derived_terminal.v1"',
            f'"issue": {issue.strip("#")}',
            f'"repository": "{required["repository"]}"',
            f'"canonical_generation": {required["canonical_generation"]}',
            f'"canonical_digest": "{required["canonical_digest"]}"',
            f'"pull_request": {required["pull_request"]}',
            '"disposition": "merged"',
            f'"merge_sha": "{required["merge_sha"]}"',
            '"issue_state": "closed_by_merged_pr"',
        ):
            if needle not in retained_text:
                fail(f"retained predecessor evidence {retained_ref} for {issue} lacks {needle}")


def normalized_command(value: str) -> str:
    return " ".join(value.strip("` ").split())


def validate(root: Path) -> None:
    paths = [DEMO, COVERAGE, LEDGER, INDEX]
    for rel in paths:
        read(root / rel)
        ensure_no_forbidden_claims(root / rel)

    validate_predecessor_gate(root)

    index_rows = table_with_columns(
        root / INDEX,
        {"Row", "Owner", "Surface", "Status", "Exact revision", "Positive artifact", "Negative artifact", "Review state", "Command"},
    )
    by_row: dict[str, dict[str, str]] = {}
    accepted_surfaces: dict[tuple[str, str], str] = {}
    for row in index_rows:
        row_id = row["Row"]
        if row_id in by_row:
            fail(f"duplicate artifact-index row: {row_id}")
        by_row[row_id] = row
        status = normalize_status(row["Status"], f"artifact index {row_id}")
        if status == "accepted":
            surface_key = (row["Owner"], row["Surface"])
            if surface_key in accepted_surfaces:
                fail(
                    f"accepted artifact index rows {accepted_surfaces[surface_key]} and {row_id} "
                    f"duplicate owner/surface {surface_key!r}"
                )
            accepted_surfaces[surface_key] = row_id
            for column in ("Exact revision", "Positive artifact", "Negative artifact", "Review state", "Command"):
                value = row[column].strip()
                if not value or value.startswith("pending-"):
                    fail(f"accepted artifact index {row_id} lacks concrete {column}")
            validate_accepted_revision(row["Exact revision"], f"accepted artifact index {row_id}")
            validate_accepted_review_state(row["Review state"], f"accepted artifact index {row_id}")
            artifact_exists(root, row["Positive artifact"], f"accepted artifact index {row_id} positive proof")
            artifact_exists(root, row["Negative artifact"], f"accepted artifact index {row_id} negative proof")

    coverage_rows = table_with_columns(
        root / COVERAGE,
        {"Outcome", "Owner", "Proof route", "Status", "Artifact index row"},
    )
    for row in coverage_rows:
        row_id = row["Artifact index row"]
        if row_id not in by_row:
            fail(f"coverage row {row['Outcome']!r} references missing artifact row {row_id}")
        status = normalize_status(row["Status"], f"coverage row {row['Outcome']!r}")
        index_status = normalize_status(by_row[row_id]["Status"], f"artifact index {row_id}")
        if status != index_status:
            fail(f"coverage row {row_id} status {status} does not match artifact index status {index_status}")
        if row["Owner"] != by_row[row_id]["Owner"]:
            fail(f"coverage row {row_id} owner {row['Owner']!r} does not match artifact index owner {by_row[row_id]['Owner']!r}")

    demo_rows = table_with_columns(
        root / DEMO,
        {
            "Demo ID",
            "Demo / proof surface",
            "Milestone claim",
            "Primary proof surface",
            "Owner",
            "Status",
            "Exact revision",
            "Command",
            "Artifact index row",
        },
    )
    seen_demo_ids: set[str] = set()
    for row in demo_rows:
        demo_id = row["Demo ID"]
        if demo_id in seen_demo_ids:
            fail(f"duplicate demo row id: {demo_id}")
        seen_demo_ids.add(demo_id)
        row_id = row["Artifact index row"]
        if row_id not in by_row:
            fail(f"demo row {demo_id} references missing artifact row {row_id}")
        status = normalize_status(row["Status"], f"demo row {demo_id}")
        index_status = normalize_status(by_row[row_id]["Status"], f"artifact index {row_id}")
        if status == "accepted" and index_status != "accepted":
            fail(f"demo row {demo_id} is accepted but artifact index row {row_id} is {index_status}")
        if status != "accepted" and index_status == "accepted" and demo_id != "D9":
            fail(f"demo row {demo_id} is non-accepted but artifact index row {row_id} is accepted")
        if row["Owner"] != by_row[row_id]["Owner"]:
            fail(f"demo row {demo_id} owner {row['Owner']!r} does not match artifact index owner {by_row[row_id]['Owner']!r}")
        if row["Exact revision"] != by_row[row_id]["Exact revision"]:
            fail(
                f"demo row {demo_id} exact revision {row['Exact revision']!r} "
                f"does not match artifact index exact revision {by_row[row_id]['Exact revision']!r}"
            )
        if normalized_command(row["Command"]) != normalized_command(by_row[row_id]["Command"]):
            fail(f"demo row {demo_id} command does not match artifact index command")

    ledger_rows = table_with_columns(
        root / LEDGER,
        {"Artifact index row", "Owner", "Status", "Exact revision", "Command"},
    )
    seen_ledger_rows: set[str] = set()
    for row in ledger_rows:
        row_id = row["Artifact index row"]
        if row_id in seen_ledger_rows:
            fail(f"duplicate activation ledger row: {row_id}")
        seen_ledger_rows.add(row_id)
        if row_id not in by_row:
            fail(f"activation ledger row references missing artifact row {row_id}")
        ledger_status = normalize_status(row["Status"], f"activation ledger row {row_id}")
        index_status = normalize_status(by_row[row_id]["Status"], f"artifact index {row_id}")
        if ledger_status != index_status:
            fail(f"activation ledger row {row_id} status {ledger_status} does not match artifact index status {index_status}")
        if row["Owner"] != by_row[row_id]["Owner"]:
            fail(f"activation ledger row {row_id} owner {row['Owner']!r} does not match artifact index owner {by_row[row_id]['Owner']!r}")
        if row["Exact revision"] != by_row[row_id]["Exact revision"]:
            fail(
                f"activation ledger row {row_id} exact revision {row['Exact revision']!r} "
                f"does not match artifact index exact revision {by_row[row_id]['Exact revision']!r}"
            )
        if normalized_command(row["Command"]) != normalized_command(by_row[row_id]["Command"]):
            fail(f"activation ledger row {row_id} command does not match artifact index command")

    if "agent-logic/agent-design-language#308" not in read(root / DEMO):
        fail("demo matrix does not bind current issue #308")
    if "agent-logic/agent-design-language#308" not in read(root / INDEX):
        fail("artifact index does not bind current issue #308")
    if "AEE-018" not in seen_ledger_rows:
        fail("activation ledger does not bind WP-20 artifact row AEE-018")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--root", default=".", help="repository root")
    args = parser.parse_args()
    root = Path(args.root).resolve()
    validate(root)
    print("v0.92 demo proof coverage validation: PASS")
    return 0


if __name__ == "__main__":
    sys.exit(main())
