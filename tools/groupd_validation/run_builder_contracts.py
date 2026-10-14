#!/usr/bin/env python3
"""PVF: deterministic local owner-contract routing, small CPU/disk, no network.
Preserves original builder assertions; no image build/publication acceptance.
"""
import argparse
import hashlib
import os
from pathlib import Path
import subprocess
import sys

# Reviewed Infrastructure source 61c2806b6274b1003aa42f02adec99faf5f2a74b.
RESOURCES = {'adl/docker/adl-builder/Dockerfile': '04e86121bbdf2998a71002beaf402bec3a06f4513370f1e543b04ecb23dc9eb7', 'adl/docker/adl-builder/INPUT_PROVENANCE.json': '9dae5f0624135bd08788007695ca93d08bba6a63132ce7d168b9f92d3f009002', 'adl/docker/adl-builder/verify_sha256.sh': '7b6108ebc145fc84b25dbb7b659ba7adbbf68cbd2dfab2013bf24faeeae452e8', 'adl/docker/adl-builder/test_builder_inputs.py': '3bdc3668cac222a6e266fe85e6881648a8deb0e69fd22cdccf9801b3411b0a77', 'adl/tools/setup_adl_builder_image.sh': 'bb837912164cebb903953f28b3ab51412ea73cd33e02399bdcc936e9c4492205', 'adl/tools/test_adl_builder_image.sh': 'd56338761b09cb17caff0ed94256314dd4d64ec6bc7dcc6670cb26631fb80258', 'adl/tools/publish_adl_builder_image_codebuild.sh': '3d134752f87ec8b25c976432d4f15d46d8715b26baff6f6d5485d472b91abfda', 'adl/tools/test_publish_adl_builder_image_codebuild.sh': 'a41716a543719082ce2c505822f856132961a914cb5c2d16c6c3309ab6b180ca', 'adl/tools/import_adl_builder_image_from_s3_to_ecr.sh': 'da04ed7d0764993e9b2cd3f91874baf97ba0e5d2666cc6ff9760393cee157d01', 'adl/tools/test_import_adl_builder_image_from_s3_to_ecr.sh': '208a0314bda6f5028799d0907ab9679c29a694796f63e4a22c17cf215317ae4e'}


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--suite", choices=("inputs", "setup-import"), default="inputs")
    args = parser.parse_args(argv)
    selected = os.environ.get("ADL_INFRASTRUCTURE_SOURCE_ROOT", "")
    root = Path(selected)
    if not selected or not root.is_absolute() or not root.is_dir():
        print("builder_contracts: set ADL_INFRASTRUCTURE_SOURCE_ROOT to the reviewed absolute Infrastructure source directory", file=sys.stderr)
        return 2
    for relative, expected in RESOURCES.items():
        try:
            data = (root / relative).read_bytes()
        except OSError:
            print(f"builder_contracts: missing owner resource: {relative}", file=sys.stderr)
            return 2
        if hashlib.sha256(data).hexdigest() != expected:
            print(f"builder_contracts: owner resource identity mismatch: {relative}", file=sys.stderr)
            return 2
    commands = ([[sys.executable, "-B", "adl/docker/adl-builder/test_builder_inputs.py"]]
                if args.suite == "inputs" else
                [["bash", "adl/tools/test_adl_builder_image.sh"],
                 ["bash", "adl/tools/test_import_adl_builder_image_from_s3_to_ecr.sh"]])
    for command in commands:
        result = subprocess.run(command, cwd=root,
                                env={**os.environ, "PYTHONDONTWRITEBYTECODE": "1"})
        if result.returncode:
            return result.returncode
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
