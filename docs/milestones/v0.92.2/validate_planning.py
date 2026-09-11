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


def markdown_table_column(path, column):
    values = []
    for line in path.read_text().splitlines():
        if not line.startswith("|"):
            continue
        cells = [cell.strip() for cell in line.strip("|").split("|")]
        if len(cells) <= column or cells[column].startswith(("---", ":--")):
            continue
        values.append(cells[column])
    return values


def check(wave, specs, source_manifest=None, reconciliation=None):
    failures = []
    rows = wave["work_packages"]
    ids = [r["id"] for r in rows]
    by_id = {r["id"]: r for r in rows}
    spec_rows = specs["specifications"]
    spec_ids = [r["id"] for r in spec_rows]
    spec_by_id = {r["id"]: r for r in spec_rows}
    if len(ids) != 45 or len(set(ids)) != 45:
        failures.append("Expected 45 unique work packages")
    if len(spec_ids) != len(set(spec_ids)) or set(spec_ids) != set(ids):
        failures.append("Specification and wave denominators differ")
    atomic_results = wave.get("atomic_results", {})
    if set(atomic_results) != set(ids):
        failures.append("Atomic-result register must cover every work package exactly once")
    if len(set(atomic_results.values())) != len(ids):
        failures.append("Every work package must have a distinct primary result")
    if any(not re.fullmatch(r"[a-z0-9]+(?:_[a-z0-9]+)*", str(result)) for result in atomic_results.values()):
        failures.append("Atomic results must be one normalized result identifier, not a compound work list")
    for row in rows:
        if not row.get("deliverables") or not row.get("proof"):
            failures.append(f"{row['id']} must name implementation output and proving evidence")
    admitted_tbd_sources = {
        row.get("planning_source")
        for row in rows
        if str(row.get("planning_source", "")).startswith(".adl/docs/TBD/")
    }
    if source_manifest is not None:
        missing = sorted(admitted_tbd_sources - source_manifest)
        if missing:
            failures.append("Admitted TBD sources missing from audit manifest: " + ", ".join(missing))
    if reconciliation is not None:
        missing = sorted(path for path in admitted_tbd_sources if f"`{path}`" not in reconciliation)
        if missing:
            failures.append("Admitted TBD sources missing a reconciliation disposition: " + ", ".join(missing))
    expected_existing = {"OBS-LIVE": 720}
    actual_existing = {r["id"]: r["issue"] for r in rows if r.get("issue") is not None}
    if actual_existing != expected_existing:
        failures.append("Existing issue bindings must be exactly OBS-LIVE=720")
    for key, issue in expected_existing.items():
        row = by_id.get(key, {})
        if row.get("depends_on") != [] or row.get("creation_policy") != "reuse_existing":
            failures.append(f"{key} must reuse existing authority without new-wave dependencies")
        if spec_by_id.get(key, {}).get("issue") != issue:
            failures.append(f"{key} specification lost existing identity")
    for n in range(1, 10):
        key = f"SIM-{n:02}"
        expected = [] if n == 1 else [f"SIM-{n-1:02}"]
        row = by_id.get(key, {})
        if row.get("depends_on") != expected or row.get("startup_policy") != "dedicated_sprint_own_readiness_parallel_runtime":
            failures.append(f"{key} must preserve first-sprint ordering and independent Runtime-parallel startup")
    if by_id.get("SIM-UMBRELLA", {}).get("depends_on") != ["SIM-09"]:
        failures.append("SIM umbrella completion must follow SIM-09")
    if "SIM-UMBRELLA" in by_id["CF-INTEGRATE"]["depends_on"]:
        failures.append("SIM sprint must not gate product integration")
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
    if by_id.get("PLAT-PAIR", {}).get("depends_on") != ["PLAT-PROVIDER"]:
        failures.append("PLAT-PAIR must consume the canonical provider-definition contract")
    if by_id.get("OPS-GCP", {}).get("depends_on") != ["WP-01"]:
        failures.append("OPS-GCP must remain a separately owned post-opening foundation track")
    if by_id.get("OBS-S3", {}).get("depends_on") != ["WP-01", "OBS-LIVE"]:
        failures.append("OBS-S3 must wait for issue creation and the live Observatory baseline")
    if by_id.get("OBS-S3", {}).get("issue") is not None:
        failures.append("OBS-S3 must remain unassigned until WP-01 creates its issue")
    if set(by_id.get("OBS-S3", {}).get("external_dependencies", [])) != {"completed-v0.92.1-issue-679", "merged-v0.92.1-pr-685"}:
        failures.append("OBS-S3 must consume completed #679 and merged PR #685")
    if by_id.get("ARCH-ADR", {}).get("depends_on") != ["WP-01"]:
        failures.append("ARCH-ADR must remain a post-opening milestone work package")
    if by_id.get("ARCH-ADR", {}).get("issue") is not None:
        failures.append("ARCH-ADR must remain unassigned until WP-01 creates its issue")
    if by_id.get("OPS-AWS", {}).get("title") != "Produce one current AWS inventory packet from the #484 baseline":
        failures.append("OPS-AWS title lost its complete #484-bound atomic result")
    support = {"PLAT-MLX", "PLAT-PAIR", "PLAT-UTS", "PLAT-RUST", "OPS-AWS", "OPS-GCP", "PUB-MEDIUM", "PUB-CSDLC", "SPEC-RETEST"}
    if set(by_id["TAIL-01"]["depends_on"]) != support | set(expected_existing) | {"CF-INTEGRATE", "SIM-UMBRELLA"}:
        failures.append("Milestone support convergence differs")
    additional = {"OBS-S3", "ARCH-ADR"}
    if additional & set(by_id["CF-INTEGRATE"]["depends_on"]) or additional & set(by_id["TAIL-01"]["depends_on"]):
        failures.append("OBS-S3 and ARCH-ADR must not gate product integration or the release tail")
    obs_s3_acceptance = set(spec_by_id.get("OBS-S3", {}).get("acceptance", []))
    required_obs_s3 = {
        "agent_logic_admin_profile_resolves_business_account",
        "access_logging_configured",
        "security_headers_configured",
        "cache_invalidation_completed",
    }
    if not required_obs_s3 <= obs_s3_acceptance:
        failures.append("OBS-S3 lost required profile, logging, headers, or invalidation acceptance")
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


def negative_checks(wave, specs, source_manifest, reconciliation):
    cases = []
    broken = copy.deepcopy(wave)
    broken["work_packages"] = [r for r in broken["work_packages"] if r["id"] != "OBS-LIVE"]
    cases.append(("missing admitted existing issue", broken, specs))
    broken = copy.deepcopy(wave)
    del broken["atomic_results"]["CF-SHELL"]
    cases.append(("missing atomic result", broken, specs))
    broken = copy.deepcopy(wave)
    broken["atomic_results"]["CF-SHELL"] = broken["atomic_results"]["CF-ADAPTER"]
    cases.append(("duplicate atomic result", broken, specs))
    broken = copy.deepcopy(wave)
    next(r for r in broken["work_packages"] if r["id"] == "OBS-LIVE")["depends_on"] = ["WP-01"]
    cases.append(("existing issue waits for new wave", broken, specs))
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
    broken = copy.deepcopy(wave)
    next(r for r in broken["work_packages"] if r["id"] == "SIM-01")["depends_on"] = ["WP-01"]
    cases.append(("SIM sprint waits for CodeFriend startup", broken, specs))
    broken = copy.deepcopy(wave)
    next(r for r in broken["work_packages"] if r["id"] == "SIM-04")["depends_on"] = []
    cases.append(("SIM sprint loses internal sequence", broken, specs))
    broken = copy.deepcopy(wave)
    next(r for r in broken["work_packages"] if r["id"] == "TAIL-01")["depends_on"].remove("SIM-UMBRELLA")
    cases.append(("SIM result omitted from milestone convergence", broken, specs))
    broken = copy.deepcopy(wave)
    next(r for r in broken["work_packages"] if r["id"] == "OBS-S3")["depends_on"] = ["WP-01"]
    cases.append(("Observatory deployment loses live baseline", broken, specs))
    broken = copy.deepcopy(wave)
    next(r for r in broken["work_packages"] if r["id"] == "TAIL-01")["depends_on"].append("OBS-S3")
    cases.append(("Observatory deployment gates release tail", broken, specs))
    broken = copy.deepcopy(specs)
    next(r for r in broken["specifications"] if r["id"] == "OBS-S3")["acceptance"].remove("security_headers_configured")
    cases.append(("Observatory deployment loses security headers", wave, broken))
    broken = copy.deepcopy(wave)
    next(r for r in broken["work_packages"] if r["id"] == "ARCH-ADR")["issue"] = 999998
    cases.append(("ADR work package assigned before WP-01", broken, specs))
    broken = copy.deepcopy(wave)
    next(r for r in broken["work_packages"] if r["id"] == "TAIL-01")["depends_on"].append("ARCH-ADR")
    cases.append(("ADR issue is folded into release tail", broken, specs))
    missed = [name for name, w, s in cases if not check(w, s, source_manifest, reconciliation)]
    admitted = next(
        row["planning_source"]
        for row in wave["work_packages"]
        if str(row.get("planning_source", "")).startswith(".adl/docs/TBD/")
    )
    if not check(wave, specs, source_manifest - {admitted}, reconciliation):
        missed.append("source omitted from audit manifest")
    if not check(wave, specs, source_manifest, reconciliation.replace(f"`{admitted}`", "`omitted-source`")):
        missed.append("source omitted from reconciliation")
    return missed, len(cases) + 2


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true", help="Also prove rejection of bad planning fixtures")
    args = parser.parse_args()
    root = Path(__file__).resolve().parent
    wave = read_yaml(root / "WP_ISSUE_WAVE_v0.92.2.yaml")
    specs = read_yaml(root / "WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml")
    source_manifest = {
        line.strip()
        for line in (root / "TBD_SOURCE_AUDIT_MANIFEST_v0.92.2.txt").read_text().splitlines()
        if line.strip()
    }
    reconciliation = (root / "TBD_SCHEDULING_RECONCILIATION_v0.92.2.md").read_text()
    failures = check(wave, specs, source_manifest, reconciliation)
    expected_ids = {row["id"] for row in wave["work_packages"]}
    catalog_ids = {
        re.sub(r"\s+\(#\d+\)$", "", value)
        for value in markdown_table_column(root / "PLANNED_ISSUE_CATALOG_v0.92.2.md", 1)
        if value != "Planned ID"
    }
    if catalog_ids != expected_ids:
        failures.append("Planned issue catalog and issue wave identifiers differ")
    wbs_ids = set(markdown_table_column(root / "WBS_v0.92.2.md", 0))
    wbs_ids.discard("WP")
    wbs_ids = {re.sub(r"\s+\(#\d+\)$", "", value) for value in wbs_ids}
    if "TAIL-01..10" in wbs_ids:
        wbs_ids.remove("TAIL-01..10")
        wbs_ids.update(f"TAIL-{n:02}" for n in range(1, 11))
    if wbs_ids != expected_ids:
        failures.append("WBS and issue wave identifiers differ")
    coverage = (root / "FEATURE_PROOF_COVERAGE_v0.92.2.md").read_text()
    supporting = (root / "features/SUPPORTING_PLATFORM_TRACKS_v0.92.2.md").read_text()
    adr_plan = (root / "ADR_PLAN_v0.92.2.md").read_text()
    readme = (root / "README.md").read_text()
    for required in ("OBS-S3", "ARCH-ADR"):
        if required not in coverage or required not in supporting:
            failures.append(f"{required} missing from proof or supporting-track projection")
    if "owned by the `ARCH-ADR` work package" not in adr_plan:
        failures.append("ADR plan is not explicitly owned by ARCH-ADR")
    if "complete work denominator is 45 rows" not in readme:
        failures.append("README denominator does not match the 45-row issue wave")
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
        missed, rejected = negative_checks(wave, specs, source_manifest, reconciliation)
        failures.extend("Negative fixture not rejected: " + x for x in missed)
    print(json.dumps({"status": "fail" if failures else "pass", "work_packages": len(wave["work_packages"]),
                      "existing_issues": [720], "v0921_predecessor_issues": [717, 718], "negative_fixtures": rejected,
                      "failures": failures, "nonclaim": "No runtime, lifecycle-publication or release proof"}, indent=2))
    return bool(failures)


if __name__ == "__main__":
    raise SystemExit(main())
