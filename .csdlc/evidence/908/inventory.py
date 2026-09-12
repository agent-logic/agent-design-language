#!/usr/bin/env python3
"""Issue #908: read-only, allowlisted AWS census; raw responses stay in memory."""
import concurrent.futures
import datetime as dt
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[2]
BASE = ROOT / 'docs/milestones/v0.92.1/evidence/cloud/aws-a/readbacks'
PROFILE = 'agent-logic-admin'
# Matches #484's denominator exactly; supplementary tags cover each current region.
REGIONAL = {
    'ec2-instances': ('ec2', 'describe-instances', 'Reservations', 'InstanceId'),
    'ec2-volumes': ('ec2', 'describe-volumes', 'Volumes', 'VolumeId'),
    'vpcs': ('ec2', 'describe-vpcs', 'Vpcs', 'VpcId'),
    'subnets': ('ec2', 'describe-subnets', 'Subnets', 'SubnetId'),
    'security-groups': ('ec2', 'describe-security-groups', 'SecurityGroups', 'GroupId'),
    'load-balancers': ('elbv2', 'describe-load-balancers', 'LoadBalancers', 'LoadBalancerArn'),
    'acm-certificates': ('acm', 'list-certificates', 'CertificateSummaryList', 'CertificateArn'),
    'cloudformation-stacks': ('cloudformation', 'list-stacks', 'StackSummaries', 'StackId'),
    'tagged-resources': ('resourcegroupstaggingapi', 'get-resources', 'ResourceTagMappingList', 'ResourceARN'),
}
GLOBAL = {
    's3-buckets': ('s3api', 'list-buckets', 'Buckets', 'Name'),
    'route53-hosted-zones': ('route53', 'list-hosted-zones', 'HostedZones', 'Id'),
    'cloudfront-distributions': ('cloudfront', 'list-distributions', 'DistributionList', 'Id'),
    'global-tagged-resources': REGIONAL['tagged-resources'],
}
ALLOW = {(x[0], x[1]) for x in [*REGIONAL.values(), *GLOBAL.values()]} | {
    ('sts', 'get-caller-identity'), ('ec2', 'describe-regions'),
    ('s3api', 'get-bucket-location'), ('s3api', 'get-bucket-tagging'),
    ('s3api', 'list-objects-v2'),
}

def now():
    return dt.datetime.now(dt.timezone.utc).isoformat()

def digest(value):
    return hashlib.sha256(value.encode()).hexdigest()

def aws(service, action, *args):
    assert (service, action) in ALLOW, 'mutation/non-allowlisted API rejected'
    env = os.environ.copy()
    # Explicit profile only; never inherit ambient access-key or endpoint overrides.
    for key in list(env):
        if key.startswith('AWS_'):
            del env[key]
    env['AWS_PAGER'] = ''
    cmd = ['aws', '--profile', PROFILE, '--region', 'us-west-2', '--output', 'json',
           '--cli-connect-timeout', '10', '--cli-read-timeout', '30', service, action, *args]
    started = now()
    try:
        result = subprocess.run(cmd, env=env, capture_output=True, text=True, timeout=120)
        if result.returncode:
            match = re.search(r'\(([A-Za-z0-9]+)\)', result.stderr)
            return {'status': 'read-failed', 'error_code': match.group(1) if match else 'aws_cli_failed',
                    'started_at': started, 'captured_at': now()}, None
        data = json.loads(result.stdout)
        return {'status': 'observed', 'started_at': started, 'captured_at': now()}, data
    except (subprocess.TimeoutExpired, json.JSONDecodeError, FileNotFoundError):
        return {'status': 'read-failed', 'error_code': 'timeout_or_invalid_response',
                'started_at': started, 'captured_at': now()}, None

def identity_ok(actual, expected):
    return bool(actual and re.fullmatch(r'\d{12}', expected or '') and actual.get('Account') == expected
                and actual.get('Arn', '').startswith('arn:aws:'))

def items(data, surface, key):
    if surface == 'ec2-instances':
        return [x for r in data.get('Reservations', []) for x in r.get('Instances', [])]
    if surface == 'cloudfront-distributions':
        distributions = data.get('DistributionList')
        if not isinstance(distributions, dict) or not isinstance(distributions.get('Items'), list):
            raise ValueError('invalid CloudFront response')
        # AWS CLI pagination aggregates Items and omits per-page Quantity.
        values = distributions['Items']
        return values
    return data.get(key, [])

def project(data, surface, spec):
    if data is None:
        return []
    if spec[2] not in data:
        raise ValueError('required response field absent')
    if surface != 'cloudfront-distributions' and not isinstance(data[spec[2]], list):
        raise ValueError('invalid response collection')
    result = []
    for x in items(data, surface, spec[2]):
        rid = x.get(spec[3])
        if not rid:
            raise ValueError('missing resource identity')
        row = {'resource_ref': digest(rid), 'disposition': 'frozen-unknown'}
        # Preserve only selected operational state; never copy arbitrary tag values or names.
        state = x.get('State')
        if isinstance(state, dict): state = state.get('Name')
        if state in {'running', 'stopped', 'terminated', 'pending', 'stopping', 'shutting-down',
                     'available', 'in-use', 'creating', 'deleting', 'deleted', 'error'}:
            row['state'] = state
        for k in ('CreateTime', 'CreationDate', 'LaunchTime'):
            if k in x and isinstance(x[k], str): row['resource_timestamp'] = x[k]
        if surface == 's3-buckets':
            row['purpose_hint'] = ('scr' if 'strategic-cognitive-reserve' in rid or rid.startswith('scr-')
                                   else 'model-artifacts' if 'model' in rid else 'other')
        result.append(row)
    return sorted(result, key=lambda x: x['resource_ref'])

def capture_surface(job):
    region, surface, spec = job
    args = [] if region == 'global' else ['--region', region]
    if surface == 'cloudformation-stacks':
        args += ['--stack-status-filter', 'CREATE_COMPLETE', 'UPDATE_COMPLETE', 'UPDATE_ROLLBACK_COMPLETE', 'IMPORT_COMPLETE']
    meta, data = aws(spec[0], spec[1], *args)
    meta.update(region=region, surface=surface, command=[spec[0], spec[1], *args])
    if data is not None:
        try:
            meta['resources'] = project(data, surface, spec)
            meta['status'] = 'observed' if meta['resources'] else 'not-observed'
        except (ValueError, KeyError, TypeError):
            meta.update(status='read-failed', error_code='invalid_response_shape')
            data = None
    return meta, data

def baseline_surfaces():
    out = {}
    for surface, spec in GLOBAL.items():
        p = BASE / (surface + '.json')
        data = json.loads(p.read_text())
        out['global/' + surface] = project(data, surface, spec)
    for p in (BASE / 'regions').glob('*.json'):
        for surface, spec in REGIONAL.items():
            if p.stem.endswith('-' + surface):
                region = p.stem[:-len(surface)-1]
                out[region + '/' + surface] = project(json.loads(p.read_text()), surface, spec)
                break
    return out

def make_delta(observations):
    previous = baseline_surfaces()
    current = {x['region'] + '/' + x['surface']: x for x in observations}
    rows = []
    for key in sorted(previous.keys() | current.keys()):
        old = {x['resource_ref'] for x in previous.get(key, [])}
        new = current.get(key)
        if new is None or new['status'] == 'read-failed':
            rows.append({'surface_ref': key, 'status': 'read-failed' if new else 'not-surveyed',
                         'baseline_count': len(old), 'baseline_resources': sorted(old),
                         'absence_claim': False})
            continue
        live = {x['resource_ref'] for x in new['resources']}
        rows.append({'surface_ref': key, 'status': new['status'], 'captured_at': new['captured_at'],
                     'baseline_count': len(old), 'current_count': len(live),
                     'new': sorted(live-old), 'not_observed_now': sorted(old-live),
                     'retained': sorted(old & live), 'deletion_proven': False,
                     'baseline_surface': key in previous})
    return rows

def capture():
    expected = json.loads((BASE / 'account-identity.json').read_text())['Account']
    meta, identity = aws('sts', 'get-caller-identity')
    verified = identity_ok(identity, expected)
    packet = {'schema': 'adl.aws.inventory.v1', 'issue': 908, 'umbrella': 934,
              'profile': PROFILE, 'started_at': meta['started_at'],
              'identity': {'verified_business_account': verified, 'captured_at': meta['captured_at'],
                           'basis': 'live STS equality with approved #484 business baseline',
                           'status': meta['status']}, 'cloud_mutation': False}
    if not verified:
        (HERE / 'identity-blocker.json').write_text(json.dumps(packet, indent=2)+'\n')
        raise SystemExit('business identity verification failed; no inventory reads attempted')
    region_meta, region_data = aws('ec2', 'describe-regions', '--all-regions')
    if not region_data or not region_data.get('Regions'):
        raise SystemExit('region discovery failed; inventory stopped')
    packet['regions'] = sorted([{'name': x['RegionName'], 'opt_in': x['OptInStatus']} for x in region_data['Regions']], key=lambda x:x['name'])
    packet['region_capture'] = region_meta
    enabled = [x['name'] for x in packet['regions'] if x['opt_in'] in ('opt-in-not-required', 'opted-in')]
    jobs = [('global', surface, spec) for surface, spec in GLOBAL.items()]
    jobs += [(region, surface, spec) for region in enabled for surface, spec in REGIONAL.items()]
    with concurrent.futures.ThreadPoolExecutor(max_workers=6) as pool:
        results = list(pool.map(capture_surface, jobs))
    packet['surfaces'] = [x[0] for x in results]
    # Bucket metadata only. Object content, keys, tag values and account IDs are never retained.
    buckets = next((d['Buckets'] for m,d in results if m['surface']=='s3-buckets' and d), [])
    packet['buckets'] = []
    for bucket in buckets:
        name = bucket['Name']
        row = {'resource_ref': digest(name), 'disposition': 'frozen-unknown'}
        for action in ['get-bucket-location', 'get-bucket-tagging', 'list-objects-v2']:
            args = ['--bucket', name]
            if action == 'list-objects-v2': args += ['--max-items', '10000']
            m, data = aws('s3api', action, *args)
            if data is not None:
                if action == 'get-bucket-location': m['region'] = data.get('LocationConstraint') or 'us-east-1'
                elif action == 'get-bucket-tagging': m['tag_count'] = len(data.get('TagSet', []))
                else:
                    objects = data.get('Contents', [])
                    dates = sorted(x['LastModified'] for x in objects)
                    m.update(observed_object_count=len(objects), observed_bytes=sum(x['Size'] for x in objects),
                             newest_object=dates[-1] if dates else None, oldest_object=dates[0] if dates else None,
                             complete=not bool(data.get('NextToken') or data.get('NextContinuationToken') or data.get('IsTruncated')))
            row[action] = m
        packet['buckets'].append(row)
    packet['delta'] = make_delta(packet['surfaces'])
    packet['completed_at'] = now()
    packet['collector_sha256'] = hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
    packet['source_revision'] = subprocess.check_output(['git','rev-parse','HEAD'],cwd=ROOT,text=True).strip()
    packet['baseline_hashes'] = {str(p.relative_to(ROOT)): hashlib.sha256(p.read_bytes()).hexdigest()
                                 for p in sorted(BASE.rglob('*')) if p.is_file()}
    for script in ['run-readonly-inventory.sh', 'build-inventory-summary.sh']:
        path = BASE.parent / script
        packet['baseline_hashes'][str(path.relative_to(ROOT))] = hashlib.sha256(path.read_bytes()).hexdigest()
    packet['baseline_hashes']['docs/operations/cloud/aws/inventory/AWS_RESOURCE_OWNERSHIP_INVENTORY.md'] = hashlib.sha256((ROOT/'docs/operations/cloud/aws/inventory/AWS_RESOURCE_OWNERSHIP_INVENTORY.md').read_bytes()).hexdigest()
    (HERE / 'inventory.json').write_text(json.dumps(packet, indent=2)+'\n')
    print(json.dumps({'surfaces': len(packet['surfaces']), 'enabled_regions':len(enabled), 'buckets':len(buckets),
                      'read_failures':sum(x['status']=='read-failed' for x in packet['surfaces'])}))

def validate(packet, reference=None):
    reference = reference or dt.datetime.now(dt.timezone.utc)
    assert packet['profile'] == PROFILE and packet['identity']['verified_business_account'] is True, 'wrong account'
    assert packet['cloud_mutation'] is False
    times = [packet['identity']['captured_at'], packet['completed_at'], packet['region_capture']['captured_at'], *[x['captured_at'] for x in packet['surfaces']]]
    times += [b[a]['captured_at'] for b in packet['buckets'] for a in ['get-bucket-location','get-bucket-tagging','list-objects-v2']]
    for value in times:
        age = (reference-dt.datetime.fromisoformat(value)).total_seconds()
        assert -300 <= age <= 86400, 'stale or future evidence'
    enabled = [x['name'] for x in packet['regions'] if x['opt_in'] in ('opt-in-not-required','opted-in')]
    assert enabled, 'missing regions'
    required = {('global',s) for s in GLOBAL} | {(r,s) for r in enabled for s in REGIONAL}
    observed = {(x['region'],x['surface']) for x in packet['surfaces']}
    assert observed == required and len(observed)==len(packet['surfaces']), 'missing or duplicate surface'
    for row in packet['surfaces']:
        assert row['status'] in {'observed','not-observed','read-failed'}
        if row['status']=='read-failed': assert 'resources' not in row, 'failure conflated with absence'
        else:
            assert (row['status']=='observed') == bool(row['resources'])
            for resource in row['resources']:
                assert resource['disposition'] in {'owned','externally-owned','frozen-unknown'}
                assert re.fullmatch('[a-f0-9]{64}',resource['resource_ref'])
    bucket_ids={x['resource_ref'] for x in next(x for x in packet['surfaces'] if x['surface']=='s3-buckets').get('resources',[])}
    assert {x['resource_ref'] for x in packet['buckets']}==bucket_ids, 'missing bucket'
    for row in packet['buckets']:
        for action in ['get-bucket-location','get-bucket-tagging','list-objects-v2']:
            assert row[action]['status'] in {'observed','read-failed'}, 'missing bucket surface'
    assert packet['delta']==make_delta(packet['surfaces']), 'delta mismatch'
    for path, sha in packet['baseline_hashes'].items():
        assert hashlib.sha256((ROOT/path).read_bytes()).hexdigest()==sha, 'baseline changed'
    assert packet['collector_sha256'] == hashlib.sha256(Path(__file__).read_bytes()).hexdigest(), 'collector changed'
    numeric_keys = {'issue','umbrella','baseline_count','current_count','tag_count','observed_object_count','observed_bytes'}
    def check_numbers(value, key=None):
        if isinstance(value, bool): return
        if isinstance(value, (int,float)):
            assert key in numeric_keys and isinstance(value,int) and value >= 0, 'unexpected numeric field'
        elif isinstance(value, dict):
            for k,v in value.items(): check_numbers(v,k)
        elif isinstance(value, list):
            for v in value: check_numbers(v,key)
    check_numbers(packet)
    def strings(value):
        if isinstance(value, str): return [value]
        if isinstance(value, list): return [s for x in value for s in strings(x)]
        if isinstance(value, dict): return [s for k,v in value.items() for s in [k, *strings(v)]]
        return []
    raw='\n'.join(strings(packet))
    assert not re.search(r'(?<![a-f0-9])\d{12}(?![a-f0-9])|arn:aws:|AKIA[A-Z0-9]{16}|ASIA[A-Z0-9]{16}|/Users/|/Volumes/',raw), 'redaction failure'
    return {'status':'passed','surface_count':len(required),'identity':'verified','freshness_seconds':86400}

if __name__ == '__main__':
    if sys.argv[1:] == ['capture']: capture()
    elif sys.argv[1:] == ['validate']: print(json.dumps(validate(json.loads((HERE/'inventory.json').read_text()))))
    else: raise SystemExit('usage: inventory.py capture|validate')
