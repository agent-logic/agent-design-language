#!/usr/bin/env python3
"""PVF runtime: real installed graph producer, CF-MEMORY retention/comparison and drift readback."""
import argparse
import hashlib
import json
import platform
import subprocess
from pathlib import Path


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--binary', type=Path, required=True)
    p.add_argument('--output-root', type=Path, required=True)
    a = p.parse_args()
    binary = a.binary.resolve(strict=True)
    provenance = binary.parent / '.provenance' / (binary.name + '.sha256')
    assert 'target' not in binary.parts and provenance.is_file()
    root = a.output_root.resolve()
    root.mkdir(parents=True, exist_ok=False)

    def execute(args, success=True):
        r = subprocess.run([str(x) for x in args], capture_output=True, timeout=60)
        assert (r.returncode == 0) == success, r.stderr.decode()
        return r

    def cli(*args, success=True):
        r = execute([binary, 'codefriend', *args], success)
        assert str(root).encode() not in r.stdout + r.stderr
        if success:
            json.loads(r.stdout)
            assert b'adl_event' in r.stderr
        return r

    calibration = []
    scenarios = []
    for variant in ['changed', 'unchanged', 'relocated', 'partial', 'narrower', 'policy']:
        case = root / variant
        source = case / 'source'
        source.mkdir(parents=True)
        files = {'lib.rs': 'mod a; mod b; mod c;', 'a.rs': 'use crate::b::B;',
                 'b.rs': 'pub struct B;', 'c.rs': 'pub struct C;', 'LICENSE': 'MIT fixture license'}
        for name, text in files.items():
            (source / name).write_text(text)
        def git(*args):
            return execute(['git', '-C', source, *args]).stdout.decode().strip()
        git('init', '-b', 'main')
        git('remote', 'add', 'origin', 'https://example.com/owner/repo')
        scope = {'analysis': ['a.rs', 'b.rs', 'c.rs', 'lib.rs'], 'context': ['LICENSE'],
                 'max_files': 5, 'max_bytes': 65536, 'max_file_bytes': 32768}
        policy = {'schema': 'codefriend.structure.v1', 'crate_root': 'lib.rs', 'manifest_path': None,
                  'layers': {x: ('api' if x == 'c.rs' else 'core') for x in scope['analysis']},
                  'allowed': [], 'coupling_threshold': 2}
        store, baselines = case / 'store', case / 'baselines'
        graphs = []
        for state in ['baseline', 'current']:
            if state == 'current':
                if variant == 'changed':
                    (source / 'a.rs').write_text('use crate::c::C;')
                elif variant == 'relocated':
                    (source / 'a.rs').write_text('\n\nuse crate::b::B;')
                elif variant == 'partial':
                    (source / 'a.rs').write_text('opaque!();')
                elif variant == 'narrower':
                    scope['analysis'].remove('c.rs')
                    policy['layers'].pop('c.rs')
                elif variant == 'policy':
                    policy['layers']['a.rs'] = 'api'
            git('add', '.')
            git('-c', 'user.name=fixture', '-c', 'user.email=fixture@example.com', 'commit', '--allow-empty', '-m', state)
            revision = git('rev-parse', 'HEAD')
            (case / 'scope.json').write_text(json.dumps(scope))
            (case / 'policy.json').write_text(json.dumps(policy))
            packet = json.loads(cli('evidence', 'admit-local', '--checkout', source, '--repository',
                'https://example.com/owner/repo', '--revision', revision, '--scope', case / 'scope.json',
                '--store', store, '--retention-seconds', '3600').stdout)['packet_id']
            graphpath = case / (state + '.json')
            cli('architecture', 'report', '--store', store, '--packet-id', packet,
                '--policy', case / 'policy.json', '--out', graphpath)
            graph = json.loads(graphpath.read_text())
            graphs.append(graph)
        assert graphs[0]['record']['run']['revision'] != graphs[1]['record']['run']['revision']
        def drift(name, success=True):
            return cli('architecture', 'drift', '--store', store, '--baselines', baselines,
                '--baseline', case / 'baseline.json', '--current', case / 'current.json',
                '--out', case / (name + '.json'), success=success)
        first = drift('first')
        report = json.loads((case / 'first.json').read_text())
        drift('repeat')
        assert report == json.loads((case / 'repeat.json').read_text())
        read = cli('architecture', 'drift-read', '--store', store, '--baselines', baselines, '--input', case / 'first.json')
        assert json.loads(first.stdout) == json.loads(read.stdout)
        delta = report['structural_comparison']
        outcomes = [x['comparison']['outcome'] for x in delta['changes']]
        if variant in ['partial', 'narrower', 'policy']:
            assert not delta['comparable'] and outcomes == ['not_comparable'] and delta['reasons']
        else:
            expected = ['added', 'resolved'] if variant == 'changed' else []
            observed = sorted(x for x in outcomes if x != 'unchanged')
            assert observed == expected and delta['comparable']
            calibration.append({'fixture': variant, 'expected_deltas': expected, 'observed_deltas': observed,
                                'false_positives': 0, 'false_negatives': 0})
        assert report['graph_comparison']['comparable'] == delta['comparable']
        for side in ['baseline', 'current']:
            facts = report[side + '_facts']
            assert facts['admission'] == report[side]['record']['admission']
            assert all(t['locations'] for t in report[side + '_traces'])
        if variant == 'changed':
            added = next(x for x in report['current_facts']['findings'] if x['rule'] == 'edge')
            assert 'crate::c' in added['rationale'] and 'cross_layer=true' in added['rationale']
            reference = report['graph_comparison']['baseline']
            (case / 'reference.json').write_text(json.dumps(reference))
            loaded = json.loads(cli('memory', 'read', '--store', store, '--baselines', baselines,
                '--reference', case / 'reference.json').stdout)
            assert loaded == report['baseline']['record']
            original = (case / 'first.json').read_bytes()
            drift('first', False)
            assert (case / 'first.json').read_bytes() == original
            broken = json.loads(original)
            broken['structural_comparison']['changes'] = []
            (case / 'tampered.json').write_text(json.dumps(broken))
            bad = cli('architecture', 'drift-read', '--store', store, '--baselines', baselines, '--input', case / 'tampered.json', success=False)
            assert b'drift_artifact_mismatch' in bad.stderr
            cli('memory', 'delete', '--store', store, '--baselines', baselines, '--reference', case / 'reference.json')
            cli('architecture', 'drift-read', '--store', store, '--baselines', baselines, '--input', case / 'first.json', success=False)
            drift('after-deletion', False)
            assert not (case / 'after-deletion.json').exists()
            scenarios.extend(['actual-memory-read', 'overwrite', 'tampered-delta', 'deleted-baseline-read', 'no-resurrection'])
        assert git('status', '--porcelain') == ''
        scenarios.append(variant + '-repeat-readback')
    proof = {'schema': 'codefriend.drift_installed_proof.v1', 'issue': 886,
             'platform': platform.system() + '-' + platform.machine(),
             'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
             'installation_provenance': provenance.read_text().strip(), 'scenarios': scenarios,
             'executed_scenarios': len(scenarios), 'calibration': calibration, 'source_unchanged': True,
             'stdout_json_stderr_events': True,
             'nonclaims': ['Synthetic scoped reference-change accuracy, not runtime architecture drift or semantic rename inference',
                           'No provider or external source execution', 'No Linux qualification from macOS proof']}
    (root / 'proof.json').write_text(json.dumps(proof, indent=2) + '\n')
    print(json.dumps(proof, indent=2))


if __name__ == '__main__':
    main()
