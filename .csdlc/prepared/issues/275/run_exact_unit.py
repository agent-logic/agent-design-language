#!/usr/bin/env python3
"""Run the exact nonzero #275 private unit denominator."""
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[4]
NAME = "distributed::integrated_serving_authority_snapshot::tests::normalized_receipt_rejects_tamper"
BASE = ["cargo", "test", "--locked", "--manifest-path", "adl-runtime/Cargo.toml", "--lib", "--features", "internal-test-fixtures"]
listed = subprocess.run(BASE + ["--", "--list"], cwd=ROOT, text=True, capture_output=True)
if listed.returncode:
    sys.stderr.write(listed.stdout + listed.stderr); raise SystemExit(listed.returncode)
selected = [line.split(": test", 1)[0] for line in listed.stdout.splitlines() if line.endswith(": test") and "integrated_serving_authority_snapshot" in line]
if selected != [NAME]:
    raise SystemExit(f"FAIL: exact private unit denominator drifted: {selected!r}")
result = subprocess.run(BASE + [NAME, "--", "--exact"], cwd=ROOT, text=True, capture_output=True)
sys.stdout.write(result.stdout); sys.stderr.write(result.stderr)
output = result.stdout + result.stderr
if result.returncode or not re.search(r"running 1 test\b", output) or not re.search(r"test result: ok\. 1 passed; 0 failed; 0 ignored;", output):
    raise SystemExit(result.returncode or 1)
print("PASS: exact #275 private unit 1 passed, 0 failed, 0 ignored")
