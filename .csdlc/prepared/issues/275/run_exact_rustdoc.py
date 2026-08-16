#!/usr/bin/env python3
"""Run and parse the exact three-case #275 compile-denial denominator."""
import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[4]
command = ["cargo", "test", "--locked", "--manifest-path", "adl-runtime/Cargo.toml", "--doc", "integrated_serving_authority_snapshot"]
result = subprocess.run(command, cwd=ROOT, text=True, capture_output=True)
sys.stdout.write(result.stdout); sys.stderr.write(result.stderr)
output = result.stdout + result.stderr
cases = re.findall(r"integrated_serving_authority_snapshot.*compile fail \.\.\. ok", output)
if result.returncode or len(cases) != 3 or not re.search(r"test result: ok\. 3 passed; 0 failed; 0 ignored;", output):
    raise SystemExit(result.returncode or 1)
print("PASS: exact #275 rustdoc denial 3 passed, 0 failed, 0 ignored")
