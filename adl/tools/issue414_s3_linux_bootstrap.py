#!/usr/bin/env python3
"""Validate, fetch, or publish #414 Linux/x86 bootstrap manifests."""
from __future__ import annotations
import argparse, hashlib, json, pathlib, platform, subprocess, sys

BUCKET = "adl-shepherd-model-artifacts-b05e1f4379b5c745-us-west-2"
REGION = "us-west-2"
OLLAMA_VERSION = "0.31.1"
RUNTIME_KEY = "shepherd/runtime/ollama/0.31.1/ollama-linux-amd64.tar.zst"
MODELS = ("llama3.1:8b", "qwen3:8b", "phi4-mini")

def sha256(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1024 * 1024), b""): digest.update(chunk)
    return digest.hexdigest()

def exact_sha256(value: object, field: str) -> str:
    if not isinstance(value, str) or len(value) != 64 or any(c not in "0123456789abcdef" for c in value):
        raise ValueError(f"invalid {field}")
    return value

def load_and_validate(path: pathlib.Path, expected_reviewed_git_sha: str | None = None) -> dict:
    data = json.loads(path.read_text())
    if data.get("schema") != "adl.issue414.linux_x86_bootstrap.v1": raise ValueError("unexpected schema")
    if data.get("bucket") != BUCKET or data.get("region") != REGION: raise ValueError("unapproved bucket/region")
    commit = data.get("reviewed_git_sha", "")
    if len(commit) != 40 or any(c not in "0123456789abcdef" for c in commit): raise ValueError("invalid reviewed Git SHA")
    if data.get("immutable_installer_prefix") != f"shepherd/issue-414/{commit}/installer": raise ValueError("prefix not bound to reviewed Git SHA")
    if expected_reviewed_git_sha is not None and commit != expected_reviewed_git_sha:
        raise ValueError("bootstrap manifest is not bound to the executing reviewed Git SHA")
    exact_sha256(data.get("continuity_binary_sha256"), "continuity binary SHA256")
    exact_sha256(data.get("runner_sha256"), "runner SHA256")
    if data.get("platform") != {"os": "linux", "arch": "x86_64"}: raise ValueError("only Linux/x86_64 is admissible")
    if data.get("ollama_version") != OLLAMA_VERSION: raise ValueError("unexpected Ollama version")
    artifacts = data.get("artifacts")
    if not isinstance(artifacts, list) or len(artifacts) != 4: raise ValueError("one runtime and three model stores required")
    runtime = [a for a in artifacts if a.get("kind") == "ollama_runtime"]
    models = [a for a in artifacts if a.get("kind") == "ollama_model_store"]
    if len(runtime) != 1 or runtime[0].get("source_key") != RUNTIME_KEY: raise ValueError("runtime key is not approved Linux amd64 object")
    if sorted(a.get("model") for a in models) != sorted(MODELS): raise ValueError("model matrix incomplete or substituted")
    for artifact in artifacts:
        checksum = artifact.get("sha256", "")
        if len(checksum) != 64 or any(c not in "0123456789abcdef" for c in checksum): raise ValueError("artifact lacks exact SHA256")
        key = artifact.get("source_key", "")
        relative = pathlib.Path(artifact.get("relative_path", ""))
        if relative.is_absolute() or not relative.parts or ".." in relative.parts: raise ValueError("unsafe relative path")
        expected_bundle_key = f"{data['immutable_installer_prefix']}/artifacts/{relative.as_posix()}"
        if artifact.get("bundle_key") != expected_bundle_key: raise ValueError("artifact bundle key is not under immutable installer prefix")
        if artifact.get("kind") == "ollama_model_store":
            model = artifact["model"].replace(":", "-"); digest = artifact.get("source_digest", "")
            if len(digest) != 64 or any(c not in "0123456789abcdef" for c in digest): raise ValueError("invalid model source digest")
            if not key.startswith(f"shepherd/{model}/ollama-{OLLAMA_VERSION}/{digest}/model-store/"): raise ValueError("model key violates Linux layout")
        if any(word in key.lower() for word in ("darwin", "metal", "mlx")): raise ValueError("Mac/Metal/MLX artifact forbidden")
    if data.get("continuity_authority") != "none_bootstrap_cache_only": raise ValueError("S3 cannot be continuity authority")
    return data

def fetch(manifest: pathlib.Path, publication_receipt: pathlib.Path, destination: pathlib.Path, expected_reviewed_git_sha: str) -> None:
    data = load_and_validate(manifest, expected_reviewed_git_sha); destination.mkdir(parents=True, exist_ok=True)
    receipt = json.loads(publication_receipt.read_text())
    if receipt.get("schema") != "adl.issue414.linux_x86_bootstrap_publication.v1" or receipt.get("manifest_sha256") != sha256(manifest):
        raise ValueError("publication receipt is absent or does not bind the manifest")
    versions = {item["key"]: item["version_id"] for item in receipt.get("objects", [])}
    for artifact in data["artifacts"]:
        relative = pathlib.Path(artifact["relative_path"])
        if relative.is_absolute() or ".." in relative.parts: raise ValueError("unsafe relative path")
        target = destination / relative; target.parent.mkdir(parents=True, exist_ok=True)
        version_id = versions.get(artifact["bundle_key"])
        if not version_id: raise ValueError("artifact lacks a pinned S3 VersionId")
        subprocess.run(["aws", "s3api", "get-object", "--region", REGION, "--bucket", BUCKET,
                        "--key", artifact["bundle_key"], "--version-id", version_id, str(target)], check=True)
        if sha256(target) != artifact["sha256"]: raise ValueError(f"checksum mismatch for {relative}")

def publish(manifest: pathlib.Path, repository: pathlib.Path, artifact_root: pathlib.Path) -> None:
    data = load_and_validate(manifest)
    if platform.system() != "Linux" or platform.machine() not in ("x86_64", "amd64"): raise ValueError("publication requires Linux/x86_64")
    head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=repository, text=True).strip()
    dirty = subprocess.check_output(["git", "status", "--porcelain"], cwd=repository, text=True)
    if dirty or head != data["reviewed_git_sha"]: raise ValueError("publication requires clean exact reviewed commit")
    published = []
    for artifact in data["artifacts"]:
        source = artifact_root / artifact["relative_path"]
        if not source.is_file() or sha256(source) != artifact["sha256"]: raise ValueError("staged artifact is missing or checksum-mismatched")
        result = subprocess.run(["aws", "s3api", "put-object", "--region", REGION, "--bucket", BUCKET,
                                 "--key", artifact["bundle_key"], "--body", str(source),
                                 "--server-side-encryption", "AES256", "--if-none-match", "*"],
                                check=True, text=True, capture_output=True)
        response = json.loads(result.stdout)
        if not response.get("VersionId"): raise ValueError("versioned bucket did not return artifact VersionId")
        published.append({"key": artifact["bundle_key"], "version_id": response["VersionId"], "sha256": artifact["sha256"]})
    key = f"{data['immutable_installer_prefix']}/bootstrap-manifest.json"
    result = subprocess.run(["aws", "s3api", "put-object", "--region", REGION, "--bucket", BUCKET,
                             "--key", key, "--body", str(manifest), "--server-side-encryption", "AES256",
                             "--if-none-match", "*"], check=True, text=True, capture_output=True)
    manifest_response = json.loads(result.stdout)
    if not manifest_response.get("VersionId"): raise ValueError("versioned bucket did not return manifest VersionId")
    published.append({"key": key, "version_id": manifest_response["VersionId"], "sha256": sha256(manifest)})
    receipt = {"schema":"adl.issue414.linux_x86_bootstrap_publication.v1", "bucket":BUCKET,
               "reviewed_git_sha":data["reviewed_git_sha"], "manifest_sha256":sha256(manifest), "objects":published}
    receipt_path = artifact_root / "publication-receipt.json"
    receipt_path.write_text(json.dumps(receipt, sort_keys=True, indent=2) + "\n")
    print(json.dumps(receipt, sort_keys=True))

def main() -> int:
    parser = argparse.ArgumentParser(); sub = parser.add_subparsers(dest="command", required=True)
    v = sub.add_parser("validate"); v.add_argument("manifest", type=pathlib.Path); v.add_argument("--expected-reviewed-git-sha")
    f = sub.add_parser("fetch"); f.add_argument("manifest", type=pathlib.Path); f.add_argument("destination", type=pathlib.Path); f.add_argument("--publication-receipt", type=pathlib.Path, required=True); f.add_argument("--expected-reviewed-git-sha", required=True)
    p = sub.add_parser("publish"); p.add_argument("manifest", type=pathlib.Path); p.add_argument("--repository", type=pathlib.Path, required=True); p.add_argument("--artifact-root", type=pathlib.Path, required=True)
    args = parser.parse_args()
    try:
        if args.command == "validate":
            data = load_and_validate(args.manifest, args.expected_reviewed_git_sha); print(json.dumps({"status":"passed","reviewed_git_sha":data["reviewed_git_sha"]}, sort_keys=True))
        elif args.command == "fetch": fetch(args.manifest, args.publication_receipt, args.destination, args.expected_reviewed_git_sha)
        else: publish(args.manifest, args.repository, args.artifact_root)
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        print(f"issue414 bootstrap failure: {error}", file=sys.stderr); return 1
    return 0
if __name__ == "__main__": raise SystemExit(main())
