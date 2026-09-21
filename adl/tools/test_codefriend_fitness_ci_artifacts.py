#!/usr/bin/env python3
"""Transport-only fixtures; native recomputation is covered by Rust CLI tests."""
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

TOOL = Path(__file__).with_name('codefriend_fitness_ci_artifacts.py')

class TransportContract(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.out = Path(self.temp.name).resolve()
        self.pins = dict(candidate='a' * 40, packet_id='b' * 64, policy_digest='c' * 64)
        self.write('expectations.json', self.pins)

    def write(self, name, value):
        (self.out / name).write_text(json.dumps(value))

    def fixture(self, version=2, status='unknown', exit_code=2):
        self.report = dict(schema=f'codefriend.fitness.v{version}', digest='d' * 64,
                           policy_digest=self.pins['policy_digest'], status=status,
                           record=dict(run=dict(revision=self.pins['candidate'], packet_id=self.pins['packet_id'])))
        self.receipt = dict(schema=f'codefriend.fitness.ci.v{version}', original_exit=exit_code,
                            exit_code=exit_code, artifact_valid=True, assessment=status,
                            report_digest=self.report['digest'], error=None, **self.pins)
        self.write('report.json', self.report)
        self.write('receipt.json', self.receipt)
        (self.out / 'runner-exit.txt').write_text(str(exit_code))

    def check(self, status):
        args = [sys.executable, str(TOOL), 'after', str(status), '--store', str(self.out / 'store'),
                '--packet-id', self.pins['packet_id'], '--policy', str(self.out / 'policy.json'),
                '--candidate', self.pins['candidate'], '--policy-digest', self.pins['policy_digest'], '--out', str(self.out)]
        return subprocess.run(args, capture_output=True, check=False).returncode

    def test_v2_unknown_never_becomes_success(self):
        self.fixture()
        self.assertEqual(self.check(2), 2)
        self.assertEqual(self.check(0), 2)

    def test_legacy_pass_and_v2_pass_keep_success(self):
        for version in (1, 2):
            self.fixture(version, 'pass', 0)
            self.assertEqual(self.check(0), 0)

    def test_cross_schema_and_unsupported_assessment_rejected(self):
        self.fixture(2, 'pass', 0)
        self.report['schema'] = 'codefriend.fitness.v1'
        self.write('report.json', self.report)
        self.assertEqual(self.check(0), 2)
        self.fixture(2, 'pass', 0)
        self.report['status'] = 'error'
        self.write('report.json', self.report)
        self.assertEqual(self.check(0), 2)

    def test_duplicate_schema_rejected(self):
        self.fixture(2, 'pass', 0)
        original = (self.out / 'report.json').read_text()
        (self.out / 'report.json').write_text('{"schema":"codefriend.fitness.v2",' + original[1:])
        self.assertEqual(self.check(0), 2)

    def test_raw_v2_envelope_bound(self):
        self.fixture(2, 'pass', 0)
        with (self.out / 'report.json').open('a') as f:
            f.write(' ' * (4 * 1024 * 1024))
        self.assertEqual(self.check(0), 2)

if __name__ == '__main__':
    unittest.main()
