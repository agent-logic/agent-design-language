#!/usr/bin/env bash
# PVF: authorized live AWS/SSH proof, controlled external, small; not a release gate.
# This script never applies Terraform or expands ingress. Dispose using the saved issue state.
set -euo pipefail
exec python3 - "$@" <<'PY'
import argparse, ipaddress, json, os, pathlib, secrets, shlex, socket, subprocess, sys, time
p = argparse.ArgumentParser(description='Prove recovery and isolated application ingress on the approved #770 instance.')
p.add_argument('--instance-id', required=True)
p.add_argument('--identity-file', type=pathlib.Path, required=True)
p.add_argument('--expected-account', required=True)
p.add_argument('--expected-name', required=True)
p.add_argument('--key-name', required=True)
p.add_argument('--ssh-cidr', required=True)
p.add_argument('--region', default='us-west-2')
p.add_argument('--evidence-dir', type=pathlib.Path, required=True)
a = p.parse_args()
if os.environ.get('AWS_PROFILE') != 'agent-logic-admin': p.error('AWS_PROFILE must be agent-logic-admin')
net = ipaddress.ip_network(a.ssh_cidr, strict=True)
if net.version != 4 or net.prefixlen != 32: p.error('live proof requires one IPv4 /32')
if not a.identity_file.is_file(): p.error('selected identity file is unavailable')
a.evidence_dir.mkdir(parents=True, exist_ok=True)
def aws(*args):
    r = subprocess.run(['aws', '--profile', 'agent-logic-admin', '--region', a.region, *args, '--output', 'json'], capture_output=True, text=True, timeout=45)
    if r.returncode: raise RuntimeError('AWS readback failed')
    return json.loads(r.stdout)
def ssh(command):
    r = subprocess.run(['ssh', '-i', str(a.identity_file), '-o', 'BatchMode=yes', '-o', 'IdentitiesOnly=yes', '-o', 'ConnectTimeout=10', '-o', 'StrictHostKeyChecking=accept-new', '-o', 'UserKnownHostsFile='+str(a.evidence_dir/'known_hosts'), 'ec2-user@'+ip, command], capture_output=True, text=True, timeout=30)
    if r.returncode: raise RuntimeError('SSH command failed (command output withheld)')
    return r.stdout.strip()
record = {'issue':770, 'status':'failed', 'cloud_disposal':'pending; this probe does not destroy infrastructure', 'host_key_policy':'first-connection trust in isolated known_hosts; subsequent mismatches rejected'}
pid = None
try:
    if aws('sts', 'get-caller-identity')['Account'] != a.expected_account: raise RuntimeError('AWS account mismatch')
    instances = aws('ec2', 'describe-instances', '--instance-ids', a.instance_id)['Reservations'][0]['Instances']
    if len(instances) != 1: raise RuntimeError('expected exactly one instance')
    i = instances[0]
    if {t['Key']:t['Value'] for t in i.get('Tags', [])}.get('Name') != a.expected_name or 'issue-770-' not in a.expected_name: raise RuntimeError('issue-owned instance name mismatch')
    if i.get('KeyName') != a.key_name or i['State']['Name'] != 'running': raise RuntimeError('instance key or state mismatch')
    ip = str(ipaddress.IPv4Address(i['PublicIpAddress']))
    if i['MetadataOptions']['HttpTokens'] != 'required': raise RuntimeError('IMDSv2 not required')
    if len(i['SecurityGroups']) != 1: raise RuntimeError('unexpected additional security groups')
    sg = aws('ec2', 'describe-security-groups', '--group-ids', i['SecurityGroups'][0]['GroupId'])['SecurityGroups'][0]
    rules = sg['IpPermissions']
    if len(rules) != 1 or rules[0].get('IpProtocol') != 'tcp' or rules[0].get('FromPort') != 22 or rules[0].get('ToPort') != 22 or [r['CidrIp'] for r in rules[0].get('IpRanges',[])] != [a.ssh_cidr] or rules[0].get('Ipv6Ranges') or rules[0].get('UserIdGroupPairs') or rules[0].get('PrefixListIds'): raise RuntimeError('ingress differs from SSH-only approved /32')
    roots = [d['Ebs'] for d in i['BlockDeviceMappings'] if d['DeviceName']==i['RootDeviceName']]
    if len(roots)!=1 or not roots[0]['DeleteOnTermination']: raise RuntimeError('root volume disposal policy mismatch')
    volume = aws('ec2','describe-volumes','--volume-ids',roots[0]['VolumeId'])['Volumes'][0]
    if not volume['Encrypted']: raise RuntimeError('root volume not encrypted')
    record.update(instance_id=a.instance_id,public_ip=ip,root_volume_id=volume['VolumeId'],security_group_id=sg['GroupId'],ssh_cidr=a.ssh_cidr,imds_v2=True,encrypted_root=True,application_ingress=[])
    if ssh("printf ssh-recovery-ok") != 'ssh-recovery-ok': raise RuntimeError('SSH recovery marker mismatch')
    # Empty directory only, temporary non-TLS listener; no Runtime/TLS claim.
    client = ssh('printf "%s" "$SSH_CONNECTION"').split()[0]
    if ipaddress.ip_address(client) != net.network_address: raise RuntimeError('SSH source is not the approved /32')
    record['observed_ssh_source'] = client
    nonce = secrets.token_hex(24)
    server = f"""import http.server, threading
class Handler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.end_headers()
        self.wfile.write({nonce!r}.encode())
    def log_message(self, *args): pass
server = http.server.HTTPServer(('0.0.0.0', 20997), Handler)
threading.Timer(120, lambda: __import__('os')._exit(0)).start()
server.serve_forever()
"""
    launcher = "import subprocess; p = subprocess.Popen(['python3', '-c', " + repr(server) + "], stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True); print(p.pid)"
    pid = ssh('python3 -c ' + shlex.quote(launcher))
    if not pid.isdigit(): raise RuntimeError('probe PID invalid')
    local_probe = 'python3 -c ' + shlex.quote("import urllib.request; print(urllib.request.urlopen('http://127.0.0.1:20997', timeout=5).read().decode())")
    for attempt in range(10):
        try:
            if ssh(local_probe) == nonce: break
        except RuntimeError:
            if attempt == 9: raise
            time.sleep(1)
    else: raise RuntimeError('local health probe failed')
    try:
        with socket.create_connection((ip,20997),timeout=5): pass
    except TimeoutError: pass
    else: raise RuntimeError('application port unexpectedly reachable from recovery address')
    if ssh(local_probe) != nonce: raise RuntimeError('listener stopped during isolation proof')
    record.update(status='passed',ssh_authenticated=True,local_health=True,external_application_port_blocked=True)
except Exception as error:
    record['error'] = str(error) if isinstance(error,RuntimeError) else type(error).__name__
finally:
    if pid and pid.isdigit():
        try:
            cleanup = f"""import os, pathlib, signal, time
p = pathlib.Path('/proc/{pid}')
def alive():
    if not p.exists(): return False
    return (p / 'stat').read_text().rsplit(')', 1)[1].split()[0] != 'Z'
if alive():
    if {nonce!r}.encode() not in (p / 'cmdline').read_bytes():
        raise RuntimeError('listener ownership mismatch')
    os.kill({pid}, signal.SIGTERM)
for attempt in range(20):
    if not alive(): break
    time.sleep(0.1)
else: raise RuntimeError('listener did not stop')
print('listener-stopped')
"""
            if ssh('python3 -c ' + shlex.quote(cleanup)) != 'listener-stopped':
                raise RuntimeError('listener cleanup not verified')
            record['temporary_listener_stopped'] = True
        except Exception:
            record['temporary_listener_stopped'] = False
            record.update(status='failed', cleanup_error='temporary listener cleanup not verified')
    # The bounded listener also expires automatically; destroy the instance next.
    (a.evidence_dir/'recovery-proof.json').write_text(json.dumps(record,indent=2)+'\n')
print(json.dumps(record))
if record['status'] != 'passed': sys.exit(1)
PY
