"""Preserve the existing planning checks and their nonzero fixture denominator."""
import json
import subprocess
import sys
result = subprocess.run([sys.executable, "docs/milestones/v0.92.2/validate_planning.py", "--self-test"], capture_output=True, text=True)
print(result.stdout, end="")
print(result.stderr, end="", file=sys.stderr)
if result.returncode:
    raise SystemExit(result.returncode)
data = json.loads(result.stdout)
assert data["status"] == "pass" and data["failures"] == []
assert data["work_packages"] == 69
assert data["negative_fixtures"] == 298
