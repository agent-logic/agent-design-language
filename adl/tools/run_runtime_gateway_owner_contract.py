#!/usr/bin/env python3
"""PVF: explicit owner fixture routing; local mocked gateway contract.
Requires operator-selected CSM; does not authenticate that binary or run Cargo.
"""
import hashlib
import os
from pathlib import Path
import subprocess
import sys
import tempfile

REVISION = '7d0db620ad79c50ff5d8fabf0fb543183583d05c'
RESOURCES = {'adl/tools/test_run_v0917_csm_api_gateway_bridge_proof.sh': '3ec45e8dc970ba6f22e85aca40d34c9e0e27dec42ffa8c41a3305288479735dc', 'adl/tools/validate_v0917_csm_api_gateway_bridge_proof.py': 'f1a726b5130c1f1a37fe290d61ea752878a6eb9c375b25121126b37364a5a1a1'}


def main():
    selected = os.environ.get("ADL_RUNTIME_SOURCE_ROOT", "")
    binary = os.environ.get("RUNTIME_TEST_CSM", "")
    root, csm = Path(selected), Path(binary)
    if not selected or not root.is_absolute() or not root.is_dir():
        print("gateway_contract: require absolute ADL_RUNTIME_SOURCE_ROOT with Runtime164 Git objects", file=sys.stderr)
        return 2
    if not binary or not csm.is_absolute() or not csm.is_file() or not os.access(csm, os.X_OK):
        print("gateway_contract: require absolute executable RUNTIME_TEST_CSM; no Cargo fallback", file=sys.stderr)
        return 2
    # Read the two complete fixture resources from pinned Git objects, not dirty
    # worktree files. The original test embeds its own fake AWS/curl commands.
    inputs = {}
    for relative, expected in RESOURCES.items():
        result = subprocess.run(["git", "-C", str(root), "show", REVISION + ":" + relative],
                                stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if result.returncode or hashlib.sha256(result.stdout).hexdigest() != expected:
            print("gateway_contract: missing or changed Runtime164 resource: " + relative, file=sys.stderr)
            return 2
        inputs[relative] = result.stdout
    with tempfile.TemporaryDirectory(prefix="adl-runtime-gateway-contract-") as directory:
        fixture = Path(directory)
        for relative, data in inputs.items():
            path = fixture / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        # Nonempty explicit binary selects the original no-build seam. Assertions,
        # mock cloud calls and failure status remain those of the original script.
        return subprocess.run(["bash", str(fixture / "adl/tools/test_run_v0917_csm_api_gateway_bridge_proof.sh")],
                              cwd=fixture, env={**os.environ, "RUNTIME_TEST_CSM": str(csm),
                                                "PYTHONDONTWRITEBYTECODE": "1"}).returncode


if __name__ == "__main__":
    raise SystemExit(main())
