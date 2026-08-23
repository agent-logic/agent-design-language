#!/usr/bin/env python3
"""Verify #309 hosted-Linux evidence against live GitHub Actions."""
from __future__ import annotations
import argparse, hashlib, json, pathlib, re, subprocess

REPO = "agent-logic/agent-design-language"
PR = 460
REQUIRED = {"adl-path-policy", "adl-tooling-contracts", "adl-rust-fmt-clippy", "adl-rust-tests", "adl-coverage", "adl-ci"}
EVIDENCE_PATHS = {
    ".csdlc/evidence/309/github-linux-ci.json",
    ".csdlc/prepared/issues/309/test_validate_hosted_linux_receipt.py",
    ".csdlc/prepared/issues/309/validate_hosted_linux_receipt.py",
    ".csdlc/issues/309/audit.jsonl",
    ".csdlc/issues/309/cards/sip.values.json",
    ".csdlc/issues/309/cards/sor.md",
    ".csdlc/issues/309/cards/sor.values.json",
    ".csdlc/issues/309/cards/spp.values.json",
    ".csdlc/issues/309/cards/srp.md",
    ".csdlc/issues/309/cards/srp.values.json",
    ".csdlc/issues/309/cards/stp.values.json",
    ".csdlc/issues/309/cards/vpp.values.json",
    ".csdlc/issues/309/index.json",
}
HEX40 = re.compile(r"^[0-9a-f]{40}$")
HEX64 = re.compile(r"^[0-9a-f]{64}$")
ANSI = re.compile(rb"\x1b\[[0-9;]*m")
SUMMARY = re.compile(r"Summary.*?\b(\d+) tests run:\s*(\d+) passed", re.S)

def output(argv: list[str], *, text: bool = True):
    return subprocess.check_output(argv, text=text)

def git(root: pathlib.Path, *args: str) -> str:
    return str(output(["git", "-C", str(root), *args])).strip()

def gh(endpoint: str):
    return json.loads(str(output(["gh", "api", endpoint])))

def validate(root: pathlib.Path, receipt: dict) -> list[str]:
    errors: list[str] = []
    source = str(receipt.get("source_head_sha", ""))
    head = git(root, "rev-parse", "HEAD")
    if receipt.get("schema") != "adl.issue309.github_linux_ci.v2": errors.append("schema mismatch")
    if receipt.get("repository") != REPO or receipt.get("pull_request") != PR: errors.append("repository/PR mismatch")
    if not HEX40.fullmatch(source): errors.append("source head invalid")
    elif subprocess.run(["git", "-C", str(root), "merge-base", "--is-ancestor", source, head]).returncode: errors.append("source head not ancestor")
    elif source != head:
        changed = set(filter(None, git(root, "diff", "--name-only", f"{source}..{head}").splitlines()))
        if not changed or not changed <= EVIDENCE_PATHS: errors.append("non-evidence drift after tested head")
    run_id = receipt.get("workflow_run_id")
    try:
        workflow = gh(f"repos/{REPO}/actions/runs/{run_id}")
        jobs_payload = gh(f"repos/{REPO}/actions/runs/{run_id}/jobs?per_page=100")
    except (subprocess.CalledProcessError, json.JSONDecodeError, TypeError, ValueError) as exc:
        return errors + [f"GitHub lookup failed:{type(exc).__name__}"]
    prs = {row.get("number") for row in workflow.get("pull_requests", []) if isinstance(row, dict)}
    if (workflow.get("event") != "pull_request" or workflow.get("status") != "completed" or
        workflow.get("conclusion") != "success" or workflow.get("head_sha") != source or
        PR not in prs or workflow.get("html_url") != receipt.get("workflow_run_url")):
        errors.append("workflow provenance mismatch")
    rows = receipt.get("jobs", []); required = receipt.get("required_jobs", [])
    recorded = {row.get("name"): row for row in rows if isinstance(row, dict)}
    live: dict[str, list[dict]] = {}
    for row in jobs_payload.get("jobs", []): live.setdefault(row.get("name"), []).append(row)
    if set(required) != REQUIRED or set(recorded) != REQUIRED or len(rows) != len(REQUIRED): errors.append("job denominator mismatch")
    for name in sorted(REQUIRED):
        rec = recorded.get(name, {}); matches = live.get(name, [])
        if len(matches) != 1: errors.append(f"job cardinality:{name}"); continue
        job = matches[0]; job_id = rec.get("job_id")
        if (job.get("id") != job_id or job.get("conclusion") != "success" or
            job.get("html_url") != rec.get("job_url") or rec.get("conclusion") != "success"):
            errors.append(f"job provenance:{name}"); continue
        log = bytes(output(["gh", "api", f"repos/{REPO}/actions/jobs/{job_id}/logs"], text=False))
        digest = str(rec.get("log_sha256", ""))
        if not HEX64.fullmatch(digest) or hashlib.sha256(log).hexdigest() != digest: errors.append(f"log digest:{name}")
        clean = ANSI.sub(b"", log).decode(errors="replace")
        if "Image: ubuntu-" not in clean: errors.append(f"Linux runner missing:{name}")
        if name == "adl-rust-tests":
            match = SUMMARY.search(clean); passed = rec.get("tests_passed")
            if not match or not isinstance(passed, int) or passed <= 0 or int(match.group(1)) != passed or int(match.group(2)) != passed:
                errors.append("test denominator mismatch")
    return errors

def main() -> int:
    parser = argparse.ArgumentParser(); parser.add_argument("receipt"); parser.add_argument("--root", default="."); args = parser.parse_args()
    path = pathlib.Path(args.receipt)
    if not path.is_file(): print(json.dumps({"status":"blocked","missing":str(path)})); return 2
    errors = validate(pathlib.Path(args.root).resolve(), json.loads(path.read_text()))
    print(json.dumps({"status":"pass" if not errors else "fail", "errors":errors}, sort_keys=True))
    return 0 if not errors else 1

if __name__ == "__main__": raise SystemExit(main())
