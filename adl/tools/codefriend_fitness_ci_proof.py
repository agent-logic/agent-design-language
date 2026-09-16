#!/usr/bin/env python3
"""Read-only qualification assertions; never changes production adapter exit semantics."""
import argparse
import json
from pathlib import Path


def require(condition):
    if not condition:
        raise ValueError('fitness_ci_contract_rejected')


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--root', type=Path, required=True)
    ap.add_argument('--gate-outcome', choices=['success', 'failure'], required=True)
    args = ap.parse_args()
    root = args.root
    context = json.loads((root / 'context.json').read_text())
    receipt = json.loads((root / 'ci/receipt.json').read_text())
    local = json.loads((root / 'local-report.json').read_text())
    actual = json.loads((root / 'ci/report.json').read_text())
    expected = {'candidate': 0, 'pass': 0, 'fail': 1, 'error': 2}[context['case']]
    require(int((root / 'ci/runner-exit.txt').read_text()) == expected)
    require(context['local_exit'] == receipt['original_exit'] == receipt['exit_code'] == expected)
    require(receipt['artifact_valid'] is True)
    require(args.gate_outcome == ('success' if expected == 0 else 'failure'))
    for key in ['candidate', 'packet_id', 'policy_digest']:
        require(receipt[key] == context[key])
    require(receipt['candidate'] == actual['record']['run']['revision'])
    require(receipt['packet_id'] == actual['record']['run']['packet_id'])
    require(receipt['policy_digest'] == actual['policy_digest'])
    require(local == actual)
    semantic = dict(candidate=context['candidate'], policy_digest=context['policy_digest'], status=actual['status'], violations=[{k: v[k] for k in ['rule_id', 'path', 'line']} for v in actual['violations']], errors=actual['errors'], unassessed=actual['unassessed'])
    result = dict(context, original_exit=expected, observed_gate_outcome=args.gate_outcome, classification='production_policy_pass' if context['case'] == 'candidate' else 'fixture_contract_assertion', local_result_parity=True, semantic_result=semantic)
    (root / 'proof.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result))


if __name__ == '__main__':
    main()
