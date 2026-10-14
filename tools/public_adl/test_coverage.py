#!/usr/bin/env python3
"""PVF local_contract: synthetic LLVM reports; deterministic, no compiler/network.
Required regression proof for public coverage denominator and threshold handling.
"""
import copy
from pathlib import Path
import unittest
import coverage as owner


class CoverageContract(unittest.TestCase):
    def setUp(self):
        self.root = Path('/synthetic/repo')
        self.inventory = {'packages': [{'name': 'a', 'sources': ['a/src/lib.rs', 'a/src/main.rs'], 'declaration_only': ['a/src/lib.rs']}]}
        self.reports = {'a': {'type': 'llvm.coverage.json.export', 'data': [{'files': [{'filename': '/synthetic/repo/a/src/main.rs', 'summary': {'lines': {'count': 100, 'covered': 90}}}]}]}}

    def check(self):
        return owner.check_rows(self.reports, self.inventory, self.root)

    def test_explicit_declaration_exception_and_integer_thresholds(self):
        rows, missing = self.check()
        self.assertEqual(missing, ['a/src/lib.rs'])
        self.assertEqual(owner.enforce(rows, 'full', set()), [])
        rows['a/src/main.rs']['covered'] = 89
        self.assertEqual(owner.enforce(rows, 'full', set()), ['public aggregate below 90% line floor'])
        self.assertEqual(owner.enforce(rows, 'pull_request', {'a/src/main.rs'}), [])

    def test_pr_changed_file_floor_and_full_existing_file_floor(self):
        rows, _ = self.check()
        rows['a/src/main.rs']['covered'] = 79
        self.assertTrue(owner.enforce(rows, 'pull_request', {'a/src/main.rs'}))
        self.assertEqual(owner.enforce(rows, 'pull_request', set()), [])
        self.assertEqual(len(owner.enforce(rows, 'full', set())), 2)

    def test_missing_measurement_is_not_a_pass(self):
        self.reports['a']['data'][0]['files'] = []
        with self.assertRaisesRegex(ValueError, 'missing production'):
            self.check()

    def test_test_like_source_requires_explicit_inventory_exclusion(self):
        files = self.reports['a']['data'][0]['files']
        for path in ('a/src/tests.rs', 'a/src/tests/production.rs', 'a/tests/integration.rs'):
            with self.subTest(path=path):
                extra = copy.deepcopy(files[0])
                extra['filename'] = str(self.root / path)
                extra['summary']['lines']['covered'] = 0
                files.append(extra)
                with self.assertRaisesRegex(ValueError, 'unexpected repository source'):
                    self.check()
                self.inventory['packages'][0]['test_sources'] = [path]
                rows, _ = self.check()
                self.assertNotIn(path, rows)
                self.inventory['packages'][0]['test_sources'] = []
                files.pop()

    def test_test_exclusion_cannot_hide_owned_production_source(self):
        self.inventory['packages'][0]['test_sources'] = ['a/src/main.rs']
        with self.assertRaisesRegex(ValueError, 'overlaps production'):
            self.check()

    def test_missing_package_and_malformed_report(self):
        for report in [{}, {'a': {}}, {'a': {'type': 'llvm.coverage.json.export', 'data': []}}]:
            with self.subTest(report=report), self.assertRaises(ValueError):
                owner.check_rows(report, self.inventory, self.root)

    def test_duplicate_source_owner_and_duplicate_measurement(self):
        self.inventory['packages'].append(copy.deepcopy(self.inventory['packages'][0]))
        with self.assertRaisesRegex(ValueError, 'duplicate source'):
            self.check()
        self.inventory['packages'].pop()
        files = self.reports['a']['data'][0]['files']
        files.append(copy.deepcopy(files[0]))
        with self.assertRaisesRegex(ValueError, 'duplicate measured'):
            self.check()

    def test_private_source_refused_and_dependency_not_double_counted(self):
        files = self.reports['a']['data'][0]['files']
        extra = copy.deepcopy(files[0]); extra['filename'] = '/synthetic/repo/private/src/lib.rs'
        files.append(extra)
        with self.assertRaisesRegex(ValueError, 'unexpected repository source'):
            self.check()
        files.pop()
        for retired in ('/synthetic/repo/adl/src/codefriend/review/runner.rs',
                        '/synthetic/repo/adl-runtime/src/runtime_api_auth.rs'):
            extra['filename'] = retired
            files.append(extra)
            with self.assertRaisesRegex(ValueError, 'unexpected repository source'):
                self.check()
            files.pop()
        self.inventory['packages'].append({'name': 'b', 'sources': ['b/src/lib.rs']})
        self.reports['b'] = copy.deepcopy(self.reports['a'])
        extra['filename'] = '/synthetic/repo/b/src/lib.rs'
        files.append(extra)
        self.reports['b']['data'][0]['files'].append(copy.deepcopy(extra))
        rows, _ = self.check()
        self.assertEqual(sum(r['count'] for r in rows.values()), 200)

    def test_zero_executable_file_cannot_hide_beside_measured_file(self):
        files = self.reports['a']['data'][0]['files']
        zero = copy.deepcopy(files[0])
        zero['filename'] = '/synthetic/repo/a/src/new.rs'
        zero['summary']['lines'] = {'count': 0, 'covered': 0}
        files.append(zero)
        self.inventory['packages'][0]['sources'].append('a/src/new.rs')
        with self.assertRaisesRegex(ValueError, 'zero production'):
            self.check()
        self.inventory['packages'][0]['declaration_only'].append('a/src/new.rs')
        rows, _ = self.check()
        self.assertEqual(rows['a/src/main.rs']['count'], 100)

    def test_invalid_counts_and_empty_denominator(self):
        for count, covered in [(1, 2), (1, -1), (True, 1), (100, 90.0)]:
            self.reports['a']['data'][0]['files'][0]['summary']['lines'] = {'count': count, 'covered': covered}
            with self.subTest(count=count, covered=covered), self.assertRaises(ValueError):
                self.check()
        with self.assertRaisesRegex(ValueError, 'empty coverage'):
            owner.enforce({}, 'pull_request', set())


if __name__ == '__main__':
    unittest.main()
