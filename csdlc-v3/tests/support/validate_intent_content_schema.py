#!/usr/bin/env python3
"""#1161 DOC-002/003/004: required deterministic tooling contract proof.

Local CPU/files only, no commands dispatched. Requires jsonschema 4.26.0.
Native installed-command proof is retained separately in operator_man_pages.
"""
import copy
import json
from pathlib import Path
from jsonschema import Draft202012Validator

ROOT = Path(__file__).resolve().parents[3]
schema = json.loads((ROOT / 'docs/csdlc-v3/intent-request.schema.json').read_text())
Draft202012Validator.check_schema(schema)

def validate(kind, value):
    contract = {'$schema': schema['$schema'], '$defs': schema['$defs'], '$ref': '#/$defs/' + kind}
    return Draft202012Validator(contract).is_valid(value)

validators = [
    {'id': 'cargo', 'program': 'cargo', 'args': ['test', '--lib'], 'success_marker': 'test result: ok.'},
    {'id': 'docs', 'program': 'python3', 'args': ['check.py', '--self-test'], 'success_marker': 'PASS'},
    {'id': 'diff', 'program': 'git', 'args': ['diff', '--check'], 'success_marker': 'clean'},
    {'id': 'review', 'program': 'manual-review', 'args': ['independent-review'], 'success_marker': 'accepted'},
]
plan = {'schema': 'csdlc.v3.intent_plan.v1', 'slug': 'schema-proof',
        'cards': {k: {} for k in ['sip', 'stp', 'spp', 'vpp', 'srp', 'sor']},
        'validators': validators, 'publication': {'base': 'main', 'title': 'Proof', 'body': 'Closes #1161', 'draft': True}}
changes = {'schema': 'csdlc.v3.intent_changes.v1', 'validators': validators}
positives = [('plan', plan), ('changes', changes)]
for v in validators:
    positives.append(('validator', v))
manual = json.loads((ROOT / 'docs/csdlc-v3/man/manual.json').read_text())
walk = next(p for p in manual['pages'] if p['name'] == 'csdlc-workflow')['sections']['1 PREPARE']
edit = next(json.loads(p['code']) for p in walk if isinstance(p, dict) and p['code'].startswith('{'))
positives.append(('changes', edit))
doc = (ROOT / 'docs/csdlc-v3/NO_PR_CLOSEOUT.md').read_text()
disposition = json.loads(doc.split('```json\n', 1)[1].split('```', 1)[0])
positives.append(('no_pr_disposition', disposition))
negatives = []
for extra in ['cards', 'publication']:
    v = copy.deepcopy(changes); v[extra] = edit['cards'] if extra == 'cards' else plan['publication']; negatives.append(('changes', v))
v = copy.deepcopy(changes); v['validators'] = []; negatives.append(('changes', v))
for field in ['expected_issue_updated_at', 'expected_issue_closed_at']:
    v = copy.deepcopy(disposition); v[field] = '2026-09-09T00:00:00Z'; negatives.append(('no_pr_disposition', v))
for program, args in [('sh', ['-c', 'true']), ('git', ['clean', '-fd']), ('manual-review', ['../outside']), ('python3', ['-c', 'print(1)'])]:
    negatives.append(('validator', dict(id='bad', program=program, args=args, success_marker='PASS')))
for timeout in [0, 301]:
    v = copy.deepcopy(validators[0]); v['timeout_seconds'] = timeout; negatives.append(('validator', v))
v = copy.deepcopy(validators[0]); v['success_marker'] = 'PASS'; negatives.append(('validator', v))
creation = {'schema': 'csdlc.v3.creation_journal_recovery_disposition.v1', 'operation_id': 'semantic-operation-v1:' + 'a' * 64}
proof = {'schema': 'csdlc.v3.semantic_proof_recovery_disposition.v1', 'action': 'abandon_indeterminate_proof', 'operation_id': creation['operation_id'], 'rationale': 'Known interrupted fixture'}
positives += [('creation_journal_recovery_disposition', creation), ('proof_recovery_disposition', proof)]
for field in ['schema', 'operation_id']:
    invalid = copy.deepcopy(creation); del invalid[field]; negatives.append(('creation_journal_recovery_disposition', invalid))
invalid = copy.deepcopy(creation); invalid['operation_id'] = 'arbitrary'; negatives.append(('creation_journal_recovery_disposition', invalid))
invalid = copy.deepcopy(creation); invalid['extra'] = True; negatives.append(('creation_journal_recovery_disposition', invalid))
invalid = copy.deepcopy(proof); invalid['rationale'] = ' '; negatives.append(('proof_recovery_disposition', invalid))
review = dict(proof, schema='csdlc.v3.semantic_review_recovery_disposition.v1', action='abandon_stale_review')
cleanup = {'schema': 'csdlc.v3.semantic_cleanup_absence_recovery_disposition.v1',
           'disposition': 'reconcile_already_absent_cleanup', 'repository': 'fixture/repo',
           'issue': 1161, 'worktree': '/fixture/worktree', 'branch': 'codex/1161-fixture',
           'head': 'b' * 40, 'terminal_receipt_digest': 'c' * 64, 'operator': 'fixture',
           'rationale': 'Fixture externally removed checkout', 'evidence_refs': ['fixture:absence']}
positives += [('review_recovery_disposition', review), ('cleanup_absence_recovery_disposition', cleanup)]
invalid = copy.deepcopy(review); invalid['action'] = 'abandon_indeterminate_proof'; negatives.append(('review_recovery_disposition', invalid))
for field in cleanup:
    invalid = copy.deepcopy(cleanup); del invalid[field]; negatives.append(('cleanup_absence_recovery_disposition', invalid))
invalid = copy.deepcopy(cleanup); invalid['extra'] = True; negatives.append(('cleanup_absence_recovery_disposition', invalid))

# Exercise dispatch through the complete public schema, not just isolated defs.
snapshot = {'repository': 'fixture/repo', 'issue': 1161, 'platform': 'fixture',
            'authority': {'selector_digest': 'd' * 64},
            'version': {'generation': 1, 'digest': 'e' * 64},
            'checkout': {'branch': 'codex/1161-fixture', 'head': 'b' * 40, 'root': '/fixture/worktree'}}
def envelope(command, content, execute=False):
    return {'schema': 'csdlc.v3.intent_request.v1', 'command': command,
            'content': content, 'execute': execute,
            'preview': 'semantic-recovery-v1:' + 'f' * 64 if execute else None,
            'snapshot': snapshot}
full = Draft202012Validator(schema)
full_positives = [envelope(command, content) for command, content in
                  [('prepare', plan), ('edit', changes), ('edit', edit), ('finish', disposition)]]
for content in [None, creation, proof, review, cleanup]:
    full_positives += [envelope('recover', content), envelope('recover', content, True)]
full_negatives = [envelope('recover', value) for kind, value in negatives if 'recovery_disposition' in kind]
full_negatives += [envelope('recover', {'schema': 'unsupported'}), envelope('recover', {'schema': 'csdlc.v3.semantic_review_recovery_disposition.v1'}), envelope('recover', [])]
for value in full_positives: assert full.is_valid(value), list(full.iter_errors(value))
for value in full_negatives: assert not full.is_valid(value), value
for kind, value in positives: assert validate(kind, value), (kind, value)
for kind, value in negatives: assert not validate(kind, value), (kind, value)
print(f'PASS: {len(positives)} positive and {len(negatives)} negative content cases; {len(full_positives)} positive and {len(full_negatives)} negative complete request cases')
