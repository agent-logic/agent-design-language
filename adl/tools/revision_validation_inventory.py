#!/usr/bin/env python3
"""Enumerate Cargo registrations and libtest cases at an immutable Git revision.

No test bodies are run. Raw command transcripts are retained as measurement
provenance, not authenticated remote execution or behavioral qualification.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tarfile

SCHEMA = 'adl.revision_validation_inventory.v1'
PROFILE = {'manifest': 'adl/Cargo.toml', 'package': 'adl',
           'features': 'default', 'toolchain': '1.92.0',
           'selection': 'tests', 'test_bodies': 'not_run'}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def write_json(path, value):
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + '\n')


def git(repo, *args):
    return subprocess.check_output(['git', '-C', str(repo), *args])


def revision_identity(repo, revision):
    if not re.fullmatch(r'[0-9a-f]{40}', revision):
        raise ValueError('revision must be a full immutable commit')
    if git(repo, 'rev-parse', revision + '^{commit}').decode().strip() != revision:
        raise ValueError('revision is not a commit')
    return {'commit': revision,
            'tree': git(repo, 'rev-parse', revision + '^{tree}').decode().strip(),
            'manifest_sha256': digest(git(repo, 'show', revision + ':adl/Cargo.toml')),
            'lock_sha256': digest(git(repo, 'show', revision + ':adl/Cargo.lock'))}


def cases(text):
    rows = []
    for line in text.splitlines():
        match = re.fullmatch(r'(.+): (test|benchmark)', line)
        if match:
            rows.append({'name': match[1], 'kind': match[2]})
    if len({(r['name'], r['kind']) for r in rows}) != len(rows):
        raise ValueError('duplicate enumerated case')
    return sorted(rows, key=lambda row: (row['name'], row['kind']))


def target_key(target):
    return ':'.join(target['kind']) + ':' + target['name']


def targets_from_metadata(metadata):
    packages = [p for p in metadata['packages'] if p['name'] == PROFILE['package']
                and p['manifest_path'] == 'SNAPSHOT/adl/Cargo.toml']
    if len(packages) != 1:
        raise ValueError('wrong package or manifest in metadata')
    package = packages[0]
    enabled = set(package['features'].get('default', []))
    while True:
        expanded = enabled | {v for f in enabled for v in package['features'].get(f, [])}
        if expanded == enabled:
            break
        enabled = expanded
    rows = []
    for target in package['targets']:
        reason = None
        if not set(target['kind']) & {'lib', 'bin', 'test'} or not target['test']:
            reason = 'outside_tests_selection'
        elif not set(target.get('required-features', [])) <= enabled:
            reason = 'required_features_disabled'
        rows.append({'key': target_key(target), 'name': target['name'],
                     'kind': target['kind'], 'source': target['src_path'],
                     'required_features': target.get('required-features', []),
                     'selection': 'selected' if reason is None else reason})
    if not rows or len({r['key'] for r in rows}) != len(rows):
        raise ValueError('empty or duplicate registered target set')
    return sorted(rows, key=lambda row: row['key'])


def load_transcript(directory, record):
    results = {}
    for stream in ['stdout', 'stderr']:
        name = record[stream]['file']
        if Path(name).name != name:
            raise ValueError('transcript path escapes inventory directory')
        data = (directory / name).read_bytes()
        if digest(data) != record[stream]['sha256']:
            raise ValueError('transcript digest mismatch')
        results[stream] = data.decode()
    return results


def derive(directory, commands):
    if len({c['role'] for c in commands}) != len(commands):
        raise ValueError('duplicate command role')
    records = {c['role']: (c, load_transcript(directory, c)) for c in commands}
    for role in ['rustc', 'metadata', 'build', 'doctest']:
        if role not in records:
            raise ValueError('missing required command: ' + role)
    metadata_command, metadata_output = records['metadata']
    if metadata_command['exit_code'] != 0:
        raise ValueError('metadata process failed; denominator unavailable')
    shared = ['--manifest-path', 'SNAPSHOT/adl/Cargo.toml', '--locked']
    expected = {
        'rustc': ['rustc', '+1.92.0', '-vV'],
        'metadata': ['cargo', '+1.92.0', 'metadata'] + shared + ['--no-deps', '--format-version', '1'],
        'build': ['cargo', '+1.92.0', 'test'] + shared + ['--package', 'adl', '--tests', '--no-run', '--message-format=json'],
        'doctest': ['cargo', '+1.92.0', 'test'] + shared + ['--package', 'adl', '--doc', '--', '--list'],
    }
    for role, argv in expected.items():
        if records[role][0]['argv'] != argv:
            raise ValueError('command/profile mismatch: ' + role)
    rows = targets_from_metadata(json.loads(metadata_output['stdout']))
    build, output = records['build']
    artifacts = {}
    for line in output['stdout'].splitlines():
        message = json.loads(line)
        if message.get('reason') == 'compiler-artifact' and message.get('profile', {}).get('test') and message.get('executable'):
            key = target_key(message['target'])
            if key in artifacts:
                raise ValueError('duplicate built harness')
            artifacts[key] = (message['target']['src_path'], message['executable'])
    failures = []
    if build['exit_code'] != 0:
        failures.append('build_failed')
    if records['rustc'][0]['exit_code'] != 0:
        failures.append('toolchain_failed')
    selected = {r['key'] for r in rows if r['selection'] == 'selected'}
    if set(artifacts) - selected:
        raise ValueError('unexpected test harness artifact')
    expected_roles = {'rustc', 'metadata', 'build', 'doctest'}
    for row in rows:
        row.update(cases=None, ignored_cases=None, execution='not_run')
        if row['selection'] != 'selected':
            continue
        key = row['key']
        if key not in artifacts:
            row['enumeration'] = 'unavailable'
            failures.append('missing_harness:' + key)
            continue
        if artifacts[key][0] != row['source']:
            raise ValueError('artifact source differs from registered source')
        row['enumeration'] = 'listed'
        for prefix, field in [('list:', 'cases'), ('ignored:', 'ignored_cases')]:
            role = prefix + key
            expected_roles.add(role)
            if role not in records or records[role][0]['exit_code'] != 0:
                row['enumeration'] = 'unavailable'
                failures.append('listing_failed:' + role)
            else:
                suffix = ['--list', '--format', 'terse'] if prefix == 'list:' else ['--list', '--ignored', '--format', 'terse']
                if records[role][0]['argv'] != [artifacts[key][1]] + suffix:
                    raise ValueError('harness listing command mismatch')
                row[field] = cases(records[role][1]['stdout'])
        if row['cases'] is not None and row['ignored_cases'] is not None:
            if any(c not in row['cases'] for c in row['ignored_cases']):
                raise ValueError('ignored case absent from all-case list')
    if set(records) != expected_roles:
        raise ValueError('omitted or unexpected command records')
    doc_command, doc_output = records['doctest']
    docs = {'status': 'listed' if doc_command['exit_code'] == 0 else 'unavailable',
            'cases': cases(doc_output['stdout']) if doc_command['exit_code'] == 0 else None,
            'execution': 'not_run', 'ignored': 'not separately classified by rustdoc list'}
    if doc_command['exit_code'] != 0:
        failures.append('doctest_list_failed')
    counts = {'registered_targets': len(rows), 'selected_targets': len(selected),
              'listed_targets': sum(r.get('enumeration') == 'listed' for r in rows),
              'enumerated_cases': sum(len(r['cases'] or []) for r in rows),
              'ignored_cases': sum(len(r['ignored_cases'] or []) for r in rows),
              'test_bodies_run': 0}
    if counts['enumerated_cases'] == 0:
        failures.append('no_executable_cases')
    return {'targets': rows, 'doctests': docs, 'counts': counts,
            'status': 'complete' if not failures else 'incomplete', 'failures': sorted(failures)}


def collect(args):
    repo, directory, target = args.repo.resolve(), args.out.resolve(), args.target_dir.resolve()
    identity = revision_identity(repo, args.revision)
    directory.mkdir(parents=True, exist_ok=False)
    commands = []
    snapshot = directory / 'snapshot'
    snapshot.mkdir()
    # Git archive contains only this immutable revision, no inherited worktree edits.
    archive = subprocess.Popen(['git', '-C', str(repo), 'archive', args.revision], stdout=subprocess.PIPE)
    with tarfile.open(fileobj=archive.stdout, mode='r|') as tar:
        tar.extractall(snapshot, filter='data')
    if archive.wait() != 0:
        raise ValueError('git archive failed')
    env = os.environ.copy()
    env['CARGO_TARGET_DIR'] = str(target)
    env['CARGO_BUILD_JOBS'] = str(args.jobs)
    # Remove caller build instrumentation so profile identity is explicit.
    for key in ['RUSTFLAGS', 'CARGO_ENCODED_RUSTFLAGS', 'RUSTC_WRAPPER', 'RUSTC_WORKSPACE_WRAPPER']:
        env.pop(key, None)

    def portable(text):
        return text.replace(str(snapshot), 'SNAPSHOT').replace(str(target), 'TARGET')

    def run(role, argv):
        print('inventory:', args.revision[:12], role, file=sys.stderr, flush=True)
        number = len(commands)
        stdout_path = directory / f'{number:03d}.stdout'
        stderr_path = directory / f'{number:03d}.stderr'
        with stdout_path.open('w') as stdout, stderr_path.open('w') as stderr:
            result = subprocess.run(argv, cwd=snapshot, env=env, text=True, stdout=stdout, stderr=stderr)
        raw = stdout_path.read_text()
        record = {'role': role, 'argv': [portable(a) for a in argv], 'exit_code': result.returncode}
        for stream, path in [('stdout', stdout_path), ('stderr', stderr_path)]:
            data = portable(path.read_text()).encode()
            path.write_bytes(data)
            record[stream] = {'file': path.name, 'sha256': digest(data)}
        commands.append(record)
        write_json(directory / 'commands.json', commands)
        return result.returncode, raw

    _, rustc = run('rustc', ['rustc', '+1.92.0', '-vV'])
    cargo = ['cargo', '+1.92.0']
    shared = ['--manifest-path', str(snapshot / 'adl/Cargo.toml'), '--locked']
    code, metadata = run('metadata', cargo + ['metadata'] + shared + ['--no-deps', '--format-version', '1'])
    if code:
        write_json(directory / 'failure.json', {'status': 'incomplete', 'reason': 'metadata_failed', 'identity': identity})
        raise ValueError('metadata failed; see retained transcript')
    # Do not invoke custom test binaries as though they were libtest harnesses.
    import tomllib
    manifest = tomllib.loads((snapshot / 'adl/Cargo.toml').read_text())
    for kind in ['lib', 'bin', 'test']:
        entries = manifest.get(kind, [])
        if isinstance(entries, dict):
            entries = [entries]
        if any(entry.get('harness') is False for entry in entries):
            raise ValueError('custom harness requires an explicit enumeration adapter')
    _, build = run('build', cargo + ['test'] + shared + ['--package', 'adl', '--tests', '--no-run', '--message-format=json'])
    for line in build.splitlines():
        message = json.loads(line)
        if message.get('reason') == 'compiler-artifact' and message.get('profile', {}).get('test') and message.get('executable'):
            key = target_key(message['target'])
            run('list:' + key, [message['executable'], '--list', '--format', 'terse'])
            run('ignored:' + key, [message['executable'], '--list', '--ignored', '--format', 'terse'])
    run('doctest', cargo + ['test'] + shared + ['--package', 'adl', '--doc', '--', '--list'])
    result = {'schema': SCHEMA, 'identity': identity, 'profile': PROFILE,
              'rustc': rustc, 'commands': commands,
              'limitations': ['Other packages, benches and examples are outside the selected execution scope.',
                              'Default-feature-disabled cases are not enumerated; required-feature target exclusions are explicit.',
                              'Generated cases registered by selected harnesses are included. No test bodies run.',
                              'Retained transcripts are local measurements, not independent approval or authenticated execution.'],
              **derive(directory, commands)}
    write_json(directory / 'inventory.json', result)
    return 0 if result['status'] == 'complete' else 2


def verify(repo, path, expected_revision=None):
    report = json.loads(path.read_text())
    if report['schema'] != SCHEMA or report['profile'] != PROFILE:
        raise ValueError('unsupported schema or mismatched manifest/features/profile')
    if expected_revision and report['identity']['commit'] != expected_revision:
        raise ValueError('wrong revision')
    if report['identity'] != revision_identity(repo, report['identity']['commit']):
        raise ValueError('source identity mismatch')
    rustc_records = [c for c in report['commands'] if c['role'] == 'rustc']
    if len(rustc_records) != 1 or report['rustc'] != load_transcript(path.parent, rustc_records[0])['stdout']:
        raise ValueError('toolchain transcript mismatch')
    derived = derive(path.parent, report['commands'])
    allowed = {'schema', 'identity', 'profile', 'rustc', 'commands', 'limitations'} | set(derived)
    if set(report) != allowed:
        raise ValueError('unsupported claim or missing report field')
    for key, value in derived.items():
        if report[key] != value:
            raise ValueError('derived inventory mismatch: ' + key)
    if derived['status'] != 'complete':
        raise ValueError('required measurement incomplete: ' + ', '.join(derived['failures']))
    return report


def compare(args):
    baseline = verify(args.repo, args.baseline, 'a71d699d52831b32bb68ed9c7c7e837925949de4')
    candidate = verify(args.repo, args.candidate, 'e986de6d06aacd385de93dd033def77a718c1581')
    if baseline['rustc'] != candidate['rustc']:
        raise ValueError('toolchain/platform mismatch')
    def identities(report):
        return {row['key'] + '::' + c['name'] for row in report['targets'] for c in row['cases'] or []}
    before, after = identities(baseline), identities(candidate)
    old_targets = {r['key'] for r in baseline['targets']}
    new_targets = {r['key'] for r in candidate['targets']}
    write_json(args.out, {'schema': 'adl.revision_validation_comparison.v1',
                         'baseline': baseline['identity'], 'candidate': candidate['identity'],
                         'profile': PROFILE, 'baseline_counts': baseline['counts'], 'candidate_counts': candidate['counts'],
                         'targets_added': sorted(new_targets - old_targets), 'targets_removed': sorted(old_targets - new_targets),
                         'cases_added': sorted(after - before), 'cases_removed': sorted(before - after),
                         'moves': 'not inferred; changed target-qualified names appear as additions/removals',
                         'behavior_reduction': 'not_claimed', 'runtime_qualification': 'not_claimed'})
    return 0


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    subs = parser.add_subparsers(dest='command', required=True)
    collector = subs.add_parser('collect')
    collector.add_argument('--repo', type=Path, required=True)
    collector.add_argument('--revision', required=True)
    collector.add_argument('--out', type=Path, required=True)
    collector.add_argument('--target-dir', type=Path, required=True)
    collector.add_argument('--jobs', type=int, default=2)
    checker = subs.add_parser('verify')
    checker.add_argument('--repo', type=Path, required=True)
    checker.add_argument('--inventory', type=Path, required=True)
    checker.add_argument('--expected-revision', required=True)
    comparison = subs.add_parser('compare')
    comparison.add_argument('--repo', type=Path, required=True)
    comparison.add_argument('--baseline', type=Path, required=True)
    comparison.add_argument('--candidate', type=Path, required=True)
    comparison.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    try:
        if args.command == 'collect':
            return collect(args)
        if args.command == 'compare':
            return compare(args)
        verify(args.repo, args.inventory, args.expected_revision)
        print(json.dumps({'status': 'verified', 'runtime_qualification': False}))
        return 0
    except (ValueError, KeyError, OSError, subprocess.SubprocessError) as error:
        print('revision_validation_inventory: ' + str(error), file=sys.stderr)
        return 2


if __name__ == '__main__':
    sys.exit(main())
