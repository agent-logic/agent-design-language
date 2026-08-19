#!/usr/bin/env python3
import argparse, hashlib, json, os, re, subprocess, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
EVIDENCE = ROOT / ".csdlc/evidence/343/terminal-children.json"
HEX40 = re.compile(r"[0-9a-f]{40}")
HEX64 = re.compile(r"[0-9a-f]{64}")

def exact_keys(value, expected, label, errors):
    if not isinstance(value, dict) or set(value) != set(expected):
        errors.append(f"{label} keys must equal {sorted(expected)}")
        return False
    return True

def repo_file(value, label, errors):
    if not isinstance(value, str) or value.startswith("/") or ".." in Path(value).parts:
        errors.append(f"{label} must be a safe repo-relative path")
        return None
    path = ROOT / value
    if not path.is_file():
        errors.append(f"{label} does not name a retained file")
        return None
    return path

def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def commit(sha, label, errors):
    if not isinstance(sha, str) or not HEX40.fullmatch(sha):
        errors.append(f"{label} must be lowercase 40-hex")
        return False
    if subprocess.run(["git", "cat-file", "-e", f"{sha}^{{commit}}"], cwd=ROOT, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL).returncode:
        errors.append(f"{label} is not a local commit")
        return False
    return True

def ancestor(sha, base, label, errors):
    if subprocess.run(["git", "merge-base", "--is-ancestor", sha, base], cwd=ROOT).returncode:
        errors.append(f"{label} is not ancestral to candidate base")

def canonical_terminal(issue, errors):
    configured = os.environ.get("CSDLC_V2_BIN_DIR")
    if not configured:
        errors.append("CSDLC_V2_BIN_DIR must name the operator-resolved stable v2 binary directory")
        return None
    bin_dir = Path(configured)
    if not bin_dir.is_absolute():
        errors.append("CSDLC_V2_BIN_DIR must be absolute")
        return None
    installer = bin_dir / "csdlc-install"
    owner = bin_dir / "csdlc-finish"
    resolved = subprocess.run([str(installer), "resolve", "--repo", str(ROOT), "--issue", "343"], cwd=ROOT, text=True, capture_output=True)
    if resolved.returncode or resolved.stdout.strip() != '"v2"' or not owner.is_file():
        errors.append("stable typed v2 terminal owner is unavailable or not selected")
        return None
    result = subprocess.run([
        str(owner),
        "--root", str(ROOT), "--validate-cached-issue", str(issue),
    ], cwd=ROOT, text=True, capture_output=True)
    if result.returncode:
        errors.append(f"typed terminal validation failed for #{issue}")
        return None
    try: receipt = json.loads(result.stdout)
    except json.JSONDecodeError:
        errors.append(f"typed terminal validation output is invalid for #{issue}")
        return None
    if receipt.get("schema") != "csdlc.derived_terminal_validation.v1" or receipt.get("canonical_match") is not True:
        errors.append(f"typed terminal validation is noncanonical for #{issue}")
        return None
    terminal = receipt.get("terminal")
    if not isinstance(terminal, dict) or terminal.get("issue") != issue:
        errors.append(f"typed terminal validation identity mismatch for #{issue}")
        return None
    return terminal

def retained(path_value, digest_value, label, errors):
    path = repo_file(path_value, f"{label}.path", errors)
    if not isinstance(digest_value, str) or not HEX64.fullmatch(digest_value):
        errors.append(f"{label}.sha256 must be lowercase 64-hex")
    elif path and sha256(path) != digest_value:
        errors.append(f"{label} digest mismatch")

def validate_terminal(payload, errors):
    root_keys = {"schema", "candidate_base_sha", "children", "historical", "packet"}
    if not exact_keys(payload, root_keys, "root", errors): return
    if payload["schema"] != "adl.issue343.closeout-evidence.v2": errors.append("wrong schema")
    base = payload["candidate_base_sha"]
    base_ok = commit(base, "candidate_base_sha", errors)
    actual_base = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    if base != actual_base: errors.append("candidate_base_sha must equal current immutable HEAD")
    children = payload["children"]
    if not exact_keys(children, {"256", "341"}, "children", errors): children = {}
    child_keys = {"issue","issue_state","pull_request","reviewed_revision","merge_sha","terminal_generation","terminal_digest","terminal_cache_path","terminal_cache_sha256","canonical_match","merge_ancestral","review_artifact_path","review_artifact_sha256","checks_artifact_path","checks_artifact_sha256","demo_paths","artifact_sha256"}
    for key in ("256", "341"):
        item = children.get(key, {})
        if not exact_keys(item, child_keys, f"children.{key}", errors): continue
        terminal = canonical_terminal(int(key), errors)
        if item["issue"] != int(key) or item["issue_state"] != "closed": errors.append(f"children.{key} must identify closed issue #{key}")
        if not isinstance(item["pull_request"], int) or item["pull_request"] <= 0: errors.append(f"children.{key}.pull_request must be positive integer")
        commit(item["reviewed_revision"], f"children.{key}.reviewed_revision", errors)
        merge_ok = commit(item["merge_sha"], f"children.{key}.merge_sha", errors)
        if base_ok and merge_ok: ancestor(item["merge_sha"], base, f"children.{key}.merge_sha", errors)
        if not isinstance(item["terminal_generation"], int) or item["terminal_generation"] < 0: errors.append(f"children.{key}.terminal_generation invalid")
        if not isinstance(item["terminal_digest"], str) or not HEX64.fullmatch(item["terminal_digest"]): errors.append(f"children.{key}.terminal_digest invalid")
        retained(item["terminal_cache_path"], item["terminal_cache_sha256"], f"children.{key}.terminal_cache", errors)
        if item["canonical_match"] is not True or item["merge_ancestral"] is not True: errors.append(f"children.{key} canonical/ancestral assertions must be true")
        review_path = repo_file(item["review_artifact_path"], f"children.{key}.review_artifact_path", errors)
        checks_path = repo_file(item["checks_artifact_path"], f"children.{key}.checks_artifact_path", errors)
        retained(item["review_artifact_path"], item["review_artifact_sha256"], f"children.{key}.review_artifact", errors)
        retained(item["checks_artifact_path"], item["checks_artifact_sha256"], f"children.{key}.checks_artifact", errors)
        for path, schema, label in ((review_path,"adl.issue343.child-review.v1","review"),(checks_path,"adl.issue343.child-checks.v1","checks")):
            if path:
                try: proof = json.loads(path.read_text())
                except (OSError,json.JSONDecodeError): errors.append(f"children.{key} {label} artifact invalid"); continue
                required = {"schema":schema,"issue":int(key),"pull_request":item["pull_request"],"revision":item["reviewed_revision"],"result":"passed"}
                for field,value in required.items():
                    if proof.get(field) != value: errors.append(f"children.{key} {label} artifact {field} mismatch")
        if terminal:
            expected = {
                "issue": int(key), "issue_state": "closed_by_merged_pr", "pull_request": item["pull_request"],
                "head_sha": item["reviewed_revision"], "merge_sha": item["merge_sha"],
                "canonical_generation": item["terminal_generation"], "canonical_digest": item["terminal_digest"],
            }
            for field, value in expected.items():
                if terminal.get(field) != value: errors.append(f"children.{key}.{field} disagrees with typed terminal authority")
        if not isinstance(item["demo_paths"], list) or not item["demo_paths"]: errors.append(f"children.{key}.demo_paths must be nonempty")
        else:
            for i, value in enumerate(item["demo_paths"]): repo_file(value, f"children.{key}.demo_paths[{i}]", errors)
        artifacts = item["artifact_sha256"]
        if not isinstance(artifacts, dict) or not artifacts: errors.append(f"children.{key}.artifact_sha256 must be nonempty")
        else:
            for path, digest in artifacts.items(): retained(path, digest, f"children.{key}.artifact", errors)
    historical = payload["historical"]
    if not exact_keys(historical, {"WP-17", "WP-19"}, "historical", errors): historical = {}
    historical_keys = {"issue","terminal_generation","terminal_digest","canonical_match","merge_sha","merge_ancestral","validated","evidence_path","evidence_sha256"}
    for wp in ("WP-17", "WP-19"):
        item = historical.get(wp, {})
        if not exact_keys(item, historical_keys, f"historical.{wp}", errors): continue
        terminal = canonical_terminal(item.get("issue"), errors) if isinstance(item.get("issue"), int) and item["issue"] > 0 else None
        if not isinstance(item["issue"], int) or item["issue"] <= 0: errors.append(f"historical.{wp}.issue invalid")
        if not isinstance(item["terminal_generation"], int) or item["terminal_generation"] < 0: errors.append(f"historical.{wp}.terminal_generation invalid")
        if not isinstance(item["terminal_digest"], str) or not HEX64.fullmatch(item["terminal_digest"]): errors.append(f"historical.{wp}.terminal_digest invalid")
        merge_ok = commit(item["merge_sha"], f"historical.{wp}.merge_sha", errors)
        if base_ok and merge_ok: ancestor(item["merge_sha"], base, f"historical.{wp}.merge_sha", errors)
        if item["canonical_match"] is not True or item["merge_ancestral"] is not True or item["validated"] is not True: errors.append(f"historical.{wp} assertions must be true")
        retained(item["evidence_path"], item["evidence_sha256"], f"historical.{wp}.evidence", errors)
        if terminal:
            expected = {"canonical_generation":item["terminal_generation"],"canonical_digest":item["terminal_digest"],"merge_sha":item["merge_sha"]}
            for field, value in expected.items():
                if terminal.get(field) != value: errors.append(f"historical.{wp}.{field} disagrees with typed terminal authority")

def validate_packet(payload, errors):
    validate_terminal(payload, errors)
    packet = payload.get("packet", {})
    keys = {"sprint_packet_path","sprint_packet_sha256","release_truth_reconciled","redaction_passed","credentials_retained","private_evidence_retained","unsupported_publication_claims","excluded_issues","handoff_issues","review"}
    if not exact_keys(packet, keys, "packet", errors): return
    retained(packet["sprint_packet_path"], packet["sprint_packet_sha256"], "packet.sprint_packet", errors)
    if packet["release_truth_reconciled"] is not True or packet["redaction_passed"] is not True: errors.append("release truth and redaction must pass")
    if packet["credentials_retained"] is not False or packet["private_evidence_retained"] is not False or packet["unsupported_publication_claims"] is not False: errors.append("credential/private/unsupported-publication assertions must be false")
    if packet["excluded_issues"] != [342,340,84,251]: errors.append("excluded issues must equal [342,340,84,251]")
    if packet["handoff_issues"] != [307,308]: errors.append("handoff issues must equal [307,308]")
    review = packet["review"]
    review_keys = {"result","reviewer","session_uuid","reviewed_revision","packet_sha256","artifact_path","artifact_sha256"}
    if not exact_keys(review, review_keys, "packet.review", errors): return
    uuid = re.compile(r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}")
    if review["result"] != "passed": errors.append("packet review result must equal passed")
    if not isinstance(review["session_uuid"], str) or not uuid.fullmatch(review["session_uuid"]): errors.append("packet review session_uuid must be canonical lowercase UUID")
    if review["reviewer"] != f"fresh-session:{review['session_uuid']}": errors.append("packet reviewer must bind the fresh session UUID")
    if review["reviewed_revision"] != payload.get("candidate_base_sha"): errors.append("packet review must bind candidate_base_sha")
    if review["packet_sha256"] != packet["sprint_packet_sha256"]: errors.append("packet review must bind sprint packet digest")
    retained(review["artifact_path"], review["artifact_sha256"], "packet.review.artifact", errors)
    review_path = repo_file(review["artifact_path"], "packet.review.artifact_path", errors)
    if review_path:
        try: proof = json.loads(review_path.read_text())
        except (OSError,json.JSONDecodeError): errors.append("packet review artifact is invalid JSON")
        else:
            expected = {"schema":"adl.issue343.sprint-review.v1","result":"passed","reviewer":review["reviewer"],"session_uuid":review["session_uuid"],"reviewed_revision":review["reviewed_revision"],"packet_sha256":review["packet_sha256"],"findings":[],"unresolved_findings":[]}
            if set(proof) != set(expected): errors.append("packet review artifact has unexpected fields")
            for field,value in expected.items():
                if proof.get(field) != value: errors.append(f"packet review artifact {field} mismatch")

parser = argparse.ArgumentParser()
mode = parser.add_mutually_exclusive_group()
mode.add_argument("--terminal", action="store_true")
mode.add_argument("--packet", action="store_true")
args = parser.parse_args()
if not args.terminal and not args.packet:
    print(json.dumps({"schema":"adl.issue343.readiness.v2","status":"pass","preparation_only":True}))
    raise SystemExit(0)
errors = []
try: payload = json.loads(EVIDENCE.read_text())
except FileNotFoundError: payload = {}; errors.append("terminal child evidence missing")
except (OSError, json.JSONDecodeError) as exc: payload = {}; errors.append(f"terminal child evidence unreadable: {exc}")
if not errors: (validate_packet if args.packet else validate_terminal)(payload, errors)
print(json.dumps({"schema":"adl.issue343.readiness.v2","mode":"packet" if args.packet else "terminal","status":"fail" if errors else "pass","preparation_only":False,"errors":errors}, sort_keys=True))
raise SystemExit(1 if errors else 0)
