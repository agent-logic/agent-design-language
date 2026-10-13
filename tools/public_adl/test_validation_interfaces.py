#!/usr/bin/env python3
"""PVF local_contract: public validation interfaces; deterministic Git/filesystem
fixtures, small CPU/disk, no Cargo/provider/network; required public routing gate.
Original monorepo lane membership is an explicitly scoped retirement decision,
not a claim that extracted owner product obligations have passed.
"""
import contextlib
import importlib.util
import io
import json
import os
from pathlib import Path
import shlex
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
TOOLS = ('select_validation_lanes', 'validation_manager', 'validation_inventory')
CONFIGS = ('validation_lane_selector.public.v1.json', 'slow_proof_families.public.v1.json')
PACKAGES = ('adl', 'adl-schema', 'adl-uts', 'adl-v2/crates/adl-language',
            'adl-v2/crates/adl-compiler', 'adl-legacy-contracts')


class PublicValidationInterfaces(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='public-validation-')
        self.root = Path(self.temp.name)
        for name in TOOLS:
            for suffix in ('.py', '.sh'):
                self.copy(Path('adl/tools') / (name + suffix))
        for name in CONFIGS:
            self.copy(Path('adl/config') / name)
        for package in PACKAGES:
            source = self.root / package / 'src/lib.rs'
            source.parent.mkdir(parents=True, exist_ok=True)
            source.write_text('#[test]\nfn passes() {}\n')
        (self.root / 'adl/Cargo.toml').write_text('[package]\nname="adl"\nversion="0.1.0"\n')
        (self.root / 'docs').mkdir()
        (self.root / 'docs/check.md').write_text('clean\n')
        (self.root / 'old.rs').write_text('old\n')
        self.git('init', '-q')
        self.git('add', '.')
        self.git('-c', 'user.name=Public fixture', '-c', 'user.email=fixture@example.invalid',
                 '-c', 'commit.gpgsign=false', '-c', 'core.hooksPath=/dev/null',
                 'commit', '-qm', 'fixture')
        self.changed = self.root / 'changed.txt'
        self.changed.write_text('adl/src/main.rs\n')
        self.manifest_path = self.root / 'adl/config' / CONFIGS[0]
        spec = importlib.util.spec_from_file_location('public_selector', self.root / 'adl/tools/select_validation_lanes.py')
        self.selector = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(self.selector)

    def tearDown(self):
        self.temp.cleanup()

    def copy(self, rel):
        dest = self.root / rel
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(ROOT / rel, dest)

    def git(self, *args):
        return subprocess.check_output(['git', '-C', str(self.root), *args], stderr=subprocess.PIPE).decode().strip()

    def tool(self, name, *args):
        return subprocess.run(['bash', str(self.root / 'adl/tools' / (name + '.sh')), *args],
                              cwd=self.root, capture_output=True, text=True,
                              env=dict(os.environ, PYTHONDONTWRITEBYTECODE='1'))

    def profile(self, *extra):
        return self.tool('validation_manager', '--changed-files', str(self.changed), '--json', *extra)

    def manifest(self):
        return json.loads(self.manifest_path.read_text())

    def save(self, value):
        self.manifest_path.write_text(json.dumps(value))

    def test_public_source_and_docs_use_real_entrypoints(self):
        result = self.tool('select_validation_lanes', '--changed-files', str(self.changed), '--json')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(set(json.loads(result.stdout)['lanes']), {'public_source'})
        result = self.profile()
        self.assertEqual(result.returncode, 0, result.stderr)
        profile = json.loads(result.stdout)
        self.assertEqual(profile['status'], 'ready_to_run')
        self.assertEqual(profile['run'][0]['command'], 'bash adl/tools/batched_checks.sh')
        self.assertEqual({row['surface'] for row in profile['not_run']}, {'coverage', 'installed_public', 'owner_contracts'})
        self.changed.write_text('docs/check.md\n')
        result = self.profile('--run')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(json.loads(result.stdout)['run_status'], 'passed')

    def test_workspace_only_changes_select_public_rust_proof(self):
        for path in ('adl-v2/Cargo.toml', 'adl-v2/Cargo.lock'):
            with self.subTest(path=path):
                self.changed.write_text(path + '\n')
                result = self.profile()
                self.assertEqual(result.returncode, 0, result.stderr)
                profile = json.loads(result.stdout)
                self.assertEqual(profile['status'], 'ready_to_run')
                self.assertEqual([row['command'] for row in profile['run']],
                                 ['bash adl/tools/batched_checks.sh'])

    def test_unknown_and_private_paths_refuse_execution(self):
        for paths in ['unmapped.bin\n', 'adl/src/runtime_v2/mod.rs\n', 'adl/src/main.rs\nunmapped.bin\n']:
            with self.subTest(paths=paths):
                self.changed.write_text(paths)
                result = self.profile('--run')
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(json.loads(result.stdout)['status'], 'escalation_required')

    def test_failure_and_report_remain_failure(self):
        manifest = self.manifest()
        manifest['special_surfaces']['rust_pr_fast']['run_command'] = 'exit 7'
        self.save(manifest)
        report = self.root / 'report.json'
        result = self.profile('--run', '--report-out', str(report), '--build-action-log-dir', str(self.root / 'logs'))
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertEqual(json.loads(result.stdout)['run_status'], 'failed')
        self.assertEqual(json.loads(report.read_text())['run_status'], 'failed')
        manifest['special_surfaces']['rust_pr_fast']['run_command'] = "printf '%s\\n' child-output"
        self.save(manifest)
        result = self.tool('select_validation_lanes', '--changed-files', str(self.changed), '--json', '--run')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(json.loads(result.stdout)['run_status'], 'passed')
        self.assertIn('child-output', result.stderr)

    def test_manifest_missing_metadata_and_invalid_vpp_refused(self):
        baseline = self.manifest()
        invalid = []
        for key in ('surface_defaults', 'lanes', 'release_gate_hints', 'rust_path_hints'):
            value = json.loads(json.dumps(baseline)); value.pop(key); invalid.append(value)
        for key in ('owner', 'resource_class', 'determinism_posture', 'proof_role', 'risk_class', 'escalation_rule'):
            value = json.loads(json.dumps(baseline)); value['surface_defaults']['public'][key] = ''; invalid.append(value)
        for record in ({}, {'contract_version': 'v1', 'artifacts': []}):
            value = json.loads(json.dumps(baseline)); value['lanes'][0]['vpp_record'] = record; invalid.append(value)
        value = json.loads(json.dumps(baseline)); value['special_surfaces']['rust_pr_fast']['planner'] = 'unknown'; invalid.append(value)
        for index, value in enumerate(invalid):
            with self.subTest(index=index), contextlib.redirect_stderr(io.StringIO()):
                self.save(value)
                with self.assertRaises(SystemExit):
                    self.selector.load_manifest(self.manifest_path)

    def test_valid_metadata_vpp_and_special_surface_preserved(self):
        manifest = self.manifest()
        vpp = {'contract_version': 'v1', 'artifacts': ['report.json'], 'expected_runtime_class': 'small',
               'parallel_group': 'public', 'cache_equivalence_group': 'public', 'failure_semantics': 'fail_closed'}
        manifest['special_surfaces']['rust_pr_fast']['vpp_record'] = vpp
        self.save(manifest)
        loaded = self.selector.load_manifest(self.manifest_path)
        plan = self.selector.select_lanes(self.manifest_path, loaded, ['adl/src/main.rs'], self.changed, 'HEAD', 'HEAD')
        self.assertEqual(plan['lanes']['public_source']['vpp_record'], vpp)
        self.assertEqual(plan['lanes']['public_source']['owner'], 'adl')

    def test_git_deletion_rename_and_untracked_observed(self):
        self.git('mv', 'old.rs', 'renamed.rs')
        (self.root / 'adl/src/lib.rs').unlink()
        (self.root / 'docs/untracked.md').write_text('new\n')
        paths = self.selector.changed_paths_from_git('HEAD', 'HEAD', True)
        self.assertTrue({'old.rs', 'renamed.rs', 'adl/src/lib.rs', 'docs/untracked.md'} <= set(paths))
        with self.assertRaises(SystemExit), contextlib.redirect_stderr(io.StringIO()):
            self.selector.changed_paths_from_git('not-a-revision', 'HEAD', False)

    def test_public_fixture_inputs_are_selected(self):
        # Literal include_str inputs of the retained resolve/schema integration
        # suites; nearby private fixtures must remain outside this public scope.
        paths = ['adl/tests/fixtures/cli_smoke.adl.yaml', 'adl/examples/adl-0.1.yaml', 'adl/examples/v0-2-multi-step-basic.adl.yaml', 'adl/examples/v0-2-multi-step-file-input.adl.yaml', 'adl/examples/v0-2-coordinator-agents-sdk.adl.yaml']
        for path in paths:
            with self.subTest(path=path):
                self.changed.write_text(path + '\n')
                result = self.profile()
                self.assertEqual(result.returncode, 0, result.stderr)
                profile = json.loads(result.stdout)
                self.assertEqual(profile['status'], 'ready_to_run')
                self.assertEqual(profile['run'][0]['lane_id'], 'public_source')
        self.changed.write_text('adl/tests/fixtures/private_runtime.adl.yaml\n')
        self.assertEqual(json.loads(self.profile().stdout)['status'], 'escalation_required')

    def test_six_package_inventory_and_required_input_refusal(self):
        result = self.tool('validation_inventory', '--format', 'json')
        self.assertEqual(result.returncode, 0, result.stderr)
        inventory = json.loads(result.stdout)
        self.assertEqual(inventory['rust']['rust_lib_unit_tests']['test_entry_count'], 6)
        self.assertEqual(inventory['rust']['owner_summary'], {'adl': 6})
        self.assertEqual(inventory['slow_proof_family_inventory'], [])
        missing = str(self.root / 'missing.json')
        self.assertNotEqual(self.tool('validation_inventory', '--slow-proof-families', missing, '--format', 'json').returncode, 0)
        self.assertNotEqual(self.profile('--slow-proof-families', missing).returncode, 0)

    def test_historical_dispositions_cannot_bypass_public_escalation(self):
        spec = importlib.util.spec_from_file_location('public_manager', self.root / 'adl/tools/validation_manager.py')
        manager = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(manager)
        guardrails = manager.manager_guardrails(self.manifest(), 8)
        plans = [
            {'release_gate_review': {'status': 'release_gate_required'}},
            {'rust_pr_fast': {'status': 'escalated', 'mode': 'contract_only',
                             'reason': 'slow_proof_inventory_change_covered_by_contract_check'},
             'slow_proof_review': {'status': 'release_gate_required'}},
        ]
        for lanes in plans:
            with self.subTest(lanes=lanes):
                paths = ['.github/workflows/ci.yaml', 'adl/tools/test_ci_runtime_contracts.sh']
                for lane in lanes.values():
                    lane['matched_paths'] = paths
                plan = {'lanes': lanes, 'changed_paths': paths, 'aggregate_status': 'escalated',
                        'pr_publication_sufficient': False}
                profile = manager.build_profile(plan, guardrails, self.manifest_path)
                self.assertEqual(profile['status'], 'escalation_required')
                self.assertFalse(profile['pr_publication_sufficient'])
                self.assertEqual(profile['run'], [])
        # The fixture deliberately contains no private source, old workflows or
        # monorepo contract scripts: escalation must not try to read them.
        historical = self.root / 'historical-families.json'
        historical.write_text(json.dumps({'schema_version': 'adl.slow_proof_families.v1',
                                          'families': [{'id': 'private-runtime'}]}))
        result = self.profile('--slow-proof-families', str(historical))
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('retired private slow-proof families', result.stderr)

    def test_real_shell_and_javascript_syntax_failures_propagate(self):
        self.assertIsNotNone(shutil.which('node'), 'Node required for preserved JavaScript syntax contract')
        for program, name, valid, invalid in [('bash -n', 'syntax.sh', 'true\n', 'if then\n'),
                                               ('node --check', 'syntax.mjs', 'export const valid = true;\n', 'const = ;\n')]:
            with self.subTest(program=program):
                fixture = self.root / name
                manifest = self.manifest()
                manifest['special_surfaces']['rust_pr_fast']['run_command'] = program + ' ' + shlex.quote(str(fixture))
                self.save(manifest)
                fixture.write_text(valid)
                self.assertEqual(self.profile('--run').returncode, 0)
                fixture.write_text(invalid)
                self.assertNotEqual(self.profile('--run').returncode, 0)

    def test_aws_spot_requires_explicit_executable_owner_runner(self):
        spec = importlib.util.spec_from_file_location('public_manager_owner', self.root / 'adl/tools/validation_manager.py')
        manager = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(manager)
        eligibility = dict(command='echo public', status='ready_to_run', escalation_required=False,
                           runtime_class='medium', deterministic_or_evidence_bound=True)
        runner = self.root / 'owner runner'
        runner.write_text('#!/bin/sh\nexit 99\n')
        for candidate in (None, 'relative/runner', str(self.root / 'missing'), str(self.root), str(runner)):
            result = manager.aws_spot_platform_candidate({}, eligibility, candidate)
            self.assertEqual(result['decision'], 'rejected')
            self.assertFalse(result.get('command'))
        runner.chmod(0o755)
        result = manager.aws_spot_platform_candidate({}, eligibility, str(runner))
        self.assertEqual(result['decision'], 'eligible')
        self.assertEqual(shlex.split(result['command'])[0], str(runner))
        self.assertIn('--print-command', shlex.split(result['command']))
        eligibility['escalation_required'] = True
        self.assertEqual(manager.aws_spot_platform_candidate({}, eligibility, str(runner))['decision'], 'rejected')


if __name__ == '__main__':
    unittest.main()
