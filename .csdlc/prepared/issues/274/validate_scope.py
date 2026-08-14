#!/usr/bin/env python3
"""Post-bind exact changed-path and registration guard for issue #274."""
import json, pathlib, subprocess

root = pathlib.Path(__file__).resolve().parents[4]
implementation_base = "4db4c2b9a5d622fb7af6ffa1346b4d5406d4a699"
allowed = {
    "adl-runtime/src/distributed/observatory_serving_eligibility.rs",
    "adl-runtime/src/distributed/mod.rs",
    "adl-runtime/tests/distributed_observatory_serving_eligibility.rs",
    "adl/tools/check_coverage_impact.sh",
    "adl/tools/test_check_coverage_impact.sh",
    "adl/tools/run_pr_fast_coverage_lane.sh",
    "adl/tools/test_run_pr_fast_coverage_lane.sh",
}
result = subprocess.run(
    ["git", "diff", "--name-only", f"{implementation_base}...HEAD"],
    cwd=root,
    text=True,
    capture_output=True,
)
if result.returncode:
    raise SystemExit(result.returncode)
changed = {line for line in result.stdout.splitlines() if line and not line.startswith(".csdlc/")}
extra = changed - allowed
missing = {
    "adl-runtime/src/distributed/observatory_serving_eligibility.rs",
    "adl-runtime/tests/distributed_observatory_serving_eligibility.rs",
} - changed
if extra or missing:
    print(json.dumps({"status":"failed","extra":sorted(extra),"missing":sorted(missing)})); raise SystemExit(1)

registration = subprocess.run(
    [
        "git", "diff", "--unified=0", f"{implementation_base}...HEAD", "--",
        "adl-runtime/src/distributed/mod.rs",
    ],
    cwd=root,
    text=True,
    capture_output=True,
)
if registration.returncode:
    raise SystemExit(registration.returncode)
patch_lines = [
    line
    for line in registration.stdout.splitlines()
    if (line.startswith("+") and not line.startswith("+++"))
    or (line.startswith("-") and not line.startswith("---"))
]
expected_registration = "+pub mod observatory_serving_eligibility;"
if patch_lines != [expected_registration]:
    print(json.dumps({
        "status": "failed",
        "message": "distributed/mod.rs must contain exactly one additive Observatory declaration",
        "registration_patch": patch_lines,
    }, sort_keys=True))
    raise SystemExit(1)

print(json.dumps({
    "schema":"adl.issue274.scope.v1",
    "status":"passed",
    "implementation_base": implementation_base,
    "changed":sorted(changed),
    "registration_patch": patch_lines,
}, sort_keys=True))
