#!/usr/bin/env python3
"""Install the #268 Linux/x86 Runtime stack once onto its retained EBS volume."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import pathlib
import platform
import shutil
import subprocess
import sys

SOURCE_SCHEMA = "adl.issue268.s3_source_receipt.v1"
INSTALL_SCHEMA = "adl.issue268.runtime_volume_installation.v1"
BUCKET = "adl-shepherd-model-artifacts-b05e1f4379b5c745-us-west-2"
REGION = "us-west-2"
MODELS = ("llama3.1:8b", "qwen3:8b", "phi4-mini")
OBJECT_KEYS = (
    "shepherd/runtime/ollama/0.31.1/ollama-linux-amd64.tar.zst",
    "shepherd/phi4-mini/ollama-0.31.1/78fad5d182a7c33065e153a5f8ba210754207ba9d91973f57dffa7f487363753/model-store/store.tar.zst",
    "shepherd/qwen3-8b/ollama-0.31.1/500a1f067a9f782620b40bee6f7b0c89e17ae61f686b92c24933e4ca4b2b8b41/model-store/store.tar.zst",
    "shepherd/llama3.1-8b/ollama-0.31.1/46e0c10c039e019119339687c3c1757cc81b9da49709a3b3924863ba87ca666e/model-store/store.tar.zst",
)
MODEL_MANIFESTS = (
    "manifests/registry.ollama.ai/library/phi4-mini/latest",
    "manifests/registry.ollama.ai/library/qwen3/8b",
    "manifests/registry.ollama.ai/library/llama3.1/8b",
)


def sha256(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def exact_digest(value: object, label: str) -> str:
    if not isinstance(value, str) or len(value) != 64 or any(c not in "0123456789abcdef" for c in value):
        raise ValueError(f"invalid {label}")
    return value


def load_contract(source_receipt_path: pathlib.Path, reviewed_sha: str) -> dict:
    receipt = json.loads(source_receipt_path.read_text())
    if receipt.get("schema") != SOURCE_SCHEMA or receipt.get("reviewed_git_sha") != reviewed_sha:
        raise ValueError("unexpected S3 source receipt schema")
    if receipt.get("bucket") != BUCKET or receipt.get("region") != REGION:
        raise ValueError("S3 source receipt bucket or region mismatch")
    objects = receipt.get("objects") or []
    if [item.get("key") for item in objects] != list(OBJECT_KEYS):
        raise ValueError("exact four-object S3 source receipt required")
    for source in objects:
        exact_digest(source.get("sha256"), "artifact SHA-256")
        if not source.get("version_id"):
            raise ValueError("artifact lacks immutable S3 VersionId")
        lowered = str(source["key"]).lower()
        if any(marker in lowered for marker in ("darwin", "metal", "mlx")):
            raise ValueError("Mac/Metal/MLX artifacts are forbidden")
    return receipt


def extract(archive: pathlib.Path, destination: pathlib.Path) -> None:
    destination.mkdir(parents=True, exist_ok=True)
    command = ["tar", "--zstd", "-xf", str(archive), "-C", str(destination)]
    result = subprocess.run(command, check=False)
    if result.returncode != 0:
        subprocess.run(
            ["tar", "--use-compress-program", "zstd", "-xf", str(archive), "-C", str(destination)],
            check=True,
        )


def validate_installed(receipt_path: pathlib.Path, expected: dict) -> dict:
    installed = json.loads(receipt_path.read_text())
    for key, value in expected.items():
        if installed.get(key) != value:
            raise ValueError(f"installed Runtime receipt mismatch: {key}")
    for field in ("ollama_binary", "continuity_binary"):
        path = pathlib.Path(installed.get(field, ""))
        if not path.is_file() or sha256(path) != installed.get(f"{field}_sha256"):
            raise ValueError(f"installed Runtime binary mismatch: {field}")
    model_root = pathlib.Path(installed.get("ollama_models", ""))
    if not model_root.is_dir():
        raise ValueError("installed Ollama model store is absent")
    for relative in MODEL_MANIFESTS:
        if not (model_root / relative).is_file():
            raise ValueError(f"installed Ollama model manifest is absent: {relative}")
    return installed


def install(args: argparse.Namespace) -> dict:
    if platform.system() != "Linux" or platform.machine() not in ("x86_64", "amd64"):
        raise ValueError("Runtime-volume installation requires Linux/x86_64")
    volume_root = args.volume_root.resolve()
    build_cache = args.build_cache.resolve()
    source_root = args.source_root.resolve()
    if volume_root == build_cache or volume_root in build_cache.parents or build_cache in volume_root.parents:
        raise ValueError("build cache must remain separate from retained Runtime volume")
    if len(args.reviewed_git_sha) != 40 or any(c not in "0123456789abcdef" for c in args.reviewed_git_sha):
        raise ValueError("invalid reviewed #414 Git SHA")
    exact_digest(args.volume_identity_sha256, "Runtime volume identity SHA-256")
    head = subprocess.check_output(["git", "-C", str(source_root), "rev-parse", "HEAD"], text=True).strip()
    if head != args.source_revision:
        raise ValueError("source checkout does not match requested #268 revision")
    source_receipt = load_contract(args.source_receipt, args.reviewed_git_sha)
    install_root = volume_root / "install"
    final_root = install_root / "current"
    staging = install_root / ".staging"
    receipt_path = install_root / "installation-receipt.json"
    expected = {
        "schema": INSTALL_SCHEMA,
        "source_revision": args.source_revision,
        "reviewed_414_git_sha": args.reviewed_git_sha,
        "volume_identity_sha256": args.volume_identity_sha256,
        "source_receipt_sha256": sha256(args.source_receipt),
    }
    if receipt_path.exists():
        return validate_installed(receipt_path, expected)
    if staging.exists() or final_root.exists():
        raise ValueError("partial or unsealed Runtime-volume installation exists")
    staging.mkdir(parents=True)
    archives = staging / "source-cache"
    archives.mkdir()
    for index, source in enumerate(source_receipt["objects"]):
        archive = archives / f"{index:02d}.tar.zst"
        subprocess.run(
            [
                "aws", "s3api", "get-object", "--region", source_receipt["region"],
                "--bucket", source_receipt["bucket"], "--key", source["key"],
                "--version-id", source["version_id"], str(archive),
            ],
            check=True,
        )
        if sha256(archive) != source["sha256"]:
            raise ValueError(f"S3 checksum mismatch: {source['key']}")
        destination = staging / ("ollama" if index == 0 else "ollama-models")
        extract(archive, destination)
    ollama_candidates = [path for path in (staging / "ollama").rglob("ollama") if path.is_file()]
    if len(ollama_candidates) != 1:
        raise ValueError("exact installed Ollama binary not found")
    build_cache.mkdir(parents=True, exist_ok=True)
    environment = os.environ.copy()
    environment["CARGO_TARGET_DIR"] = str(build_cache / "target")
    subprocess.run(
        ["cargo", "build", "--locked", "--release", "--manifest-path", str(source_root / "adl/Cargo.toml"),
         "--bin", "adl_resident_shepherd_continuity"],
        cwd=source_root,
        env=environment,
        check=True,
    )
    built = build_cache / "target/release/adl_resident_shepherd_continuity"
    if not built.is_file():
        raise ValueError("canonical continuity binary build output is absent")
    binary_dir = staging / "bin"
    binary_dir.mkdir()
    continuity = binary_dir / built.name
    shutil.copy2(built, continuity)
    final_root.parent.mkdir(parents=True, exist_ok=True)
    os.replace(staging, final_root)
    ollama = final_root / ollama_candidates[0].relative_to(staging)
    continuity = final_root / continuity.relative_to(staging)
    installed = {
        **expected,
        "ollama_binary": str(ollama),
        "ollama_binary_sha256": sha256(ollama),
        "continuity_binary": str(continuity),
        "continuity_binary_sha256": sha256(continuity),
        "ollama_models": str(final_root / "ollama-models/models"),
        "s3_bootstrap_only": True,
        "build_cache_separate": True,
    }
    temporary = receipt_path.with_suffix(".tmp")
    temporary.write_text(json.dumps(installed, indent=2, sort_keys=True) + "\n")
    os.replace(temporary, receipt_path)
    return validate_installed(receipt_path, expected)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--volume-root", type=pathlib.Path, required=True)
    parser.add_argument("--build-cache", type=pathlib.Path, required=True)
    parser.add_argument("--source-root", type=pathlib.Path, required=True)
    parser.add_argument("--source-revision", required=True)
    parser.add_argument("--source-receipt", type=pathlib.Path, required=True)
    parser.add_argument("--reviewed-git-sha", required=True)
    parser.add_argument("--volume-identity-sha256", required=True)
    args = parser.parse_args()
    try:
        print(json.dumps(install(args), sort_keys=True))
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print(f"issue268 Runtime-volume installation failed: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
