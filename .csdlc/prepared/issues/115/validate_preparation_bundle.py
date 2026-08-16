#!/usr/bin/env python3
import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]

def git_common_dir():
    out = subprocess.check_output(
        ["git", "rev-parse", "--git-common-dir"],
        cwd=ROOT,
        text=True,
    ).strip()
    path = Path(out)
    return path if path.is_absolute() else (ROOT / path).resolve()

def load_json(path):
    return json.loads((ROOT / path).read_text())

def load_common_json(path):
    return json.loads((git_common_dir() / path).read_text())

def load_terminal(issue_num):
    path = git_common_dir() / "csdlc-v2" / "derived-terminal" / f"{issue_num}.json"
    if not path.exists():
        errors.append(f"#{issue_num} terminal cache missing")
        return None
    terminal = json.loads(path.read_text())
    finish_binary = git_common_dir().parent / ".adl/bin/csdlc-v2/csdlc-finish"
    if not finish_binary.is_file():
        errors.append(f"typed csdlc-finish binary missing: {finish_binary}")
        return None
    finish = subprocess.run(
        [
            str(finish_binary),
            "--root", str(ROOT),
            "--validate-cached-issue", str(issue_num),
        ],
        cwd=ROOT,
        text=True,
        capture_output=True,
        check=False,
    )
    if finish.returncode != 0:
        errors.append(f"#{issue_num} typed canonical cache validation failed: {finish.stderr.strip()}")
        return None
    canonical = json.loads(finish.stdout)
    if canonical.get("canonical_match") is not True:
        errors.append(f"#{issue_num} terminal cache does not match canonical issue truth")
    canonical_terminal = canonical.get("terminal", {})
    for field in ["issue", "disposition", "issue_state", "head_sha", "merge_sha", "digest", "canonical_digest", "canonical_generation"]:
        if canonical_terminal.get(field) != terminal.get(field):
            errors.append(f"#{issue_num} terminal cache field {field} differs from typed canonical validation")
    if terminal.get("issue") != issue_num:
        errors.append(f"#{issue_num} terminal cache issue mismatch")
    if terminal.get("disposition") != "merged":
        errors.append(f"#{issue_num} terminal cache is not merged")
    if terminal.get("issue_state") != "closed_by_merged_pr":
        errors.append(f"#{issue_num} terminal cache is not closed_by_merged_pr")
    merge_sha = terminal.get("merge_sha")
    if not merge_sha:
        errors.append(f"#{issue_num} terminal cache missing merge_sha")
    else:
        result = subprocess.run(
            ["git", "merge-base", "--is-ancestor", merge_sha, "origin/main"],
            cwd=ROOT,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=False,
        )
        if result.returncode != 0:
            errors.append(f"#{issue_num} merge_sha is not ancestral to origin/main: {merge_sha}")
    return terminal

errors = []
index_path = ROOT / ".csdlc/issues/115/index.json"
index = load_json(".csdlc/issues/115/index.json") if index_path.exists() else None
issue = load_common_json("csdlc-v2/requests/issue115-typed-read-canonical-recovery-20260813T1705Z.result.json")["issue"]
graph = load_json(".csdlc/prepared/issues/110/graph.json")
readiness_packet = ROOT / ".csdlc/prepared/issues/115/readiness-packet.md"
design = (ROOT / ".csdlc/prepared/issues/115/design.md").read_text()
sip_values = load_json(".csdlc/issues/115/cards/sip.values.json")["content"]["values"]
stp_values = load_json(".csdlc/issues/115/cards/stp.values.json")["content"]["values"]
spp_values = load_json(".csdlc/issues/115/cards/spp.values.json")["content"]["values"]

if issue["number"] != 115:
    errors.append("typed read does not describe issue 115")
if "[WP-18C.05]" not in issue["title"] or "multi-agent rooms" not in issue["title"]:
    errors.append("live title is not expected #115 identity")
if issue["state"] != "open":
    errors.append("issue 115 is not open")
body = issue["body"]
for needle in ["#111 canonical conversation sessions", "#112 Layer 8 authority", "#113 complete live roster", "#270 trusted recipient-acknowledgement", "csdlc-graph-reconciliation:wp18c-115-add-270-from-112-v1"]:
    if needle not in body:
        errors.append(f"live body missing required dependency/marker text: {needle}")
graph_deps = graph["nodes"]["115"]["depends_on"]
if graph_deps != [111, 112, 113, 270]:
    errors.append(f"#110 graph has unexpected #115 dependencies: {graph_deps}")
if not readiness_packet.exists():
    errors.append("readiness packet missing")
else:
    readiness_text = readiness_packet.read_text()
    for needle in [
        "#111",
        "#112",
        "#113",
        "#270",
        "derived-terminal caches",
        "does not redefine #112",
        "does not redefine #270",
        "initialized and unbound",
    ]:
        if needle not in readiness_text:
            errors.append(f"readiness packet missing marker: {needle}")
terminals = {issue_num: load_terminal(issue_num) for issue_num in [111, 112, 113, 270]}
if index:
    if index["issue"] != 115:
        errors.append("local index issue mismatch")
    if index["phase"] not in {"initialized", "ready"}:
        errors.append("local index phase is beyond safe preparation")
    if index.get("worktree") is not None:
        errors.append("#115 unexpectedly has a bound worktree")
    if index.get("branch") is not None:
        errors.append("#115 unexpectedly has a bound branch")

required_scope = [
    "Versioned room, participant, mention, routing, and delivery contracts",
    "Runtime membership and policy enforcement",
    "Observatory room list, participant list, transcript, composer, and delivery states",
    "Ordering, fan-out, partial-failure, replay, and adversarial proof",
]
if sip_values.get("declared_scope") != required_scope:
    errors.append("STP declared scope does not match the governed-room preparation boundary")
for needle in [
    "Unbounded broadcast",
    "Implicit recipient selection by browser",
    "Cross-Polis federation policy",
    "Redefining #112 authority or #270 acknowledgement trust",
    "Branch/worktree bind, implementation, publication, merge, or closeout",
]:
    if needle not in stp_values.get("non_goals", []):
        errors.append(f"STP non-goals missing required boundary: {needle}")
for needle in [
    "explicit room, participant, mention, routing, and delivery contracts",
    "does not redefine",
    "#112 Layer 8 authority and audit",
    "#270 trusted recipient-acknowledgement Runtime API protocol",
    "No branch/worktree is bound",
    "No product code is changed",
]:
    if needle not in design:
        errors.append(f"design missing concrete scope/non-absorption statement: {needle}")

expected_affected_areas = [
    ".csdlc/prepared/issues/115",
    ".csdlc/prepared/issues/115/validate_preparation_bundle.py",
    ".csdlc/issues/115",
]
if spp_values.get("affected_areas") != expected_affected_areas:
    errors.append(f"SPP affected areas drifted: {spp_values.get('affected_areas')}")
status_lines = subprocess.check_output(
    ["git", "status", "--porcelain=v1", "--untracked-files=all"], cwd=ROOT, text=True
).splitlines()
allowed_prefixes = (
    ".csdlc/issues/115/",
    ".csdlc/prepared/issues/115/",
)
for line in status_lines:
    path = line[3:]
    if path in {".csdlc/locks/115.lock", ".csdlc/prepared/issues/110/graph.json"}:
        continue
    if not path.startswith(allowed_prefixes):
        errors.append(f"preparation changed path escapes #115/read-only graph boundary: {path}")

if errors:
    print(json.dumps({"schema": "adl.issue_115.preparation_validator.v1", "status": "failed", "errors": errors}, indent=2))
    raise SystemExit(1)

print(json.dumps({
    "schema": "adl.issue_115.preparation_validator.v1",
    "status": "passed",
    "issue": 115,
    "phase": index["phase"] if index else "unbootstrapped",
    "generation": index["generation"] if index else None,
    "execution_ready": False,
    "dependencies": {
        str(issue_num): {
            "state": "terminal-cache",
            "merge_sha": terminals[issue_num].get("merge_sha") if terminals[issue_num] else None,
            "ancestral_to_origin_main": terminals[issue_num] is not None,
            "canonical_generation": terminals[issue_num].get("canonical_generation") if terminals[issue_num] else None,
            "canonical_digest": terminals[issue_num].get("canonical_digest") if terminals[issue_num] else None,
            "terminal_digest": terminals[issue_num].get("digest") if terminals[issue_num] else None,
            "head_sha": terminals[issue_num].get("head_sha") if terminals[issue_num] else None,
            "canonical_match": terminals[issue_num] is not None,
        }
        for issue_num in [111, 112, 113, 270]
    }
}))
