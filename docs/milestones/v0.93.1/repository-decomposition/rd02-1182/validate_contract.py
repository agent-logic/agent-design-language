#!/usr/bin/env python3
"""RD02 technical preparation checks; never an acceptance or export authority.

Stdlib-only, deterministic and offline. --self-test mutates loaded records and
private disposable packet copies; originals remain unchanged. It never authorizes
downstream extraction.
"""
import argparse
import copy
import gzip
import hashlib
import json
from pathlib import Path
import sys
import shutil
import tempfile

REPOS = {'agent-design-language': 'public', 'codefriend.ai': 'private',
         'cognitive-sdlc': 'private', 'agent-logic-runtime': 'private',
         'codefriend': 'private', 'agent-logic-infrastructure': 'private',
         'agent-logic-enterprise-security': 'private'}
ARTIFACTS = dict(zip(('public-adl', 'csdlc-owner', 'runtime', 'resilience',
                     'codefriend-service', 'website', 'delivery', 'enterprise-policy'),
                    ('agent-design-language', 'cognitive-sdlc', 'agent-logic-runtime',
                     'agent-logic-runtime', 'codefriend', 'codefriend.ai',
                     'agent-logic-infrastructure', 'agent-logic-enterprise-security')))
IDENTITY = {'source_commit', 'asset_identity', 'sha256', 'package_version',
            'dependency_lock', 'build_provenance', 'supported_consumer_versions'}
POLICIES = {'inherited_organization_member_read', 'access_evidence_rollback_approver',
            'artifact_delivery', 'source_license_history_preservation'}
MAPPING_ORDER = ('mixed-generation-dispositions.json', 'docs-dispositions.json',
                 'demo-root-dispositions.json', 'delivery-dispositions.json',
                 'tool-runtime-infra-dispositions.json', 'tool-skill-dispositions.json',
                 'tool-residual-dispositions.json', 'override-adjudications.json',
                 'provider-boundary-dispositions.json')
META = ('path', 'git_object', 'mode', 'size_bytes', 'type')


def require(ok, code, detail=''):
    if not ok:
        raise ValueError(code + (': ' + str(detail) if detail else ''))


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def rows(path):
    with gzip.open(path, 'rt', encoding='utf-8') as stream:
        return [json.loads(line) for line in stream]


def index(values, key):
    result = {}
    for row in values:
        name = key(row)
        require(isinstance(name, str) and name, 'missing_path')
        require(name not in result, 'duplicate_path', name)
        result[name] = row
    return result


def load(packet):
    baseline = packet.parent / 'rd01-1181'
    summary = json.loads((baseline / 'summary.json').read_text())
    for name, expected in summary['reports'].items():
        require(Path(name).name == name, 'unsafe_baseline_path')
        require(digest(baseline / name) == expected, 'baseline_digest', name)
    audit = json.loads((packet / 'mapping-audit.json').read_text())
    require(set(audit['input_sha256']) == set(MAPPING_ORDER), 'mapping_input_set')
    proposals = {}
    for name in MAPPING_ORDER:
        expected = audit['input_sha256'][name]
        require(Path(name).name == name, 'unsafe_mapping_path')
        require(digest(packet / 'mapping-inputs' / name) == expected, 'mapping_digest', name)
        entries = json.loads((packet / 'mapping-inputs' / name).read_text())['paths']
        index(entries, lambda r: r['path'])
        for entry in entries:
            proposals[entry['path']] = (name, entry.get('resolution', entry))
    require(digest(packet / 'ownership.jsonl.gz') == audit['output_sha256'], 'ownership_digest')
    return {'proposals': proposals, 'contract': json.loads((packet / 'contract.json').read_text()),
            'ownership': rows(packet / 'ownership.jsonl.gz'),
            'opening': rows(packet / 'opening-ownership.jsonl.gz'),
            'census': rows(baseline / 'census.jsonl.gz'),
            'delta': rows(baseline / 'delta-opening.jsonl.gz'),
            'summary': summary}


def validate(data, require_policy=False):
    c = data['contract']
    require(c['source_revision'] == '682be6b8babb69f73c5e9fef8580f1b3c88966d5', 'accepted_rd01_candidate')
    require(c['schema'] == 'rd02.cutover_contract.v1', 'schema')
    require(c['status'] == 'proposed_not_accepted', 'fabricated_acceptance')
    require(c['policy_acceptance'] == {'status': 'pending', 'approved_decisions': [],
                                      'approval_reference': None}, 'fabricated_acceptance')
    require(isinstance(c['pending_operator_policy'], dict), 'policy_shape')
    require(set(c['pending_operator_policy']) == POLICIES, 'missing_policy_authority')
    require(all(isinstance(v, str) and 'pending' in v.lower()
                for v in c['pending_operator_policy'].values()), 'fabricated_policy_acceptance')
    gate = c['execution_gate']
    require(gate['observed_satisfied'] is True and gate['effects_unknown'] is False
            and gate['native_finish'] == 'terminal_closed_out'
            and isinstance(gate['native_generation'], int) and gate['native_generation'] > 0
            and len(gate['merge_commit']) == 40, 'rd01_gate')
    inputs, summary = c['source_inputs'], data['summary']
    require(inputs['frozen_revision'] == summary['frozen_predecessor']
            and inputs['opening_revision'] == summary['opening_integration']
            and inputs['rd01_candidate'] == c['source_revision'], 'source_revision')
    require(inputs['census'] == '../rd01-1181/census.jsonl.gz'
            and inputs['opening_delta'] == '../rd01-1181/delta-opening.jsonl.gz', 'source_location')
    repos = index(c['repositories'], lambda r: r['name'])
    require(set(repos) == set(REPOS), 'repository_set')
    for name, visibility in REPOS.items():
        require(repos[name]['visibility'] == visibility, 'wrong_visibility', name)
        require(repos[name]['creation_authorized'] is False, 'bootstrap_not_authorized')
    artifacts = index(c['artifact_contracts'], lambda a: a['id'])
    require(set(artifacts) == set(ARTIFACTS), 'artifact_set')
    for name, owner in ARTIFACTS.items():
        a = artifacts[name]
        require(a['single_implementation_owner'] == owner, 'artifact_owner', name)
        consumers = a['consumers']
        require(consumers and len(set(consumers)) == len(consumers)
                and set(consumers) <= set(REPOS), 'artifact_consumers')
        require(set(a['consumer_usage']) == set(consumers), 'artifact_edge_usage')
        for consumer, usage in a['consumer_usage'].items():
            require(usage in ('runtime_or_build', 'development_tool'), 'artifact_edge_usage')
            if usage == 'development_tool':
                require(name == 'csdlc-owner' and bool(a.get('public_consumer_limit')),
                        'development_tool_exception')
            require(not (REPOS[owner] == 'private' and REPOS[consumer] == 'public'
                         and usage == 'runtime_or_build'), 'public_private_inversion')
        require(set(a['identity_fields']) == IDENTITY, 'artifact_identity')
        for field in ('release_writer', 'consumer_access', 'rollback', 'signature_claim',
                      'contents', 'invariant'):
            require(isinstance(a.get(field), str) and a[field].strip(), 'missing_artifact_authority', field)
        require(a['mutable_latest_allowed'] is False and a['qualified'] is False,
                'fabricated_artifact_qualification')
    require(artifacts['enterprise-policy']['distribution_status'] ==
            'future_contract_only_not_available', 'fabricated_enterprise_implementation')
    census = index(data['census'], lambda r: r['path'])
    ownership = index(data['ownership'], lambda r: r['source']['path'])
    require(len(census) == summary['frozen_paths'] and set(ownership) == set(census), 'census_coverage')
    mixed_count = 0
    for path, row in ownership.items():
        require(row['decision_group'] in ({None, 'cohesive_product'} if census[path]['decision_id'] is None else {census[path]['decision_id']}), 'decision_group', path)
        require(row['source'] == {k: census[path][k] for k in META}, 'source_metadata', path)
        require(row['accountable_repository'] in REPOS, 'unknown_owner', path)
        require(row['status'] == 'proposed_not_accepted' and row['export_authorized'] is False,
                'fabricated_export_authority', path)
        detail = row['mapping_detail']
        require(isinstance(detail, dict), 'mapping_detail_shape', path)
        require(row['proposed_disposition'] != 'retain_pending_consumer_mapping', 'unresolved_owner', path)
        require(bool(detail.get('rationale')), 'missing_rationale', path)
        for field in ('owner', 'accountable_repository'):
            if field in detail:
                require(detail[field] == row['accountable_repository'], 'owner_conflict', path)
        splits = detail.get('component_splits', [])
        sections = detail.get('section_contracts', [])
        if row['proposed_disposition'] in ('split_before_export', 'split_integration_surface',
                                           'retain_mixed_custodian_split_contract_sections'):
            require(bool(splits or sections), 'empty_mixed_split', path)
            mixed_count += 1
        selectors = set()
        for split in splits:
            require(split.get('owner') in REPOS, 'unknown_component_owner', path)
            selector = split.get('selector') or split.get('component')
            if isinstance(selector, list):
                require(selector and all(isinstance(v, str) and v.strip() for v in selector),
                        'invalid_component_split', path)
                selector = tuple(selector)
            require(isinstance(selector, (str, tuple)) and bool(selector) and selector not in selectors,
                    'invalid_component_split', path)
            selectors.add(selector)
        for section in sections:
            require(section['custodian'] == row['accountable_repository']
                    and section['primary_section_contact'] in REPOS, 'section_owner', path)
            require(0 < section['start_line'] <= section['end_line'], 'section_range', path)
            require(bool(section['contract_owners']), 'empty_section_contract', path)
            for contact in section['contract_owners']:
                require(contact['repository'] in REPOS and bool(contact['contract']), 'section_contract_owner', path)
    for path, row in ownership.items():
        if path in data['proposals']:
            source, detail = data['proposals'][path]
        elif census[path]['owner'] in REPOS:
            source = 'RD01 census cohesive candidate'
            detail = {'owner': census[path]['owner'], 'owner_role': 'component_owner',
                      'disposition': 'cohesive_product_candidate',
                      'rationale': 'RD01 cohesive source candidate; independent installed proof required downstream.'}
        else:
            require(census[path]['decision_id'] == 'D-HISTORY', 'missing_path_proposal', path)
            source = 'RD02 original repository custody proposal'
            detail = {'owner': 'agent-design-language', 'owner_role': 'original_evidence_custodian',
                      'disposition': 'retain_original_custody',
                      'rationale': 'Original source/evidence remains traceable; custody does not retire active planning/validators or authorize duplicated implementation.'}
        owner = detail.get('owner', detail.get('accountable_repository'))
        disposition = detail.get('disposition', detail.get('proposed_disposition', detail.get('disposition_group')))
        require(row['mapping_source'] == source and row['mapping_detail'] == detail
                and row['accountable_repository'] == owner
                and row['proposed_disposition'] == disposition, 'mapping_precedence', path)
    require(set(data['proposals']) <= set(census), 'extraneous_proposal')
    delta = index(data['delta'], lambda r: r['path'])
    opening = index(data['opening'], lambda r: r['path'])
    require(set(opening) == set(delta), 'opening_coverage')
    for path, row in opening.items():
        require(all(row.get(k) == v for k, v in delta[path].items()), 'opening_metadata', path)
        disposition = delta[path]['disposition']
        if disposition['owner'] in REPOS:
            expected_owner, expected_custody = disposition['owner'], 'product_source'
        else:
            require(disposition['decision_id'] == 'D-HISTORY', 'unresolved_opening_disposition', path)
            expected_owner, expected_custody = 'agent-design-language', 'original_repository_planning_evidence_custody'
        require(row['proposed_owner'] == expected_owner and row['proposed_custody'] == expected_custody,
                'opening_owner', path)
        require(row['export_authorized'] is False and row['qualification'] == 'proposed_not_accepted',
                'opening_acceptance', path)
    if require_policy:
        # No authenticated operator approval verifier exists in this preparation
        # contract. Self-asserted JSON approval must never unlock extraction.
        raise ValueError('policy_acceptance_pending: authenticated acceptance verifier not implemented')
    return {'ownership_paths': len(ownership), 'opening_paths': len(opening),
            'mixed_surfaces': mixed_count, 'repositories': len(repos),
            'artifacts': len(artifacts), 'policy_acceptance': 'pending',
            'proof_scope': 'technical_preparation_not_installed_qualification'}


def self_test(data, packet):
    # Mutate actual loaded records while preserving the independent RD01 oracle.
    scenarios = []
    def case(name, mutate, expected):
        changed = copy.deepcopy(data)
        mutate(changed)
        try:
            validate(changed)
        except (ValueError, KeyError, TypeError) as error:
            require(str(error).startswith(expected), 'unexpected_test_rejection', name + ': ' + str(error))
        else:
            raise ValueError('negative_test_accepted: ' + name)
        scenarios.append(name)
    case('omitted_path', lambda d: d['ownership'].pop(), 'census_coverage')
    case('duplicate_path', lambda d: d['ownership'].append(d['ownership'][0]), 'duplicate_path')
    case('corrupted_blob', lambda d: d['ownership'][0]['source'].update(git_object='0'*40), 'source_metadata')
    case('wrong_decision_group', lambda d: d['ownership'][0].update(decision_group='D-UNKNOWN'), 'decision_group')
    case('wrong_candidate', lambda d: d['contract'].update(source_revision='0'*40), 'accepted_rd01_candidate')
    case('wrong_mode', lambda d: d['ownership'][0]['source'].update(mode='100755'), 'source_metadata')
    case('unknown_owner', lambda d: d['ownership'][0].update(accountable_repository='unknown'), 'unknown_owner')
    case('unresolved_owner', lambda d: d['ownership'][0].update(proposed_disposition='retain_pending_consumer_mapping'), 'unresolved_owner')
    case('wrong_visibility', lambda d: d['contract']['repositories'][0].update(visibility='public'), 'wrong_visibility')
    case('missing_repository', lambda d: d['contract']['repositories'].pop(), 'repository_set')
    case('missing_registry_writer', lambda d: d['contract']['artifact_contracts'][0].pop('release_writer'), 'missing_artifact_authority')
    case('missing_rollback', lambda d: d['contract']['artifact_contracts'][0].pop('rollback'), 'missing_artifact_authority')
    case('missing_policy_authority', lambda d: d['contract']['pending_operator_policy'].pop('access_evidence_rollback_approver'), 'missing_policy_authority')
    case('fabricated_acceptance', lambda d: d['contract']['policy_acceptance'].update(status='approved'), 'fabricated_acceptance')
    case('fabricated_approval_reference', lambda d: d['contract']['policy_acceptance'].update(approval_reference='operator-said-yes'), 'fabricated_acceptance')
    case('private_public_inversion', lambda d: d['contract']['artifact_contracts'][1]['consumer_usage'].update({'agent-design-language':'runtime_or_build'}), 'public_private_inversion')
    case('opening_wrong_known_owner', lambda d: d['opening'][0].update(proposed_owner='agent-design-language'), 'opening_owner')
    case('opening_wrong_custody', lambda d: d['opening'][0].update(proposed_custody='original_repository_planning_evidence_custody'), 'opening_owner')
    case('opening_omitted', lambda d: d['opening'].pop(), 'opening_coverage')
    case('opening_blob_corrupt', lambda d: d['opening'][0]['after'].update(git_object='0'*40), 'opening_metadata')
    def synchronized_owner_drift(d):
        row = d['ownership'][0]
        row['accountable_repository'] = 'codefriend'
        row['mapping_detail'] = copy.deepcopy(row['mapping_detail'])
        row['mapping_detail']['owner'] = 'codefriend'
    case('synchronized_owner_drift', synchronized_owner_drift, 'mapping_precedence')
    case('invented_mapping_source', lambda d: d['ownership'][0].update(mapping_source='invented.json'), 'mapping_precedence')
    case('malformed_mapping_detail', lambda d: d['ownership'][0].update(mapping_detail=[]), 'mapping_detail_shape')
    case('malformed_policy_shape', lambda d: d['contract'].update(pending_operator_policy=list(POLICIES)), 'policy_shape')
    case('export_claim', lambda d: d['ownership'][0].update(export_authorized=True), 'fabricated_export_authority')
    def empty_split(d):
        row = next(r for r in d['ownership'] if r['proposed_disposition'] == 'split_integration_surface')
        row['mapping_detail']['component_splits'] = []
    case('empty_mixed_split', empty_split, 'empty_mixed_split')
    try:
        validate(data, require_policy=True)
    except ValueError as error:
        require(str(error).startswith('policy_acceptance_pending'), 'wrong_policy_failure')
    else:
        raise ValueError('pending_policy_accepted')
    scenarios.append('pending_policy_gate')
    # Exercise physical integrity and parser failures through the same loader as
    # the command. Copies are private disposable evidence, never packet edits.
    with tempfile.TemporaryDirectory(prefix='rd02-contract-') as temporary:
        parent = Path(temporary)
        copied = parent / packet.name
        shutil.copytree(packet, copied)
        shutil.copytree(packet.parent / 'rd01-1181', parent / 'rd01-1181')
        targets = [('mapping_bytes', copied / 'mapping-inputs' / 'mixed-generation-dispositions.json', 'mapping_digest'),
                   ('ownership_bytes', copied / 'ownership.jsonl.gz', 'ownership_digest'),
                   ('baseline_bytes', parent / 'rd01-1181' / 'census.jsonl.gz', 'baseline_digest'),
                   ('opening_gzip', copied / 'opening-ownership.jsonl.gz', None)]
        for name, target, expected in targets:
            original = target.read_bytes()
            target.write_bytes(b'corrupt packet bytes')
            try:
                load(copied)
            except (ValueError, OSError, EOFError) as error:
                require(expected is None or str(error).startswith(expected), 'unexpected_io_rejection', name)
            else:
                raise ValueError('corrupt_file_accepted: ' + name)
            finally:
                target.write_bytes(original)
            scenarios.append(name)
    return {'passed': len(scenarios), 'scenarios': scenarios}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--packet-dir', type=Path, default=Path(__file__).resolve().parent)
    parser.add_argument('--require-policy-acceptance', action='store_true')
    parser.add_argument('--self-test', action='store_true')
    args = parser.parse_args()
    try:
        data = load(args.packet_dir)
        result = validate(data, args.require_policy_acceptance)
        if args.self_test:
            result['adversarial_tests'] = self_test(data, args.packet_dir)
        print(json.dumps({'ok': True, **result}, sort_keys=True))
        return 0
    except (ValueError, KeyError, TypeError, AttributeError, OSError, EOFError) as error:
        print(json.dumps({'ok': False, 'error': str(error)}, sort_keys=True))
        return 1


if __name__ == '__main__':
    sys.exit(main())
