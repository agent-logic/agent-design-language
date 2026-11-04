#!/usr/bin/env python3
"""PVF docs/contract: deterministic small local CPU; no runtime release gate.

Validate the proposed public packet, never implementation or empirical success.
"""
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PACKET = ROOT / 'docs/milestones/v0.93.2/ghb'
p = json.loads((PACKET / 'CONTRACT_PROFILE_1414.json').read_text())
spec = (PACKET / 'NORMATIVE_SPEC_1414.md').read_text()
migration = (PACKET / 'MIGRATION_1414.md').read_text()
expected = ['observe', 'hypothesize', 'propose_candidate', 'admit_experiment',
            'execute_experiment', 'measure', 'update_belief', 'disposition',
            'authorize_activation', 'reconcile_activation', 'retain_lineage']
assert p['canonical_transitions'] == expected
for transition in expected:
    assert f'| `{transition}` |' in spec, transition
assert p['status'] == 'proposed_unexecuted'
f = p['first_profile']
assert f['mutable'] == ['task_policy_parameters', 'reasoning_guidance']
assert f['execution_admissible'] is False
assert all(f[k] is None for k in ['corpus_digest', 'split_seed', 'accepted_thresholds'])
assert f['max_candidates_per_attempt'] == f['max_parallel_experiments'] == 1
assert len(set(f['development_tasks'] + f['heldout_tasks'])) == 8
assert {'evaluator_rubric', 'authority_credentials', 'system_control_templates',
        'source', 'persistent_resident_state'} <= set(f['protected'])
assert {'denied', 'deferred', 'interrupted', 'recovery_required'} <= set(p['operation_outcomes'])
assert 'recommend_activation' in p['dispositions'] and 'adopt' not in p['dispositions']
assert set(p['belief_modes']) == {'quantitative', 'qualitative', 'unmodeled'}
for revision in p['sources'].values():
    assert re.fullmatch('[0-9a-f]{40}', revision)
    assert revision in spec
for name in ['NORMATIVE_SPEC_1414.md', 'MIGRATION_1414.md']:
    text = (PACKET / name).read_text()
    assert not re.search(r'/Users/|/Volumes/|BEGIN .*PRIVATE KEY', text), name
    for link in re.findall(r'\]\(([^)]+)\)', text):
        if not link.startswith(('https://', '#')):
            target = (PACKET / link.split('#')[0]).resolve()
            assert target.is_relative_to(ROOT) and target.is_file(), (name, link)
assert 'no cross-store atomic transaction' in spec
assert 'MUST NOT be treated as a failed operation eligible for blind replay' in ' '.join(spec.split())
assert 'not automatically load a new GHB v1' in migration
assert 'not an accepted architecture decision' in migration
print('PASS: GHB1414 proposed specification/profile/links; no runtime conformance claimed')
