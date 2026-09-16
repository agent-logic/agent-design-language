#!/usr/bin/env python3
"""PVF runtime: deterministic installed local fitness contract proof; no network."""
import argparse
import hashlib
import json
import platform
import subprocess
from pathlib import Path


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--fixture-root', type=Path, required=True)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    root = args.fixture_root.resolve()
    root.mkdir(parents=True, exist_ok=False)
    fixtures = Path(__file__).resolve().parents[1] / 'tests/fixtures/codefriend/fitness'
    assert 'target' not in binary.parts
    provenance = binary.parent / '.provenance' / (binary.name + '.sha256')
    assert provenance.is_file()

    def git(source, *argv):
        return subprocess.run(['git', '-C', str(source), *argv], capture_output=True, check=True).stdout.decode().strip()

    def cli(*argv, code=0, event=True):
        r = subprocess.run([str(binary), 'codefriend', *map(str, argv)], capture_output=True, timeout=60)
        assert r.returncode == code, f'expected {code}, got {r.returncode}'
        value = json.loads(r.stdout)
        if event:
            assert b'adl_event' in r.stderr
            assert str(root).encode() not in r.stderr
        return value

    results = []
    for name, code in [('pass', 0), ('fail', 1), ('error', 2)]:
        case = root / name
        case.mkdir()
        source = case / 'source'
        source.mkdir()
        git(source, 'init', '-b', 'main')
        git(source, 'remote', 'add', 'origin', 'https://example.com/owner/repo')
        (source / 'lib.rs').write_bytes((fixtures / (name + '.rs')).read_bytes())
        git(source, 'add', '.')
        git(source, '-c', 'user.name=fixture', '-c', 'user.email=fixture@example.com', 'commit', '-m', 'fixture')
        revision = git(source, 'rev-parse', 'HEAD')
        scope = case / 'scope.json'
        scope.write_text(json.dumps(dict(analysis=['lib.rs'], context=[], max_files=4, max_bytes=10000, max_file_bytes=10000)))
        admitted = cli('evidence', 'admit-local', '--checkout', source, '--repository', 'https://example.com/owner/repo', '--revision', revision, '--scope', scope, '--store', case / 'store', '--retention-seconds', '3600', event=False)
        common = ['--store', case / 'store']
        run = ['fitness', 'run', *common, '--packet-id', admitted['packet_id'], '--policy', fixtures / 'policy.json']
        report = cli(*run, '--out', case / 'report.json', code=code)
        assert report['status'] == name
        assert cli(*run, '--out', case / 'repeat.json', code=code) == report
        assert cli('fitness', 'read', *common, '--input', case / 'report.json', code=code) == report
        assert report['policy'] == json.loads((fixtures / 'policy.json').read_text())
        assert report['record']['run']['packet_id'] == admitted['packet_id']
        assert 'architecture_quality' in report['unassessed']
        if name == 'fail':
            assert report['violations'][0]['line'] == 1
            assert report['record']['findings']
        tampered = dict(report, violations=[] if name == 'fail' else [{'forged': True}])
        (case / 'tampered.json').write_text(json.dumps(tampered))
        cli('fitness', 'read', *common, '--input', case / 'tampered.json', code=2)
        cli('evidence', 'delete', *common, '--packet-id', admitted['packet_id'], event=False)
        cli('fitness', 'read', *common, '--input', case / 'report.json', code=2)
        assert not git(source, 'status', '--porcelain')
        results.append(dict(fixture=name, revision=revision, exit=code, repeat=True, readback=True, tamper_rejected=True, deletion_rejected=True, source_unchanged=True))
    proof = dict(schema='codefriend.fitness_installed_proof.v1', issue=887, platform=platform.system() + '-' + platform.machine(), binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(), installation_provenance=provenance.read_text().strip(), scenarios=results, scenarios_passed=len(results), actual_ci_integration=False)
    (root / 'installed-proof.json').write_text(json.dumps(proof, indent=2) + '\n')
    print(json.dumps(proof, indent=2))


if __name__ == '__main__':
    main()
