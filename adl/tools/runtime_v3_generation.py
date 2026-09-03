#!/usr/bin/env python3
"""Stage, verify, activate, and roll back matched Runtime v3 generations."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import stat
import sys
import tempfile
from pathlib import Path

SCHEMA = "adl.runtime_v3.install_generation.v1"
ARTIFACTS = {
    "csm": "csm",
    "guardian": "adl-runtime-guardian",
    "kernel": "adl-runtime-kernel",
}


def sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def atomic_link(root: Path, name: str, target: str) -> None:
    temporary = root / f".{name}.{os.getpid()}"
    temporary.symlink_to(target)
    os.replace(temporary, root / name)
    sync_directory(root)


def sync_directory(path: Path) -> None:
    directory = os.open(path, os.O_RDONLY)
    try:
        os.fsync(directory)
    finally:
        os.close(directory)


def load_receipt(generation: Path) -> dict:
    receipt_path = generation / "receipt.json"
    try:
        receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise ValueError(f"invalid generation receipt {receipt_path}: {error}") from error
    if receipt.get("schema") != SCHEMA:
        raise ValueError("generation receipt schema is unsupported")
    if receipt.get("generation") != generation.name:
        raise ValueError("generation receipt identity does not match its directory")
    for required in ("source_revision", "platform", "build_profile", "runtime_init_schema"):
        if not isinstance(receipt.get(required), str) or not receipt[required].strip():
            raise ValueError(f"generation receipt field is missing: {required}")
    predecessor = receipt.get("predecessor_generation")
    if predecessor is not None and (
        not isinstance(predecessor, str)
        or predecessor in ("", ".", "..")
        or Path(predecessor).name != predecessor
    ):
        raise ValueError("generation predecessor identity is invalid")
    artifacts = receipt.get("artifacts")
    if not isinstance(artifacts, dict) or set(artifacts) != set(ARTIFACTS):
        raise ValueError("generation receipt must describe exactly csm, guardian, and kernel")
    for key, filename in ARTIFACTS.items():
        record = artifacts[key]
        path = generation / "bin" / filename
        if path.is_symlink() or not path.is_file() or not os.access(path, os.X_OK):
            raise ValueError(f"generation artifact is missing or not executable: {filename}")
        if record != {"file": f"bin/{filename}", "sha256": sha256(path)}:
            raise ValueError(f"generation artifact receipt mismatch: {filename}")
    return receipt


def resolve_link(root: Path, name: str) -> Path:
    link = root / name
    if not link.is_symlink():
        raise ValueError(f"{name} is not a generation symlink")
    target = os.readlink(link)
    if Path(target).is_absolute() or Path(target).parts[:1] != ("generations",):
        raise ValueError(f"{name} has an invalid generation target")
    raw_generation = root / target
    if raw_generation.is_symlink():
        raise ValueError(f"{name} targets a generation-directory symlink")
    generation = raw_generation.resolve(strict=True)
    generations = (root / "generations").resolve(strict=True)
    if generation.parent != generations:
        raise ValueError(f"{name} escapes the generations directory")
    return generation


def resolve_generation(root: Path, generation_name: str) -> Path:
    raw_generation = root / "generations" / generation_name
    if raw_generation.is_symlink():
        raise ValueError("receipt predecessor targets a generation-directory symlink")
    generation = raw_generation.resolve(strict=True)
    generations = (root / "generations").resolve(strict=True)
    if generation.parent != generations or generation.name != generation_name:
        raise ValueError("receipt predecessor escapes the generations directory")
    return generation


def verify_current(root: Path) -> dict:
    generation = resolve_link(root, "current")
    return load_receipt(generation)


def stage(args: argparse.Namespace) -> dict:
    root = args.root.resolve()
    if args.generation in ("", ".", "..") or Path(args.generation).name != args.generation:
        raise ValueError("generation must be one non-empty path component")
    generations = root / "generations"
    generations.mkdir(parents=True, exist_ok=True)
    final = generations / args.generation
    if final.exists():
        raise ValueError(f"generation already exists: {args.generation}")
    staging = Path(tempfile.mkdtemp(prefix=f".{args.generation}.", dir=generations))
    try:
        predecessor = None
        if (root / "current").is_symlink():
            previous = resolve_link(root, "current")
            load_receipt(previous)
            predecessor = previous.name
        bindir = staging / "bin"
        bindir.mkdir()
        sources = {"csm": args.csm, "guardian": args.guardian, "kernel": args.kernel}
        artifacts = {}
        for key, filename in ARTIFACTS.items():
            supplied = sources[key]
            if supplied.is_symlink():
                raise ValueError(f"source artifact must not be a symlink: {supplied}")
            source = supplied.resolve(strict=True)
            if not source.is_file() or not os.access(source, os.X_OK):
                raise ValueError(f"source artifact is missing or not executable: {source}")
            destination = bindir / filename
            shutil.copy2(source, destination)
            destination.chmod(destination.stat().st_mode | stat.S_IXUSR | stat.S_IXGRP | stat.S_IXOTH)
            with destination.open("rb") as installed:
                os.fsync(installed.fileno())
            artifacts[key] = {"file": f"bin/{filename}", "sha256": sha256(destination)}
        receipt = {
            "schema": SCHEMA,
            "generation": args.generation,
            "source_revision": args.source_revision,
            "platform": args.platform,
            "build_profile": args.build_profile,
            "runtime_init_schema": args.runtime_init_schema,
            "predecessor_generation": predecessor,
            "artifacts": artifacts,
        }
        receipt_path = staging / "receipt.json"
        receipt_path.write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
        with receipt_path.open("rb") as receipt_file:
            os.fsync(receipt_file.fileno())
        sync_directory(bindir)
        sync_directory(staging)
        staging.rename(final)
        sync_directory(generations)
        load_receipt(final)
        atomic_link(root, "current", f"generations/{args.generation}")
        verify_current(root)
        return receipt
    except Exception:
        if staging.exists():
            shutil.rmtree(staging)
        raise


def rollback(root: Path) -> dict:
    root = root.resolve()
    current = resolve_link(root, "current")
    current_receipt = load_receipt(current)
    predecessor = current_receipt.get("predecessor_generation")
    if predecessor is None:
        raise ValueError("current generation has no verified predecessor")
    previous = resolve_generation(root, predecessor)
    receipt = load_receipt(previous)
    atomic_link(root, "current", f"generations/{previous.name}")
    verify_current(root)
    return receipt


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser()
    subcommands = result.add_subparsers(dest="command", required=True)
    install = subcommands.add_parser("install")
    install.add_argument("--root", type=Path, required=True)
    install.add_argument("--generation", required=True)
    install.add_argument("--csm", type=Path, required=True)
    install.add_argument("--guardian", type=Path, required=True)
    install.add_argument("--kernel", type=Path, required=True)
    install.add_argument("--source-revision", required=True)
    machine = {"arm64": "aarch64"}.get(os.uname().machine, os.uname().machine)
    system = {"darwin": "macos"}.get(sys.platform, sys.platform)
    install.add_argument("--platform", default=f"{system}-{machine}")
    install.add_argument("--build-profile", required=True)
    install.add_argument("--runtime-init-schema", default="adl.runtime_v3.init.v1")
    verify = subcommands.add_parser("verify")
    verify.add_argument("--root", type=Path, required=True)
    revert = subcommands.add_parser("rollback")
    revert.add_argument("--root", type=Path, required=True)
    return result


def main() -> int:
    args = parser().parse_args()
    try:
        if args.command == "install":
            receipt = stage(args)
        elif args.command == "verify":
            receipt = verify_current(args.root.resolve())
        else:
            receipt = rollback(args.root)
        print(json.dumps(receipt, sort_keys=True))
        return 0
    except (OSError, ValueError) as error:
        print(f"runtime-v3-generation: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
