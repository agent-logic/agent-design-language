#!/usr/bin/env python3
"""Validate the bounded v0.92.2 planning contract without lifecycle writes.

PVF: docs; deterministic contract proof; local CPU; issue gate, not Runtime
or release proof. Requires Python 3 and Ruby's standard YAML library.
"""
import argparse
import copy
import json
from pathlib import Path
import re
import subprocess


def read_yaml(path):
    script = "require 'yaml'; require 'json'; puts JSON.generate(YAML.safe_load(File.read(ARGV[0]), aliases: false))"
    return json.loads(subprocess.check_output(["ruby", "-e", script, str(path)], text=True))


def check(wave, specs):
    failures = []
    rows = wave["work_packages"]
    ids = [r["id"] for r in rows]
    by_id = {r["id"]: r for r in rows}
    spec_rows = specs["specifications"]
    spec_ids = [r["id"] for r in spec_rows]
    spec_by_id = {r["id"]: r for r in spec_rows}
    if len(ids) != 33 or len(set(ids)) != 33:
        failures.append("Expected 33 unique work packages")
    if len(spec_ids) != len(set(spec_ids)) or set(spec_ids) != set(ids):
        failures.append("Specification and wave denominators differ")
    expected_existing = {"RT-A2A": 718, "RT-ORIENT": 717, "OBS-LIVE": 720}
    actual_existing = {r["id"]: r["issue"] for r in rows if r.get("issue") is not None}
    if actual_existing != expected_existing:
        failures.append("Existing issue bindings must be exactly RT-A2A=718, RT-ORIENT=717, OBS-LIVE=720")
    for key, issue in expected_existing.items():
        row = by_id.get(key, {})
        if row.get("depends_on") != [] or row.get("creation_policy") != "reuse_existing":
            failures.append(f"{key} must reuse existing authority without new-wave dependencies")
        if spec_by_id.get(key, {}).get("issue") != issue:
            failures.append(f"{key} specification lost existing identity")
    if by_id.get("RT-A2A", {}).get("priority") != "urgent":
        failures.append("#718 urgency missing")
    visiting, visited = set(), set()

    def visit(key):
        if key not in by_id:
            failures.append(f"Unknown dependency {key}")
            return
        if key in visiting:
            failures.append(f"Dependency cycle at {key}")
            return
        if key in visited:
            return
        visiting.add(key)
        for dep in by_id[key].get("depends_on", []):
            visit(dep)
        visiting.remove(key)
        visited.add(key)

    for key in ids:
        visit(key)
    tail = [f"TAIL-{n:02}" for n in range(1, 11)]
    if wave.get("canonical_release_tail") != tail or specs["release_tail"]["order"] != tail:
        failures.append("Canonical tail order differs")
    for previous, current in zip(tail, tail[1:]):
        if by_id.get(current, {}).get("depends_on") != [previous]:
            failures.append(f"Tail edge {previous} -> {current} missing")
    product = {r for r in ids if r.startswith("CF-")} - {"CF-INTEGRATE"}
    if set(by_id["CF-INTEGRATE"]["depends_on"]) != product | {"PLAT-PROVIDER", "PLAT-MEMORY"}:
        failures.append("Product integration prerequisites differ")
    support = {"PLAT-MLX", "PLAT-UTS", "PLAT-RUST", "OPS-AWS", "PUB-MEDIUM", "PUB-CSDLC", "SPEC-RETEST"}
    if set(by_id["TAIL-01"]["depends_on"]) != support | set(expected_existing) | {"CF-INTEGRATE"}:
        failures.append("Milestone support convergence differs")
    obligations = {
        "CF-EVIDENCE": {"finding_run_contract_merged_before_consumers"},
        "CF-REVIEW": {"isolated_pre_synthesis_inputs", "disagreement_preserved"},
        "TAIL-06": {"changed_candidate_artifacts_rebuilt", "affected_proof_rerun", "current_internal_external_review"},
        "TAIL-10": {"final_candidate_and_manifest_match_review", "no_unresolved_p1"},
    }
    for key, required in obligations.items():
        if not required <= set(spec_by_id.get(key, {}).get("acceptance", [])):
            failures.append(f"{key} lost required acceptance obligations")
    return failures


def negative_checks(wave, specs):
    cases = []
    broken = copy.deepcopy(wave)
    broken["work_packages"] = [r for r in broken["work_packages"] if r["id"] != "RT-A2A"]
    cases.append(("missing urgent existing issue", broken, specs))
    broken = copy.deepcopy(wave)
    next(r for r in broken["work_packages"] if r["id"] == "RT-A2A")["depends_on"] = ["WP-01"]
    cases.append(("urgent issue waits for new wave", broken, specs))
    broken = copy.deepcopy(wave)
    next(r for r in broken["work_packages"] if r["id"] == "CF-INTEGRATE")["depends_on"].append("PUB-MEDIUM")
    cases.append(("editorial work blocks integration", broken, specs))
    broken = copy.deepcopy(wave)
    next(r for r in broken["work_packages"] if r["id"] == "CF-ADAPTER")["depends_on"] = ["CF-EVIDENCE"]
    cases.append(("dependency cycle", broken, specs))
    broken = copy.deepcopy(specs)
    next(r for r in broken["specifications"] if r["id"] == "TAIL-06")["acceptance"].remove("current_internal_external_review")
    cases.append(("stale review after remediation", wave, broken))
    broken = copy.deepcopy(wave)
    next(r for r in broken["work_packages"] if r["id"] == "PLAT-RUST")["issue"] = 999999
    cases.append(("unadmitted extra issue", broken, specs))
    return [name for name, w, s in cases if not check(w, s)], len(cases)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="Also prove rejection of six bad planning fixtures")
    args = parser.parse_args()
    root = Path(__file__).resolve().parent
    wave = read_yaml(root / "WP_ISSUE_WAVE_v0.92.2.yaml")
    specs = read_yaml(root / "WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml")
    failures = check(wave, specs)
    for path in root.rglob("*.md"):
        for target in re.findall(r"\]\(([^)]+)\)", path.read_text()):
            if target.startswith(("http:", "https:", "#")):
                continue
            if not (path.parent / target.split("#")[0]).exists():
                failures.append(f"Broken link in {path.name}: {target}")
    current = read_yaml(root.parent / "v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml")
    current_rows = {r["id"]: r for r in current["work_packages"]}
    if current_rows["TAIL-07"]["depends_on"] != []:
        failures.append("#523 still waits for current closeout")
    if not {"TAIL-06", "TAIL-09"} <= set(current_rows["TAIL-10"]["depends_on"]):
        failures.append("Current release lost remediation or successor-review gate")
    rejected = 0
    if args.self_test:
        missed, rejected = negative_checks(wave, specs)
        failures.extend("Negative fixture not rejected: " + x for x in missed)
    print(json.dumps({"status": "fail" if failures else "pass", "work_packages": 33,
                      "existing_issues": [717, 718, 720], "negative_fixtures": rejected,
                      "failures": failures, "nonclaim": "No runtime, lifecycle-publication or release proof"}, indent=2))
    return bool(failures)


if __name__ == "__main__":
    raise SystemExit(main())
