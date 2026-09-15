#!/usr/bin/env python3
"""PVF runtime: installed impact consumer, deterministic inert fixtures, no network."""
import argparse
import hashlib
import json
import platform
import subprocess
from pathlib import Path


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--binary', type=Path, required=True)
    parser.add_argument('--output-root', type=Path, required=True)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    provenance = binary.parent / '.provenance' / (binary.name + '.sha256')
    if 'target' in binary.parts or not provenance.is_file():
        raise ValueError('Use an isolated installed binary with installer provenance')
    root = args.output_root.resolve()
    root.mkdir(parents=True, exist_ok=False)

    def execute(argv, success=True):
        result = subprocess.run([str(x) for x in argv], capture_output=True, timeout=60)
        assert (result.returncode == 0) == success, (result.returncode, result.stderr.decode())
        return result

    def cli(*args, success=True):
        result = execute([binary, 'codefriend', *args], success)
        assert str(root).encode() not in result.stdout + result.stderr
        if success:
            json.loads(result.stdout)
            assert b'adl_event' in result.stderr
        return result

    fixtures = Path(__file__).resolve().parents[1] / 'tests/fixtures/codefriend/impact'
    scenarios = []
    calibration = []
    for variant in ['chain', 'cycle', 'unknown']:
        case = root / variant
        source = case / 'source'
        (source / 'src').mkdir(parents=True)
        for fixture in fixtures.glob('*.rs'):
            (source / 'src' / fixture.name).write_bytes(fixture.read_bytes())
        if variant == 'cycle':
            (source / 'src/storage.rs').write_text('use crate::domain::Service;\n')
        if variant == 'unknown':
            (source / 'src/api.rs').write_text('opaque!();\n')
        (source / 'LICENSE').write_text('MIT fixture license\n')

        def git(*args):
            return execute(['git', '-C', source, *args]).stdout.decode().strip()
        git('init', '-b', 'main')
        git('remote', 'add', 'origin', 'https://example.com/owner/repo')
        git('add', '.')
        git('-c', 'user.name=fixture', '-c', 'user.email=fixture@example.com', 'commit', '-m', 'fixture')
        revision = git('rev-parse', 'HEAD')
        analysis = sorted('src/' + p.name for p in fixtures.glob('*.rs'))
        scope = {'analysis': analysis, 'context': ['LICENSE'], 'max_files': 6,
                 'max_bytes': 65536, 'max_file_bytes': 8192}
        (case / 'scope.json').write_text(json.dumps(scope))
        store = case / 'store'
        packet = json.loads(cli('evidence', 'admit-local', '--checkout', source,
            '--repository', 'https://example.com/owner/repo', '--revision', revision,
            '--scope', case / 'scope.json', '--store', store, '--retention-seconds', '3600').stdout)['packet_id']
        policy = {'schema': 'codefriend.structure.v1', 'crate_root': 'src/lib.rs',
                  'manifest_path': None, 'layers': {p: 'core' for p in analysis},
                  'allowed': [], 'coupling_threshold': 2}
        policy['layers']['src/api.rs'] = 'api'
        (case / 'policy.json').write_text(json.dumps(policy))
        graph_path = case / 'graph.json'
        cli('architecture', 'report', '--store', store, '--packet-id', packet,
            '--policy', case / 'policy.json', '--out', graph_path)
        graph = json.loads(graph_path.read_text())
        names = {n['id']: n['module'] for n in graph['nodes']}
        changes = {'schema': 'codefriend.impact.v1', 'repository': graph['record']['run']['repository'],
                   'revision': revision, 'graph_digest': graph['digest'],
                   'targets': [{'kind': 'module', 'name': 'crate::storage'}]}

        def run(name, value, success=True, error=None):
            input_path = case / (name + '-changes.json')
            input_path.write_text(json.dumps(value))
            out = case / (name + '.json')
            result = cli('architecture', 'impact', '--store', store, '--graph', graph_path,
                         '--changes', input_path, '--out', out, success=success)
            if not success:
                assert not out.exists()
                assert error.encode() in result.stderr
                return None
            read = cli('architecture', 'impact-read', '--store', store, '--input', out)
            assert json.loads(read.stdout) == json.loads(result.stdout)
            return json.loads(out.read_text())

        first = run('first', changes)
        repeat = run('repeat', changes)
        assert first == repeat
        impacted = {names[i['dependent_node']] for i in first['impacts']}
        expected = {'crate::domain'} if variant == 'unknown' else {'crate::api', 'crate::domain'}
        assert impacted == expected
        assert first['analysis_complete'] == (variant != 'unknown')
        if variant == 'unknown':
            assert first['scoped_unimpacted_nodes'] == [] and first['unknowns']
        else:
            assert {names[n] for n in first['scoped_unimpacted_nodes']} == {'crate', 'crate::unused'}
            calibration.append({'fixture': variant, 'expected_potential_modules': sorted(expected),
                'observed_potential_modules': sorted(impacted), 'sample_false_positives': 0, 'sample_false_negatives': 0})
        for impact in first['impacts']:
            assert impact['path'][0]['from'] == impact['dependent_node']
            assert impact['path'][-1]['to'] == impact['changed_node']
            assert impact['change_digest'] == first['change_digest']
            assert impact['graph_revision'] == revision and impact['graph_digest'] == graph['digest']
            assert all(a['to'] == b['from'] for a, b in zip(impact['path'], impact['path'][1:]))
            assert impact['inference'] and all(e in graph['edges'] for e in impact['path'])
        if variant == 'cycle':
            domain = next(i for i in first['impacts'] if names[i['dependent_node']] == 'crate::domain')
            assert 'cycle_with_changed_root' in domain['risks']
        scenarios.append(variant + '-reachability-repeat-readback')
        if variant == 'chain':
            multiple = dict(changes, targets=[{'kind': 'module', 'name': 'crate::domain'}, *changes['targets']])
            assert len(run('multiple', multiple)['impacts']) == 3
            unknown = dict(changes, targets=[{'kind': 'module', 'name': 'crate::missing'}, {'kind': 'symbol', 'name': 'crate::storage::Record'}])
            partial = run('unsupported', unknown)
            assert not partial['analysis_complete'] and not partial['scoped_unimpacted_nodes'] and len(partial['unknowns']) == 2
            run('stale-revision', dict(changes, revision='0' * 40), False, 'stale_change_revision')
            run('stale-graph', dict(changes, graph_digest='0' * 64), False, 'stale_change_graph_digest')
            run('malformed', dict(changes, extra=True), False, 'invalid_change_input')
            run('bound', dict(changes, targets=changes['targets'] * 65), False, 'change_target_bounds_exceeded')
            original = case / 'first.json'
            tampered = dict(first, impacts=[])
            (case / 'tampered.json').write_text(json.dumps(tampered))
            failure = cli('architecture', 'impact-read', '--store', store, '--input', case / 'tampered.json', success=False)
            assert b'impact_artifact_mismatch' in failure.stderr
            before = original.read_bytes()
            failure = cli('architecture', 'impact', '--store', store, '--graph', graph_path, '--changes', case / 'first-changes.json', '--out', original, success=False)
            assert b'impact_output_unavailable' in failure.stderr and original.read_bytes() == before
            cli('evidence', 'delete', '--store', store, '--packet-id', packet)
            cli('architecture', 'impact-read', '--store', store, '--input', original, success=False)
            scenarios.extend(['multiple-roots', 'unsupported-partial', 'stale-revision', 'stale-graph', 'malformed', 'target-bound', 'tamper', 'overwrite', 'deletion'])
        assert git('status', '--porcelain') == ''
    proof = {'schema': 'codefriend.impact_installed_proof.v1', 'issue': 883,
             'platform': platform.system() + '-' + platform.machine(),
             'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
             'installation_provenance': provenance.read_text().strip(),
             'scenarios': scenarios, 'executed_scenarios': len(scenarios), 'calibration': calibration,
             'source_unchanged': True, 'stdout_json_stderr_events': True,
             'nonclaims': ['Synthetic syntactic potential-impact calibration is not measured runtime impact accuracy',
                           'No provider, source build or external repository execution', 'No Linux qualification from a macOS run']}
    (root / 'proof.json').write_text(json.dumps(proof, indent=2) + '\n')
    print(json.dumps(proof, indent=2))


if __name__ == '__main__':
    main()
