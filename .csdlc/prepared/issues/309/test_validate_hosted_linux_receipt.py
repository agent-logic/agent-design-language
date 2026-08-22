#!/usr/bin/env python3
"""Live-provenance positive and tamper-negative tests for #309."""
from __future__ import annotations
import copy, json, pathlib, shutil, subprocess

ROOT = pathlib.Path(__file__).resolve().parents[4]
VALIDATOR = ROOT / ".csdlc/prepared/issues/309/validate_hosted_linux_receipt.py"
RECEIPT = ROOT / ".csdlc/evidence/309/github-linux-ci.json"
WORK = ROOT / ".csdlc/evidence/309/.hosted-validator-test"

def invoke(name: str, value: dict, expected: int) -> None:
    path = WORK / f"{name}.json"; path.write_text(json.dumps(value) + "\n")
    result = subprocess.run(["python3", str(VALIDATOR), str(path), "--root", str(ROOT)], capture_output=True, text=True)
    if result.returncode != expected: raise AssertionError(f"{name}: {result.returncode}: {result.stdout}{result.stderr}")

def main() -> int:
    if WORK.exists(): shutil.rmtree(WORK)
    WORK.mkdir(parents=True)
    try:
        valid = json.loads(RECEIPT.read_text())
        invoke("live-valid", valid, 0)
        wrong_run = copy.deepcopy(valid); wrong_run["workflow_run_id"] = 1; invoke("wrong-run", wrong_run, 1)
        wrong_job = copy.deepcopy(valid); wrong_job["jobs"][0]["job_id"] = 1; invoke("wrong-job", wrong_job, 1)
        wrong_digest = copy.deepcopy(valid); wrong_digest["jobs"][0]["log_sha256"] = "0" * 64; invoke("wrong-log-digest", wrong_digest, 1)
        wrong_pr = copy.deepcopy(valid); wrong_pr["pull_request"] = 459; invoke("wrong-pr", wrong_pr, 1)
        missing = copy.deepcopy(valid); missing["jobs"].pop(); invoke("missing-job", missing, 1)
    finally: shutil.rmtree(WORK, ignore_errors=True)
    print("PASS test_validate_hosted_linux_receipt live=1 negatives=5")
    return 0

if __name__ == "__main__": raise SystemExit(main())
