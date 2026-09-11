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
import sys

sys.dont_write_bytecode = True
from validate_atomic_tasks import atomic_failures, atomic_negative_checks


REQUIRED_OBS_S3_ACCEPTANCE = {
    "agent_logic_admin_profile_resolves_business_account",
    "terraform_plan_reviewed_before_apply",
    "private_s3_origin_uses_oac",
    "cloudfront_acm_route53_converged",
    "access_logging_configured",
    "security_headers_configured",
    "exact_observatory_revision_deployed",
    "cache_invalidation_completed",
    "browser_https_and_runtime_wss_pass",
    "rollback_and_cost_recorded",
}


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


def markdown_table_rows(path):
    rows = []
    for line in path.read_text().splitlines():
        if not line.startswith("|"):
            continue
        cells = [cell.strip() for cell in line.strip("|").split("|")]
        if not cells or all(cell.startswith(("---", ":--")) for cell in cells):
            continue
        rows.append(cells)
    return rows


def normalized_planned_id(value):
    return re.sub(r"\s+\(#\d+\)$", "", value)


def projected_dependency_ids(text, expected_ids):
    return {
        planned_id
        for planned_id in expected_ids
        if re.search(rf"(?<![A-Z0-9-]){re.escape(planned_id)}(?![A-Z0-9-])", text)
    }


def projection_failures(catalog_rows, wbs_rows, expected_ids, wave_by_id):
    failures = []
    catalog_ids = [normalized_planned_id(row[1]) for row in catalog_rows if len(row) > 3 and row[1] != "Planned ID"]
    if len(catalog_ids) != len(set(catalog_ids)):
        failures.append("Planned issue catalog contains duplicate identifiers")
    if set(catalog_ids) != expected_ids:
        failures.append("Planned issue catalog and issue wave identifiers differ")

    raw_wbs_ids = [normalized_planned_id(row[0]) for row in wbs_rows if len(row) > 3 and row[0] != "WP"]
    if len(raw_wbs_ids) != len(set(raw_wbs_ids)):
        failures.append("WBS contains duplicate identifiers")
    expanded_wbs_ids = set(raw_wbs_ids)
    if "TAIL-01..10" in expanded_wbs_ids:
        expanded_wbs_ids.remove("TAIL-01..10")
        expanded_wbs_ids.update(f"TAIL-{n:02}" for n in range(1, 11))
    if expanded_wbs_ids != expected_ids:
        failures.append("WBS and issue wave identifiers differ")

    for key in expected_ids:
        expected = set(wave_by_id[key]["depends_on"])
        for label, rows, id_column in (("catalog", catalog_rows, 1), ("WBS", wbs_rows, 0)):
            matches = [row for row in rows if len(row) > 3 and normalized_planned_id(row[id_column]) == key]
            if len(matches) != 1 or projected_dependency_ids(matches[0][3], expected_ids) != expected:
                failures.append(f"{key} {label} dependencies differ from the issue wave")
            if len(matches) == 1 and matches[0][2] != wave_by_id[key]["title"]:
                failures.append(f"{key} {label} task outcome differs from the issue wave")
    return failures


def check(wave, specs, source_manifest=None, reconciliation=None):
    failures = []
    rows = wave["work_packages"]
    ids = [r["id"] for r in rows]
    by_id = {r["id"]: r for r in rows}
    spec_rows = specs["specifications"]
    spec_ids = [r["id"] for r in spec_rows]
    spec_by_id = {r["id"]: r for r in spec_rows}
    if len(ids) != 69 or len(set(ids)) != 69:
        failures.append("Expected 69 unique work packages")
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
    expected_existing = {
        "WP-01": 864,
        "OBS-LIVE": 720,
        "ARCH-SPLIT": 848,
        "CSDLC-MERGE": 849,
        "QUAL-RUNTIME": 852,
        "RT-COST": 854,
        "RT-PROVIDER": 855,
        "CSDLC-MAN": 861,
        "CSDLC-DECOMPOSE": 862,
    }
    expected_existing_dependencies = {
        "WP-01": [],
        "OBS-LIVE": [],
        "ARCH-SPLIT": ["WP-01"],
        "CSDLC-MERGE": ["WP-01"],
        "QUAL-RUNTIME": ["WP-01"],
        "RT-COST": ["WP-01"],
        "RT-PROVIDER": ["PLAT-PROVIDER"],
        "CSDLC-MAN": ["WP-01"],
        "CSDLC-DECOMPOSE": ["WP-01"],
    }
    created_sprint01 = {'SIM-UMBRELLA': 866, 'SIM-01': 867, 'SIM-02': 868, 'SIM-03': 869, 'SIM-04': 870, 'SIM-05': 871, 'SIM-06': 872, 'SIM-07': 873, 'SIM-08': 874, 'SIM-09': 875}
    actual_existing = {r["id"]: r["issue"] for r in rows if r.get("issue") is not None}
    if actual_existing != expected_existing | created_sprint01:
        failures.append("Existing issue bindings differ from the reconciled v0.92.2 inventory")
    for key, issue in expected_existing.items():
        row = by_id.get(key, {})
        if row.get("depends_on") != expected_existing_dependencies[key] or row.get("creation_policy") != "reuse_existing":
            failures.append(f"{key} lost its existing-authority dependency contract")
        if spec_by_id.get(key, {}).get("issue") != issue:
            failures.append(f"{key} specification lost existing identity")
    for key, issue in created_sprint01.items():
        if by_id.get(key, {}).get("creation_policy") != "created_sprint01" or spec_by_id.get(key, {}).get("issue") != issue:
            failures.append(f"{key} lost its reviewed first-sprint issue identity")
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
        expected = [previous, "OBS-S3", "ARCH-ADR"] if current == "TAIL-10" else [previous]
        if by_id.get(current, {}).get("depends_on") != expected:
            failures.append(f"Tail edge {previous} -> {current} missing")
    product = {r for r in ids if r.startswith("CF-")} - {"CF-INTEGRATE", "CF-PROOF"}
    if set(by_id["CF-INTEGRATE"]["depends_on"]) != product | {"PLAT-PROVIDER", "RT-PROVIDER", "PLAT-MEMORY"}:
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
    support = {"PLAT-MLX", "PLAT-PAIR", "PLAT-UTS", "PLAT-RUST", "OPS-AWS", "OPS-GCP", "PUB-MEDIUM", "PUB-CSDLC", "SPEC-RETEST", "OBS-LIVE", "ARCH-SPLIT", "CSDLC-MERGE", "QUAL-RUNTIME", "RT-COST", "CSDLC-MAN", "CSDLC-DECOMPOSE", "CSDLC-REMOTE", "QUAL-RESIDENT", "QUAL-PROVIDER", "QUAL-INVENTORY", "QUAL-EVIDENCE"}
    if set(by_id["TAIL-01"]["depends_on"]) != support | {"CF-INTEGRATE", "CF-PROOF", "SIM-UMBRELLA"}:
        failures.append("Milestone support convergence differs")
    additional = {"OBS-S3", "ARCH-ADR"}
    if additional & set(by_id["CF-INTEGRATE"]["depends_on"]) or additional & set(by_id["TAIL-01"]["depends_on"]):
        failures.append("OBS-S3 and ARCH-ADR must not gate product integration or the early TAIL-01 quality gate")
    final_wave = by_id.get("TAIL-10", {})
    final_spec = spec_by_id.get("TAIL-10", {})
    if final_spec.get("depends_on") != final_wave.get("depends_on"):
        failures.append("TAIL-10 specification/wave dependencies differ")
    if final_spec.get("acceptance") != final_wave.get("acceptance"):
        failures.append("TAIL-10 specification/wave acceptance differs")
    obs_s3_acceptance = set(spec_by_id.get("OBS-S3", {}).get("acceptance", []))
    if not REQUIRED_OBS_S3_ACCEPTANCE <= obs_s3_acceptance:
        failures.append("OBS-S3 lost one or more required deployment acceptance obligations")
    obligations = {
        "CF-EVIDENCE": {"finding_run_contract_merged_before_consumers"},
        "CF-REVIEW": {"isolated_pre_synthesis_inputs"},
        "CF-SYNTHESIS": {"disagreement_preserved"},
        "TAIL-06": {"changed_candidate_artifacts_rebuilt", "affected_proof_rerun", "current_internal_external_review"},
        "TAIL-10": {"final_candidate_and_manifest_match_review", "no_unresolved_p1", "obs_s3_deployment_acceptance_complete", "arch_adr_decision_set_acceptance_complete"},
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
    cases.append(("Observatory deployment gates early TAIL-01", broken, specs))
    for obligation in sorted(REQUIRED_OBS_S3_ACCEPTANCE):
        broken = copy.deepcopy(specs)
        next(r for r in broken["specifications"] if r["id"] == "OBS-S3")["acceptance"].remove(obligation)
        cases.append((f"Observatory deployment loses {obligation}", wave, broken))
    broken = copy.deepcopy(wave)
    next(r for r in broken["work_packages"] if r["id"] == "ARCH-ADR")["issue"] = 999998
    cases.append(("ADR work package assigned before WP-01", broken, specs))
    broken = copy.deepcopy(wave)
    next(r for r in broken["work_packages"] if r["id"] == "TAIL-01")["depends_on"].append("ARCH-ADR")
    cases.append(("ADR issue gates early TAIL-01", broken, specs))
    # Mutate both projections to prove substantive obligations, then one side
    # independently to prove parity. These are final-closeout, not early gates.
    for dependency, obligation in (("OBS-S3", "obs_s3_deployment_acceptance_complete"), ("ARCH-ADR", "arch_adr_decision_set_acceptance_complete")):
        for field, item in (("depends_on", dependency), ("acceptance", obligation)):
            for mutation in ("both", "wave", "spec"):
                w, s = copy.deepcopy(wave), copy.deepcopy(specs)
                if mutation in ("both", "wave"):
                    next(r for r in w["work_packages"] if r["id"] == "TAIL-10")[field].remove(item)
                if mutation in ("both", "spec"):
                    next(r for r in s["specifications"] if r["id"] == "TAIL-10")[field].remove(item)
                cases.append((f"final closeout loses {item} in {mutation}", w, s))
    for key in ["SIM-UMBRELLA"] + [f"SIM-{n:02}" for n in range(1, 10)]:
        broken = copy.deepcopy(wave)
        next(r for r in broken["work_packages"] if r["id"] == key)["issue"] = None
        cases.append((f"lost first-sprint identity {key}", broken, specs))
        broken = copy.deepcopy(specs)
        next(r for r in broken["specifications"] if r["id"] == key)["issue"] = 999999
        cases.append((f"incorrect first-sprint specification identity {key}", wave, broken))
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
    atomic = json.loads((root / "ATOMIC_TASK_CONTRACTS_v0.92.2.json").read_text())
    failures.extend(atomic_failures(wave, specs, atomic))
    expected_ids = {row["id"] for row in wave["work_packages"]}
    catalog_rows = markdown_table_rows(root / "PLANNED_ISSUE_CATALOG_v0.92.2.md")
    wbs_rows = markdown_table_rows(root / "WBS_v0.92.2.md")
    failures.extend(projection_failures(catalog_rows, wbs_rows, expected_ids, {row["id"]: row for row in wave["work_packages"]}))
    coverage = (root / "FEATURE_PROOF_COVERAGE_v0.92.2.md").read_text()
    supporting = (root / "features/SUPPORTING_PLATFORM_TRACKS_v0.92.2.md").read_text()
    adr_plan = (root / "ADR_PLAN_v0.92.2.md").read_text()
    readme = (root / "README.md").read_text()
    for required in ("OBS-S3", "ARCH-ADR"):
        if required not in coverage or required not in supporting:
            failures.append(f"{required} missing from proof or supporting-track projection")
    if "owned by the `ARCH-ADR` work package" not in adr_plan:
        failures.append("ADR plan is not explicitly owned by ARCH-ADR")
    if "complete work denominator is 69 rows" not in readme:
        failures.append("README denominator does not match the 69-row issue wave")
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
        projection_cases = []
        broken_catalog = copy.deepcopy(catalog_rows)
        broken_catalog.append(copy.deepcopy(next(row for row in catalog_rows if len(row) > 3 and normalized_planned_id(row[1]) == "OBS-S3")))
        projection_cases.append(("duplicate catalog row", broken_catalog, wbs_rows))
        broken_wbs = copy.deepcopy(wbs_rows)
        broken_wbs.append(copy.deepcopy(next(row for row in wbs_rows if len(row) > 3 and normalized_planned_id(row[0]) == "OBS-S3")))
        projection_cases.append(("duplicate WBS row", catalog_rows, broken_wbs))
        broken_wbs = copy.deepcopy(wbs_rows)
        next(row for row in broken_wbs if len(row) > 3 and normalized_planned_id(row[0]) == "OBS-S3")[3] = "WP-01; completed #679 / merged PR #685"
        projection_cases.append(("WBS bypasses OBS-LIVE", catalog_rows, broken_wbs))
        broken_catalog = copy.deepcopy(catalog_rows)
        next(row for row in broken_catalog if len(row) > 3 and normalized_planned_id(row[1]) == "OBS-S3")[3] = "After WP-01; consume completed #679 / merged PR #685"
        projection_cases.append(("catalog bypasses OBS-LIVE", broken_catalog, wbs_rows))
        broken_catalog = copy.deepcopy(catalog_rows)
        next(row for row in broken_catalog if len(row) > 3 and normalized_planned_id(row[1]) == "CF-PROOF")[2] = "Write a proof plan"
        projection_cases.append(("catalog substitutes a planning task", broken_catalog, wbs_rows))
        broken_wbs = copy.deepcopy(wbs_rows)
        next(row for row in broken_wbs if len(row) > 3 and normalized_planned_id(row[0]) == "PUB-CSDLC")[2] = "Advance an unspecified packet"
        projection_cases.append(("WBS substitutes partial progress", catalog_rows, broken_wbs))
        for name, catalog_case, wbs_case in projection_cases:
            if not projection_failures(catalog_case, wbs_case, expected_ids, {row["id"]: row for row in wave["work_packages"]}):
                failures.append("Negative fixture not rejected: " + name)
        rejected += len(projection_cases)
        atomic_missed, atomic_count = atomic_negative_checks(wave, specs, atomic)
        failures.extend(atomic_missed)
        rejected += atomic_count
    print(json.dumps({"status": "fail" if failures else "pass", "work_packages": len(wave["work_packages"]),
                      "existing_issues": sorted(r["issue"] for r in wave["work_packages"] if r.get("issue") is not None), "v0921_predecessor_issues": [717, 718], "negative_fixtures": rejected,
                      "failures": failures, "nonclaim": "No runtime, lifecycle-publication or release proof"}, indent=2))
    return bool(failures)


if __name__ == "__main__":
    raise SystemExit(main())
