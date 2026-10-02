#!/usr/bin/env python3
"""Run the public ADL install proof into fresh Git-local evidence storage."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys
from datetime import datetime, timezone
import uuid


ROOT = Path(__file__).resolve().parents[2]
VERIFIER = ROOT / "tools/public_adl/verify_install.py"


def run(args):
    return subprocess.run(args, cwd=ROOT, text=True, capture_output=True)


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def validate_artifacts_and_cases(report, proof_dir):
    packages = report.get("packages", [])
    names = {"adl-schema", "adl-uts", "adl-language", "adl-compiler", "adl-legacy-contracts"}
    if len(packages) != 5 or {p.get("package") for p in packages} != names:
        raise ValueError("the five public package artifacts are required")
    if report.get("public_packages_consumed") != 5:
        raise ValueError("consumer did not consume five public packages")
    artifacts = set()
    for package in packages:
        name = package.get("artifact")
        if not isinstance(name, str) or Path(name).name != name or name in artifacts:
            raise ValueError("invalid or duplicate artifact name")
        artifacts.add(name)
        artifact = proof_dir / name
        if artifact.is_symlink() or not artifact.is_file() or sha256(artifact) != package.get("sha256"):
            raise ValueError("package artifact hash mismatch")
    lock = report.get("dependency_lock", {})
    if lock.get("path") != "consumer.Cargo.lock":
        raise ValueError("consumer dependency lock is missing")
    lock_path = proof_dir / lock["path"]
    if lock_path.is_symlink() or not lock_path.is_file() or sha256(lock_path) != lock.get("sha256"):
        raise ValueError("consumer dependency lock hash mismatch")
    consumer = report.get("consumer", {})
    expected_cases = {
        "language-compile-determinism", "legacy-resolution", "provider-dto-roundtrip",
        "declaration-endpoint-denials", "uts-three-versioned-schemas",
        "unsupported-uts-version-denied", "uts-inert-conformance",
    }
    cases = consumer.get("cases", [])
    if len(cases) != 7 or set(cases) != expected_cases:
        raise ValueError("required consumer cases are missing or duplicated")
    uts = consumer.get("uts_cases", [])
    if (consumer.get("uts_fixture_count") != 21 or len(uts) != 21
            or len({c.get("id") for c in uts}) != 21
            or any(not c.get("id") or c.get("passed") is not True
                   or c.get("execution_granted") is not False for c in uts)):
        raise ValueError("21 distinct inert UTS fixtures must pass")
    negatives = report.get("negative_cases", [])
    expected_negatives = {
        "private-credential-denied", "producer-source-denied", "network-denied",
        "sibling-producer-dependency-denied", "incompatible-package-version",
    }
    if (len(negatives) != 5 or {c.get("case") for c in negatives} != expected_negatives
            or any(c.get("status") != "rejected" or type(c.get("exit_code")) is not int
                   or c["exit_code"] == 0 for c in negatives)):
        raise ValueError("all five classified negative cases must reject")
    return {"public_packages_consumed": 5, "uts_fixtures_executed": 21,
            "artifact_hashes": {p["package"]: p["sha256"] for p in packages},
            "dependency_lock_sha256": lock["sha256"]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--issue", type=int, required=True)
    args = parser.parse_args()
    if args.issue <= 0:
        parser.error("--issue must be positive")

    git_dir_result = run(["git", "rev-parse", "--path-format=absolute", "--git-common-dir"])
    if git_dir_result.returncode:
        raise SystemExit(git_dir_result.stderr.strip() or "unable to resolve Git metadata")
    git_dir = Path(git_dir_result.stdout.strip()).resolve()
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    run_id = f"{stamp}-{uuid.uuid4().hex}"
    evidence_dir = git_dir / "csdlc-v3/local/public-adl-proof" / str(args.issue) / run_id
    proof_dir = evidence_dir / "qualification"
    evidence_dir.mkdir(parents=True, exist_ok=False)

    completed = run([sys.executable, str(VERIFIER), "--output", str(proof_dir)])
    (evidence_dir / "stdout.txt").write_text(completed.stdout)
    (evidence_dir / "stderr.txt").write_text(completed.stderr)

    report_path = proof_dir / "report.json"
    result = {
        "schema": "adl.public.install-validator.v1",
        "status": "failed",
        "issue": args.issue,
        "exit_code": completed.returncode,
        "evidence_directory": str(evidence_dir),
        "report_path": str(report_path),
        "report_sha256": None,
        "source_revision": None,
        "consumer_cases_executed": 0,
        "negative_cases_executed": 0,
    }
    exit_code = completed.returncode or 2
    if completed.returncode == 0 and report_path.is_file():
        try:
            report = json.loads(report_path.read_text())
            head = run(["git", "rev-parse", "HEAD"])
            if head.returncode:
                raise ValueError("unable to resolve candidate revision")
            consumer = report.get("consumer", {})
            executed = consumer.get("executed_cases")
            expected = consumer.get("expected_cases")
            negative_executed = report.get("negative_executed")
            negative_expected = report.get("negative_expected")
            if report.get("schema") != "adl.public.install-proof.v1":
                raise ValueError("unexpected install-proof schema")
            if report.get("source_revision") != head.stdout.strip():
                raise ValueError("install proof is not bound to current HEAD")
            if (
                consumer.get("passed") is not True
                or type(executed) is not int
                or executed != 7
                or executed != expected
            ):
                raise ValueError("consumer scenario denominator did not pass")
            if (
                type(negative_executed) is not int
                or negative_executed != 5
                or negative_executed != negative_expected
            ):
                raise ValueError("negative scenario denominator did not pass")
            result.update(validate_artifacts_and_cases(report, proof_dir))
            result.update(
                {
                    "status": "passed",
                    "exit_code": 0,
                    "report_sha256": sha256(report_path),
                    "source_revision": report["source_revision"],
                    "consumer_cases_executed": executed,
                    "negative_cases_executed": negative_executed,
                }
            )
            exit_code = 0
        except (OSError, ValueError, TypeError, KeyError, AttributeError) as error:
            result["validation_error"] = str(error)
    elif completed.returncode == 0:
        result["validation_error"] = "verifier exited zero without report.json"

    result_path = evidence_dir / "validator-result.json"
    result_path.write_text(json.dumps(result, indent=2, sort_keys=True) + "\n")
    print(json.dumps(result, sort_keys=True, separators=(",", ":")))
    raise SystemExit(exit_code)


if __name__ == "__main__":
    main()
