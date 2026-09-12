#!/usr/bin/env python3
"""PVF docs_only: deterministic local packet contracts, not content acceptance."""
import argparse
import copy
import hashlib
import json
from pathlib import Path
import re
import subprocess

PACKET = Path(__file__).resolve().parent
ROOT = PACKET.parents[4]
EXPECTED = {f"ADR-CF-{i:02}" for i in range(1, 10)} | {
    "ADR-PLAT-01", "ADR-CSDLC-01", "ADR-CSDLC-02"
}
HEADINGS = ["Status", "Decision Question", "Context", "Decision",
            "Alternatives Considered", "Consequences", "Reversibility",
            "Validation Notes", "Supersession Relationships", "Source Evidence",
            "Approval Boundary"]
SOURCE_RECORDS = {
    "0025": ("docs/adr/0025-codefriend-review-packet-product-boundary.md", "accepted"),
    "0058": ("docs/adr/0058-memory-palace-context-handoff-architecture.md", "accepted"),
    "0004": ("docs/adr/0004-provider-profiles.md", "accepted"),
    "0041": ("docs/adr/0041-provider-model-suitability-boundary-v2.md", "accepted"),
    "0075": ("docs/architecture/adr/0075-provider-profile-and-shadow-authority.md", "proposed"),
    "0072": ("docs/architecture/adr/0072-csdlc-v3-native-authority.md", "proposed"),
}
EXPECTED_RELATIONS = {(f"ADR-CF-{i:02}", "0025") for i in range(1, 10)} | {
    ("ADR-CF-07", "0058"),
    ("ADR-PLAT-01", "0004"), ("ADR-PLAT-01", "0041"), ("ADR-PLAT-01", "0075"),
    ("ADR-CSDLC-01", "0072"), ("ADR-CSDLC-02", "0072"),
}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def check_contract(inventory, launch, relations, records):
    candidates = inventory["candidates"]
    require(len(candidates) == 12, "candidate denominator must be 12")
    require({e["candidate"] for e in candidates} == EXPECTED,
            "candidate identities differ from reviewed selection")
    mappings = inventory["task_mapping"]
    require(len(mappings) == len(launch) == 69, "task denominator must be 69")
    require({e["task"]: e["issue"] for e in mappings} == launch,
            "task identities differ from launch map")
    require(all(e["disposition"] for e in mappings), "task lacks disposition")
    for item in candidates:
        ident = item["candidate"]
        require(item["status"] == "proposed", f"{ident}: unexpected acceptance")
        require(item["acceptance_status"] == "pending_operator_decision",
                f"{ident}: acceptance must remain pending")
        owner = item["accountable_scope_owner"]
        require(owner in item["participating_owners"] and owner in launch,
                f"{ident}: missing declared accountable scope owner")
        require(item["source_refs"], f"{ident}: source references missing")
        text = records[ident]
        require(re.search(r"## Status\n\n\*\*Proposed\.\*\*", text),
                f"{ident}: rendered status drift")
        for heading in HEADINGS:
            require(f"## {heading}\n" in text, f"{ident}: missing {heading}")
        require(f"{owner} (#{launch[owner]})" in text,
                f"{ident}: rendered accountability missing")
        require(inventory["source_revision"] in text,
                f"{ident}: source revision absent")
        require(not re.search(r"\b(?:TODO|TBD)\b|<[a-z_]+>", text),
                f"{ident}: unresolved authoring placeholder")
    require(relations["supersessions"] == [], "unapproved supersession")
    seen = set()
    for edge in relations["relationships"]:
        pair = (edge["candidate"], edge["source_record"])
        require(pair not in seen, "duplicate refinement relationship")
        seen.add(pair)
        require(edge["candidate"] in EXPECTED, "unknown refinement candidate")
        require(edge["relation"] == "proposed_refinement" and
                edge["supersession_enacted"] is False, "unapproved relation")
        require(edge["source_record"] in SOURCE_RECORDS, "unknown source record")
        expected_path, expected_status = SOURCE_RECORDS[edge["source_record"]]
        require(edge["source_path"] == expected_path, "source relationship path drift")
        require(edge["source_status"] == expected_status, "source status drift")
    require(seen == EXPECTED_RELATIONS, "reviewed refinement mapping is incomplete or changed")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    inventory = json.loads((PACKET / "decision-inventory.json").read_text())
    launch = json.loads((ROOT / ".csdlc/evidence/864/all-issue-launch.json").read_text())["all_bindings"]
    relations = json.loads((PACKET / "relationships.json").read_text())
    records = {e["candidate"]: (ROOT / e["path"]).read_text() for e in inventory["candidates"]}
    check_contract(inventory, launch, relations, records)
    manifest = json.loads((PACKET / "source-manifest.json").read_text())
    require(manifest["revision"] == inventory["source_revision"], "source revision mismatch")
    paths = [entry["path"] for entry in manifest["sources"]]
    require(len(paths) == len(set(paths)), "duplicate source manifest path")
    for entry in manifest["sources"]:
        # Check pinned Git blobs: current ADR_PLAN intentionally links this packet.
        data = subprocess.check_output(["git", "-C", str(ROOT), "show",
                                        f'{manifest["revision"]}:{entry["path"]}'])
        require(len(data) == entry["bytes"] and hashlib.sha256(data).hexdigest() == entry["sha256"],
                f'source digest mismatch: {entry["path"]}')
    require({src for e in inventory["candidates"] for src in e["source_refs"]} <= set(paths),
            "candidate source missing from pinned manifest")
    md_files = list(PACKET.glob("*.md")) + list((ROOT / "docs/architecture/adr/issue-911").glob("*.md"))
    for path in md_files:
        text = path.read_text()
        require("/Users/" not in text and "/Volumes/" not in text, "host path in packet")
        require(all(line == line.rstrip() for line in text.splitlines()), "trailing whitespace")
        for href in re.findall(r"\]\(([^)]+)\)", text):
            if href.startswith(("https://", "http://", "#")):
                continue
            target = (path.parent / href.split("#")[0]).resolve()
            require(target.is_relative_to(ROOT) and target.is_file(), f"bad local link: {href}")
    negatives = 0
    if args.self_test:
        mutations = [
            lambda d: d["candidates"].pop(),
            lambda d: d["candidates"][0].update(status="accepted"),
            lambda d: d["candidates"][0].update(accountable_scope_owner=""),
            lambda d: d["task_mapping"].pop(),
            lambda d: d["task_mapping"][0].update(issue=999999),
        ]
        for mutate in mutations:
            bad = copy.deepcopy(inventory)
            mutate(bad)
            try:
                check_contract(bad, launch, relations, records)
            except ValueError:
                negatives += 1
            else:
                raise ValueError("negative fixture unexpectedly passed")
        bad = copy.deepcopy(relations)
        bad["supersessions"] = [{"old": "0025", "new": "ADR-CF-01"}]
        try:
            check_contract(inventory, launch, bad, records)
        except ValueError:
            negatives += 1
        else:
            raise ValueError("unapproved supersession fixture passed")
        relation_mutations = [
            lambda d: d.update(relationships=[e for e in d["relationships"]
                if (e["candidate"], e["source_record"]) != ("ADR-CF-07", "0058")]),
            lambda d: d["relationships"][0].update(source_record="9999"),
            lambda d: d["relationships"][0].update(source_path="docs/adr/unrelated.md"),
            lambda d: d["relationships"][0].update(source_status="proposed"),
        ]
        for mutate in relation_mutations:
            bad = copy.deepcopy(relations)
            mutate(bad)
            try:
                check_contract(inventory, launch, bad, records)
            except ValueError:
                negatives += 1
            else:
                raise ValueError("relationship negative fixture unexpectedly passed")
    print(json.dumps({"status": "pass", "candidates": 12, "task_identities": 69,
                      "pinned_sources": len(paths), "negative_fixtures": negatives,
                      "pvf": "docs_only", "architectural_acceptance": "not_claimed"}))


if __name__ == "__main__":
    main()
