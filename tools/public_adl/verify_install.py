#!/usr/bin/env python3
"""RD03 deterministic source distributions and enforced macOS consumer proof.

No publication or provider execution. Run from a committed candidate. All five
archives are made solely from Git blobs; extracted packages use only each other
and public registry dependencies. The consumer runs with network, home secrets,
and producer checkouts denied by the OS sandbox, not merely a sanitized env.
"""
import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[2]
PACKAGES = {
    'adl-schema': 'adl-schema',
    'adl-uts': 'adl-uts',
    'adl-language': 'adl-v2/crates/adl-language',
    'adl-compiler': 'adl-v2/crates/adl-compiler',
    'adl-legacy-contracts': 'adl-legacy-contracts',
}


def call(args, cwd=ROOT, env=None, check=True):
    result = subprocess.run(args, cwd=cwd, env=env, text=True, capture_output=True)
    if check and result.returncode:
        raise RuntimeError(f'{args[0]} failed ({result.returncode}): {result.stderr[-8000:]}')
    return result


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    head = call(['git', 'rev-parse', 'HEAD']).stdout.strip()
    paths = list(PACKAGES.values()) + ['tools/public_adl']
    if call(['git', 'status', '--porcelain', '--', *paths]).stdout.strip():
        raise SystemExit('Commit package and qualification sources before candidate proof.')
    if not Path('/usr/bin/sandbox-exec').exists():
        raise SystemExit('This qualification requires macOS sandbox-exec; no unenforced fallback.')
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    cargo = Path(call(['rustup', 'which', 'cargo']).stdout.strip()).resolve()
    rustc = Path(call(['rustup', 'which', 'rustc']).stdout.strip()).resolve()
    toolchain = cargo.parent.parent
    original_home = Path.home().resolve()
    cache = original_home / '.cargo/registry'
    versions = {}
    artifacts = []
    with tempfile.TemporaryDirectory(prefix='adl-rd03-', dir='/private/tmp') as temporary:
        work = Path(temporary).resolve()
        vendor = work / 'vendor'
        vendor.mkdir()
        for name, source in PACKAGES.items():
            raw = subprocess.check_output(['git', 'archive', head, source], cwd=ROOT)
            staging = work / f'stage-{name}'
            with tarfile.open(fileobj=io.BytesIO(raw)) as archive:
                archive.extractall(staging, filter='data')
            package = staging / source
            manifest = package / 'Cargo.toml'
            parsed = tomllib.loads(manifest.read_text())
            versions[name] = parsed['package']['version']
            # Normalization is explicit in the artifact manifest. Both dependency
            # edges are now sibling extracted artifacts, never producer paths.
            text = manifest.read_text()
            if name == 'adl-compiler':
                assert 'path = "../adl-language"' in text
            if name == 'adl-legacy-contracts':
                assert 'path = "../adl-schema"' in text
            # Isolate from the original language/compiler workspace.
            text += '\n[workspace]\n'
            manifest.write_text(text)
            destination = vendor / name
            shutil.copytree(package, destination)
            artifact = output / f'{name}-{versions[name]}.tar'
            with tarfile.open(artifact, 'w') as archive:
                for path in sorted(destination.rglob('*')):
                    if not path.is_file():
                        continue
                    info = archive.gettarinfo(str(path), arcname=str(Path(name) / path.relative_to(destination)))
                    info.uid = info.gid = info.mtime = 0
                    info.uname = info.gname = ''
                    info.mode = 0o644
                    with path.open('rb') as stream:
                        archive.addfile(info, stream)
            shutil.rmtree(destination)
            with tarfile.open(artifact) as archive:
                archive.extractall(vendor, filter='data')
            artifacts.append({'package': name, 'version': versions[name], 'source': source,
                              'artifact': artifact.name, 'sha256': digest(artifact),
                              'manifest_normalization': 'standalone workspace; existing sibling public dependencies retained'})
        consumer = work / 'consumer'
        (consumer / 'src').mkdir(parents=True)
        dependencies = '\n'.join(f'{name} = {{ version = "={version}", path = "../vendor/{name}" }}' for name, version in versions.items())
        manifest = '[package]\nname="rd03-independent-consumer"\nversion="0.0.0"\nedition="2021"\n[dependencies]\nserde_json="1"\nserde_yaml="0.9"\n' + dependencies + '\n'
        (consumer / 'Cargo.toml').write_text(manifest)
        fixture = vendor / 'adl-language/fixtures/six-primitives.adl.yaml'
        shutil.copyfile(fixture, consumer / 'fixture.yaml')
        (consumer / 'src/main.rs').write_bytes((ROOT / 'tools/public_adl/consumer.rs').read_bytes())
        home = work / 'home'
        cargo_home = work / 'cargo-home'
        home.mkdir()
        cargo_home.mkdir()
        # Only public registry cache is reachable; no global Cargo config/token.
        (cargo_home / 'registry').symlink_to(cache, target_is_directory=True)
        # Owned synthetic credential sentinel; never inspect real credentials.
        secret = work / 'private-credential'
        secret.write_text('synthetic-private-credential-denial-probe\n')
        denied = [str(original_home), '/Volumes/FastWork', '/Volumes/home/builds', str(ROOT), str(secret)]
        def quote(value):
            return json.dumps(str(value))
        profile = '(version 1)\n(allow default)\n(deny network*)\n'
        profile += ''.join(f'(deny file-read* (subpath {quote(path)}))\n' for path in denied)
        profile += f'(allow file-read* (subpath {quote(toolchain)}) (subpath {quote(cache)}))\n'
        # Cargo canonicalizes public cache paths through their parent directories.
        # Metadata traversal reveals no credential or producer file contents.
        profile += f'(allow file-read-metadata (subpath {quote(original_home)}))\n'
        sandbox = work / 'consumer.sb'
        sandbox.write_text(profile)
        env = {'HOME': str(home), 'CARGO_HOME': str(cargo_home), 'RUSTC': str(rustc),
               'PATH': f'{cargo.parent}:/usr/bin:/bin', 'TMPDIR': str(work),
               'CARGO_TARGET_DIR': str(work / 'target'), 'CARGO_NET_OFFLINE': 'true'}
        def isolated(command, check=True):
            return call(['/usr/bin/sandbox-exec', '-f', str(sandbox), *command], cwd=consumer, env=env, check=check)
        def denied_probe(name, command, expected):
            result = isolated(command, check=False)
            if result.returncode == 0 or not any(value in result.stderr.lower() for value in expected):
                raise RuntimeError(f'{name}: required classified rejection absent: {result.stderr[-2000:]}')
            return {'case': name, 'status': 'rejected', 'exit_code': result.returncode}
        probes = []
        isolated(['/bin/cat', str(consumer / 'fixture.yaml')])
        probes.append(denied_probe('private-credential-denied', ['/bin/cat', str(secret)], ['operation not permitted', 'permission denied']))
        probes.append(denied_probe('producer-source-denied', ['/bin/cat', str(ROOT / 'adl-schema/Cargo.toml')], ['operation not permitted', 'permission denied']))
        probes.append(denied_probe('network-denied', ['/usr/bin/ruby', '-rsocket', '-e', 'TCPSocket.new("127.0.0.1", 9)'], ['operation not permitted', 'permission denied']))
        isolated([str(cargo), 'generate-lockfile', '--offline'])
        shutil.copyfile(consumer / 'Cargo.lock', output / 'consumer.Cargo.lock')
        toolchain_versions = {tool: isolated([str(path), '--version']).stdout.strip()
                              for tool, path in [('cargo', cargo), ('rustc', rustc)]}
        result = isolated([str(cargo), 'run', '--offline', '--locked', '--quiet'])
        consumer_report = json.loads(result.stdout)
        assert consumer_report['passed'] and consumer_report['executed_cases'] == consumer_report['expected_cases'] == 7
        assert consumer_report['uts_fixture_count'] == len(consumer_report['uts_cases'])
        assert all(case['passed'] for case in consumer_report['uts_cases'])
        host = next(line.split(': ', 1)[1] for line in isolated([str(rustc), '-vV']).stdout.splitlines() if line.startswith('host: '))
        metadata = json.loads(isolated([str(cargo), 'metadata', '--offline', '--locked', '--filter-platform', host, '--format-version', '1']).stdout)
        for name in PACKAGES:
            package = next(p for p in metadata['packages'] if p['name'] == name)
            assert Path(package['manifest_path']).resolve() == vendor / name / 'Cargo.toml'
        # Actual Cargo resolution negatives, not path-string assertions.
        (consumer / 'Cargo.toml').write_text(manifest.replace('path = "../vendor/adl-schema"', f'path = "{ROOT}/adl-schema"'))
        probes.append(denied_probe('sibling-producer-dependency-denied', [str(cargo), 'metadata', '--offline', '--format-version', '1'], ['operation not permitted', 'permission denied']))
        (consumer / 'Cargo.toml').write_text(manifest.replace('adl-schema = { version = "=0.1.0"', 'adl-schema = { version = "=99.0.0"'))
        probes.append(denied_probe('incompatible-package-version', [str(cargo), 'metadata', '--offline', '--format-version', '1'], ['failed to select a version']))
        report = {'schema': 'adl.public.install-proof.v1', 'source_revision': head,
                  'packages': artifacts, 'consumer': consumer_report,
                  'dependency_lock': {'path': 'consumer.Cargo.lock', 'sha256': digest(output / 'consumer.Cargo.lock')},
                  'toolchain_versions': toolchain_versions,
                  'sandbox': 'macOS sandbox-exec: network, home and producer source denied; toolchain/public registry allowed',
                  'negative_cases': probes, 'negative_expected': 5, 'negative_executed': len(probes), 'public_packages_consumed': 5,
                  'execution_authority_claimed': False, 'registry_publication_claimed': False}
        (output / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
        print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
