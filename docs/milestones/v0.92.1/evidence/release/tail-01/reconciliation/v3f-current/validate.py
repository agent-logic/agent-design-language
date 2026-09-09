#!/usr/bin/env python3
"""Issue 771 current V3-F review/suite binding; no historical gate mutation.

PVF: required deterministic local contract, small CPU/Git, no credentials/network.
This validates retained identities; substantive review is a separate receipt.
"""
import copy
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys

HERE = Path(__file__).resolve().parent
ROOT = next(p for p in HERE.parents if (p / 'AGENTS.md').is_file())
CRITERIA = {f'V3-F-ac-{i}' for i in range(1, 5)}
PREFIXES = ['csdlc-v3', 'docs/templates/prompts', 'docs/csdlc-v3',
            '.csdlc/evidence/505', '.csdlc/issues/505',
            '.adl/worktree-policy.json', 'AGENTS.md']
SUITE_ARGV = ['cargo', 'test', '--locked', '--manifest-path', 'csdlc-v3/Cargo.toml']


def require(value, code):
    if not value:
        raise ValueError(code)


def git(*args):
    return subprocess.check_output(['git', '-C', str(ROOT), *args], text=True).strip()


def digest(data):
    return hashlib.sha256(data).hexdigest()


def read(name):
    return json.loads((HERE / name).read_text())


def scope_at(revision):
    lines = git('ls-tree', '-r', revision, '--', *PREFIXES).splitlines()
    return {line.split('\t', 1)[1]: line.split()[2] for line in lines}


def validate(mapping, assignment=None, review=None, suite=None, candidate='HEAD'):
    assignment = read('assignment.json') if assignment is None else assignment
    review = read('review.json') if review is None else review
    suite = read('suite.json') if suite is None else suite
    require(mapping['schema'] == 'adl.v3f.current_mapping.v1', 'mapping_schema')
    source = mapping['source_sha']
    require(re.fullmatch('[0-9a-f]{40}', source) is not None, 'source_sha')
    require(source == assignment['source_sha'] == review['source_sha'] == suite['source_sha'],
            'exact_source_identity')
    require(assignment['scope_prefixes'] == PREFIXES, 'scope_denominator')
    expected = scope_at(source)
    require(expected == assignment['scope'] == review['scope'], 'complete_review_scope')
    require(expected == scope_at(candidate), 'current_source_drift')
    require(subprocess.run(['git', '-C', str(ROOT), 'merge-base', '--is-ancestor',
                            source, candidate], capture_output=True).returncode == 0,
            'source_ancestry')
    require(review['reviewer'] == assignment['reviewer'] and review['independent'] is True,
            'independent_assignment')
    require(assignment['assigned_at'] <= review['completed_at'], 'assignment_order')
    require(review['result'] == 'pass' and not review['unreviewed_paths'], 'substantive_review')
    require(not any(f['actionable'] and f['in_scope'] and f['disposition'] != 'fixed'
                    for f in review['findings']), 'open_review_finding')
    require(set(review['criteria']) == CRITERIA and all(
        item['result'] == 'pass' and item['rationale'] for item in review['criteria'].values()),
        'semantic_review')
    require(suite['argv'] == SUITE_ARGV, 'full_locked_suite')
    require(suite['detached'] is True and suite['clean_before'] is True and
            suite['clean_after'] is True and suite['external_target'] is True and
            suite['head_after'] == source, 'detached_clean_execution')
    log = (HERE / 'suite.log').read_bytes()
    require(suite['log'] == 'suite.log' and digest(log) == suite['log_sha256'], 'suite_log_digest')
    counts = re.findall(r'test result: ok\. (\d+) passed; (\d+) failed;', log.decode())
    passed = sum(int(p) for p, _ in counts)
    require(suite['exit_code'] == 0 and suite['failed'] == 0 and
            suite['passed'] == passed and passed >= 188 and
            not re.search(r'test result: FAILED|error: test failed', log.decode()), 'passing_full_suite')
    for name in ['assignment.json', 'review.json', 'suite.json', 'historical-fixture.diff']:
        require(mapping['receipt_sha256'][name] == digest((HERE / name).read_bytes()), 'receipt_digest')
    historical = HERE.parent / 'current-exceptions.json'
    require(digest(historical.read_bytes()) == mapping['historical_exceptions_sha256'], 'historical_immutable')
    originals = {r['id']: r['text_digest'] for r in json.loads(historical.read_text())['post_review_505']['criteria']}
    require(len(mapping['rows']) == 4 and {r['id'] for r in mapping['rows']} == CRITERIA, 'four_rows')
    fixture = 'csdlc-v3/tests/terminal_cleanup_cutover_commands.rs'
    for row in mapping['rows']:
        require(row['criterion_text_digest'] == originals[row['id']], 'criterion_identity')
        require(row['review'] == 'review.json' and row['suite'] == 'suite.json' and
                row['source_sha'] == source and row['fixture_blob'] == expected[fixture] and
                row['disposition'] == 'current_review_and_suite_proven', 'row_proof_binding')
    old = HERE.parent / 'census.json'
    require(digest(old.read_bytes()) == mapping['historical_census_sha256'], 'historical_census_immutable')
    corp = [r for r in json.loads(old.read_text())['rows'] if
            r['row_id'] in {f'CORP-A:CORP-A-ac-{i}' for i in range(1, 4)}]
    require(corp == mapping['preserved_corp_a'] and len(corp) == 3 and all(
        r['reconciliation_class'] == 'review_freshness_resolved' for r in corp), 'corp_a_preserved')
    require(mapping['release_authorized'] is False, 'release_boundary')
    return dict(status='passed', source_sha=source, candidate=git('rev-parse', candidate),
                reviewed_paths=len(expected), passed_tests=passed, v3_f_rows=4,
                preserved_corp_a_rows=3, release_authorized=False)


def negative(mapping):
    a, r, s = read('assignment.json'), read('review.json'), read('suite.json')
    tests = [
        ('stale_review_sha', 'exact_source_identity', lambda m,a,r,s:r.update(source_sha='9d0918ea6ba4a90ec674d29bc030878c0b22527f')),
        ('different_test_bytes', 'exact_source_identity', lambda m,a,r,s:s.update(source_sha='c24f8fa65ce445b03ce6cd69007307291d78b60c')),
        ('historical_fixture_blob', 'row_proof_binding', lambda m,a,r,s:m['rows'][0].update(fixture_blob='c5d67168df2c923eb77c592b7545386ad0a39234')),
        ('missing_scope', 'complete_review_scope', lambda m,a,r,s:r['scope'].pop('csdlc-v3/src/commands/terminal.rs')),
        ('unreviewed_path', 'substantive_review', lambda m,a,r,s:r['unreviewed_paths'].append('csdlc-v3/src/authority.rs')),
        ('missing_criterion_review', 'semantic_review', lambda m,a,r,s:r['criteria'].pop('V3-F-ac-4')),
        ('zero_tests', 'passing_full_suite', lambda m,a,r,s:s.update(passed=0)),
        ('narrow_suite', 'full_locked_suite', lambda m,a,r,s:s['argv'].extend(['--test','terminal_cleanup_cutover_commands'])),
        ('dirty_checkout', 'detached_clean_execution', lambda m,a,r,s:s.update(clean_after=False)),
        ('wrong_reviewer', 'independent_assignment', lambda m,a,r,s:r.update(reviewer='unassigned')),
        ('failed_suite', 'passing_full_suite', lambda m,a,r,s:s.update(exit_code=1)),
        ('receipt_tamper', 'receipt_digest', lambda m,a,r,s:m['receipt_sha256'].update({'review.json':'0'*64})),
        ('missing_row', 'four_rows', lambda m,a,r,s:m['rows'].pop()),
        ('reopened_corp_a', 'corp_a_preserved', lambda m,a,r,s:m['preserved_corp_a'][0].update(reconciliation_class='non_proving')),
    ]
    for name, code, mutate in tests:
        values = copy.deepcopy([mapping, a, r, s]); mutate(*values)
        try:
            validate(*values)
        except ValueError as error:
            require(str(error) == code, f'{name}: unexpected rejection {error}')
        else:
            raise ValueError(f'{name}: mutation accepted')
    return dict(status='passed', negative_cases=len(tests), cases=[t[0] for t in tests])


if __name__ == '__main__':
    require(sys.argv[1:] in ([], ['--negative']), 'usage: validate.py [--negative]')
    try:
        mapping = read('mapping.json')
        print(json.dumps(negative(mapping) if sys.argv[1:] else validate(mapping), sort_keys=True))
    except (ValueError, KeyError, OSError, subprocess.CalledProcessError) as error:
        print(json.dumps(dict(status='blocked', error=str(error).replace(str(ROOT), '[repo]'))))
        sys.exit(1)
