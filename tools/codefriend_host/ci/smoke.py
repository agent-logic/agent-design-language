#!/usr/bin/env python3
"""Bounded installed-binary proof with empty credentials and a local provider trap."""
import argparse
import hashlib
import http.client
import json
import os
from pathlib import Path
import re
import signal
import socket
import stat
import struct
import subprocess
import sys
import time


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def run(installed, candidate, output):
    require(re.fullmatch('[0-9a-f]{40}', candidate), 'exact candidate required')
    installed, output = installed.resolve(), output.absolute()
    output.mkdir(mode=0o700)
    old_cwd = Path.cwd()
    cases, stops, previous_instance = [], 0, None
    provider = socket.socket()
    provider.bind(('127.0.0.1', 0)); provider.listen(); provider.setblocking(False)
    try:
        os.chdir(output)
        private = output / 'credentials.json'; private.write_text('[]\n'); private.chmod(0o600)
        route = {'provider_kind':'hosted','provider':'openai','runtime_surface':'hosted_api',
                 'provider_model_id':'fixture-model','endpoint_ref':f'http://127.0.0.1:{provider.getsockname()[1]}',
                 'credential_ref':'env:CODEFRIEND_NONEXISTENT_FIXTURE_KEY'}
        model = {'provider_kind':'hosted','provider':'openai','runtime_surface':'hosted_api',
                 'provider_model_id':'fixture-model','model_ref':'fixture-model','identity_strength':'provider_asserted',
                 'observed_at':'2026-09-18T00:00:00Z'}
        config = {'root':str(output/'state'),'credentials_file':str(private),'candidate_revision':candidate,
                  'max_concurrent':1,'max_operations_per_subject':1,'retention_seconds':60,
                  'provider':{'route':route,'model_identity':model,'prompt_contract_ref':'fixture',
                              'lane_ref':'fixture','attempt_policy':{'max_attempts':1,'timeout_ms':1000},'max_output_tokens':1}}
        (output/'config.json').write_text(json.dumps(config)); (output/'config.json').chmod(0o600)
        env = {'PATH':'/usr/bin:/bin','HOME':str(output)}  # No provider/cloud credentials inherited.
        for cycle in range(2):
            with (output/f'gateway-{cycle}.log').open('wb') as log:
                proc = subprocess.Popen([str(installed/'codefriend-server'),'--config',str(output/'config.json'),
                                         '--listen','127.0.0.1:0','--control-socket','./gateway.sock'],
                                        env=env,stdout=subprocess.DEVNULL,stderr=log)
                identity = None
                try:
                    deadline = time.monotonic()+15
                    while not Path('gateway.sock').exists():
                        require(proc.poll() is None, 'gateway startup failed')
                        require(time.monotonic()<deadline, 'gateway startup timeout'); time.sleep(.05)
                    identity = Path('gateway.sock').lstat()
                    def call(action, instance=None, attempt=None):
                        request = {'schema':'codefriend.host_control.v1','action':action}
                        if instance is not None: request.update(instance=instance,attempt=attempt)
                        with socket.socket(socket.AF_UNIX) as client:
                            client.settimeout(2); client.connect('./gateway.sock')
                            if sys.platform.startswith('linux'):
                                pid, _, _ = struct.unpack('3i',client.getsockopt(socket.SOL_SOCKET,socket.SO_PEERCRED,12))
                                require(pid==proc.pid,'kernel peer PID mismatch')
                            client.sendall(json.dumps(request).encode()+b'\n')
                            data=bytearray(); limit=time.monotonic()+2
                            while b'\n' not in data:
                                remaining=limit-time.monotonic();require(remaining>0,'control deadline')
                                client.settimeout(remaining);chunk=client.recv(4097-len(data))
                                require(chunk,'truncated reply');data.extend(chunk);require(len(data)<=4096,'oversized reply')
                            reply=json.loads(data)
                            if reply.get('ok'):
                                require(reply.get('pid')==proc.pid and reply.get('candidate_revision')==candidate,'response identity')
                            return reply
                    status=call('status');require(status['ok'] and status['quiescent_without_payloads'] and not status['draining'],'fresh status')
                    instance=status['instance'];require(instance!=previous_instance,'restart instance reused')
                    cases.append(f'cycle{cycle}_identity_and_readonly_quiescence')
                    if previous_instance:
                        require(not call('drain',previous_instance,'a'*32)['ok'],'stale instance admitted')
                        cases.append('stale_instance_rejected')
                    require(call('drain',instance,'a'*32)['drained_without_payloads'],'drain failed')
                    require(not call('resume',instance,'b'*32)['ok'],'foreign resume admitted')
                    require(not call('resume',instance,'a'*32)['draining'],'resume failed')
                    require(not call('drain',instance,'a'*32)['ok'],'retired attempt admitted')
                    cases.extend([f'cycle{cycle}_drain',f'cycle{cycle}_foreign_resume_rejected',f'cycle{cycle}_resume',f'cycle{cycle}_retired_attempt_rejected'])
                    while True:
                        match=re.search(r'event=listening address=127\.0\.0\.1:(\d+)',(output/f'gateway-{cycle}.log').read_text())
                        if match: break
                        require(proc.poll() is None and time.monotonic()<deadline,'HTTP readiness timeout');time.sleep(.05)
                    client=http.client.HTTPConnection('127.0.0.1',int(match[1]),timeout=3)
                    try:
                        client.request('POST','/v1/operations',body=b'{',headers={'Content-Type':'application/json'})
                        response=client.getresponse();require(response.status==401,'authentication did not precede malformed body');response.read(4096)
                    finally:client.close()
                    cases.append(f'cycle{cycle}_unauthenticated_malformed_request_denied')
                    previous_instance=instance
                finally:
                    if proc.poll() is None:
                        proc.send_signal(signal.SIGINT)
                        try: proc.wait(timeout=10)
                        except subprocess.TimeoutExpired: proc.kill();proc.wait();raise RuntimeError('graceful stop timeout')
                    require(proc.returncode==0,'gateway did not exit cleanly')
                    stops+=1
                    if identity is not None:
                        after=Path('gateway.sock').lstat()
                        require(stat.S_ISSOCK(after.st_mode) and (identity.st_dev,identity.st_ino)==(after.st_dev,after.st_ino),'socket replaced')
                        Path('gateway.sock').unlink()  # Only our inode after confirmed process exit.
        report=json.loads((Path(__file__).resolve().parent/'interrupted-report.json').read_text())
        path=output/'report.json';path.write_text(json.dumps(report));path.chmod(0o600)
        command=[str(installed/'codefriend-agent'),'verify-report','--report-file',str(path)]
        verified=subprocess.run(command,capture_output=True,text=True,env=env,timeout=10)
        require(verified.returncode==0 and json.loads(verified.stdout)==report,'valid report was not accepted')
        cases.append('installed_agent_accepts_valid_interrupted_report')
        report['status']='complete';path.write_text(json.dumps(report))
        rejected=subprocess.run(command,capture_output=True,text=True,env=env,timeout=10)
        require(rejected.returncode!=0 and not rejected.stdout,'tampered report was accepted')
        cases.append('installed_agent_rejects_tampered_report')
        cli=subprocess.run([str(installed/'adl'),'codefriend','--help'],
                           capture_output=True,text=True,env=env,timeout=10)
        require(cli.returncode==0 and 'adl codefriend journey resume' in cli.stdout,
                'installed ADL CodeFriend entrypoint unavailable')
        cases.append('installed_adl_exposes_codefriend_journey')
        require(not list((output/'state'/'operations').iterdir()),'unexpected reservation')
        try:
            connection,_=provider.accept();connection.close();raise RuntimeError('unexpected provider connection')
        except BlockingIOError:pass
        result={'schema':'codefriend.host_smoke.v1','status':'PASS','candidate':candidate,'platform':sys.platform,
                'cases':cases,'graceful_stops':stops,'provider_connections':0,'operation_reservations':0,
                'binary_sha256':{name:hashlib.sha256((installed/name).read_bytes()).hexdigest() for name in ('codefriend-server','codefriend-agent','adl')},
                'non_claims':['real provider reviews','OAuth','systemd supervision','host poweroff','deployment']}
        (output/'result.json').write_text(json.dumps(result,indent=2)+'\n')
        print(json.dumps({'status':'PASS','cases':len(cases),'candidate':candidate}))
    finally:
        provider.close();os.chdir(old_cwd)

if __name__=='__main__':
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--installed',type=Path,required=True);parser.add_argument('--candidate',required=True)
    parser.add_argument('--output',type=Path,required=True);args=parser.parse_args()
    os.umask(0o077)
    run(args.installed,args.candidate,args.output)
