"""Check the original validator output; never substitute synthetic results."""
import json
import subprocess
import sys
from pathlib import Path

script, kind = sys.argv[1:]
result = subprocess.run([sys.executable, script, "--self-test"], capture_output=True, text=True)
print(result.stdout, end="")
print(result.stderr, end="", file=sys.stderr)
if result.returncode:
    raise SystemExit(result.returncode)
data = json.loads(result.stdout)
assert data["status"] == "pass" and data["failures"] == []
assert data["negative_fixtures"] > 0
if kind == "handoff":
    manifest = json.loads(Path("docs/milestones/v0.92.2/evidence/issue-917/HANDOFF_MANIFEST.json").read_text())
    assert data["documents"] == len(manifest["documents"]) > 0
    assert data["tasks"] == len(manifest["tasks"]) == 69
    assert data["prerequisites"] == len(manifest["prerequisites"]) == 24
    assert data["negative_fixtures"] == 8
    assert data["handoff_accepted"] is True and data["release_authorized"] is False
else:
    assert kind == "planning"
    assert data["work_packages"] == 69
    assert data["negative_fixtures"] == 298
