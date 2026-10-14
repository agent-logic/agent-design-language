#!/usr/bin/env python3
"""PVF: deterministic CLI contract routing; small offline proof, no release/live-provider authority."""
import hashlib
import os
from pathlib import Path
import sys

RESOURCES = {'adl/tools/demo_v092_provider_neutral_birthday.sh': {'sha256': '128fb6e018b337704cde7c30b01da8658be7f6920d05eccee8a7ec43407f7df8', 'mode': 493}, 'adl/tools/serve_v092_provider_neutral_observatory_api.py': {'sha256': 'd04678d701fadf62c9277fb50987bca1134f7bc6e4c5fa532aced36ef9e4ca6a', 'mode': 493}, 'adl/tools/test_v092_provider_neutral_proof.sh': {'sha256': 'c88773fd2c035b7328a783e1c84d2421344cbcec64ad33991ffe6eb37ae52784', 'mode': 493}, 'adl/tools/validate_v092_provider_neutral_proof.py': {'sha256': 'eb367dbe53995bb3ecf2dc05266e19a2dba4dc1aa68ae660c72700df0c56ebdc', 'mode': 493}}

def main():
    if sys.argv[1:]:
        raise SystemExit("This contract runner accepts no arguments")
    selected = os.environ.get("ADL_PROVIDER_NEUTRAL_SOURCE_ROOT", "")
    root = Path(selected)
    if not selected or not root.is_absolute() or not root.is_dir():
        raise SystemExit("Select an absolute Runtime source root via ADL_PROVIDER_NEUTRAL_SOURCE_ROOT")
    for name, expected in RESOURCES.items():
        path = root / name
        if not path.is_file() or path.is_symlink() or path.stat().st_mode & 0o777 != expected["mode"] or hashlib.sha256(path.read_bytes()).hexdigest() != expected["sha256"]:
            raise SystemExit(f"Runtime provider-neutral resource identity mismatch: {name}")
    # The original test selects local-proof explicitly and preserves its own output controls.
    # Source selection is not authorization for paid, live, or installed acceptance.
    os.chdir(root)
    os.execvp("bash", ["bash", "adl/tools/test_v092_provider_neutral_proof.sh"])

if __name__ == "__main__":
    main()
