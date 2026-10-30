#!/usr/bin/env python3
"""Route retained ADL fixture consumers to the exact Infrastructure validator."""
import hashlib
import os
from pathlib import Path
import sys

OWNER_REVISION = "025850d265cbd1a55b28a8e9b238b2fe0bfcaf1d"
OWNER_PATH = "adl/tools/validate_csm_visibility_packet.py"
EXPECTED_SHA256 = "dc96475f3d505d840e0126f6c2d9d83025f2fd862525f490d3d6a041ba6c40ad"
EXPECTED_MODE = 0o755


def main(argv=None):
    argv = sys.argv[1:] if argv is None else argv
    root = Path(os.environ.get("ADL_CSM_VALIDATOR_SOURCE_ROOT", ""))
    if not root.is_absolute():
        print("Select an absolute ADL_CSM_VALIDATOR_SOURCE_ROOT for the pinned Infrastructure validator.", file=sys.stderr)
        return 2
    source = root / OWNER_PATH
    try:
        valid = (source.is_file() and source.stat().st_mode & 0o777 == EXPECTED_MODE
                 and hashlib.sha256(source.read_bytes()).hexdigest() == EXPECTED_SHA256)
    except OSError:
        valid = False
    if not valid:
        print("Infrastructure visibility validator is missing or differs from the qualified source pin.", file=sys.stderr)
        return 2
    if argv == ["--check-inputs"]:
        return 0
    os.execv(sys.executable, [sys.executable, str(source), *argv])


if __name__ == "__main__":
    raise SystemExit(main())
