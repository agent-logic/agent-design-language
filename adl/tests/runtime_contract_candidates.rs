//! PVF: deterministic local contract proof; CPU/filesystem, no network/provider.
//! Required #1207 gate. Supplements the separately recorded isolated consumers.
use std::path::PathBuf;
use std::process::Command;

fn check_python(body: &str) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned();
    let scratch = tempfile::tempdir().unwrap();
    let prefix = r#"
import hashlib, json, pathlib, subprocess, sys, tarfile
root = pathlib.Path(sys.argv[1])
scratch = pathlib.Path(sys.argv[2])
builder = root / 'tools/runtime_contracts/build_candidates.py'
revision = subprocess.check_output(['git', '-C', str(root), 'rev-parse', 'HEAD'], text=True).strip()
def build(output, rev=revision):
    return subprocess.run([sys.executable, str(builder), '--repo', str(root), '--revision', rev, '--output', str(output)], capture_output=True, text=True)
def source(path):
    return subprocess.check_output(['git', '-C', str(root), 'show', revision + ':' + path])
"#;
    let output = Command::new("python3")
        .arg("-c")
        .arg(format!("{prefix}\n{body}"))
        .arg(&root)
        .arg(scratch.path())
        .output()
        .expect("Python 3.11+ required");
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn deterministic_artifacts_preserve_production_and_fixture_bytes() {
    check_python(
        r#"
a, b = scratch / 'a', scratch / 'b'
for target in (a, b):
    result = build(target)
    assert result.returncode == 0, result.stderr
assert (a / 'manifest.json').read_bytes() == (b / 'manifest.json').read_bytes()
manifest = json.loads((a / 'manifest.json').read_text())
assert manifest['accepted'] is False
assert manifest['source_revision'] == revision
assert {p['name'] for p in manifest['packages']} == {'adl-engine', 'adl-records'}
for package in manifest['packages']:
    name = package['name']
    raw = (a / package['archive']).read_bytes()
    assert raw == (b / package['archive']).read_bytes()
    assert hashlib.sha256(raw).hexdigest() == package['sha256']
    with tarfile.open(a / package['archive']) as archive:
        members = archive.getmembers()
        assert len(members) == len(set(m.name for m in members)) == package['files']
        assert set(m.name for m in members) == set(package['payload'])
        for member in members:
            assert member.isfile() and member.mtime == 0 and member.uid == member.gid == 0
            assert not pathlib.PurePosixPath(member.name).is_absolute() and '..' not in pathlib.PurePosixPath(member.name).parts
            content = archive.extractfile(member).read()
            assert hashlib.sha256(content).hexdigest() == package['payload'][member.name]['sha256']
            assert member.mode == package['payload'][member.name]['mode']
        provenance = json.load(archive.extractfile(name + '/SOURCE_PROVENANCE.json'))
        production = [p for p in provenance['source_inventory'] if p.startswith('src/')]
        assert production, name
        for path in production:
            expected = source('adl-v2/crates/' + name + '/' + path)
            assert archive.extractfile(name + '/' + path).read() == expected
            assert provenance['source_inventory'][path]['sha256'] == hashlib.sha256(expected).hexdigest()
        assert archive.extractfile(name + '/LICENSE').read() == source('LICENSE')
        if name == 'adl-engine':
            assert len(provenance['test_resources']) == 19
            for path in provenance['test_resources']:
                relative = path.removeprefix('adl-characterization/corpus/v1/fixtures/')
                assert archive.extractfile(name + '/tests/fixtures/characterization/' + relative).read() == source(path)
            original = source('adl-v2/crates/adl-engine/tests/compiler_fixture_mapping.rs')
            assert archive.extractfile(name + '/tests/compiler_fixture_mapping.rs').read() == original.replace(b'../../../adl-characterization/corpus/v1/fixtures', b'tests/fixtures/characterization')
"#,
    );
}

#[test]
fn malformed_or_missing_revision_has_no_output() {
    check_python(
        r#"
for rev in ('HEAD', revision[:12], 'g' * 40, '0' * 40):
    target = scratch / ('invalid-' + rev)
    result = build(target, rev)
    assert result.returncode != 0
    assert not target.exists()
"#,
    );
}

#[test]
fn existing_output_is_refused_without_overwrite() {
    check_python(
        r#"
target = scratch / 'existing'
target.mkdir()
(target / 'sentinel').write_bytes(b'preserve existing output')
result = build(target)
assert result.returncode != 0
assert sorted(p.name for p in target.iterdir()) == ['sentinel']
assert (target / 'sentinel').read_bytes() == b'preserve existing output'
"#,
    );
}
