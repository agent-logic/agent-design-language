#!/usr/bin/env python3
"""PVF required tooling contract: deterministic workflow fixtures, no builds/network.
Uses the same Ruby YAML reader as the existing CI workflow policy validator.
"""
import copy
import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
PROFILE = {
    'CARGO_PROFILE_DEV_DEBUG': '0',
    'CARGO_PROFILE_TEST_DEBUG': '0',
    'CARGO_PROFILE_TEST_OPT_LEVEL': '1',
    'CARGO_PROFILE_TEST_DEBUG_ASSERTIONS': 'true',
    'CARGO_PROFILE_TEST_OVERFLOW_CHECKS': 'true',
    'CARGO_INCREMENTAL': '0',
}
COMMANDS = [
    'bash adl/tools/run_cargo_validation.sh cargo test --locked --manifest-path csdlc-v3/Cargo.toml',
    'bash adl/tools/run_cargo_validation.sh cargo fmt --manifest-path csdlc-v3/Cargo.toml --all -- --check',
    'bash adl/tools/run_cargo_validation.sh cargo clippy --locked --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings',
]


def validate(workflow):
    job = workflow['jobs']['csdlc_v3_standalone']
    if job.get('env') != PROFILE:
        raise ValueError('optimized CI profile must retain assertions and overflow checks')
    if job.get('if') != "needs.adl_path_policy.outputs.csdlc_v3_standalone_required == 'true'":
        raise ValueError('required lane selection changed')
    if job.get('continue-on-error', False):
        raise ValueError('required lane cannot ignore failures')
    steps = job['steps']
    for command in COMMANDS:
        matches = [step for step in steps if step.get('run') == command]
        if len(matches) != 1:
            raise ValueError('test/format/Clippy coverage or failure behavior changed')
        if any(key in matches[0] for key in ('if', 'continue-on-error', 'env')):
            raise ValueError('required commands must use the job profile unconditionally')
    if 'csdlc_v3_standalone' not in workflow['jobs']['adl-ci']['needs']:
        raise ValueError('required aggregate no longer waits for the lane')


def main():
    workflow = json.loads(subprocess.check_output([
        'ruby', '-rjson', '-ryaml', '-e',
        'puts JSON.generate(YAML.safe_load(File.read(ARGV[0]), permitted_classes: [], permitted_symbols: [], aliases: true))',
        str(ROOT / '.github/workflows/ci.yaml'),
    ], text=True))
    validate(workflow)
    mutations = [
        lambda j: j['env'].update(CARGO_PROFILE_TEST_DEBUG_ASSERTIONS='false'),
        lambda j: j['env'].update(CARGO_PROFILE_TEST_OVERFLOW_CHECKS='false'),
        lambda j: j.update({'continue-on-error': True}),
        lambda j: j.update({'if': 'false'}),
        lambda j: j['steps'].remove(next(s for s in j['steps'] if s.get('run') == COMMANDS[0])),
        lambda j: next(s for s in j['steps'] if s.get('run') == COMMANDS[0]).update({'run': COMMANDS[0] + ' --lib'}),
        lambda j: next(s for s in j['steps'] if s.get('run') == COMMANDS[0]).update({'env': {'CARGO_PROFILE_TEST_DEBUG_ASSERTIONS': 'false'}}),
    ]
    for mutate in mutations:
        candidate = copy.deepcopy(workflow)
        mutate(candidate['jobs']['csdlc_v3_standalone'])
        try:
            validate(candidate)
        except ValueError:
            continue
        raise AssertionError('unsafe CI profile/coverage mutation accepted')
    print(f'test result: ok. {len(mutations) + 1} passed; 0 failed;')


if __name__ == '__main__':
    main()
