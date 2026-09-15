#!/usr/bin/env python3
"""PVF runtime: bounded installed CLI proof on inert local Git fixtures; no network."""
import argparse
import hashlib
import json
import platform
import subprocess
from pathlib import Path


def execute(argv, *, success=True):
    result = subprocess.run([str(x) for x in argv], capture_output=True, timeout=60)
    if (result.returncode == 0) != success:
        raise RuntimeError(f"Unexpected process outcome: {argv[0]} exit {result.returncode}")
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--output-root", type=Path, required=True)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    if "target" in binary.parts:
        raise ValueError("Use the isolated installed binary, not Cargo target output")
    provenance = binary.parent / ".provenance" / (binary.name + ".sha256")
    if not provenance.is_file():
        raise ValueError("Installer provenance is required")
    root = args.output_root.absolute()
    root.mkdir(parents=True, exist_ok=False)
    source = root / "source"
    source.mkdir()
    fixtures = Path(__file__).resolve().parents[1] / "tests/fixtures/codefriend/architecture/allowed"
    (source / "src").mkdir()
    for name in ["lib.rs", "api.rs", "domain.rs", "helper.rs"]:
        (source / "src" / name).write_bytes((fixtures / name).read_bytes())
    (source / "LICENSE").write_text("MIT fixture license\n")
    def git(*args):
        return execute(["git", "-C", source, *args]).stdout.decode().strip()
    git("init", "-b", "main")
    git("remote", "add", "origin", "https://example.com/owner/repo")
    git("add", ".")
    git("-c", "user.name=fixture", "-c", "user.email=fixture@example.com", "commit", "-m", "fixture")
    revision = git("rev-parse", "HEAD")
    analysis = sorted(f"src/{n}" for n in ["lib.rs", "api.rs", "domain.rs", "helper.rs"])
    scope = {"analysis": analysis, "context": ["LICENSE"], "max_files": 5, "max_bytes": 65536, "max_file_bytes": 8192}
    (root / "scope.json").write_text(json.dumps(scope))
    store = root / "store"
    def cli(*args, success=True):
        return execute([binary, "codefriend", *args], success=success)
    admitted = json.loads(cli("evidence", "admit-local", "--checkout", source,
        "--repository", "https://example.com/owner/repo", "--revision", revision,
        "--scope", root / "scope.json", "--store", store, "--retention-seconds", "3600").stdout)
    packet = admitted["packet_id"]
    policy = {"schema": "codefriend.structure.v1", "crate_root": "src/lib.rs", "manifest_path": None,
              "layers": {p: "core" for p in analysis}, "allowed": [], "coupling_threshold": 2}
    policy_file = root / "policy.json"
    policy_file.write_text(json.dumps(policy))
    def report(name):
        path = root / name
        result = cli("architecture", "report", "--store", store, "--packet-id", packet,
                     "--policy", policy_file, "--out", path)
        summary = json.loads(result.stdout)
        assert b"adl_event" in result.stderr
        readback = cli("architecture", "read", "--store", store, "--input", path)
        assert json.loads(readback.stdout) == summary
        return summary, path
    first, path = report("first.json")
    repeat, _ = report("repeat.json")
    assert first == repeat and first["nodes"] == 4 and first["edges"] == 2
    assert first["findings"] == 1 and first["unknowns"] == 0 and first["analysis_complete"]
    policy["layers"]["src/api.rs"] = "api"
    policy_file.write_text(json.dumps(policy))
    changed, _ = report("changed-policy.json")
    assert changed["findings"] == 2 and changed["run_id"] != first["run_id"]
    tampered = json.loads(path.read_text())
    tampered["edges"] = []
    tampered_path = root / "tampered.json"
    tampered_path.write_text(json.dumps(tampered))
    cli("architecture", "read", "--store", store, "--input", tampered_path, success=False)
    cli("evidence", "delete", "--store", store, "--packet-id", packet)
    cli("architecture", "read", "--store", store, "--input", path, success=False)
    assert git("status", "--porcelain") == ""
    resource_summaries = []
    for name, content in [
        ("unary", "pub fn f() { let _ = " + "!" * 12000 + "true; }"),
        ("keyword", "pub fn f() { " + "return " * 4000 + "true }"),
        ("delimiter", "pub fn f() { let _ = " + "(" * 12000 + "true" + ")" * 12000 + "; }"),
    ]:
        (source / "src/lib.rs").write_text(content)
        git("add", "src/lib.rs")
        git("-c", "user.name=fixture", "-c", "user.email=fixture@example.com", "commit", "-m", name)
        bounded_scope = {"analysis": ["src/lib.rs"], "context": ["LICENSE"],
                         "max_files": 2, "max_bytes": 65536, "max_file_bytes": 32768}
        (root / "scope.json").write_text(json.dumps(bounded_scope))
        packet = json.loads(cli("evidence", "admit-local", "--checkout", source,
            "--repository", "https://example.com/owner/repo", "--revision", git("rev-parse", "HEAD"),
            "--scope", root / "scope.json", "--store", store, "--retention-seconds", "3600").stdout)["packet_id"]
        policy["layers"] = {"src/lib.rs": "core"}
        policy_file.write_text(json.dumps(policy))
        summary, resource_path = report(name + ".json")
        assert not summary["analysis_complete"] and summary["nodes"] == 0 and summary["findings"] == 0
        assert "rust_syntax_complexity_limit" in resource_path.read_text()
        assert git("status", "--porcelain") == ""
        resource_summaries.append({"fixture": name, "summary": summary})
    proof = {"schema": "codefriend.structure_installed_proof.v1", "issue": 882,
             "platform": platform.system() + "-" + platform.machine(),
             "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
             "installation_provenance": provenance.read_text().strip(), "fixture_revision": revision,
             "first_summary": first, "changed_policy_summary": changed,
             "repeat_deterministic": True, "persisted_readback": True, "policy_identity_distinct": True,
             "tampering_denied": True, "post_delete_read_denied": True, "source_unchanged": True,
             "stdout_json_stderr_events": True, "executed_scenarios": 9,
             "pathological_source_partial": resource_summaries,
             "nonclaims": ["No provider or external repository execution", "No Linux qualification from this local run"]}
    (root / "proof.json").write_text(json.dumps(proof, indent=2) + "\n")
    print(json.dumps(proof, indent=2))


if __name__ == "__main__":
    main()
