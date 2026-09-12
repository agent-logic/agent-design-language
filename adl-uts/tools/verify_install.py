#!/usr/bin/env python3
"""Required #877 PVF: clean Git artifact + isolated consumer, CPU/disk only.

Writes only under ignored .adl/877-install/<head>/; no registry publication.
Uses the committed package, never untracked/dirty source. Cargo dependencies
must be cached: offline mode makes accidental network/provider use impossible.
"""
import hashlib
import json
from pathlib import Path
import subprocess
import tarfile
import io
import os

ROOT = Path(__file__).resolve().parents[2]

def run(args, cwd):
    return subprocess.run(args, cwd=cwd, check=True, text=True, capture_output=True).stdout


def main():
    head = run(['git', 'rev-parse', 'HEAD'], ROOT).strip()
    # Only package-owned and mirrored source cleanliness matters for this bounded proof.
    dirty = run(['git', 'status', '--porcelain', '--', 'adl-uts', 'adl-spec/schemas/uts'], ROOT)
    if dirty.strip():
        raise SystemExit('Commit the intended package/schema revision before clean-artifact proof.')
    dest = ROOT / '.adl' / '877-install' / head
    dest.mkdir(parents=True, exist_ok=True)
    snapshot = dest / 'snapshot'
    snapshot.mkdir(exist_ok=True)
    archive = subprocess.check_output(['git', 'archive', head, '.gitignore', 'adl-uts', 'adl-spec/schemas/uts'], cwd=ROOT)
    with tarfile.open(fileobj=io.BytesIO(archive)) as tar:
        tar.extractall(snapshot, filter='data')
    for path in (snapshot / 'adl-uts/schemas').rglob('*.schema.json'):
        assert path.read_bytes() == (snapshot / 'adl-spec/schemas/uts' / path.relative_to(snapshot / 'adl-uts/schemas')).read_bytes(), path
    # Do not let the standalone Cargo package discover the enclosing worktree Git root.
    run(['git', 'init', '--quiet'], snapshot)
    run(['git', 'add', '.gitignore', 'adl-uts', 'adl-spec'], snapshot)
    source_epoch = run(['git', 'show', '-s', '--format=%ct', head], ROOT).strip()
    identity_env = {**os.environ, 'GIT_AUTHOR_DATE': f'{source_epoch} +0000', 'GIT_COMMITTER_DATE': f'{source_epoch} +0000'}
    # Repeat invocation at the same revision reuses the immutable snapshot commit.
    if run(['git', 'status', '--porcelain'], snapshot).strip():
        subprocess.run(['git', '-c', 'user.name=UTS package proof', '-c', 'user.email=uts-proof@example.invalid', 'commit', '--quiet', '-m', 'Exact source snapshot'], cwd=snapshot, env=identity_env, check=True, capture_output=True)
    run(['cargo', 'package', '--offline', '--locked', '--manifest-path', 'adl-uts/Cargo.toml'], snapshot)
    artifact = snapshot / 'adl-uts/target/package/adl-uts-0.1.0.crate'
    vendor = dest / 'vendor'
    vendor.mkdir(exist_ok=True)
    with tarfile.open(artifact) as tar:
        tar.extractall(vendor, filter='data')
    consumer = dest / 'consumer'
    (consumer / 'src').mkdir(parents=True, exist_ok=True)
    (consumer / 'Cargo.toml').write_text('''[package]
name = "isolated-uts-consumer"
version = "0.0.0"
edition = "2021"
[dependencies]
adl-uts = { version = "=0.1.0", path = "../vendor/adl-uts-0.1.0" }
serde_json = "1"
''')
    (consumer / 'src/main.rs').write_text('''fn main() {
    assert_eq!(adl_uts::PACKAGE_VERSION, "0.1.0");
    let wire: serde_json::Value = serde_json::from_str(include_str!("../../vendor/adl-uts-0.1.0/schemas/examples/readonly.v1.json")).unwrap();
    let declaration = adl_uts::load_tool_declaration(wire).unwrap();
    assert_eq!(declaration.name, "fixture.safe_read");
    assert_eq!(declaration.side_effect_class, adl_uts::UtsSideEffectClassV1::Read);
    for schema in [adl_uts::V1_SCHEMA_JSON, adl_uts::V1_1_SCHEMA_JSON, adl_uts::V1_1_INVOCATION_SCHEMA_JSON] {
        let value: serde_json::Value = serde_json::from_str(schema).unwrap();
        assert!(value["$id"].is_string());
    }
    assert!(adl_uts::load_tool_declaration(serde_json::json!({"schema_version":"uts.v2"})).is_err());
    println!("adl-uts {} isolated artifact consumed", adl_uts::PACKAGE_VERSION);
}
''')
    output = run(['cargo', 'run', '--offline', '--quiet', '--manifest-path', 'Cargo.toml'], consumer).strip()
    metadata = json.loads(run(['cargo', 'metadata', '--offline', '--format-version', '1', '--manifest-path', 'Cargo.toml'], consumer))
    package = next(p for p in metadata['packages'] if p['name'] == 'adl-uts')
    assert Path(package['manifest_path']).resolve() == (vendor / 'adl-uts-0.1.0/Cargo.toml').resolve()
    report = {'schema':'adl.uts.install_proof.v1', 'source_revision':head, 'package':'adl-uts', 'version':package['version'], 'artifact_sha256':hashlib.sha256(artifact.read_bytes()).hexdigest(), 'consumer_uses_extracted_artifact':True, 'schema_mirrors_equal':True, 'consumer_output':output, 'execution_authority_claimed':False}
    (dest / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))

if __name__ == '__main__':
    main()
