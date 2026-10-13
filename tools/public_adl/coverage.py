#!/usr/bin/env python3
"""PVF deterministic coverage contract: bounded local instrumentation, no providers.

Produces public-only LLVM evidence. PRs retain the existing 80% changed-file
floor; full validation retains 90% aggregate and 80% per-file floors.
"""
import argparse
import hashlib
import json
import os
import shlex
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
CONFIG = Path(__file__).with_name('coverage_inventory.json')


def check_rows(reports, inventory, root):
    """Choose exactly one owner report per source file, without percentage averaging."""
    owners = {}
    declarations = set()
    for package in inventory['packages']:
        declarations.update(package.get('declaration_only', []))
        for path in package['sources']:
            if path in owners:
                raise ValueError('duplicate source ownership: ' + path)
            owners[path] = package['name']
    if set(reports) != {p['name'] for p in inventory['packages']}:
        raise ValueError('missing or unexpected package reports')
    rows = {}
    for package, report in reports.items():
        if report.get('type') != 'llvm.coverage.json.export' or not report.get('data'):
            raise ValueError('malformed LLVM report: ' + package)
        seen = set()
        for data in report['data']:
            for item in data['files']:
                path = Path(item['filename'])
                if not path.is_absolute():
                    path = root / path
                try:
                    relative = path.resolve().relative_to(root.resolve()).as_posix()
                except ValueError:
                    continue  # Rust standard library or registry sources.
                if relative in seen:
                    raise ValueError('duplicate measured file: ' + relative)
                seen.add(relative)
                if relative not in owners:
                    if '/tests/' in '/' + relative or relative.endswith('/tests.rs'):
                        continue
                    raise ValueError('unexpected repository source in coverage: ' + relative)
                if owners[relative] != package:
                    continue  # Measured dependency belongs to its own report.
                lines = item['summary']['lines']
                count, covered = lines['count'], lines['covered']
                if type(count) is not int or type(covered) is not int or not 0 <= covered <= count:
                    raise ValueError('invalid line counts: ' + relative)
                if count == 0 and relative not in declarations:
                    raise ValueError('zero production source denominator: ' + relative)
                rows[relative] = {'package': package, 'count': count, 'covered': covered}
    missing = set(owners) - set(rows) - declarations
    if missing:
        raise ValueError('missing production source measurements: ' + ', '.join(sorted(missing)))
    return rows, sorted(set(owners) - set(rows))


def enforce(rows, mode, changed):
    if not rows or sum(r['count'] for r in rows.values()) == 0:
        raise ValueError('empty coverage denominator')
    failures = []
    for path, row in rows.items():
        if row['count'] and (mode == 'full' or path in changed):
            if row['covered'] * 100 < row['count'] * 80:
                failures.append(path + ': below 80% line floor')
    if mode == 'full' and sum(r['covered'] for r in rows.values()) * 100 < sum(r['count'] for r in rows.values()) * 90:
        failures.append('public aggregate below 90% line floor')
    return failures


def run(output, mode, base):
    inventory = json.loads(CONFIG.read_text())
    output = output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    source_paths = sorted({path for package in inventory['packages'] for path in package['sources']})
    # Include tests, helpers, schemas, fixtures and workspace configuration.
    # Native issue metadata is operational state, not a compiler input.
    input_paths = ['.', ':(exclude).csdlc']
    def clean_inputs():
        if subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=all', '--', *input_paths], cwd=ROOT, text=True):
            raise ValueError('coverage checkout inputs must be committed and clean')
    clean_inputs()
    source_hashes = {path: hashlib.sha256((ROOT / path).read_bytes()).hexdigest() for path in source_paths}
    head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()
    changed = set()
    if mode == 'pull_request':
        if not base:
            raise ValueError('PR coverage requires an explicit base revision')
        changed = set(subprocess.check_output(['git', 'diff', '--name-only', '--diff-filter=ACMR', base + '...HEAD'], cwd=ROOT, text=True).splitlines())
    env = dict(os.environ, CARGO_BUILD_JOBS='1', CARGO_INCREMENTAL='0')
    reports, executions = {}, []
    for package in inventory['packages']:
        name = package['name']
        work = output / name
        work.mkdir()
        package_env = dict(env, CARGO_TARGET_DIR=str(work / 'target'),
                           CARGO_LLVM_COV_TARGET_DIR=str(work / 'target'))
        # cargo-llvm-cov chooses its own profile pattern. Use its declared
        # value for external CLI tests so report collects those profiles too.
        declared = subprocess.check_output(
            ['cargo', 'llvm-cov', 'show-env', '--manifest-path', package['manifest']],
            cwd=ROOT, env=package_env, text=True)
        profile = [line.partition('=')[2] for line in declared.splitlines()
                   if line.startswith('LLVM_PROFILE_FILE=')]
        if len(profile) != 1 or len(shlex.split(profile[0])) != 1:
            raise ValueError('missing or malformed LLVM profile declaration')
        package_env['LLVM_PROFILE_FILE'] = shlex.split(profile[0])[0]
        selection = ['--manifest-path', package['manifest'], '--package', name] + package['selection']
        commands = [
            ['cargo', 'llvm-cov', '--locked', '--no-report'] + selection,
            ['cargo', 'llvm-cov', 'report', '--manifest-path', package['manifest'], '--package', name, '--json', '--summary-only', '--output-path', str(work / 'summary.json')],
            ['cargo', 'llvm-cov', 'report', '--manifest-path', package['manifest'], '--package', name, '--lcov', '--output-path', str(work / 'coverage.lcov')],
        ]
        if name == 'adl':
            binary = work / 'target/debug/adl'
            package_env['ADL_PUBLIC_CLI'] = str(binary)
            commands[1:1] = [
                ['python3', 'tools/public_adl/test_public_runtime_dispatch.py', '--binary', str(binary)],
                ['python3', 'tools/public_adl/test_public_codefriend_dispatch.py'],
            ]
        for index, argv in enumerate(commands):
            with (work / f'{index}.log').open('xb') as log:
                result = subprocess.run(argv, cwd=ROOT, env=package_env, stdout=log, stderr=subprocess.STDOUT)
            executions.append({'package': name, 'argv': argv, 'exit_code': result.returncode})
            (output / 'executions.json').write_text(json.dumps(executions, indent=2) + '\n')
            if result.returncode:
                raise ValueError('coverage producer failed: ' + name)
        reports[name] = json.loads((work / 'summary.json').read_text())
    clean_inputs()
    if subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip() != head:
        raise ValueError('source HEAD changed during coverage')
    if source_hashes != {path: hashlib.sha256((ROOT / path).read_bytes()).hexdigest() for path in source_paths}:
        raise ValueError('production source changed during coverage')
    rows, unmeasured = check_rows(reports, inventory, ROOT)
    failures = enforce(rows, mode, changed)
    receipt = {'schema': 'adl.public_coverage.v1', 'source_head': head, 'mode': mode,
               'base': base, 'changed_files': sorted(changed), 'files': rows,
               'source_sha256': source_hashes,
               'declaration_only_without_measurement': unmeasured,
               'line_count': sum(r['count'] for r in rows.values()),
               'covered_lines': sum(r['covered'] for r in rows.values()),
               'thresholds': {'changed_file': 80, 'full_file': 80, 'full_aggregate': 90},
               'inventory_sha256': hashlib.sha256(CONFIG.read_bytes()).hexdigest(),
               'toolchain': subprocess.check_output(['rustc', '-vV'], cwd=ROOT, text=True),
               'failures': failures, 'status': 'failed' if failures else 'passed'}
    (output / 'result.json').write_text(json.dumps(receipt, indent=2) + '\n')
    if failures:
        raise ValueError('; '.join(failures))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--mode', choices=['pull_request', 'full'], required=True)
    parser.add_argument('--base')
    args = parser.parse_args()
    run(args.output, args.mode, args.base)
