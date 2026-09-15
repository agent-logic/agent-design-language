#!/usr/bin/env python3
"""PVF runtime: installed comparison consumer using live production-contract fixtures."""
import argparse
import hashlib
import json
import platform
import subprocess
from pathlib import Path


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--binary', type=Path, required=True)
    p.add_argument('--fixture-root', type=Path, required=True)
    args = p.parse_args()
    binary = args.binary.resolve(strict=True)
    root = args.fixture_root.resolve(strict=True)
    assert 'target' not in binary.parts
    provenance = binary.parent / '.provenance' / (binary.name + '.sha256')
    assert provenance.is_file()
    def cli(*argv, success=True):
        r = subprocess.run([str(binary), 'codefriend', 'memory', *map(str, argv)], capture_output=True, timeout=60)
        assert (r.returncode == 0) == success, f'CLI exit {r.returncode}'
        if success:
            assert b'adl_event' in r.stderr
            return json.loads(r.stdout)
        return r
    common = ['--store', root / 'store', '--baselines', root / 'baselines']
    refs = {}
    for name in ['baseline', 'current', 'current-incompatible', 'current-partial']:
        refs[name] = cli('retain', *common, '--input', root / (name + '.json'))
        (root / (name + '-ref.json')).write_text(json.dumps(refs[name]))
        readback = cli('read', *common, '--reference', root / (name + '-ref.json'))
        assert readback['run']['id'] == refs[name]['run_id']
    def compare(current, output):
        return cli('compare', *common, '--baseline', root / 'baseline-ref.json', '--current', root / (current + '-ref.json'), '--out', root / output)
    delta = compare('current', 'delta.json')
    repeat = compare('current', 'repeat.json')
    assert repeat == delta
    assert sorted(c['comparison']['outcome'] for c in delta['changes']) == ['added', 'changed', 'resolved', 'unchanged']
    same = next(c for c in delta['changes'] if c['comparison']['outcome'] == 'unchanged')
    assert same['before']['finding_id'] == same['after']['finding_id']
    assert same['before']['evidence'] != same['after']['evidence']
    assert cli('delta-read', *common, '--input', root / 'delta.json') == delta
    for name in ['current-incompatible', 'current-partial']:
        d = compare(name, name + '-delta.json')
        assert not d['comparable'] and all(c['comparison']['outcome'] == 'not_comparable' for c in d['changes'])
    altered = dict(delta, changes=[])
    (root / 'tampered.json').write_text(json.dumps(altered))
    cli('delta-read', *common, '--input', root / 'tampered.json', success=False)
    cli('delete', *common, '--reference', root / 'baseline-ref.json')
    cli('delta-read', *common, '--input', root / 'delta.json', success=False)
    cli('retain', *common, '--input', root / 'baseline.json', success=False)
    status = subprocess.run(['git', '-C', str(root / 'source'), 'status', '--porcelain'], capture_output=True, check=True)
    assert not status.stdout
    proof = {'schema': 'codefriend.memory_installed_proof.v1', 'issue': 885,
             'platform': platform.system() + '-' + platform.machine(),
             'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
             'installation_provenance': provenance.read_text().strip(),
             'fixture_proof': json.loads((root / 'fixture-proof.json').read_text()),
             'outcomes': [c['comparison']['outcome'] for c in delta['changes']],
             'scenarios_passed': 8, 'deterministic': True, 'moved_evidence_stable_identity': True,
             'incompatible_and_partial_no_resolution': True, 'tampering_denied': True,
             'deletion_denies_saved_delta_and_resurrection': True, 'source_unchanged': True,
             'stdout_json_stderr_events': True,
             'nonclaims': ['Known fixture assessments, not a new source analysis producer', 'No provider or external source execution']}
    (root / 'installed-proof.json').write_text(json.dumps(proof, indent=2) + '\n')
    print(json.dumps(proof, indent=2))


if __name__ == '__main__':
    main()
