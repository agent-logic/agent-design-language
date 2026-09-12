#!/usr/bin/env python3
"""PVF installed integration: local fixtures or separately approved hosted mode.

Requires Python 3.11+ and cryptography. Every run uses a new retained output root.
No fixed fixture cleanup, production source changes, or service installation.
"""
import argparse
import base64
import ctypes
import datetime as dt
import hashlib
import http.server
import ipaddress
import json
import os
import plistlib
from pathlib import Path
import secrets
import signal
import socket
import ssl
import struct
import subprocess
import threading
import time
import tomllib
import urllib.request

from cryptography import x509
from cryptography.hazmat.primitives import hashes, serialization
from cryptography.hazmat.primitives.asymmetric import ec, ed25519
from cryptography.x509.oid import ExtendedKeyUsageOID, NameOID

REPO = Path(__file__).resolve().parents[2]
PROVIDERS = ('openai', 'anthropic', 'vertex_ai', 'ollama', 'openai-compatible')
HOSTED_MODELS = {'openai': 'gpt-5.4-mini', 'anthropic': 'claude-haiku-4-5-20251001', 'vertex_ai': 'gemini-2.5-flash'}
HOSTED_ENVS = {'openai': 'OPENAI_API_KEY', 'anthropic': 'ANTHROPIC_API_KEY', 'vertex_ai': 'ADL_VERTEX_ACCESS_TOKEN'}
APPROVED_PROJECT = 'cs-host-377d41e71a824f92802120'


def require(condition, message):
    if not condition:
        raise AssertionError(message)


def write(path, value, secret=False):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(value if isinstance(value, str) else json.dumps(value, indent=2) + '\n')
    if secret:
        path.chmod(0o600)
    return path


def run(argv, **kwargs):
    result = subprocess.run([str(x) for x in argv], capture_output=True, text=True,
                            timeout=90, **kwargs)
    if result.returncode:
        raise RuntimeError(f'{Path(str(argv[0])).name} failed ({result.returncode}): {result.stderr[-3000:]}')
    return result.stdout


def port():
    with socket.socket() as sock:
        sock.bind(('127.0.0.1', 0))
        return sock.getsockname()[1]


def toml(document):
    lines = []
    def table(value, prefix):
        if prefix:
            lines.append('[' + '.'.join(prefix) + ']')
        for key, item in value.items():
            if not isinstance(item, dict):
                lines.append(f'{key} = {json.dumps(item)}')
        for key, item in value.items():
            if isinstance(item, dict):
                table(item, prefix + [key])
    table(document, [])
    return '\n'.join(lines) + '\n'


def certificates(root):
    now = dt.datetime.now(dt.timezone.utc)
    ca_key = ec.generate_private_key(ec.SECP256R1())
    name = x509.Name([x509.NameAttribute(NameOID.COMMON_NAME, 'issue855-local-proof-ca')])
    ca = (x509.CertificateBuilder().subject_name(name).issuer_name(name)
          .public_key(ca_key.public_key()).serial_number(x509.random_serial_number())
          .not_valid_before(now - dt.timedelta(days=1)).not_valid_after(now + dt.timedelta(days=7))
          .add_extension(x509.BasicConstraints(ca=True, path_length=0), True)
          .sign(ca_key, hashes.SHA256()))
    ca_path = write(root / 'ca.pem', ca.public_bytes(serialization.Encoding.PEM).decode())
    output = {'ca': ca_path}
    for label, usage in [('server', ExtendedKeyUsageOID.SERVER_AUTH), ('guardian', ExtendedKeyUsageOID.CLIENT_AUTH)]:
        key = ec.generate_private_key(ec.SECP256R1())
        cert = (x509.CertificateBuilder().subject_name(x509.Name([x509.NameAttribute(NameOID.COMMON_NAME, label)]))
                .issuer_name(name).public_key(key.public_key()).serial_number(x509.random_serial_number())
                .not_valid_before(now - dt.timedelta(days=1)).not_valid_after(now + dt.timedelta(days=7))
                .add_extension(x509.SubjectAlternativeName([x509.DNSName('localhost'), x509.IPAddress(ipaddress.ip_address('127.0.0.1'))]), False)
                .add_extension(x509.ExtendedKeyUsage([usage]), False).sign(ca_key, hashes.SHA256()))
        output[label] = write(root / f'{label}.pem', (cert.public_bytes(serialization.Encoding.PEM) + ca.public_bytes(serialization.Encoding.PEM)).decode())
        output[label + '_key'] = write(root / f'{label}.key', key.private_bytes(serialization.Encoding.PEM, serialization.PrivateFormat.PKCS8, serialization.NoEncryption()).decode(), True)
        output[label + '_spki'] = hashlib.sha256(key.public_key().public_bytes(serialization.Encoding.DER, serialization.PublicFormat.SubjectPublicKeyInfo)).hexdigest()
    return output


class LocalTime:
    """SNTP responder uses host time; no external network or authority claim."""
    def __init__(self):
        self.sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
        self.sock.bind(('127.0.0.1', 0))
        self.address = f'127.0.0.1:{self.sock.getsockname()[1]}'
        threading.Thread(target=self.serve, daemon=True).start()

    def serve(self):
        while True:
            try:
                request, peer = self.sock.recvfrom(512)
            except OSError:
                return
            if len(request) < 48:
                continue
            stamp = time.time() + 2208988800
            encoded = struct.pack('!II', int(stamp), int((stamp % 1) * 2**32))
            reply = bytearray(48)
            reply[0:4] = bytes([0x24, 1, 4, 0xec])
            reply[12:16] = b'LOCL'
            reply[16:24] = encoded
            reply[24:32] = request[40:48]
            reply[32:40] = reply[40:48] = encoded
            self.sock.sendto(reply, peer)


class Fixture:
    def __init__(self, tls):
        self.calls = []
        self.lock = threading.Lock()
        owner = self
        class Handler(http.server.BaseHTTPRequestHandler):
            def log_message(self, *args):
                pass
            def do_GET(self):
                self.reply({'models': [{'name': 'fixture-model'}, {'name': 'replacement-model'}]})
            def reply(self, body):
                data = json.dumps(body).encode()
                self.send_response(200)
                self.send_header('Content-Type', 'application/json')
                self.send_header('Content-Length', str(len(data)))
                self.end_headers()
                self.wfile.write(data)
            def do_POST(self):
                size = int(self.headers.get('Content-Length', 0))
                require(size <= 1000000, 'fixture request exceeds bound')
                data = json.loads(self.rfile.read(size))
                prompt = data.get('input', data.get('prompt', ''))
                if 'messages' in data:
                    prompt = data['messages'][0]['content']
                if 'contents' in data:
                    prompt = data['contents'][0]['parts'][0]['text']
                if not isinstance(prompt, str):
                    prompt = json.dumps(prompt)
                model = data.get('model', self.path.split('/models/')[-1].split(':')[0])
                # Production Runtime generates the signed A2A envelope from this action.
                if 'A governed agent-to-agent action you initiated' in prompt:
                    answer = 'Fixture operator synthesis of verified peer reply.'
                elif 'ISSUE855_A2A:' in prompt:
                    target = prompt.split('ISSUE855_A2A:', 1)[1].split()[0].strip('.,"')
                    answer = json.dumps({'schema': 'adl.runtime.provider_agent_action.v1', 'message': 'Contacting the fixture peer.', 'action': {'recipient_name': target, 'message': 'Give a bounded fixture peer answer.', 'message_parts': []}})
                else:
                    answer = 'Fixture generated response for the requested local lifecycle proof.'
                with owner.lock:
                    owner.calls.append({'path': self.path, 'model': model, 'prompt_bytes': len(prompt.encode()), 'wire_output_cap': data.get('max_output_tokens', data.get('max_tokens', data.get('generationConfig', {}).get('maxOutputTokens', data.get('options', {}).get('num_predict')))), 'answer_digest': hashlib.sha256(answer.encode()).hexdigest()})
                if self.path.endswith('/responses'):
                    self.reply({'output_text': answer})
                elif self.path.endswith('/messages'):
                    self.reply({'content': [{'type': 'text', 'text': answer}]})
                elif ':generateContent' in self.path:
                    self.reply({'candidates': [{'content': {'parts': [{'text': answer}]}}]})
                elif self.path.endswith('/generate'):
                    self.reply({'response': answer, 'done': True})
                else:
                    self.reply({'choices': [{'message': {'content': answer}}]})
        self.resident_server = http.server.ThreadingHTTPServer(('127.0.0.1', 0), Handler)
        self.resident_url = f'http://127.0.0.1:{self.resident_server.server_port}'
        threading.Thread(target=self.resident_server.serve_forever, daemon=True).start()
        self.server = http.server.ThreadingHTTPServer(('127.0.0.1', 0), Handler)
        ctx = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
        ctx.load_cert_chain(tls['server'], tls['server_key'])
        self.server.socket = ctx.wrap_socket(self.server.socket, server_side=True)
        self.url = f'https://127.0.0.1:{self.server.server_port}'
        threading.Thread(target=self.server.serve_forever, daemon=True).start()


class WebSocket:
    """Minimal bounded RFC6455 JSON client for the repository's existing API."""
    def __init__(self, address, context, origin):
        self.sock = context.wrap_socket(socket.create_connection(('127.0.0.1', address), timeout=15), server_hostname='localhost')
        key = base64.b64encode(os.urandom(16)).decode()
        self.sock.sendall((f'GET /v1/observatory/ws?schema=v3 HTTP/1.1\r\nHost: localhost:{address}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: {key}\r\nSec-WebSocket-Version: 13\r\nOrigin: {origin}\r\n\r\n').encode())
        header = b''
        while not header.endswith(b'\r\n\r\n'):
            header += self.sock.recv(1)
            require(len(header) < 8192, 'websocket header bound')
        require(b' 101 ' in header, f'websocket upgrade failed: {header[:100]!r}')
        expected = base64.b64encode(hashlib.sha1((key + '258EAFA5-E914-47DA-95CA-C5AB0DC85B11').encode()).digest())
        require(expected.lower() in header.lower(), 'websocket handshake identity')

    def send(self, item, opcode=1):
        data = json.dumps(item).encode() if opcode == 1 else item
        mask = os.urandom(4)
        size = len(data)
        header = bytes([0x80 | opcode, 0x80 | (size if size < 126 else 126)])
        if size >= 126:
            header += struct.pack('!H', size)
        self.sock.sendall(header + mask + bytes(x ^ mask[i % 4] for i, x in enumerate(data)))

    def exact(self, n):
        data = b''
        while len(data) < n:
            more = self.sock.recv(n-len(data))
            require(more, 'websocket closed before result')
            data += more
        return data

    def receive(self):
        while True:
            first, second = self.exact(2)
            size = second & 127
            if size == 126:
                size = struct.unpack('!H', self.exact(2))[0]
            elif size == 127:
                size = struct.unpack('!Q', self.exact(8))[0]
            require(size <= 4_194_304 and not second & 128, 'invalid websocket frame bound/mask')
            data = self.exact(size)
            if first & 15 == 9:
                self.send(data, 10)
                continue
            require(first & 15 == 1 and first & 128, 'unsupported websocket frame')
            return json.loads(data)

    def until(self, predicate, timeout=30):
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            item = self.receive()
            if predicate(item):
                return item
        raise TimeoutError('websocket result deadline exceeded')


def prepare_init(root, tls, args, time_source, fixture):
    state = root / 'state'
    state.mkdir(exist_ok=True)
    state.chmod(0o700)
    config = tomllib.loads((REPO / 'infra/runtime-v3/runtime-init.toml').read_text())
    config['state_root'] = str(state)
    config['binaries']['kernel_path'] = str(root / 'runtime-v3/current/bin/adl-runtime-kernel')
    api_port = port()
    config['api'].update(address=f'127.0.0.1:{api_port}', public_base_url=f'https://localhost:{api_port}')
    config['api']['tls'] = dict(certificate_chain_path=str(tls['server']), private_key_path=str(tls['server_key']), trust_roots_path=str(tls['ca']), server_name='localhost')
    config['polis'].update(id='issue855-fixture', public_domain='localhost', observatory_public_origin='https://localhost:8765')
    config['polis'].pop('vertex_ai', None)
    config['observatory']['allowed_origins'] = ['https://localhost:8765']
    config['resident_shepherd'].update(model='fixture-model', endpoint=fixture.resident_url)
    config['resident_shepherd']['preload'] = {'enabled': False, 'timeout_millis': 60000, 'retry_initial_millis': 100, 'retry_max_millis': 1000}
    config['observability_pipeline']['vector_binary_path'] = str(args.vector)
    config['observability_pipeline'].pop('cloudwatch', None)
    config['observability_pipeline']['revision'] = args.source_revision
    config['credentials']['sntp_server'] = time_source.address
    credentials = state / 'credentials'
    keys = {}
    for label in ('control', 'operation', 'migration-decision', 'continuity'):
        key = ed25519.Ed25519PrivateKey.generate()
        keys[label] = key
        if label == 'continuity':
            output = key.private_bytes(serialization.Encoding.Raw, serialization.PrivateFormat.Raw, serialization.NoEncryption())
            target = 'continuity_signing_key_path'
        else:
            output = key.public_key().public_bytes(serialization.Encoding.Raw, serialization.PublicFormat.Raw)
            target = label.replace('-', '_') + '_public_key_path'
        config['credentials'][target] = str(write(credentials / (label + '.hex'), output.hex(), True))
    tokens = {'observatory': secrets.token_hex(24), 'acip_write': secrets.token_hex(24)}
    for kind, value in tokens.items():
        config['credentials'][kind + '_token_path'] = str(write(credentials / (kind + '.txt'), value, True))
    authorities = []
    for index, role in enumerate(('identity_continuity', 'memory_capability', 'negative_case_guard', 'handoff_consumer')):
        key = ed25519.Ed25519PrivateKey.generate()
        authorities.append(dict(witness_id=f'witness-{index}', role=role, signing_key_id=f'witness-key-{index}', verifying_key=key.public_key().public_bytes(serialization.Encoding.Raw, serialization.PublicFormat.Raw).hex()))
    config['credentials']['birth_witness_trust_manifest_path'] = str(write(credentials / 'birth-witness-trust.json', dict(schema='adl.runtime.birth_witness_trust.v1', authority_context='runtime-v3-birth-witness-authority', authorities=authorities)))
    continuity = dict(address=f'127.0.0.1:{port()}', trust_domain='agent-logic.test', polis='issue855-fixture', source_node='node-source', target_node='node-target', guardian_id='guardian-logical', kernel_control_id='kernel-control', channel_epoch=1)
    for field, directory in [('guardian_state_dir', 'guardian-continuity'), ('state_dir', 'kernel-continuity-control'), ('staging_dir', 'continuity-staging')]:
        path = state / directory
        path.mkdir(mode=0o700)
        continuity[field] = str(path)
    server_ca = write(state / 'tls/continuity-server-ca.pem', tls['ca'].read_text())
    guardian_ca = write(state / 'tls/continuity-guardian-ca.pem', tls['ca'].read_text())
    continuity['tls'] = dict(server_certificate_chain_path=str(tls['server']), server_private_key_path=str(tls['server_key']), server_trust_roots_path=str(server_ca), server_name='localhost', guardian_certificate_chain_path=str(tls['guardian']), guardian_private_key_path=str(tls['guardian_key']), guardian_trust_roots_path=str(guardian_ca), guardian_spki_sha256=tls['guardian_spki'], server_spki_sha256=tls['server_spki'], certificate_generation=1)
    continuity['bounds'] = dict(max_frame_bytes=65536, max_blob_bytes=65536, max_total_bytes=524288, max_services=5, max_journal_entries=64, max_open_handles=8)
    config['continuity_control'] = continuity
    config['guardian']['restart_budget'] = 0
    config['agent_partial_checkpoints']['enabled'] = False
    init = write(root / 'runtime-init.toml', toml(config))
    # JSON is a YAML subset; the real production sidecar parser validates these.
    definitions = {}
    for kind in (tuple(HOSTED_MODELS) + ('ollama',) if args.hosted_mode else PROVIDERS):
        native = {'vertex_ai': 'vertex_ai_gemini', 'openai-compatible': 'http'}.get(kind, kind)
        endpoint = fixture.url
        if kind == 'openai':
            endpoint += '/v1/responses'
        elif kind == 'anthropic':
            endpoint += '/v1/messages'
        elif kind == 'vertex_ai':
            endpoint += '/v1/projects/fixture/locations/us-central1/publishers/google/models/fixture-model:generateContent'
        elif kind == 'openai-compatible':
            endpoint += '/v1/chat/completions'
        model = HOSTED_MODELS.get(kind, 'fixture-model') if args.hosted_mode else 'fixture-model'
        if kind == 'vertex_ai' and args.hosted_mode:
            endpoint = endpoint.replace('/fixture-model:', '/' + model + ':')
        if args.hosted_approved and kind in HOSTED_MODELS:
            endpoint = {'openai': 'https://api.openai.com/v1/responses', 'anthropic': 'https://api.anthropic.com/v1/messages', 'vertex_ai': f'https://us-central1-aiplatform.googleapis.com/v1/projects/{APPROVED_PROJECT}/locations/us-central1/publishers/google/models/{model}:generateContent'}[kind]
        details = dict(endpoint=endpoint, runtime_max_attempts=1, runtime_max_output_tokens=256, max_tokens=256, max_output_tokens=256)
        if args.hosted_mode and kind in HOSTED_MODELS:
            details.update(runtime_max_calls=6, runtime_max_input_bytes=32000, runtime_stop_after_failure=True)
        if kind != 'ollama':
            details['auth'] = dict(type='bearer', env=HOSTED_ENVS[kind] if args.hosted_mode and kind in HOSTED_MODELS else 'ADL_PROVIDER_FIXTURE_TOKEN')
        if kind == 'openai-compatible':
            details['api_format'] = 'openai_chat_completions'
        if kind == 'vertex_ai':
            details.update(project=APPROVED_PROJECT if args.hosted_approved else 'fixture', location='us-central1')
            if args.hosted_mode:
                details['thinking_budget'] = 0
        definitions[kind] = dict(type=native, default_model=model, config=details)
    write(root / 'providers.yaml', dict(schema='adl.provider_reload_sidecar.v1', version='0.5', providers=definitions))
    return init, api_port, tokens


def api(context, api_port, token, path):
    request = urllib.request.Request(f'https://localhost:{api_port}{path}', headers={'Authorization': 'Bearer ' + token})
    with urllib.request.urlopen(request, context=context, timeout=15) as response:
        return json.load(response)


def identity(snapshot):
    return {key: snapshot[key] for key in ('runtime_incarnation_id', 'runtime_process_id')}


def conversation(api_port, context, token, agent_id, message):
    ws = WebSocket(api_port, context, 'https://localhost:8765')
    try:
        ws.send(dict(schema='adl.runtime_v3.observatory_ws_auth.v1', bearer_token=token))
        ws.until(lambda x: x.get('status') == 'authenticated')
        turn = secrets.token_hex(8)
        ws.send(dict(schema='adl.runtime_v3.observatory_conversation_intent.v1', conversation_id='fixture-' + turn, turn_id=turn, recipient_id=agent_id, correlation_id=secrets.token_hex(16), message=message, message_parts=[]))
        result = ws.until(lambda x: x.get('schema') == 'adl.runtime_v3.observatory_conversation_result.v1' and x.get('turn_id') == turn and x.get('status') not in ('accepted', 'pending'))
        require(result.get('status') == 'delivered', f'conversation failed: {result}')
        require(result.get('reply'), 'missing generated operator reply')
        return result
    finally:
        ws.sock.close()


def execute(args):
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    root.chmod(0o700)
    tls = certificates(root / 'state/tls')
    fixture = Fixture(tls)
    clock = LocalTime()
    env = dict(os.environ)
    # Never forward ambient provider credentials into the proof process.
    for key in list(env):
        if any(part in key for part in ('API_KEY', 'ACCESS_TOKEN', 'GOOGLE_APPLICATION_CREDENTIALS')):
            del env[key]
    env.update(ADL_PROVIDER_CA_FILE=str(tls['ca']), ADL_PROVIDER_FIXTURE_TOKEN='issue855-local-fixture-value')
    if args.hosted_approved:
        for provider, filename in [('openai', args.openai_key_file), ('anthropic', args.anthropic_key_file)]:
            name = HOSTED_ENVS[provider]
            value = os.environ.get(name)
            if not value:
                value = filename.read_text().strip()
            require(value and len(value) < 16384 and not any(c.isspace() for c in value), 'invalid approved credential source')
            env[name] = value
        project = run(['gcloud', 'config', 'get-value', 'project']).strip()
        require(project == APPROVED_PROJECT, 'current Google project differs from approved project')
        env[HOSTED_ENVS['vertex_ai']] = run(['gcloud', 'auth', 'print-access-token', '--project', APPROVED_PROJECT]).strip()
    elif args.hosted_fixture:
        for name in HOSTED_ENVS.values():
            env[name] = 'issue855-local-fixture-value'
    if args.hosted_mode:
        for name in HOSTED_ENVS.values():
            env[name + '_REPLACEMENT'] = env[name]
    install = root / 'runtime-v3'
    run([REPO / 'adl/tools/install_runtime_v3_generation.sh', 'install', '--root', install, '--generation', 'issue855-local', '--csm', args.csm, '--guardian', args.guardian, '--kernel', args.kernel, '--source-revision', args.source_revision, '--build-profile', 'debug'])
    run([REPO / 'adl/tools/install_runtime_v3_generation.sh', 'verify', '--root', install])
    # csmctl is an additional owner: copy exact bytes and separately retain hash.
    import shutil
    ctl = root / 'csmctl'
    shutil.copy2(args.csmctl, ctl)
    init, api_port, tokens = prepare_init(root, tls, args, clock, fixture)
    plist = root / 'fixture.plist'
    plist.write_bytes(plistlib.dumps({'Label': 'ai.agent-logic.issue855-fixture-' + str(os.getpid()), 'ProgramArguments': [str(install / 'current/bin/adl-runtime-guardian'), '--init', str(init)]}))
    status = subprocess.run([str(root / 'bin/csm'), 'runtime-v3', 'status', '--init', str(init), '--plist', str(plist), '--label', 'ai.agent-logic.issue855-fixture-' + str(os.getpid()), '--json'], env=env, capture_output=True, text=True, timeout=30)
    require(status.stdout.strip(), 'CSM config validation failed: ' + status.stderr[-1500:])
    config_status = json.loads(status.stdout)
    require(config_status['config_valid'] and not config_status['service_loaded'], 'isolated CSM configuration/service state')
    write(root / 'csm-config-status.json', config_status)
    init_digest = hashlib.sha256(init.read_bytes()).hexdigest()
    ctx = ssl.create_default_context(cafile=str(tls['ca']))
    guardian_log = (root / 'guardian.log').open('w')
    binary_hashes = {name: hashlib.sha256(getattr(args, name).read_bytes()).hexdigest() for name in ('csm', 'csmctl', 'kernel', 'guardian', 'vector')}
    source_status = run(['git', '-C', REPO, 'status', '--porcelain'])
    report = dict(binary_sha256=binary_hashes, source_checkout_dirty=bool(source_status.strip()), source_revision_boundary='base commit plus separately reviewed worktree changes; binary hashes identify executed bytes', schema='adl.issue855.installed_provider_lifecycle.v1', source_revision=args.source_revision, result='running', fixture_only=not args.hosted_approved, paid_calls='pending_runtime_accounting' if args.hosted_approved else 0, mode='hosted' if args.hosted_approved else ('hosted_fixture' if args.hosted_fixture else 'five_provider_fixture'), rows=[], init_sha256=init_digest)
    guardian = None
    expiry = None
    cleanup_lock = threading.Lock()
    cleanup_done = False
    def owned_groups():
        # macOS bounded child query, never an all-host process inventory.
        libproc = ctypes.CDLL('/usr/lib/libproc.dylib')
        pending = [guardian.pid]
        known = report.get('runtime_identity', {}).get('runtime_process_id')
        if known:
            pending.append(int(known))
        visited, groups = set(), set()
        while pending:
            pid = pending.pop()
            if pid in visited:
                continue
            visited.add(pid)
            try:
                if os.getsid(pid) != guardian.pid:
                    continue
                groups.add(os.getpgid(pid))
                children = (ctypes.c_int * 256)()
                size = libproc.proc_listchildpids(pid, children, ctypes.sizeof(children))
                require(size < ctypes.sizeof(children), 'owned child inventory exceeds bounded capacity')
                pending.extend(child for child in children[:max(0, size) // ctypes.sizeof(ctypes.c_int)] if child > 0)
            except ProcessLookupError:
                pass
        return groups
    def stop_owned_group():
        nonlocal cleanup_done
        with cleanup_lock:
            if guardian is None or cleanup_done:
                return
            groups = owned_groups()
            report['cleanup_owned_process_groups'] = sorted(groups)
            for group in groups:
                try:
                    os.killpg(group, signal.SIGTERM)
                except ProcessLookupError:
                    pass
            try:
                guardian.wait(timeout=20)
            except subprocess.TimeoutExpired:
                pass
            for group in groups:
                try:
                    # Session identity remains valid while any member survives.
                    os.killpg(group, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            guardian.wait(timeout=5)
            cleanup_done = True
    try:
        guardian = subprocess.Popen([str(install / 'current/bin/adl-runtime-guardian'), '--init', str(init)], stdout=guardian_log, stderr=subprocess.STDOUT, env=env, start_new_session=True)
        report.update(guardian_pid=guardian.pid, owned_process_group=guardian.pid)
        expiry = threading.Timer(1775, stop_owned_group)
        expiry.start()
        deadline = time.monotonic() + 45
        initial = None
        while time.monotonic() < deadline:
            require(guardian.poll() is None, 'Guardian exited; inspect retained guardian.log')
            try:
                initial = api(ctx, api_port, tokens['observatory'], '/v1/observatory?schema=v3')
                if initial.get('runtime_incarnation_id'):
                    break
            except (OSError, ValueError):
                pass
            time.sleep(.1)
        require(initial and initial.get('runtime_incarnation_id'), 'Runtime readiness deadline')
        baseline = identity(initial)
        report['runtime_identity'] = baseline
        def csmctl(*argv):
            return json.loads(run([ctl, 'agent', *argv], env=env))
        def await_agent(agent_id):
            deadline = time.monotonic() + 15
            while time.monotonic() < deadline:
                detail = csmctl('get', '--init', init, '--id', agent_id)
                if detail.get('communication_eligible'):
                    return detail
                time.sleep(.1)
            raise AssertionError('agent did not become communication eligible: ' + json.dumps(detail))
        def observe():
            snapshot = api(ctx, api_port, tokens['observatory'], '/v1/observatory?schema=v3')
            require(identity(snapshot) == baseline, 'Runtime restarted or incarnation changed')
            require(guardian.poll() is None, 'Guardian exited')
            require(hashlib.sha256(init.read_bytes()).hexdigest() == init_digest, 'Runtime init mutated')
            return snapshot
        for index, provider in enumerate(tuple(HOSTED_MODELS) if args.hosted_mode else PROVIDERS):
            ident = f'fixture-{index}'
            peer_id = f'fixture-peer-{index}'
            name = f'ember.fixture{index}'
            peer_name = 'beacon.axioma' if args.hosted_mode else f'beacon.fixture{index}'
            if args.hosted_mode:
                peer_id = 'shepherd'
            selected_model = HOSTED_MODELS[provider] if args.hosted_mode else 'fixture-model'
            replacement_model = selected_model if args.hosted_mode else 'replacement-model'
            start_calls = len(fixture.calls)
            def admission(agent_id, agent_name, model=selected_model, replacing=False):
                config = dict(schema='adl.csm.agent_config.v1', runtime={'init': str(init)}, identity=dict(id=agent_id, name=agent_name, display_name=agent_name), office='assistant', provider=dict(kind=provider, model=model, required_capabilities=['conversation', 'agent_to_agent']))
                if provider != 'ollama':
                    credential = HOSTED_ENVS[provider] if args.hosted_mode else 'ADL_PROVIDER_FIXTURE_TOKEN'
                    config['provider']['credential_ref'] = 'env:' + credential + ('_REPLACEMENT' if replacing and args.hosted_mode else '')
                if provider == 'vertex_ai' and model == 'replacement-model':
                    config['provider']['endpoint'] = fixture.url + '/v1/projects/fixture/locations/us-central1/publishers/google/models/replacement-model:generateContent'
                path = write(root / f'{agent_id}.json', config)
                return csmctl('add', '--config', path)
            require(admission(ident, name)['status'] == 'admitted', 'agent admission')
            if not args.hosted_mode:
                require(admission(peer_id, peer_name)['status'] == 'admitted', 'peer admission')
            # Admission's existing generated greeting is counted, not mislabeled metadata work.
            admission_calls = len(fixture.calls) - start_calls
            detail = await_agent(ident)
            await_agent(peer_id)
            operator = conversation(api_port, ctx, tokens['observatory'], ident, 'Give a bounded local operator reply.')
            peer_prompt = f'ISSUE855_A2A:{peer_name} Ask the peer for a bounded reply.'
            if args.hosted_approved:
                peer_prompt = 'Initiate the governed agent-to-agent action now. Return only this JSON object, without Markdown: ' + json.dumps({'schema': 'adl.runtime.provider_agent_action.v1', 'message': 'I will ask Beacon for a greeting.', 'action': {'recipient_name': peer_name, 'message': 'Please give a brief greeting.', 'message_parts': []}})
            peer = conversation(api_port, ctx, tokens['observatory'], ident, peer_prompt)
            require(peer.get('initiated_recipient_name') == peer_name and peer.get('initiated_reply'), 'canonical generated A2A reply missing')
            checkpoint = root / f'{ident}-checkpoint.json'
            csmctl('checkpoint', '--init', init, '--id', ident, '--out', checkpoint)
            require(checkpoint.is_file(), 'checkpoint not durable')
            bundle = root / f'{ident}-bundle.json'
            csmctl('migrate', '--init', init, '--id', ident, '--out', bundle)
            require(bundle.is_file(), 'migration bundle not durable')
            write(root / f'{ident}-pre-restore-proof.json', dict(detail=detail, operator=operator, agent_to_agent=peer, fixture_calls=fixture.calls[start_calls:]))
            restored = csmctl('rehydrate', '--init', init, '--bundle', bundle)
            require(restored['status'] == 'admitted', 'rehydration failed')
            await_agent(ident)
            post_restore = conversation(api_port, ctx, tokens['observatory'], ident, 'Give the post-restore local reply.')
            require(admission(ident, name, replacement_model, replacing=True)['status'] == 'replaced', 'binding replacement failed')
            replaced = await_agent(ident)
            replacement_reply = conversation(api_port, ctx, tokens['observatory'], ident, 'Give the replacement binding local reply.')
            if not args.hosted_approved:
                require(fixture.calls[-1]['model'] == replacement_model, 'replacement did not reach replacement model on actual wire')
            active_snapshot = observe()
            active_agents = active_snapshot['agents']['sample']
            projected = next(a for a in active_agents if a['id'] == ident)
            require(projected.get('provider') == provider and projected.get('model') == replacement_model, 'Observatory provider/model projection mismatch')
            csmctl('remove', '--init', init, '--id', ident)
            if not args.hosted_mode:
                csmctl('remove', '--init', init, '--id', peer_id)
            remaining = csmctl('list', '--init', init)
            require(ident not in json.dumps(remaining) and (args.hosted_mode or peer_id not in json.dumps(remaining)), 'removed agents remain in live roster')
            snapshot = observe()
            row = dict(provider=provider, local_fixture_prompt_bytes=sum(c['prompt_bytes'] for c in fixture.calls[start_calls:]), result='pass', admission_calls_observed_at_return=admission_calls, calls=fixture.calls[start_calls:], original_detail=detail, replacement_detail=replaced, operator=operator, agent_to_agent=peer, restored_reply=post_restore, replacement_reply=replacement_reply, observatory_agent=projected, identity=identity(snapshot))
            if not args.hosted_approved:
                row['complete_prompt_bytes'] = row['local_fixture_prompt_bytes']
            report['rows'].append(row)
            write(root / 'report.json', report)
        report['result'] = 'pass'
        report['local_fixture_request_count'] = len(fixture.calls)
        report['calls_outside_rows'] = len(fixture.calls) - sum(len(row['calls']) for row in report['rows'])
        report['fixture_calls'] = fixture.calls
        report['maximum_local_fixture_prompt_bytes'] = max(c['prompt_bytes'] for c in fixture.calls)
        if not args.hosted_approved:
            report['maximum_complete_prompt_bytes'] = report['maximum_local_fixture_prompt_bytes']
        report['provider_catalog'] = api(ctx, api_port, tokens['observatory'], '/v1/providers')
        report['provider_health'] = api(ctx, api_port, tokens['observatory'], '/v1/health/providers')
        report['provider_metrics'] = api(ctx, api_port, tokens['observatory'], '/v1/metrics/providers')
        report['provider_request_count'] = sum(item['requests'] for item in report['provider_metrics'])
        for row in report['rows']:
            row['runtime_provider_request_count'] = sum(item['requests'] for item in report['provider_metrics'] if item['provider'] == row['provider'])
        if args.hosted_mode:
            totals = {provider: sum(row['requests'] for row in report['provider_metrics'] if row['provider'] == provider) for provider in HOSTED_MODELS}
            require(all(count <= 6 for count in totals.values()), 'approved per-provider call cap exceeded')
            report['hosted_provider_request_counts'] = totals
            report['paid_calls'] = sum(totals.values()) if args.hosted_approved else 0
            report['approved_bounds'] = dict(calls_per_provider=6, input_bytes_per_request=32000, output_tokens_per_request=256, retries=0, total_cost_ceiling_usd=1, runtime_seconds=1800, teardown_reserved_seconds=25, graceful_shutdown_starts_seconds=1775, project=APPROVED_PROJECT, models=HOSTED_MODELS)
            if args.hosted_fixture:
                hosted_wires = [call for call in fixture.calls if not call['path'].endswith('/generate')]
                require(all(call['wire_output_cap'] == 256 and call['prompt_bytes'] <= 32000 for call in hosted_wires), 'hosted wire caps were not enforced')
    except Exception as error:
        report.update(result='failed', error=str(error), fixture_calls=fixture.calls)
        raise
    finally:
        try:
            report['provider_health'] = api(ctx, api_port, tokens['observatory'], '/v1/health/providers')
        except (OSError, ValueError):
            report['provider_health_capture'] = 'unavailable before shutdown'
        if args.hosted_approved:
            try:
                report['provider_metrics'] = api(ctx, api_port, tokens['observatory'], '/v1/metrics/providers')
                report['paid_calls'] = sum(row['requests'] for row in report['provider_metrics'] if row['provider'] in HOSTED_MODELS)
            except (OSError, ValueError):
                report['paid_calls'] = 'unavailable; do not retry without reconciling retained Runtime evidence'
        try:
            write(root / 'report.json', report)
        finally:
            if expiry is not None:
                expiry.cancel()
            stop_owned_group()
            write(root / 'report.json', report)
        guardian_log.close()
        fixture.server.shutdown()
        fixture.resident_server.shutdown()
        clock.sock.close()
    print(json.dumps({'result': report['result'], 'rows': len(report['rows']), 'report': str(root / 'report.json')}))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    for name in ('csm', 'csmctl', 'guardian', 'kernel', 'vector'):
        parser.add_argument('--' + name, type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True, help='new retained directory; must not exist')
    parser.add_argument('--source-revision', required=True)
    modes = parser.add_mutually_exclusive_group()
    modes.add_argument('--hosted-fixture', action='store_true', help='approved hosted topology with local transports, no paid calls')
    modes.add_argument('--hosted-approved', action='store_true', help='explicitly authorized single hosted run; fixed approved model/project/caps')
    parser.add_argument('--openai-key-file', type=Path, default=Path.home() / 'keys/openai2.key')
    parser.add_argument('--anthropic-key-file', type=Path, default=Path.home() / 'keys/claude2.key')
    args = parser.parse_args()
    args.hosted_mode = args.hosted_approved or args.hosted_fixture
    for name in ('csm', 'csmctl', 'guardian', 'kernel', 'vector'):
        path = getattr(args, name).resolve()
        require(path.is_file(), f'missing {name} binary')
        setattr(args, name, path)
    execute(args)


if __name__ == '__main__':
    main()
