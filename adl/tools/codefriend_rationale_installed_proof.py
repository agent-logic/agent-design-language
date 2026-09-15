#!/usr/bin/env python3
"""PVF runtime: actual installed rationale consumer on inert local evidence, no network."""
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
        raise ValueError('Isolated installed binary and installer provenance required')
    root = args.output_root.resolve()
    root.mkdir(parents=True, exist_ok=False)
    fixtures = Path(__file__).resolve().parents[1] / 'tests/fixtures/codefriend/rationale'
    results = []

    def execute(argv, success=True):
        r = subprocess.run([str(x) for x in argv], capture_output=True, timeout=60)
        assert (r.returncode == 0) == success, r.stderr.decode()
        return r

    def cli(*argv, success=True):
        r = execute([binary, 'codefriend', *argv], success)
        assert str(root).encode() not in r.stdout + r.stderr
        if success:
            json.loads(r.stdout)
            assert b'adl_event' in r.stderr
        return r

    for variant in ['accepted', 'candidate', 'superseded', 'conflict', 'different-key', 'empty', 'unknown-deployment', 'outside']:
        case = root / variant
        source = case / 'source'
        (source / 'src').mkdir(parents=True)
        (source / 'src/lib.rs').write_text('pub struct Api;\n')
        (source / 'LICENSE').write_text('MIT fixture license\n')
        compose = json.loads((fixtures / 'compose.json').read_text())
        if variant == 'unknown-deployment':
            compose['services']['api']['depends_on'] = ['db']
        (source / 'compose.json').write_text(json.dumps(compose))
        adr = (fixtures / 'accepted.md').read_text()
        if variant in ['candidate', 'superseded']:
            adr = adr.replace('accepted', variant)
        (source / 'adr.md').write_text(adr)
        context = ['LICENSE', 'adr.md', 'compose.json']
        paths = ['adr.md']
        if variant in ['conflict', 'different-key']:
            other = adr.replace('separate-service', 'combined-service')
            if variant == 'different-key':
                other = other.replace('api-deployment', 'other-policy')
            (source / 'other.md').write_text(other)
            context.append('other.md')
            paths.append('other.md')
        if variant == 'empty':
            paths = []
        if variant == 'outside':
            paths = ['unadmitted.md']
        def git(*argv):
            return execute(['git', '-C', source, *argv]).stdout.decode().strip()
        git('init', '-b', 'main')
        git('remote', 'add', 'origin', 'https://example.com/owner/repo')
        git('add', '.')
        git('-c', 'user.name=fixture', '-c', 'user.email=fixture@example.com', 'commit', '-m', 'fixture')
        revision = git('rev-parse', 'HEAD')
        scope = {'analysis': ['src/lib.rs'], 'context': sorted(context), 'max_files': 8, 'max_bytes': 65536, 'max_file_bytes': 8192}
        (case / 'scope.json').write_text(json.dumps(scope))
        store = case / 'store'
        packet = json.loads(cli('evidence', 'admit-local', '--checkout', source, '--repository', 'https://example.com/owner/repo', '--revision', revision, '--scope', case / 'scope.json', '--store', store, '--retention-seconds', '3600').stdout)['packet_id']
        policy = {'schema': 'codefriend.structure.v1', 'crate_root': 'src/lib.rs', 'manifest_path': None, 'layers': {'src/lib.rs': 'core'}, 'allowed': [], 'coupling_threshold': 2}
        (case / 'policy.json').write_text(json.dumps(policy))
        graph_path = case / 'graph.json'
        cli('architecture', 'report', '--store', store, '--packet-id', packet, '--policy', case / 'policy.json', '--out', graph_path)
        graph = json.loads(graph_path.read_text())
        selection = {'schema': 'codefriend.rationale.v1', 'graph_digest': graph['digest'], 'revision': revision,
                     'boundaries': [{'boundary': 'core', 'deployment_path': 'compose.json', 'service': 'api', 'rationale_paths': paths}]}
        selection_path = case / 'selection.json'
        selection_path.write_text(json.dumps(selection))
        def report(name):
            path = case / (name + '.json')
            summary = cli('architecture', 'rationale', '--store', store, '--graph', graph_path, '--selection', selection_path, '--out', path)
            read = cli('architecture', 'rationale-read', '--store', store, '--input', path)
            assert json.loads(summary.stdout) == json.loads(read.stdout)
            return json.loads(path.read_text())
        first = report('first')
        assert first == report('repeat')
        b = first['boundaries'][0]
        assert b['revision'] == revision and b['graph_digest'] == graph['digest']
        assert b['boundary_evidence'][0]['path'] == 'src/lib.rs'
        assert b['deployment_evidence']['path'] == 'compose.json'
        assert first['analysis_complete'] == (variant in ['accepted', 'different-key'])
        assert bool(b['conflicting_decision_keys']) == (variant == 'conflict')
        if variant in ['candidate', 'superseded']:
            assert b['rationale'][0]['decision']['status'] == variant and 'no_accepted_rationale' in b['unknowns']
        if variant == 'accepted':
            assert b['rationale'][0]['location']['path'] == 'adr.md'
            assert len(first['record']['findings'][0]['evidence']) == 3
            tampered = json.loads(json.dumps(first))
            tampered['boundaries'][0]['rationale'][0]['decision']['status'] = 'candidate'
            (case / 'tampered.json').write_text(json.dumps(tampered))
            bad = cli('architecture', 'rationale-read', '--store', store, '--input', case / 'tampered.json', success=False)
            assert b'rationale_artifact_mismatch' in bad.stderr
            selection['revision'] = '0' * 40
            selection_path.write_text(json.dumps(selection))
            bad = cli('architecture', 'rationale', '--store', store, '--graph', graph_path, '--selection', selection_path, '--out', case / 'stale.json', success=False)
            assert b'stale_rationale_graph' in bad.stderr and not (case / 'stale.json').exists()
            cli('evidence', 'delete', '--store', store, '--packet-id', packet)
            cli('architecture', 'rationale-read', '--store', store, '--input', case / 'first.json', success=False)
        assert git('status', '--porcelain') == ''
        results.append({'fixture': variant, 'complete': first['analysis_complete'], 'conflicts': b['conflicting_decision_keys'], 'unknowns': b['unknowns'], 'repeat_readback': True})
    proof = {'schema': 'codefriend.rationale_installed_proof.v1', 'issue': 884,
             'platform': platform.system() + '-' + platform.machine(),
             'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(), 'installation_provenance': provenance.read_text().strip(),
             'scenarios': results, 'executed_scenarios': len(results) + 3, 'negative_guards': ['tamper', 'stale-revision', 'post-deletion-read'],
             'calibration': {'fixed_sample': ['conflict', 'different-key', 'candidate', 'superseded'], 'expected_conflict': ['conflict'], 'observed_conflict': ['conflict'], 'false_positives': 0, 'false_negatives': 0},
             'source_unchanged': True, 'stdout_json_stderr_events': True,
             'nonclaims': ['Synthetic explicit-choice conflict calibration only; no semantic prose contradiction accuracy', 'No measured runtime independent deployability', 'No provider or external repository execution', 'No Linux qualification from a macOS run']}
    (root / 'proof.json').write_text(json.dumps(proof, indent=2) + '\n')
    print(json.dumps(proof, indent=2))


if __name__ == '__main__':
    main()
