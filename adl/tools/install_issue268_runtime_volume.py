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


def runtime_source_identity(source_root: pathlib.Path, revision: str) -> str:
    """Bind only Runtime/ACC build inputs so evidence-only commits reuse safely."""
    paths = ("adl/src", "adl-runtime/src", "adl-runtime-kernel/src", "adl/Cargo.toml", "adl/Cargo.lock")
    objects = {}
    for relative in paths:
        objects[relative] = subprocess.check_output(
            ["git", "-C", str(source_root), "rev-parse", f"{revision}:{relative}"], text=True
        ).strip()
    return hashlib.sha256(json.dumps(objects, separators=(",", ":"), sort_keys=True).encode()).hexdigest()


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


def validate_installed(
    receipt_path: pathlib.Path,
    expected: dict,
    attached_volume_identity_sha256: str,
    installed_override: dict | None = None,
) -> dict:
    exact_digest(attached_volume_identity_sha256, "attached Runtime volume identity SHA-256")
    installed = installed_override if installed_override is not None else json.loads(receipt_path.read_text())
    for key, value in expected.items():
        if installed.get(key) != value:
            raise ValueError(f"installed Runtime receipt mismatch: {key}")
    for field in ("ollama_binary", "continuity_binary", "runtime_binary", "csm_binary"):
        path = pathlib.Path(installed.get(field, ""))
        if not path.is_file() or sha256(path) != installed.get(f"{field}_sha256"):
            raise ValueError(f"installed Runtime binary mismatch: {field}")
    model_root = pathlib.Path(installed.get("ollama_models", ""))
    if not model_root.is_dir():
        raise ValueError("installed Ollama model store is absent")
    for relative in MODEL_MANIFESTS:
        if not (model_root / relative).is_file():
            raise ValueError(f"installed Ollama model manifest is absent: {relative}")
    installed = dict(installed)
    installed["installation_volume_identity_sha256"] = installed.get("volume_identity_sha256")
    installed["attached_volume_identity_sha256"] = attached_volume_identity_sha256
    installed["snapshot_clone_reuse"] = (
        installed.get("volume_identity_sha256") != attached_volume_identity_sha256
    )
    return installed


def rebase_snapshot_paths(installed: dict, final_root: pathlib.Path) -> tuple[dict, bool]:
    """Rebind sealed install paths when an EBS snapshot is mounted elsewhere."""
    rebased = dict(installed)
    changed = False
    expected_suffixes = {
        "ollama_binary": ("ollama", "bin", "ollama"),
        "ollama_models": ("ollama-models",),
        "continuity_binary": ("bin", "adl_resident_shepherd_continuity"),
        "runtime_binary": ("bin", "adl"),
        "csm_binary": ("bin", "csm"),
    }
    for field, expected_suffix in expected_suffixes.items():
        original = pathlib.Path(str(installed.get(field, "")))
        parts = original.parts
        anchors = [index for index in range(1, len(parts)) if parts[index - 1 : index + 1] == ("install", "current")]
        if len(anchors) != 1 or parts.count("current") != 1:
            raise ValueError(f"installed Runtime path lacks unique sealed install/current root: {field}")
        current_index = anchors[0]
        relative = pathlib.Path(*parts[current_index + 1 :])
        if relative.parts != expected_suffix:
            raise ValueError(f"installed Runtime path has unexpected sealed suffix: {field}")
        rebound = final_root / relative
        if rebound != original:
            rebased[field] = str(rebound)
            changed = True
    return rebased, changed


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
    required_runtime_source_identity = runtime_source_identity(source_root, args.source_revision)
    install_root = volume_root / "install"
    final_root = install_root / "current"
    staging = install_root / ".staging"
    receipt_path = install_root / "installation-receipt.json"
    expected = {
        "schema": INSTALL_SCHEMA,
        "reviewed_414_git_sha": args.reviewed_git_sha,
        "source_receipt_sha256": sha256(args.source_receipt),
        "runtime_source_identity_sha256": required_runtime_source_identity,
        "continuity_runtime_source_identity_sha256": required_runtime_source_identity,
        "csm_runtime_source_identity_sha256": required_runtime_source_identity,
    }
    if receipt_path.exists():
        installed = json.loads(receipt_path.read_text())
        installed, paths_rebased = rebase_snapshot_paths(installed, final_root)
        runtime = pathlib.Path(installed.get("runtime_binary", ""))
        if (not runtime.is_file()
                or installed.get("runtime_source_identity_sha256") != required_runtime_source_identity
                or installed.get("continuity_runtime_source_identity_sha256") != required_runtime_source_identity
                or installed.get("csm_runtime_source_identity_sha256") != required_runtime_source_identity
                or (runtime.is_file() and sha256(runtime) != installed.get("runtime_binary_sha256"))):
            build_cache.mkdir(parents=True, exist_ok=True)
            environment = os.environ.copy()
            environment["CARGO_TARGET_DIR"] = str(build_cache / "target")
            subprocess.run(
                ["cargo", "build", "--locked", "--release", "--manifest-path", str(source_root / "adl/Cargo.toml"),
                 "--bin", "adl_resident_shepherd_continuity", "--bin", "adl", "--bin", "csm"],
                cwd=source_root,
                env=environment,
                check=True,
            )
            built_continuity = build_cache / "target/release/adl_resident_shepherd_continuity"
            built_runtime = build_cache / "target/release/adl"
            built_csm = build_cache / "target/release/csm"
            if not built_continuity.is_file() or not built_runtime.is_file() or not built_csm.is_file():
                raise ValueError("canonical Runtime/continuity/CSM binary build output is absent")
            continuity = pathlib.Path(installed["continuity_binary"])
            temporary_continuity = continuity.with_suffix(".tmp")
            shutil.copy2(built_continuity, temporary_continuity)
            temporary_continuity.chmod(0o755)
            os.replace(temporary_continuity, continuity)
            runtime = pathlib.Path(installed["continuity_binary"]).parent / "adl"
            temporary_runtime = runtime.with_suffix(".tmp")
            shutil.copy2(built_runtime, temporary_runtime)
            temporary_runtime.chmod(0o755)
            os.replace(temporary_runtime, runtime)
            csm = continuity.parent / "csm"
            temporary_csm = csm.with_suffix(".tmp")
            shutil.copy2(built_csm, temporary_csm)
            temporary_csm.chmod(0o755)
            os.replace(temporary_csm, csm)
            installed["continuity_binary"] = str(continuity)
            installed["continuity_binary_sha256"] = sha256(continuity)
            installed["runtime_binary"] = str(runtime)
            installed["runtime_binary_sha256"] = sha256(runtime)
            installed["csm_binary"] = str(csm)
            installed["csm_binary_sha256"] = sha256(csm)
            installed["runtime_source_identity_sha256"] = required_runtime_source_identity
            installed["continuity_runtime_source_identity_sha256"] = required_runtime_source_identity
            installed["csm_runtime_source_identity_sha256"] = required_runtime_source_identity
            installed["qualification_source_revision"] = args.source_revision
            temporary_receipt = receipt_path.with_suffix(".tmp")
            temporary_receipt.write_text(json.dumps(installed, indent=2, sort_keys=True) + "\n")
            os.replace(temporary_receipt, receipt_path)
        validated = validate_installed(
            receipt_path, expected, args.volume_identity_sha256, installed_override=installed
        )
        if paths_rebased:
            temporary_receipt = receipt_path.with_suffix(".tmp")
            temporary_receipt.write_text(json.dumps(installed, indent=2, sort_keys=True) + "\n")
            os.replace(temporary_receipt, receipt_path)
        return validated
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
            stdout=subprocess.DEVNULL,
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
         "--bin", "adl_resident_shepherd_continuity", "--bin", "adl", "--bin", "csm"],
        cwd=source_root,
        env=environment,
        check=True,
    )
    built = build_cache / "target/release/adl_resident_shepherd_continuity"
    built_runtime = build_cache / "target/release/adl"
    built_csm = build_cache / "target/release/csm"
    if not built.is_file() or not built_runtime.is_file() or not built_csm.is_file():
        raise ValueError("canonical Runtime build output is absent")
    binary_dir = staging / "bin"
    binary_dir.mkdir()
    continuity = binary_dir / built.name
    runtime = binary_dir / built_runtime.name
    csm = binary_dir / built_csm.name
    shutil.copy2(built, continuity)
    shutil.copy2(built_runtime, runtime)
    shutil.copy2(built_csm, csm)
    final_root.parent.mkdir(parents=True, exist_ok=True)
    os.replace(staging, final_root)
    ollama = final_root / ollama_candidates[0].relative_to(staging)
    continuity = final_root / continuity.relative_to(staging)
    runtime = final_root / runtime.relative_to(staging)
    csm = final_root / csm.relative_to(staging)
    installed = {
        **expected,
        "qualification_source_revision": args.source_revision,
        "ollama_binary": str(ollama),
        "ollama_binary_sha256": sha256(ollama),
        "continuity_binary": str(continuity),
        "continuity_binary_sha256": sha256(continuity),
        "runtime_binary": str(runtime),
        "runtime_binary_sha256": sha256(runtime),
        "csm_binary": str(csm),
        "csm_binary_sha256": sha256(csm),
        "ollama_models": str(final_root / "ollama-models/models"),
        "s3_bootstrap_only": True,
        "build_cache_separate": True,
    }
    temporary = receipt_path.with_suffix(".tmp")
    temporary.write_text(json.dumps(installed, indent=2, sort_keys=True) + "\n")
    os.replace(temporary, receipt_path)
    return validate_installed(receipt_path, expected, args.volume_identity_sha256)


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
