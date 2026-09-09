#!/usr/bin/env python3
"""PVF: deterministic local documentation contract, small, required for #767.

Checks evidence integrity and projection consistency; never grants release proof.
"""
import copy
import hashlib
import json
from pathlib import Path
import re
import subprocess
from functools import lru_cache

ROOT = Path(__file__).resolve().parents[6]
BASE = ROOT / "docs/milestones/v0.92.1"
PACKET = BASE / "evidence/release/current-status"
DOCS = [BASE / f"{name}_v0.92.1.md" for name in (
    "FEATURE_PROOF_COVERAGE", "DEMO_MATRIX", "RELEASE_NOTES", "MILESTONE_CHECKLIST"
)]
STATES = {"implemented", "proved", "deferred", "blocked", "not_applicable"}


@lru_cache(maxsize=None)
def git_hash(revision, path):
    result = subprocess.run(["git", "show", f"{revision}:{path}"], cwd=ROOT,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    return hashlib.sha256(result.stdout).hexdigest() if result.returncode == 0 else None


def validate(data, texts):
    errors = []
    def check(condition, message):
        if not condition:
            errors.append(message)
    check(data["release_authorized"] is False, "projection cannot grant release authority")
    check(data["release_decision"] == "blocked", "unresolved remediation requires blocked release")
    check(bool(re.fullmatch(r"[0-9a-f]{40}", data["source_revision"])), "exact source revision missing")
    check(data.get("publication_refresh_required") is False, "publication refresh incomplete")
    check(data.get("projection_status") == "refreshed_with_explicit_release_debt", "stale projection status")
    observation = data["issue_observation"]
    observed_bytes = (ROOT / observation["path"]).read_bytes()
    check(hashlib.sha256(observed_bytes).hexdigest() == observation["sha256"], "issue observation drift")
    observed = json.loads(observed_bytes)
    check(observed["source_revision"] == data["source_revision"], "unbound issue observation")
    check(observed["observed_at"] == data["as_of"], "issue observation time mismatch")
    open_issues = {r["number"] for r in observed["issues"] if r["state"] == "OPEN"}
    for row in data["features"] + data["work_packages"]:
        owners = row.get("debt_owners", row.get("proof_debt_owners", []))
        check(set(owners) <= open_issues, "closed or unobserved active debt owner")
    for row in data["checklist"]:
        check(row["owner"] is None or row["owner"] in open_issues, "closed checklist owner")
    sources = {s["path"]: s for s in data["sources"]}
    check(len(sources) == len(data["sources"]), "duplicate source")
    for path, source in sources.items():
        file = ROOT / path
        check(file.is_file(), f"missing source: {path}")
        if file.is_file():
            check(hashlib.sha256(file.read_bytes()).hexdigest() == source["sha256"], f"source drift: {path}")
        check(source["revision"] == data["source_revision"], f"unbound source revision: {path}")
        check(git_hash(source["revision"], path) == source["sha256"], f"source does not match Git revision: {path}")
    admission = json.loads((BASE / "evidence/integration/release-tail-admission.json").read_text())
    expected = {r["planned_id"]: r for r in admission["execution_issues"]}
    work = {r["id"]: r for r in data["work_packages"]}
    check(len(work) == len(data["work_packages"]), "duplicate work package")
    check(set(work) == set(expected), "execution work-package denominator mismatch")
    for ident, row in work.items():
        if ident in expected:
            check(row["issue"] == expected[ident]["issue"], f"wrong issue: {ident}")
            check(row["implementation_revision"] == expected[ident].get("revision"), f"wrong implementation revision: {ident}")
        check(bool(row["proof_debt_owners"]), f"unowned proof debt: {ident}")
        check(all(p in sources for p in row["implementation_evidence"]), f"unhashed evidence: {ident}")
    features = data["features"]
    check(len({f["id"] for f in features}) == len(features), "duplicate feature")
    covered = [w for f in features for w in f["work_packages"]]
    check(len(covered) == len(set(covered)) and set(covered) == set(expected), "feature/work-package coverage mismatch")
    table = []
    for f in features:
        statuses = [f[k] for k in ("delivery_status", "proof_status", "demo_status", "release_status")]
        check(all(s in STATES for s in statuses), f"invalid status: {f['id']}")
        check(bool(f["debt_owners"]) and bool(f["limit"]), f"unowned or unexplained incomplete lane: {f['id']}")
        check(bool(f["demo_scope"]), f"unbounded demo claim: {f['id']}")
        table.append([f["id"], f["name"], *statuses])
    for name, text in texts.items():
        blocks = re.findall(r"<!-- release-status:start -->(.*?)<!-- release-status:end -->", text, re.S)
        check(len(blocks) == 1, f"missing/duplicate status block: {name}")
        if len(blocks) == 1:
            actual = [[cell.strip() for cell in line.split("|")[1:7]] for line in blocks[0].splitlines() if line.startswith("| ")][1:]
            check(actual == table, f"status table mismatch: {name}")
        check(data["source_revision"] in text, f"missing source binding: {name}")
        for exclusion in data["exclusions"]:
            check(f"{exclusion['id']}: {exclusion['status']}" in text, f"missing exclusion: {name}/{exclusion['id']}")
        for target in re.findall(r"\]\(([^)]+)\)", text):
            if "://" not in target and not target.startswith("#"):
                check((BASE / target.split("#")[0]).is_file(), f"broken document link: {name}/{target}")
    original = subprocess.run(
        ["git", "show", f"{data['checklist_source_revision']}:docs/milestones/v0.92.1/MILESTONE_CHECKLIST_v0.92.1.md"],
        cwd=ROOT, capture_output=True, text=True)
    obligations = [line[6:] for line in original.stdout.splitlines() if line.startswith("- [ ] ")]
    check(original.returncode == 0 and bool(obligations), "original checklist unavailable")
    check([r["obligation"] for r in data["checklist"]] == obligations, "checklist obligation denominator mismatch")
    checklist = texts["MILESTONE_CHECKLIST_v0.92.1.md"]
    for row in data["checklist"]:
        check(row["status"] in STATES, f"invalid checklist status: {row['id']}")
        check(bool(row["evidence"]) and bool(row["disposition"]), f"unexplained checklist disposition: {row['id']}")
        check(row["status"] != "blocked" or row["owner"] is not None, f"unowned checklist debt: {row['id']}")
        check(f"| {row['id']} | {row['obligation']} | {row['status']} |" in checklist,
              f"checklist projection mismatch: {row['id']}")
    return errors


def main():
    data = json.loads((PACKET / "status.json").read_text())
    texts = {p.name: p.read_text() for p in DOCS}
    errors = validate(data, texts)
    if errors:
        raise SystemExit("\n".join(errors))
    mutations = []
    missing = copy.deepcopy(data)
    missing["work_packages"].pop()
    mutations.append((missing, texts))
    unowned = copy.deepcopy(data)
    unowned["features"][0]["debt_owners"] = []
    mutations.append((unowned, texts))
    stale = copy.deepcopy(data)
    stale["sources"][0]["sha256"] = "0" * 64
    mutations.append((stale, texts))
    divergent = dict(texts)
    first = next(iter(divergent))
    divergent[first] = divergent[first].replace("| implemented | blocked |", "| proved | proved |", 1)
    mutations.append((data, divergent))
    fabricated = copy.deepcopy(data)
    fabricated["source_revision"] = "1" * 40
    for source in fabricated["sources"]:
        source["revision"] = "1" * 40
    fabricated_texts = {name: text.replace(data["source_revision"], "1" * 40) for name, text in texts.items()}
    mutations.append((fabricated, fabricated_texts))
    missing_obligation = copy.deepcopy(data)
    missing_obligation["checklist"].pop()
    mutations.append((missing_obligation, texts))
    closed_owner = copy.deepcopy(data)
    closed_owner["features"][0]["debt_owners"] = [764]
    mutations.append((closed_owner, texts))
    incomplete_refresh = copy.deepcopy(data)
    incomplete_refresh["publication_refresh_required"] = True
    mutations.append((incomplete_refresh, texts))
    for changed, documents in mutations:
        if not validate(changed, documents):
            raise SystemExit("negative fixture accepted")
    print(json.dumps({"result": "pass", "documents": len(texts), "lanes": len(data["features"]),
                      "work_packages": len(data["work_packages"]), "sources": len(data["sources"]),
                      "negative_fixtures": len(mutations), "release_authorized": False}))


if __name__ == "__main__":
    main()
