#!/usr/bin/env python3
"""PVF runtime: production installed Memory Palace first/second-run consumer proof.
Prepare separate memory fixtures with codefriend_memory_fixture and authority fixtures
with codefriend_palace_fixture. No provider/network operation is requested.
"""
import argparse
import hashlib
import json
import subprocess
from pathlib import Path


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--binary', type=Path, required=True)
    p.add_argument('--fixture-root', type=Path, required=True)
    p.add_argument('--authority-root', type=Path, required=True)
    a = p.parse_args()
    binary, root, authority = [x.resolve(strict=True) for x in
                               (a.binary, a.fixture_root, a.authority_root)]
    assert 'target' not in binary.parts
    provenance = binary.parent / '.provenance' / (binary.name + '.sha256')
    assert provenance.is_file()
    scenarios = []

    def write(name, value):
        path = root / name
        with path.open('x') as f:
            json.dump(value, f)
        return path

    def cli(*args, error=None):
        result = subprocess.run([str(binary), 'codefriend', 'memory', *map(str, args)],
                                capture_output=True, timeout=60)
        for forbidden in (str(root).encode(), str(authority).encode(),
                          b'raw private birthday state', b'PRIVATE-PALACE-CANARY'):
            assert forbidden not in result.stderr, 'stderr contains sensitive fixture data'
        if error:
            assert result.returncode != 0, 'negative case unexpectedly accepted'
            assert error.encode() in result.stderr, result.stderr.decode(errors='replace')
            assert not result.stdout.strip(), 'failure leaked stdout payload'
            return None
        assert result.returncode == 0, result.stderr.decode(errors='replace')
        assert b'adl_event' in result.stderr
        return json.loads(result.stdout)

    common = ['--store', root / 'store', '--baselines', root / 'baselines']
    refs = {}
    for name in ('baseline', 'current', 'current-incompatible', 'current-partial'):
        refs[name] = cli('retain', *common, '--input', root / (name + '.json'))
        write(name + '-ref.json', refs[name])
    palace = root / 'palace'
    identity = json.loads((authority / 'authority-summary.json').read_text())
    index = dict(schema='codefriend.palace.v1', references=[refs['baseline']],
                 observed_epoch_ms=2000, stale_after_ms=1000, max_working_set_items=8)
    def commit(value, name, trust=None, evidence=None, error=None):
        return cli('palace-index', *common, '--palace', palace,
                   '--trust', trust or authority / 'trust.json',
                   '--authority', evidence or authority / 'authority-evidence.json',
                   '--input', write(name, value), error=error)
    first = commit(index, 'first-index.json')
    assert first['generation'] == 1 and first['backend'] == 'RuntimeMemoryPalaceService'
    scenarios.append('first_run_commits_production_runtime_service')
    request = dict(schema='codefriend.palace.v1', baseline=refs['baseline'], current=refs['current'],
                   expected_identity_root=identity['identity_root'],
                   expected_continuity_head=identity['continuity_head'],
                   packet_observed_epoch_ms=2000, observed_epoch_ms=2000,
                   stale_after_ms=1000, max_working_set_items=8)
    def retrieve(value, name, error=None):
        return cli('palace-compare', *common, '--palace', palace,
                   '--input', write(name + '-request.json', value), '--out', root / (name + '.json'), error=error)
    delta = retrieve(request, 'first-comparison')
    assert delta == retrieve(request, 'repeat-comparison')
    assert sorted(c['comparison']['outcome'] for c in delta['delta']['changes']) == ['added', 'changed', 'resolved', 'unchanged']
    scenarios.append('second_process_retrieval_and_deterministic_comparison')
    second = commit(dict(index, references=[refs['current'], refs['baseline']], observed_epoch_ms=2001), 'second-index.json')
    assert second['generation'] == 2 and second['packet_sha256'] != first['packet_sha256']
    request.update(packet_observed_epoch_ms=2001, observed_epoch_ms=2001)
    loaded = retrieve(request, 'second-comparison')
    assert len(loaded['selected_references']) == 2
    scenarios.append('second_commit_advances_generation_and_retains_bounded_references')
    packet_paths = sorted((palace / 'generations').glob('*/packet.json'))
    assert len(packet_paths) == 2
    commit(dict(index, references=[dict(refs['baseline'], payload='PRIVATE-PALACE-CANARY')]),
           'raw-payload-index.json', error='invalid_memory_artifact')
    assert sorted((palace / 'generations').glob('*/packet.json')) == packet_paths
    scenarios.append('raw_payload_injection_denied_before_commit')
    for name in ('current-incompatible', 'current-partial'):
        report = retrieve(dict(request, current=refs[name]), name + '-comparison')
        assert not report['delta']['comparable']
        assert all(c['comparison']['outcome'] == 'not_comparable' for c in report['delta']['changes'])
        scenarios.append(name + '_does_not_claim_resolution')
    retrieve(dict(request, expected_identity_root='b' * 64), 'wrong-identity', error='palace_identity_mismatch')
    scenarios.append('wrong_identity_denied')
    retrieve(dict(request, expected_continuity_head='b' * 64), 'wrong-continuity', error='continuity')
    scenarios.append('wrong_continuity_denied')
    retrieve(dict(request, observed_epoch_ms=3002), 'stale', error='palace_stale_observation')
    scenarios.append('stale_observation_denied')
    altered = dict(request, baseline=dict(refs['baseline'], run_id='b' * 64))
    retrieve(altered, 'wrong-run', error='palace_selected_baseline_missing')
    scenarios.append('wrong_run_reference_denied')
    trust = json.loads((authority / 'trust.json').read_text())
    trust['identity_public_key'] = trust['continuity_public_key']
    commit(index, 'untrusted-index.json', trust=write('untrusted.json', trust), error='authority keys must be distinct')
    scenarios.append('operator_key_pin_failure_denied')
    trust = json.loads((authority / 'trust.json').read_text())
    trust['identity_public_key'], trust['private_public_key'] = trust['private_public_key'], trust['identity_public_key']
    commit(index, 'substituted-trust-index.json', trust=write('substituted-trust.json', trust), error='Runtime Memory Palace authority rejected')
    scenarios.append('distinct_but_wrong_operator_key_pin_denied')
    evidence = json.loads((authority / 'authority-evidence.json').read_text())
    evidence['continuity_manifests'][0]['signature'] = '00' * 64
    commit(index, 'bad-signature-index.json', evidence=write('bad-signature.json', evidence), error='Runtime Memory Palace authority rejected')
    scenarios.append('invalid_signed_continuity_denied')
    evidence = json.loads((authority / 'authority-evidence.json').read_text())
    evidence['private_record']['sealed_payload_hash'] = '00' * 32
    commit(index, 'bad-private-index.json', evidence=write('bad-private.json', evidence), error='Runtime Memory Palace authority rejected')
    scenarios.append('tampered_private_payload_commitment_denied')
    latest = palace / 'latest.json'
    saved = latest.read_bytes()
    latest.unlink()
    retrieve(request, 'missing-latest', error='Runtime Memory Palace')
    assert not latest.exists(), 'strict consumer must not repair missing pointer'
    latest.write_bytes(saved)
    scenarios.append('missing_latest_denied_without_repair')
    latest.write_text('{')
    retrieve(request, 'corrupt-latest', error='Runtime Memory Palace')
    latest.write_bytes(saved)
    scenarios.append('corrupt_latest_denied')
    malformed = json.loads(saved)
    malformed['generation'] = 'PRIVATE-PALACE-CANARY'
    latest.write_text(json.dumps(malformed))
    retrieve(request, 'redacted-corrupt-latest', error='Runtime Memory Palace strict validation denied')
    latest.write_bytes(saved)
    scenarios.append('malformed_generation_error_redacts_private_canary')
    assert retrieve(request, 'restored') == loaded
    scenarios.append('restored_original_service_state_replays')
    packet_path = packet_paths[-1]
    packet_bytes = packet_path.read_bytes()
    packet = json.loads(packet_bytes)
    assert packet['working_set']
    packet['working_set'][0]['citations'][0]['sha256'] = '00' * 32
    packet_path.write_text(json.dumps(packet))
    retrieve(request, 'tampered-citation', error='Runtime Memory Palace')
    packet_path.write_bytes(packet_bytes)
    scenarios.append('tampered_durable_citation_denied')
    original_packet = json.loads(packet_bytes)
    trace_path = palace / original_packet['trace_reference']['path']
    trace_bytes = trace_path.read_bytes()
    assert hashlib.sha256(trace_bytes).hexdigest() == original_packet['trace_reference']['sha256']
    trace_path.write_bytes(b'[]')
    retrieve(request, 'tampered-trace', error='palace_trace_digest_mismatch')
    trace_path.write_bytes(trace_bytes)
    scenarios.append('tampered_durable_trace_denied')
    trace_path.unlink()
    retrieve(request, 'missing-trace', error='palace_trace_unavailable')
    assert not trace_path.exists()
    trace_path.write_bytes(trace_bytes)
    scenarios.append('missing_durable_trace_denied_without_repair')
    cli('delete', *common, '--reference', root / 'baseline-ref.json')
    retrieve(request, 'deleted-baseline', error='baseline')
    scenarios.append('deleted_admission_cannot_be_resurrected_by_memory')
    assert len(packet_paths) == 2
    for packet in packet_paths:
        value = json.loads(packet.read_text())
        assert value['working_set']
        policy_bytes = json.dumps(['codefriend.palace.v1', 'digest-references-only;live-admission-required'],
                                  separators=(',', ':')).encode()
        assert value['redaction_policy_sha256'] == hashlib.sha256(policy_bytes).hexdigest()
        for record in value['working_set']:
            payload = json.loads(record['payload'])
            assert set(payload) == {'run_id', 'packet_id', 'record_digest'}
            serialized = json.dumps(payload, separators=(',', ':'), ensure_ascii=False).encode()
            expected = hashlib.sha256(serialized).hexdigest()
            assert any(c['sha256'] == expected and c['id'] == 'baseline:' + payload['record_digest']
                       for c in record['citations'])
    scenarios.append('durable_memory_contains_only_shared_digest_references')
    status = subprocess.run(['git', '-C', str(root / 'source'), 'status', '--porcelain'], capture_output=True, check=True)
    assert not status.stdout
    proof = dict(schema='codefriend.palace.installed-proof.v1', issue=889,
                 binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                 installation_provenance=provenance.read_text().strip(), scenarios_passed=len(scenarios),
                 scenarios=scenarios, source_unchanged=True, stdout_json_stderr_events=True,
                 authority_route='LiveAssembly::provision_memory_palace_authority',
                 nonclaims=['Public fixture signing seeds are not production trust',
                            'No running Runtime kernel, network, provider, or production citizen admission claimed'])
    write('palace-installed-proof.json', proof)
    print(json.dumps(proof, indent=2))

if __name__ == '__main__':
    main()
