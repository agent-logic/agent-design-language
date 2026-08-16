#!/usr/bin/env python3
"""Run the exact eight-case #275 integration denominator, failing closed."""

import pathlib
import re
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[4]
TARGET = "distributed_integrated_serving_authority"
EXPECTED = (
    "authentic_pair_snapshot_retry_restart_and_redaction",
    "immutable_multi_operation_prefix_and_four_outcomes",
    "capacity_and_invalid_operation_fail_closed",
    "checkpoint_cas_failure_preserves_last_commit",
    "corrupt_truncated_and_unknown_state_fail_closed",
    "terminal_child_combinations_remain_evidence_only",
    "authentic_ab_substitution_is_denied_before_commit",
    "independent_prefix_receipt_and_checkpoint_tamper_is_denied",
)
BASE = [
    "cargo", "test", "--locked", "--manifest-path", "adl-runtime/Cargo.toml",
    "--test", TARGET, "--features", "internal-test-fixtures",
]

listed = subprocess.run(BASE + ["--", "--list"], cwd=ROOT, text=True, capture_output=True)
if listed.returncode:
    sys.stderr.write(listed.stdout + listed.stderr)
    raise SystemExit(listed.returncode)
actual = tuple(sorted(line.split(": test", 1)[0] for line in listed.stdout.splitlines() if line.endswith(": test")))
if actual != tuple(sorted(EXPECTED)):
    raise SystemExit(f"FAIL: exact focused denominator drifted: actual={actual!r} expected={EXPECTED!r}")
for name in EXPECTED:
    result = subprocess.run(
        BASE + [name, "--", "--exact", "--test-threads=1"],
        cwd=ROOT,
        text=True,
        capture_output=True,
    )
    sys.stdout.write(result.stdout)
    sys.stderr.write(result.stderr)
    if result.returncode:
        raise SystemExit(result.returncode)
    output = result.stdout + result.stderr
    if not re.search(r"running 1 test\b", output) or not re.search(
        r"test result: ok\. 1 passed; 0 failed; 0 ignored;", output
    ):
        raise SystemExit(f"FAIL: non-proving exact denominator for {name}")
print("PASS: exact #275 focused matrix 8 passed, 0 failed, 0 ignored")
