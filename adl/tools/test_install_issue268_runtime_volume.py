#!/usr/bin/env python3
import importlib.util
import json
import pathlib
import tempfile

MODULE_PATH = pathlib.Path(__file__).with_name("install_issue268_runtime_volume.py")
SPEC = importlib.util.spec_from_file_location("issue268_install", MODULE_PATH)
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC.loader
SPEC.loader.exec_module(MODULE)

assert "stdout=subprocess.DEVNULL" in MODULE_PATH.read_text()


def fixture(root: pathlib.Path):
    reviewed = "a" * 40
    source = {
        "schema": MODULE.SOURCE_SCHEMA,
        "reviewed_git_sha": reviewed,
        "bucket": MODULE.BUCKET,
        "region": MODULE.REGION,
        "objects": [
            {"key": key, "version_id": f"v{index}", "sha256": str(index + 1) * 64}
            for index, key in enumerate(MODULE.OBJECT_KEYS)
        ],
    }
    receipt_path = root / "source.json"
    receipt_path.write_text(json.dumps(source))
    return reviewed, receipt_path


with tempfile.TemporaryDirectory() as value:
    root = pathlib.Path(value)
    reviewed, source_path = fixture(root)
    source = MODULE.load_contract(source_path, reviewed)
    assert len(source["objects"]) == 4

    tampered = json.loads(source_path.read_text())
    tampered["objects"][0]["version_id"] = ""
    source_path.write_text(json.dumps(tampered))
    try:
        MODULE.load_contract(source_path, reviewed)
        raise AssertionError("missing VersionId unexpectedly accepted")
    except ValueError as error:
        assert "immutable S3 VersionId" in str(error)

    ollama = root / "ollama"
    continuity = root / "continuity"
    models = root / "models"
    ollama.write_bytes(b"ollama")
    continuity.write_bytes(b"continuity")
    models.mkdir()
    for relative in MODULE.MODEL_MANIFESTS:
        manifest = models / relative
        manifest.parent.mkdir(parents=True, exist_ok=True)
        manifest.write_text("{}")
    expected = {
        "schema": MODULE.INSTALL_SCHEMA,
        "reviewed_414_git_sha": reviewed,
        "volume_identity_sha256": "c" * 64,
        "source_receipt_sha256": "e" * 64,
    }
    installed = {
        **expected,
        "qualification_source_revision": "b" * 40,
        "ollama_binary": str(ollama),
        "ollama_binary_sha256": MODULE.sha256(ollama),
        "continuity_binary": str(continuity),
        "continuity_binary_sha256": MODULE.sha256(continuity),
        "ollama_models": str(models),
    }
    installed_path = root / "installed.json"
    installed_path.write_text(json.dumps(installed))
    validated = MODULE.validate_installed(installed_path, expected, "c" * 64)
    assert validated["schema"] == MODULE.INSTALL_SCHEMA
    assert validated["snapshot_clone_reuse"] is False
    cloned = MODULE.validate_installed(installed_path, expected, "d" * 64)
    assert cloned["snapshot_clone_reuse"] is True
    assert cloned["installation_volume_identity_sha256"] == "c" * 64
    assert cloned["attached_volume_identity_sha256"] == "d" * 64
    assert "source_revision" not in expected
    continuity.write_bytes(b"tampered")
    try:
        MODULE.validate_installed(installed_path, expected, "c" * 64)
        raise AssertionError("tampered installed binary unexpectedly accepted")
    except ValueError as error:
        assert "continuity_binary" in str(error)

print("PASS: issue268 persistent Runtime-volume installation contract")
