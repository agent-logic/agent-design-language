"""Atomic task admission contract. PVF: deterministic local planning proof; no product proof."""
import copy

SPLITS = {
    "CF-ADAPTER": ["CF-ADAPTER", "CF-ADAPTER-GITHUB", "CF-ADAPTER-CI"],
    "CF-COG": ["CF-COG", "CF-COG-DRIFT", "CF-COG-IMPACT", "CF-COG-RATIONALE"],
    "CF-GOV": ["CF-GOV", "CF-GOV-CI"],
    "CF-REVIEW": ["CF-REVIEW", "CF-SYNTHESIS", "CF-REMEDIATE", "CF-TESTPLAN"],
    "CF-UX": ["CF-UX", "CF-RENDER-MD", "CF-RENDER-HTML", "CF-RENDER-PDF"],
    "PLAT-PROVIDER": ["PLAT-PROVIDER", "RT-PROVIDER"],
    "QUAL-RUNTIME": ["QUAL-RUNTIME", "QUAL-RESIDENT", "QUAL-PROVIDER", "QUAL-INVENTORY", "QUAL-EVIDENCE"],
    "CSDLC-DECOMPOSE": ["CSDLC-DECOMPOSE", "CSDLC-REMOTE"],
}
TIGHTENED = {"CF-SHELL", "CF-EVIDENCE", "CF-PROOF", "CF-INTEGRATE", "PLAT-UTS", "PLAT-RUST", "PUB-MEDIUM", "PUB-CSDLC", "PLAT-MEMORY", "SIM-03", "SIM-04"}
PLANNING = {"WP-01", "ARCH-SPLIT", "OPS-GCP", "ARCH-ADR", "SIM-08", "TAIL-07", "TAIL-08"}


def atomic_failures(wave, specs, manifest):
    failures = []
    rows = {r["id"]: r for r in wave["work_packages"]}
    contracts = {r["id"]: r for r in specs["specifications"]}
    required = TIGHTENED | {child for children in SPLITS.values() for child in children}
    if manifest.get("split_families") != SPLITS:
        failures.append("Eight atomic split families must retain every complete task")
    if set(manifest.get("tightened_tasks", [])) != TIGHTENED:
        failures.append("Eleven tightened task contracts must be retained")
    if set(manifest.get("planning_tasks", [])) != PLANNING or not PLANNING <= rows.keys():
        failures.append("All seven required planning tasks must remain")
    if manifest.get("task_count") != len(rows) or manifest.get("prospective_count") != sum(r.get("issue") is None for r in rows.values()):
        failures.append("Atomic task denominator differs")
    actual = {k: r["issue"] for k, r in rows.items() if r.get("issue") is not None}
    if manifest.get("existing_bindings") != actual:
        failures.append("Atomic existing issue identities differ")
    if manifest.get("expected_dependencies") != {k: r["depends_on"] for k, r in rows.items()}:
        failures.append("Atomic dependency graph differs")
    if set(manifest.get("completion_contracts", {})) != required:
        failures.append("Atomic completion contracts must cover all corrected tasks")
    for key in required:
        spec = contracts.get(key, {})
        c = spec.get("completion_contract", {})
        if not isinstance(c, dict):
            failures.append(f"{key}: completion contract must be an object")
            continue
        if c != manifest.get("completion_contracts", {}).get(key):
            failures.append(f"{key}: completion contract differs from reviewed task manifest")
        for field in ("task", "consumer"):
            if not isinstance(c.get(field), str) or not c[field].strip():
                failures.append(f"{key}: missing concrete {field}")
        for field in ("success_evidence", "failure_evidence", "reject_completion"):
            if not isinstance(c.get(field), list) or not c[field] or any(not isinstance(x, str) or not x.strip() for x in c[field]):
                failures.append(f"{key}: missing {field}")
        typ = c.get("result_type")
        if typ not in ("implementation", "qualification", "document"):
            failures.append(f"{key}: invalid result type")
        required_rejections = {"outline_only", "unspecified_revision", "future_work_list"} if typ == "document" else {"plan_only", "schema_only", "scaffold_only", "zero_executed_scenarios"}
        if not required_rejections <= set(c.get("reject_completion", [])):
            failures.append(f"{key}: partial-work rejection weakened")
        if not set(c.get("success_evidence", [])) <= set(spec.get("acceptance", [])):
            failures.append(f"{key}: complete result missing from acceptance")
        if not set(c.get("failure_evidence", [])) <= set(spec.get("pvf", [])):
            failures.append(f"{key}: failure proof missing from PVF")
    retained = manifest.get("baseline_obligation_owners", {})
    if set(retained) != set(SPLITS):
        failures.append("Every split family must retain its baseline obligations")
    for family, fields in retained.items():
        for field in ("acceptance", "pvf", "stop_conditions", "non_goals"):
            obligations = fields.get(field, {})
            if not obligations:
                failures.append(f"{family}: missing baseline {field}")
            for obligation, owners in obligations.items():
                if not owners or any(owner not in SPLITS[family] for owner in owners):
                    failures.append(f"{family}: invalid baseline obligation owner")
                for owner in owners:
                    if obligation not in contracts.get(owner, {}).get(field, []):
                        failures.append(f"{owner}: lost baseline {field} obligation {obligation}")
    matrix = contracts.get("RT-PROVIDER", {}).get("provider_acceptance_matrix", {})
    if matrix != manifest.get("provider_acceptance_matrix") or len(matrix.get("routes", [])) != 5:
        failures.append("Provider qualification must retain all five live issue routes")
    criterion = contracts.get("QUAL-EVIDENCE", {}).get("criterion_consumer_boundary", {})
    if criterion != manifest.get("criterion_consumer_boundary") or criterion.get("original_finding_denominator") != 19 or len(criterion.get("criteria", [])) != 5:
        failures.append("Five-row criterion and historical consumer truth must remain intact")
    for key in required:
        if any(x.endswith(("_rejected", "_explicit", "_preserved", "_checked")) or x in {"no_source_mutation", "credentials_never_serialized", "redaction_rechecked"} for x in contracts.get(key, {}).get("stop_conditions", [])):
            failures.append(f"{key}: stop condition names a passing proof outcome")
    # Explicitly retain the production proof chain even if the manifest is edited too.
    required_edges = {
        "CF-SHELL": {"CF-REVIEW"}, "CF-REVIEW": {"CF-EVIDENCE", "RT-PROVIDER"},
        "CF-SYNTHESIS": {"CF-REVIEW"}, "CF-REMEDIATE": {"CF-SYNTHESIS"}, "CF-TESTPLAN": {"CF-SYNTHESIS"},
        "CF-RENDER-MD": {"CF-UX", "CF-SYNTHESIS", "CF-REMEDIATE", "CF-TESTPLAN"},
        "CF-RENDER-HTML": {"CF-RENDER-MD"}, "CF-RENDER-PDF": {"CF-RENDER-MD"},
        "CF-PROOF": {"CF-INTEGRATE"}, "TAIL-01": {"CF-PROOF"},
        "TAIL-10": {"TAIL-09", "OBS-S3", "ARCH-ADR"},
        "RT-PROVIDER": {"PLAT-PROVIDER"}, "PLAT-PROVIDER": {"RT-COST"},
        "QUAL-EVIDENCE": {"QUAL-RUNTIME", "QUAL-RESIDENT", "QUAL-PROVIDER", "QUAL-INVENTORY"},
        "CSDLC-REMOTE": {"CSDLC-DECOMPOSE", "CSDLC-MERGE"},
    }
    for key, edges in required_edges.items():
        if not edges <= set(rows.get(key, {}).get("depends_on", [])):
            failures.append(f"{key}: complete-task prerequisite lost")
    return failures


def atomic_negative_checks(wave, specs, manifest):
    cases = []
    for family, children in SPLITS.items():
        w = copy.deepcopy(wave)
        w["work_packages"] = [r for r in w["work_packages"] if r["id"] != children[-1]]
        cases.append((f"missing split child {family}", w, specs, manifest))
    for key in sorted(TIGHTENED):
        s = copy.deepcopy(specs)
        next(r for r in s["specifications"] if r["id"] == key)["completion_contract"]["consumer"] = ""
        cases.append((f"missing production consumer {key}", wave, s, manifest))
    for field in ("success_evidence", "failure_evidence", "reject_completion"):
        s, m = copy.deepcopy(specs), copy.deepcopy(manifest)
        next(r for r in s["specifications"] if r["id"] == "CF-ADAPTER")["completion_contract"][field] = []
        m["completion_contracts"]["CF-ADAPTER"][field] = []
        cases.append((f"coordinated contract weakening {field}", wave, s, m))
    for key in sorted(PLANNING):
        w = copy.deepcopy(wave)
        w["work_packages"] = [r for r in w["work_packages"] if r["id"] != key]
        cases.append((f"planning task removed {key}", w, specs, manifest))
    w, m = copy.deepcopy(wave), copy.deepcopy(manifest)
    next(r for r in w["work_packages"] if r["id"] == "CF-PROOF")["depends_on"].remove("CF-INTEGRATE")
    m["expected_dependencies"]["CF-PROOF"].remove("CF-INTEGRATE")
    cases.append(("proof before integration even with edited manifest", w, specs, m))
    for dependency in ("OBS-S3", "ARCH-ADR"):
        w, s, m = copy.deepcopy(wave), copy.deepcopy(specs), copy.deepcopy(manifest)
        next(r for r in w["work_packages"] if r["id"] == "TAIL-10")["depends_on"].remove(dependency)
        next(r for r in s["specifications"] if r["id"] == "TAIL-10")["depends_on"].remove(dependency)
        m["expected_dependencies"]["TAIL-10"].remove(dependency)
        cases.append((f"final closeout loses {dependency} even with edited manifest", w, s, m))
    w = copy.deepcopy(wave)
    next(r for r in w["work_packages"] if r["id"] == "RT-PROVIDER")["issue"] = None
    cases.append(("lost existing provider identity", w, specs, manifest))
    for family, fields in manifest["baseline_obligation_owners"].items():
        obligation, owners = next(iter(fields["acceptance"].items()))
        s = copy.deepcopy(specs)
        next(r for r in s["specifications"] if r["id"] == owners[0])["acceptance"].remove(obligation)
        cases.append((f"baseline acceptance lost {family}", wave, s, manifest))
    s = copy.deepcopy(specs)
    next(r for r in s["specifications"] if r["id"] == "CF-REVIEW")["stop_conditions"].append("no_source_mutation")
    cases.append(("successful proof used as stop condition", wave, s, manifest))
    s = copy.deepcopy(specs)
    next(r for r in s["specifications"] if r["id"] == "RT-PROVIDER")["provider_acceptance_matrix"]["routes"].pop()
    cases.append(("missing provider qualification route", wave, s, manifest))
    s = copy.deepcopy(specs)
    next(r for r in s["specifications"] if r["id"] == "QUAL-EVIDENCE")["criterion_consumer_boundary"]["original_finding_denominator"] = 5
    cases.append(("historical finding denominator collapsed", wave, s, manifest))
    missed = ["Atomic negative fixture not rejected: " + name for name, w, s, m in cases if not atomic_failures(w, s, m)]
    return missed, len(cases)
