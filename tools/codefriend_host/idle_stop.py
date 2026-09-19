#!/usr/bin/env python3
"""Private Linux-only idle controller. Default invocation only observes."""
import argparse
import fcntl
import json
import math
import os
from pathlib import Path
import re
import secrets
import socket
import stat
import struct
import subprocess
import sys
import time

SCHEMA = 'codefriend.host_control.v1'
SERVICES = ('website', 'gateway')
CONFIG = Path('/etc/codefriend/idle-stop.json')
LOCK = '/run/codefriend-idle-stop/controller.lock'

class Unsafe(RuntimeError):
    pass

def require(condition, reason):
    if not condition:
        raise Unsafe(reason)

def secure_file(path):
    for parent in (path, *path.parents):
        info = parent.lstat()
        require(not stat.S_ISLNK(info.st_mode) and info.st_uid == 0 and not info.st_mode & 0o022,
                'configuration path must be root-owned and not writable by others')
    require(path.is_file(), 'configuration must be a regular file')

def load_config():
    secure_file(CONFIG)
    require(CONFIG.stat().st_size <= 4096, 'configuration too large')
    config = json.loads(CONFIG.read_text())
    require(set(config) == {'enabled', 'revisions'}, 'invalid configuration fields')
    require(type(config['enabled']) is bool and set(config['revisions']) == set(SERVICES), 'invalid configuration')
    for revision in config['revisions'].values():
        require(isinstance(revision, str) and re.fullmatch('[0-9a-f]{40}', revision), 'invalid revision')
    return config

class Supervisor:
    def command(self, *args):
        result = subprocess.run(['/usr/bin/systemctl', *args], check=True, capture_output=True,
                                text=True, timeout=45, env={'PATH': '/usr/bin:/bin', 'LANG': 'C'})
        return result.stdout

    def state(self, service):
        lines = self.command('show', f'codefriend-{service}.service',
                             '--property=MainPID,ActiveState,SubState,InvocationID').splitlines()
        values = dict(line.split('=', 1) for line in lines)
        return (int(values['MainPID']), values['ActiveState'], values['SubState'], values['InvocationID'])

    def stop(self, service):
        self.command('stop', f'codefriend-{service}.service')

    def poweroff(self):
        self.command('poweroff')

class Transport:
    def request(self, service, pid, request):
        path = f'/run/codefriend-{service}/control.sock'
        with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as client:
            deadline = time.monotonic() + 2
            client.settimeout(2)
            client.connect(path)
            peer_pid, _, _ = struct.unpack('3i', client.getsockopt(socket.SOL_SOCKET, socket.SO_PEERCRED, 12))
            require(peer_pid == pid, 'control peer differs from supervisor')
            client.sendall(json.dumps(request, separators=(',', ':')).encode() + b'\n')
            payload = bytearray()
            while b'\n' not in payload:
                remaining = deadline - time.monotonic()
                require(remaining > 0, 'control deadline exceeded')
                client.settimeout(remaining)
                chunk = client.recv(4097 - len(payload))
                require(bool(chunk), 'truncated control reply')
                payload.extend(chunk)
                require(len(payload) <= 4096, 'oversized control reply')
            require(payload.endswith(b'\n') and payload.count(b'\n') == 1, 'invalid control framing')
            return json.loads(payload)

class Controller:
    def __init__(self, supervisor, transport, revisions):
        self.supervisor, self.transport, self.revisions = supervisor, transport, revisions
        self.identities = {}
        self.instances = {}
        self.attempt = secrets.token_hex(16)
        self.owned = []

    def current(self, service):
        state = self.supervisor.state(service)
        require(state[0] > 0 and state[1:3] == ('active', 'running') and state[3], 'service is not stable and running')
        if service in self.identities:
            require(state == self.identities[service], 'supervisor identity changed')
        else:
            self.identities[service] = state
        return state

    def control(self, service, action):
        before = self.current(service)
        request = {'schema': SCHEMA, 'action': action}
        if action != 'status':
            request.update(instance=self.instances[service], attempt=self.attempt)
        reply = self.transport.request(service, before[0], request)
        require(self.current(service) == before, 'service changed during control')
        require(reply.get('schema') == SCHEMA and reply.get('ok') is True and reply.get('service') == service,
                'invalid control response')
        require(type(reply.get('pid')) is int and reply['pid'] == before[0], 'wrong response PID')
        require(reply.get('candidate_revision') == self.revisions[service], 'wrong candidate')
        instance = reply.get('instance')
        require(isinstance(instance, str) and re.fullmatch('[0-9a-f]{64}', instance), 'invalid instance')
        if service in self.instances:
            require(instance == self.instances[service], 'service instance changed')
        else:
            self.instances[service] = instance
        require('attempt' in reply, 'missing attempt status')
        require(type(reply.get('draining')) is bool and type(reply.get('drained_without_payloads')) is bool,
                'missing drain status')
        return reply

    def idle(self, reply):
        value = reply.get('idle_seconds')
        require(type(value) in (int, float) and math.isfinite(value) and value >= 1800, 'website is not idle')

    def safe(self, service):
        reply = self.control(service, 'status')
        require(reply['draining'] and reply.get('attempt') == self.attempt and reply['drained_without_payloads'],
                'drain is not owned, payload-free and certain')
        if service == 'website':
            self.idle(reply)

    def run(self, execute=False):
        try:
            for service in SERVICES:
                reply = self.control(service, 'status')
                require(not reply['draining'] and reply.get('attempt') is None, 'another drain is active')
                require(reply.get('quiescent_without_payloads') is True, 'service has retained or uncertain work')
                if service == 'website':
                    self.idle(reply)
            if not execute:
                return 'idle_observed_no_mutations'
            for service in SERVICES:
                # A lost reply may still mean drain took effect: retain ownership
                # before sending so abort attempts the exact bound resume.
                self.owned.append(service)
                reply = self.control(service, 'drain')
                require(reply['draining'] and reply.get('attempt') == self.attempt, 'drain not acknowledged')
            for service in SERVICES:
                self.safe(service)
            for service in SERVICES:
                self.safe(service)
                self.supervisor.stop(service)
                self.stopped(service)
            for service in SERVICES:
                self.stopped(service)
            self.supervisor.poweroff()
            return 'poweroff_requested'
        except Exception:
            for service in reversed(self.owned):
                try:
                    self.control(service, 'resume')
                except Exception:
                    pass  # Never resume another instance or attempt.
            raise

    def stopped(self, service):
        state = self.supervisor.state(service)
        require(state[0] == 0 and state[1:3] == ('inactive', 'dead') and
                state[3] in ('', self.identities[service][3]), 'service did not stop or changed instance')

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--execute', action='store_true')
    args = parser.parse_args()
    require(sys.platform.startswith('linux') and os.geteuid() == 0, 'requires Linux root supervisor')
    config = load_config()
    require(not args.execute or config['enabled'], 'shutdown is disabled')
    descriptor = os.open(LOCK, os.O_CREAT | os.O_RDWR | os.O_NOFOLLOW, 0o600)
    with os.fdopen(descriptor, 'w') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        result = Controller(Supervisor(), Transport(), config['revisions']).run(args.execute)
        print(json.dumps({'status': result}))

if __name__ == '__main__':
    try:
        main()
    except Exception:
        print(json.dumps({'status': 'stop_denied'}))
        sys.exit(1)
