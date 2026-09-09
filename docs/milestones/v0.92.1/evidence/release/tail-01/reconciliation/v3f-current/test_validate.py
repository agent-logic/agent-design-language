#!/usr/bin/env python3
"""PVF: required deterministic small local/Git validator contract tests.

All review and suite receipts here are explicitly synthetic fixtures; these tests
are not substantive V3-F review, suite execution, or release proof.
"""
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location('v3f_validator', Path(__file__).with_name('validate.py'))
V = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(V)


class MappingContract(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='v3f-mapping-contract-')
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.original_root, self.original_here = V.ROOT, V.HERE
        self.addCleanup(self.restore_globals)
        V.ROOT, V.HERE = self.root, self.root / 'packet'
        V.HERE.mkdir()
        self.git('init', '-q')
        self.git('config', 'user.name', 'Synthetic validator fixture')
        self.git('config', 'user.email', 'fixture@example.invalid')
        self.fixture = self.root / 'csdlc-v3/tests/terminal_cleanup_cutover_commands.rs'
        self.fixture.parent.mkdir(parents=True)
        self.fixture.write_text('// synthetic source fixture\n')
        terminal = self.root / 'csdlc-v3/src/commands/terminal.rs'
        terminal.parent.mkdir(parents=True)
        terminal.write_text('// synthetic implementation fixture\n')
        criteria = [dict(id=c, text_digest=V.digest(c.encode())) for c in sorted(V.CRITERIA)]
        corp = [dict(row_id=f'CORP-A:CORP-A-ac-{i}', reconciliation_class='review_freshness_resolved')
                for i in range(1,4)]
        self.write(self.root / 'current-exceptions.json', dict(post_review_505=dict(criteria=criteria)))
        self.write(self.root / 'census.json', dict(rows=corp))
        self.git('add', '.')
        self.git('commit', '-qm', 'synthetic source')
        self.source = self.git('rev-parse', 'HEAD')
        scope = V.scope_at(self.source)
        a = dict(source_sha=self.source, scope_prefixes=V.PREFIXES, scope=scope,
                 reviewer='independent-fixture', assigned_at='2026-01-01T00:00:00Z')
        r = dict(source_sha=self.source, scope=scope, reviewer=a['reviewer'], independent=True,
                 completed_at='2026-01-01T00:01:00Z', result='pass', unreviewed_paths=[], findings=[],
                 criteria={c:dict(result='pass', rationale='Synthetic semantic fixture') for c in V.CRITERIA})
        log = b'test result: ok. 188 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out\n'
        (V.HERE / 'suite.log').write_bytes(log)
        s = dict(source_sha=self.source, argv=V.SUITE_ARGV, detached=True, clean_before=True,
                 clean_after=True, external_target=True, head_after=self.source, log='suite.log',
                 log_sha256=V.digest(log), exit_code=0, failed=0, passed=188)
        criteria = [dict(id=c, text_digest=V.digest(c.encode())) for c in sorted(V.CRITERIA)]
        corp = [dict(row_id=f'CORP-A:CORP-A-ac-{i}', reconciliation_class='review_freshness_resolved')
                for i in range(1,4)]
        self.write(self.root / 'current-exceptions.json', dict(post_review_505=dict(criteria=criteria)))
        self.write(self.root / 'census.json', dict(rows=corp))
        for name, value in [('assignment.json', a), ('review.json', r), ('suite.json', s)]:
            self.write(V.HERE / name, value)
        (V.HERE / 'historical-fixture.diff').write_text('synthetic historical diff\n')
        self.mapping = dict(schema='adl.v3f.current_mapping.v1', source_sha=self.source,
            receipt_sha256={n:V.digest((V.HERE/n).read_bytes()) for n in
                ['assignment.json','review.json','suite.json','historical-fixture.diff']},
            historical_exceptions_sha256=V.digest((self.root/'current-exceptions.json').read_bytes()),
            historical_census_sha256=V.digest((self.root/'census.json').read_bytes()),
            rows=[dict(id=c['id'], criterion_text_digest=c['text_digest'], review='review.json',
                suite='suite.json', source_sha=self.source,
                fixture_blob=scope['csdlc-v3/tests/terminal_cleanup_cutover_commands.rs'],
                disposition='current_review_and_suite_proven') for c in criteria],
            preserved_corp_a=corp, release_authorized=False)

    def restore_globals(self):
        V.ROOT, V.HERE = self.original_root, self.original_here

    def git(self, *args):
        return subprocess.check_output(['git','-C',str(self.root),*args], stderr=subprocess.PIPE,
                                       text=True).strip()

    @staticmethod
    def write(path, value):
        path.write_text(json.dumps(value))

    def test_synthetic_baseline_and_all_substitution_cases(self):
        self.assertEqual(V.validate(self.mapping)['v3_f_rows'], 4)
        self.assertEqual(V.negative(self.mapping)['negative_cases'], 14)

    def test_uncommitted_tracked_source_is_rejected(self):
        self.fixture.write_text('// changed source\n')
        with self.assertRaisesRegex(ValueError, '^current_source_dirty$'):
            V.validate(self.mapping)

    def test_staged_source_is_rejected(self):
        self.fixture.write_text('// staged change\n')
        self.git('add', str(self.fixture.relative_to(self.root)))
        with self.assertRaisesRegex(ValueError, '^current_source_dirty$'):
            V.validate(self.mapping)

    def test_untracked_source_addition_is_rejected(self):
        (self.fixture.parent / 'new.rs').write_text('// additional source\n')
        with self.assertRaisesRegex(ValueError, '^current_source_dirty$'):
            V.validate(self.mapping)

    def test_committed_source_change_is_rejected(self):
        self.fixture.write_text('// new source bytes\n')
        self.git('add', str(self.fixture.relative_to(self.root)))
        self.git('commit', '-qm', 'source changed after review')
        with self.assertRaisesRegex(ValueError, '^current_source_drift$'):
            V.validate(self.mapping)

    def test_evidence_only_descendant_preserves_source_proof(self):
        self.git('add', 'packet', 'census.json', 'current-exceptions.json')
        self.git('commit', '-qm', 'record synthetic evidence')
        self.assertEqual(V.validate(self.mapping)['source_sha'], self.source)

    def test_actual_log_corruption_is_rejected(self):
        with (V.HERE / 'suite.log').open('a') as f:
            f.write('tampered log\n')
        with self.assertRaisesRegex(ValueError, '^suite_log_digest$'):
            V.validate(self.mapping)

    def test_filtered_test_log_is_rejected_even_with_updated_digest(self):
        log = (V.HERE / 'suite.log').read_bytes().replace(b'0 filtered out', b'1 filtered out')
        (V.HERE / 'suite.log').write_bytes(log)
        suite = V.read('suite.json')
        suite['log_sha256'] = V.digest(log)
        with self.assertRaisesRegex(ValueError, '^unfiltered_suite$'):
            V.validate(self.mapping, suite=suite)

    def test_rewritten_corp_baseline_and_refreshed_hash_are_rejected(self):
        path = self.root / 'census.json'
        census = json.loads(path.read_text())
        census['rows'][0]['evidence'] = 'altered historical evidence'
        self.write(path, census)
        self.mapping['historical_census_sha256'] = V.digest(path.read_bytes())
        self.mapping['preserved_corp_a'] = census['rows']
        with self.assertRaisesRegex(ValueError, '^historical_census_immutable$'):
            V.validate(self.mapping)

    def test_rewritten_criterion_and_refreshed_hash_are_rejected(self):
        path = self.root / 'current-exceptions.json'
        baseline = json.loads(path.read_text())
        row = baseline['post_review_505']['criteria'][0]
        row['text_digest'] = '0' * 64
        self.write(path, baseline)
        self.mapping['historical_exceptions_sha256'] = V.digest(path.read_bytes())
        self.mapping['rows'][0]['criterion_text_digest'] = row['text_digest']
        self.git('add', 'current-exceptions.json')
        self.git('commit', '-qm', 'rewrite historical baseline')
        with self.assertRaisesRegex(ValueError, '^historical_immutable$'):
            V.validate(self.mapping)

    def test_staged_historical_change_with_restored_working_bytes_is_rejected(self):
        path = self.root / 'census.json'
        original = path.read_bytes()
        path.write_text('{"rows": []}')
        self.git('add', 'census.json')
        path.write_bytes(original)
        with self.assertRaisesRegex(ValueError, '^historical_index_dirty$'):
            V.validate(self.mapping)

    def test_committed_historical_change_with_restored_working_bytes_is_rejected(self):
        path = self.root / 'census.json'
        original = path.read_bytes()
        path.write_text('{"rows": []}')
        self.git('add', 'census.json')
        self.git('commit', '-qm', 'alter candidate baseline')
        path.write_bytes(original)
        with self.assertRaisesRegex(ValueError, '^historical_census_immutable$'):
            V.validate(self.mapping)

    def test_blocked_review_is_never_accepted(self):
        review = V.read('review.json')
        review['result'] = 'blocked'
        with self.assertRaisesRegex(ValueError, '^substantive_review$'):
            V.validate(self.mapping, review=review)


if __name__ == '__main__':
    unittest.main()
