#!/usr/bin/env python3
"""PVF local_contract: public session-ledger persistence and refusal boundaries.

Required public CLI lane; bounded temporary disk and subprocesses, no network or
provider. These synthetic occupancy records do not confer lifecycle authority.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, required=True)
    binary = parser.parse_args().binary.resolve(strict=True)
    environment = {k: v for k, v in os.environ.items() if not k.startswith('ADL_')}
    environment['ADL_OBSERVABILITY'] = '0'
    count = 0
    with tempfile.TemporaryDirectory(prefix='adl-public-session-') as directory:
        root = Path(directory)
        ledger = root / 'isolated' / 'ledger.json'

        def invoke(args, *, failure=None, structured=True):
            nonlocal count
            command = [str(binary), 'session', *args, '--ledger', str(ledger)]
            if structured:
                command.append('--json')
            result = subprocess.run(command, cwd=root, env=environment,
                                    capture_output=True, text=True, timeout=10)
            count += 1
            if failure is not None:
                assert result.returncode == 1 and not result.stdout, result
                assert failure in result.stderr, result
                return None
            assert result.returncode == 0 and not result.stderr, result
            return json.loads(result.stdout) if structured else result.stdout

        assert invoke(['status'])['active_claims'] == 0
        assert not ledger.exists(), 'read-only status must not materialize a ledger'
        for command in [[], ['status'], ['claim'], ['heartbeat'], ['release']]:
            assert 'Usage:' in invoke([*command, '--help'], structured=False)
            assert not ledger.exists(), 'help must not materialize a ledger'
        claim_args = ['claim', '--session-id', 'fixture-session', '--owner', 'fixture',
                      '--resource', 'test:portable', '--purpose', 'isolated contract',
                      '--ttl-secs', '3600']
        claim = invoke(claim_args)
        identity = claim['claim_id']
        assert claim['resource'] == {'kind': 'test', 'id': 'portable'}
        assert claim['mode'] == 'active'
        assert invoke(['status'])['active_claims'] == 1
        assert identity in invoke(['status'], structured=False)
        saved = ledger.read_bytes()
        invoke([*claim_args, '--unknown'], failure='unknown session claim option')
        assert ledger.read_bytes() == saved
        invoke([*claim_args, '--owner', ''], failure='owner cannot be empty')
        assert ledger.read_bytes() == saved
        invoke([*claim_args, '--ttl-secs', '0'], failure='ttl seconds must be greater than zero')
        assert ledger.read_bytes() == saved
        for ttl in ['9223372036854775807', '9223372036854775']:
            invoke([*claim_args, '--ttl-secs', ttl], failure='ttl seconds')
            assert ledger.read_bytes() == saved
        invoke(claim_args, failure='already has active claim')
        assert ledger.read_bytes() == saved
        invoke(['heartbeat', '--claim-id', identity, '--ttl-secs', '0'],
               failure='ttl seconds must be greater than zero')
        assert ledger.read_bytes() == saved
        for ttl in ['9223372036854775807', '9223372036854775']:
            invoke(['heartbeat', '--claim-id', identity, '--ttl-secs', ttl],
                   failure='ttl seconds')
            assert ledger.read_bytes() == saved
        heartbeat = invoke(['heartbeat', '--claim-id', identity, '--ttl-secs', '7200'])
        assert heartbeat['claim_id'] == identity and heartbeat['mode'] == 'active'
        assert heartbeat['expires_at'] > claim['expires_at']
        assert invoke(['heartbeat', '--claim-id', identity], structured=False).startswith('heartbeat ')
        released = invoke(['release', '--claim-id', identity, '--reason', 'fixture complete'])
        assert released['mode'] == 'released'
        assert released['release_reason'] == 'fixture complete'
        saved = ledger.read_bytes()
        invoke(['heartbeat', '--claim-id', identity], failure='already released')
        assert ledger.read_bytes() == saved
        invoke(['release', '--claim-id', 'absent'], failure='not found')
        assert ledger.read_bytes() == saved
        status = invoke(['status'])
        assert status['active_claims'] == 0 and status['released_claims'] == 1
        assert invoke(claim_args, structured=False).startswith('claimed ')
        active = next(item for item in invoke(['status'])['claims']
                      if item['classification'] == 'active')
        assert invoke(['release', '--claim-id', active['claim_id']],
                      structured=False).startswith('released ')
        assert not ledger.with_name('ledger.json.lock').exists()
        assert not ledger.with_suffix('.json.tmp').exists()
        malformed = json.loads(ledger.read_text())
        malformed['schema'] = 'unsupported-fixture-schema'
        ledger.write_text(json.dumps(malformed))
        saved = ledger.read_bytes()
        invoke(['status'], failure='unsupported session ledger schema')
        assert ledger.read_bytes() == saved
        ledger.write_text('{invalid')
        invoke(['status'], failure='parse session ledger')
        assert ledger.read_text() == '{invalid'
        ledger.unlink()
        ledger.mkdir()
        invoke(['status'], failure='read session ledger')
        assert ledger.is_dir()
    print(json.dumps({'status': 'passed', 'commands': count,
                      'scope': 'isolated occupancy contract; no lifecycle authority'}))


if __name__ == '__main__':
    main()
