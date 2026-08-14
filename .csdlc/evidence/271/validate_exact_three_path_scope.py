#!/usr/bin/env python3
import json
import subprocess
import sys

BASE = "codex/271-prebind-main-base"
ALLOWED_PREFIXES = (
    ".csdlc/evidence/271/",
    ".csdlc/issues/271/",
    ".csdlc/prepared/issues/271/",
)
ALLOWED_EXACT = {
    ".csdlc/locks/271.lock",
    "adl/tools/validate_layer8_authority_observatory_ui.sh",
    "demos/html-observatory/app.js",
    "demos/html-observatory/styles.css",
}

result = subprocess.run(
    ["git", "diff", "--name-only", f"{BASE}...HEAD"],
    check=True,
    text=True,
    stdout=subprocess.PIPE,
)
status = subprocess.run(
    ["git", "status", "--porcelain"],
    check=True,
    text=True,
    stdout=subprocess.PIPE,
)
paths = set(line for line in result.stdout.splitlines() if line.strip())
for line in status.stdout.splitlines():
    if not line:
        continue
    path = line[3:] if line.startswith("?? ") else line[3:].strip()
    if path.endswith("/"):
        nested = subprocess.run(
            ["find", path, "-type", "f"],
            check=True,
            text=True,
            stdout=subprocess.PIPE,
        )
        paths.update(item for item in nested.stdout.splitlines() if item.strip())
    else:
        paths.add(path)
paths = sorted(paths)
rejected = [
    path for path in paths
    if path not in ALLOWED_EXACT and not path.startswith(ALLOWED_PREFIXES)
]
payload = {
    "schema": "adl.issue271.exact_three_path_scope.v1",
    "base": BASE,
    "status": "passed" if not rejected else "failed",
    "paths": paths,
    "allowed_product_test_paths": sorted(ALLOWED_EXACT),
    "rejected": rejected,
}
print(json.dumps(payload, sort_keys=True))
if rejected:
    sys.exit(1)
