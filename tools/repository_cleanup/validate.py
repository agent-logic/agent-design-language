#!/usr/bin/env python3
"""PVF local_contract: deterministic cleanup routing/identity checks; bounded
local CPU/filesystem, no build/network/provider calls; native proof adapter.
"""
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
CHECKS = (
    [sys.executable, '-B', 'tools/public_adl/test_package_checks.py'],
    [sys.executable, '-B', 'tools/repository_cleanup/test_guard.py'],
    [sys.executable, '-B', 'tools/public_adl/test_ci_contract.py'],
    ['bash', 'adl/tools/test_batched_checks_no_codexpr_usage_banner.sh'],
)


def main():
    for command in CHECKS:
        subprocess.run(command, cwd=ROOT, check=True, timeout=60)
    print('POST_SPLIT_CLEANUP_CONTRACTS_PASSED')


if __name__ == '__main__':
    main()
