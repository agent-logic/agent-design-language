#!/usr/bin/env python3
"""Transport postconditions only; fitness predicates and live verification belong to ADL."""
import json
import os
from pathlib import Path
import stat
import sys


def require(condition):
    if not condition:
        raise ValueError('fitness_ci_contract_rejected')


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result)
        result[key] = value
    return result


def regular_json(path, limit):
    require(stat.S_ISREG(path.lstat().st_mode) and not path.is_symlink())
    require(path.stat().st_size <= limit)
    with path.open('rb') as source:
        data = source.read(limit + 1)
        require(len(data) <= limit)
        return json.loads(data, object_pairs_hook=unique_object)


def check():
    phase, status, *args = sys.argv[1:]
    keys = {'--store', '--packet-id', '--policy', '--candidate', '--policy-digest', '--out'}
    require(len(args) == 2 * len(keys))
    flags = dict(zip(args[::2], args[1::2]))
    require(set(flags) == keys)
    out = Path(os.path.abspath(flags['--out']))
    require('..' not in Path(flags['--out']).parts)
    require(not any(p.is_symlink() for p in [out, *out.parents]))
    if phase == 'before':
        require(not out.exists())
        return 0
    require(phase == 'after')
    status = int(status)
    require(status in (0, 1, 2))
    pins = regular_json(out / 'expectations.json', 16384)
    require(pins == {k: flags['--' + k.replace('_', '-')] for k in ('candidate', 'packet_id', 'policy_digest')})
    receipt = regular_json(out / 'receipt.json', 32768)
    versions = {
        'codefriend.fitness.ci.v1': ('codefriend.fitness.v1', 16 * 1024 * 1024, {'pass': 0, 'fail': 1, 'error': 2}),
        'codefriend.fitness.ci.v2': ('codefriend.fitness.v2', 4 * 1024 * 1024, {'pass': 0, 'fail': 1, 'unknown': 2}),
    }
    require(receipt['schema'] in versions)
    schema, report_limit, assessments = versions[receipt['schema']]
    require(receipt['exit_code'] == status)
    original = out / 'runner-exit.txt'
    require(stat.S_ISREG(original.lstat().st_mode) and not original.is_symlink() and original.stat().st_size <= 32)
    require(receipt['original_exit'] == int(original.read_text()))
    if receipt['artifact_valid'] is False:
        require(status == 2 and receipt['assessment'] is None and receipt['report_digest'] is None)
        return 2
    require(receipt['artifact_valid'] is True and receipt['error'] is None)
    require(receipt['original_exit'] == status)
    require({k: receipt[k] for k in pins} == pins)
    report = regular_json(out / 'report.json', report_limit)
    require(report['schema'] == schema)
    require(report['digest'] == receipt['report_digest'])
    require(report['record']['run']['revision'] == pins['candidate'])
    require(report['record']['run']['packet_id'] == pins['packet_id'])
    require(report['policy_digest'] == pins['policy_digest'])
    require(report['status'] == receipt['assessment'])
    require(assessments[report['status']] == status)
    return status


if __name__ == '__main__':
    try:
        result = check()
    except (AssertionError, OSError, ValueError, KeyError, TypeError, IndexError):
        print('adl_event component=codefriend_fitness_ci_transport outcome=error', file=sys.stderr)
        result = 2
    sys.exit(result)
