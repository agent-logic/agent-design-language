"""PVF: deterministic local unit proof of live-probe verdicts; small, not a release gate."""
import errno
import contextlib
import io
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

SOURCE = Path(__file__).with_name('run_live_proof.sh').read_text().split("<<'PY'\n", 1)[1].rsplit('\nPY', 1)[0]


class LiveProofVerdicts(unittest.TestCase):
    def probe(self, scenario):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            identity = root / 'identity'
            identity.touch()
            argv = ['probe', '--instance-id', 'i-test', '--identity-file', str(identity),
                    '--expected-account', 'test-account', '--expected-name', 'issue-770-test',
                    '--key-name', 'existing', '--ssh-cidr', '203.0.113.10/32', '--evidence-dir', temp]
            instance = {'Tags': [{'Key': 'Name', 'Value': 'issue-770-test'}], 'KeyName': 'existing',
                        'State': {'Name': 'running'}, 'PublicIpAddress': '192.0.2.1',
                        'MetadataOptions': {'HttpTokens': 'required'}, 'SecurityGroups': [{'GroupId': 'sg-test'}],
                        'RootDeviceName': '/dev/xvda', 'BlockDeviceMappings': [{'DeviceName': '/dev/xvda',
                        'Ebs': {'VolumeId': 'vol-test', 'DeleteOnTermination': True}}]}
            responses = {'get-caller-identity': {'Account': 'test-account'},
                         'describe-instances': {'Reservations': [{'Instances': [instance]}]},
                         'describe-security-groups': {'SecurityGroups': [{'GroupId': 'sg-test', 'IpPermissions': [
                             {'IpProtocol': 'tcp', 'FromPort': 22, 'ToPort': 22,
                              'IpRanges': [{'CidrIp': '203.0.113.10/32'}]}]}]},
                         'describe-volumes': {'Volumes': [{'VolumeId': 'vol-test', 'Encrypted': True}]}}
            def run(args, **kwargs):
                if args[0] == 'aws':
                    return subprocess.CompletedProcess(args, 0, json.dumps(next(v for k, v in responses.items() if k in args)), '')
                command = args[-1]
                if command == 'printf ssh-recovery-ok': output = 'ssh-recovery-ok'
                elif 'SSH_CONNECTION' in command: output = '203.0.113.10 1234 192.0.2.1 22'
                elif 'subprocess.Popen' in command:
                    self.assertIn('0.0.0.0', command)
                    self.assertIn('unique-nonce', command)
                    output = '12345'
                elif 'urlopen' in command: output = 'unrelated-server' if scenario == 'collision' else 'unique-nonce'
                elif 'os.kill' in command:
                    self.assertIn('cmdline', command)
                    if scenario == 'cleanup_failure': return subprocess.CompletedProcess(args, 1, '', '')
                    output = 'listener-stopped'
                else: self.fail('unexpected SSH command')
                return subprocess.CompletedProcess(args, 0, output, '')
            connection = contextlib.nullcontext() if scenario == 'open_port' else None
            with patch.object(sys, 'argv', argv), patch.dict(os.environ, {'AWS_PROFILE': 'agent-logic-admin'}), \
                 patch('subprocess.run', side_effect=run), patch('secrets.token_hex', return_value='unique-nonce'), \
                 patch('time.sleep'), patch('socket.create_connection', return_value=connection,
                                           side_effect=(None if scenario == 'open_port' else OSError(errno.EMFILE, 'local descriptor exhaustion') if scenario == 'local_error' else TimeoutError)), \
                 contextlib.redirect_stdout(io.StringIO()):
                try:
                    exec(compile(SOURCE, 'run_live_proof.sh', 'exec'), {'__name__': '__main__'})
                    status = 0
                except SystemExit as error:
                    status = error.code
            return status, json.loads((root / 'recovery-proof.json').read_text())

    def test_success_requires_nonce_and_cleanup(self):
        status, record = self.probe('success')
        self.assertEqual(status, 0)
        self.assertTrue(record['temporary_listener_stopped'])

    def test_existing_listener_cannot_prove_isolation(self):
        status, record = self.probe('collision')
        self.assertEqual(status, 1)
        self.assertEqual(record['status'], 'failed')

    def test_cleanup_failure_fails_verdict(self):
        status, record = self.probe('cleanup_failure')
        self.assertEqual(status, 1)
        self.assertFalse(record['temporary_listener_stopped'])

    def test_local_socket_error_cannot_prove_isolation(self):
        status, record = self.probe('local_error')
        self.assertEqual(status, 1)
        self.assertEqual(record['status'], 'failed')

    def test_open_application_port_fails_verdict(self):
        status, record = self.probe('open_port')
        self.assertEqual(status, 1)
        self.assertEqual(record['status'], 'failed')


if __name__ == '__main__':
    unittest.main()
