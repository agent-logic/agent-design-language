#!/usr/bin/env python3
"""Reproduce or verify the frozen source distribution. No acceptance is implied."""
import argparse
import gzip
import hashlib
import io
import json
from pathlib import Path, PurePosixPath
import subprocess
import tarfile
import tomllib

HERE = Path(__file__).resolve().parent
MANIFEST = json.loads((HERE / "source-manifest.json").read_text())
PREFIX = "adl-remote-validation-0.92.1/"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def expected_entries():
    return {PREFIX + row["source_path"].removeprefix("tools/remote_validation/"): row
            for row in MANIFEST["files"]}


def inspect_archive(data):
    """Validate inventory and contents before allowing any destination writes."""
    expected = expected_entries()
    contents = {}
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as archive:
        for member in archive:
            path = PurePosixPath(member.name)
            if (member.name not in expected or member.name in contents or not member.isfile()
                    or path.is_absolute() or ".." in path.parts
                    or member.size != expected[member.name]["bytes"]):
                raise ValueError("archive entry violates frozen inventory")
            payload = archive.extractfile(member).read()
            if sha(payload) != expected[member.name]["sha256"]:
                raise ValueError("archive payload differs from frozen source")
            contents[member.name] = payload
    if contents.keys() != expected.keys():
        raise ValueError("archive inventory incomplete")
    package = tomllib.loads(contents[PREFIX + "Cargo.toml"].decode())["package"]
    if package["name"] != MANIFEST["package"] or package["version"] != MANIFEST["version"]:
        raise ValueError("package identity mismatch")
    return contents


def verify(artifact, output=None):
    data = Path(artifact).read_bytes()
    if len(data) != MANIFEST["artifact"]["bytes"] or sha(data) != MANIFEST["artifact"]["sha256"]:
        raise ValueError("artifact digest or size mismatch")
    contents = inspect_archive(data)
    if output is not None:
        output = Path(output)
        output.mkdir(parents=True, exist_ok=False)
        for name, payload in contents.items():
            target = output / name
            target.parent.mkdir(parents=True, exist_ok=True)
            with target.open("xb") as stream:
                stream.write(payload)
    return MANIFEST["artifact"]


def export(repository, output):
    revision = MANIFEST["source_revision"]
    def git(*args):
        return subprocess.check_output(["git", "-C", str(repository), *args])
    observed = git("ls-tree", "-r", "--name-only", revision, "tools/remote_validation/").decode().splitlines()
    expected = [row["source_path"] for row in MANIFEST["files"] if row["source_path"] != "LICENSE"]
    if sorted(observed) != sorted(expected):
        raise ValueError("source inventory mismatch")
    contents = {}
    for name, row in expected_entries().items():
        data = git("show", revision + ":" + row["source_path"])
        blob = git("rev-parse", revision + ":" + row["source_path"]).decode().strip()
        if len(data) != row["bytes"] or sha(data) != row["sha256"] or blob != row["git_blob"]:
            raise ValueError("source provenance mismatch")
        contents[name] = data
    raw = io.BytesIO()
    with tarfile.open(fileobj=raw, mode="w", format=tarfile.USTAR_FORMAT) as archive:
        for name, data in sorted(contents.items()):
            member = tarfile.TarInfo(name)
            member.size, member.mode, member.mtime = len(data), 0o644, 0
            archive.addfile(member, io.BytesIO(data))
    data = bytearray(gzip.compress(raw.getvalue(), compresslevel=9, mtime=0))
    data[9] = 255  # Pin gzip OS header independently of Python's platform default.
    data = bytes(data)
    if sha(data) != MANIFEST["artifact"]["sha256"]:
        raise ValueError("compression output differs from pinned artifact")
    inspect_archive(data)
    output = Path(output)
    output.mkdir(parents=True, exist_ok=False)
    (output / MANIFEST["artifact"]["name"]).write_bytes(data)
    (output / "source-manifest.json").write_bytes((HERE / "source-manifest.json").read_bytes())
    return MANIFEST["artifact"]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest="command", required=True)
    build = sub.add_parser("export")
    build.add_argument("--repository", type=Path, required=True)
    build.add_argument("--output", type=Path, required=True)
    check = sub.add_parser("verify")
    check.add_argument("--artifact", type=Path, required=True)
    check.add_argument("--extract", type=Path)
    args = parser.parse_args()
    result = (export(args.repository, args.output) if args.command == "export"
              else verify(args.artifact, args.extract))
    print(json.dumps({"verified_source_artifact": result, "producer_acceptance_implied": False}))


if __name__ == "__main__":
    main()
