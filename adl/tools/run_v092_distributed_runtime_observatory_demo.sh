#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "$0")/../.." && pwd -P)
phase=${1:-local}
case "$phase" in
  local) ;;
  hybrid)
    echo "hybrid phase must be invoked by the AWS qualification wrapper after local proof cleanup" >&2
    exit 69
    ;;
  *) echo "usage: $0 local" >&2; exit 64 ;;
esac

vector_bin=${ADL_RUNTIME_VECTOR_BIN:-}
if [[ -z "$vector_bin" ]]; then
  vector_bin=$(command -v vector || true)
fi
[[ -n "$vector_bin" && -x "$vector_bin" ]] || {
  echo "ADL_RUNTIME_VECTOR_BIN must name a real executable Vector binary" >&2
  exit 69
}
command -v ollama >/dev/null || { echo "Ollama is required for the local phase" >&2; exit 69; }
model_ref=${ADL_RUNTIME_LOCAL_MODEL:-gemma:2b}
model_digest=$(curl --fail --silent http://127.0.0.1:11434/api/tags | jq -r \
  --arg model "$model_ref" '.models[] | select(.name == $model or .model == $model) | .digest' | head -1)
model_digest=${model_digest#sha256:}
[[ "$model_digest" =~ ^[0-9a-f]{64}$ ]] || {
  echo "selected local Ollama model is not installed with a verifiable digest: $model_ref" >&2
  exit 69
}

target_dir=${CARGO_TARGET_DIR:-$repo_root/.adl/target/142-distributed-runtime}
evidence_root=${ADL_DISTRIBUTED_RUNTIME_EVIDENCE_ROOT:-$repo_root/.adl/runtime-v3/142-distributed-runtime}
mkdir -p "$target_dir" "$evidence_root"
target_dir=$(cd "$target_dir" && pwd -P)
evidence_root=$(cd "$evidence_root" && pwd -P)
export CARGO_TARGET_DIR="$target_dir"

lock_dir="$evidence_root/.serial-phase.lock"
if ! mkdir "$lock_dir" 2>/dev/null; then
  echo "another distributed Runtime qualification phase is active" >&2
  exit 75
fi
trap 'rmdir "$lock_dir" 2>/dev/null || true' EXIT

cargo build --locked --manifest-path "$repo_root/adl-runtime-kernel/Cargo.toml" --bin adl-runtime-kernel
cargo build --locked --manifest-path "$repo_root/adl-runtime/Cargo.toml" --bin adl-runtime-guardian

revision=$(git -C "$repo_root" rev-parse HEAD)
run_root=$(mktemp -d "$evidence_root/local-$revision.XXXXXX")
python3 - "$repo_root" "$run_root" "$target_dir/debug/adl-runtime-guardian" \
  "$target_dir/debug/adl-runtime-kernel" "$vector_bin" "$model_ref" "$model_digest" \
  "$revision" <<'PY'
import base64
import hashlib
import json
import os
import pathlib
import secrets
import signal
import socket
import ssl
import struct
import subprocess
import sys
import time

(
    repo_root_arg,
    run_root_arg,
    guardian_bin_arg,
    kernel_bin_arg,
    vector_bin_arg,
    model_ref,
    model_digest,
    revision,
) = sys.argv[1:]
repo_root = pathlib.Path(repo_root_arg).resolve()
run_root = pathlib.Path(run_root_arg).resolve()
guardian_bin = pathlib.Path(guardian_bin_arg).resolve()
kernel_bin = pathlib.Path(kernel_bin_arg).resolve()
vector_bin = pathlib.Path(vector_bin_arg).resolve()
runner = (repo_root / "adl/tools/adl_ollama_shepherd_runner.py").resolve()
template = (repo_root / "infra/runtime-v3/runtime-init.toml").read_text(encoding="utf-8")
tls_root = repo_root / "adl-runtime/tests/support/tls-fixtures"
certificate = (tls_root / "server-cert.pem").resolve()
intermediate = (tls_root / "intermediate-ca.pem").resolve()
private_key = (tls_root / "server-key.pem").resolve()
trust_roots = (tls_root / "root-ca.pem").resolve()
runner_digest = hashlib.sha256(runner.read_bytes()).hexdigest()
observatory_token = "issue-142-observatory-token-000000000001"
acip_token = "issue-142-acip-write-token-0000000000002"
local_kernel_tokens = {
    1: "issue-142-local-kernel-token-node-00000001",
    2: "issue-142-local-kernel-token-node-00000002",
    3: "issue-142-local-kernel-token-node-00000003",
}
voter_names = {1: "wuji-voter-a", 2: "wuji-voter-b", 3: "wuji-voter-c"}
guardian_names = {node: f"wuji-guardian-{node}" for node in voter_names}
processes = {}
MAX_FRAME = 65_536


def fail(message):
    raise RuntimeError(message)


def reserve_port():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as listener:
        listener.bind(("127.0.0.1", 0))
        return listener.getsockname()[1]


api_ports = {node: reserve_port() for node in voter_names}
raft_ports = {node: reserve_port() for node in voter_names}


def verifying_key(seed_byte):
    seed = bytes([seed_byte]) * 32
    pkcs8 = bytes.fromhex("302e020100300506032b657004220420") + seed
    source = run_root / f"seed-{seed_byte}.der"
    output = run_root / f"public-{seed_byte}.der"
    source.write_bytes(pkcs8)
    subprocess.run(
        ["openssl", "pkey", "-inform", "DER", "-in", str(source), "-pubout", "-outform", "DER", "-out", str(output)],
        check=True,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
    )
    public = output.read_bytes()[-32:]
    source.unlink()
    output.unlink()
    if len(public) != 32:
        fail("could not derive Ed25519 public key")
    return seed.hex(), public.hex()


voter_keys = {node: verifying_key(node) for node in voter_names}
_, control_public = verifying_key(17)
_, operation_public = verifying_key(29)


def replace_once(text, old, new):
    if text.count(old) != 1:
        fail(f"runtime init template contract changed: {old}")
    return text.replace(old, new, 1)


def write_secret(path, value):
    path.parent.mkdir(parents=True, exist_ok=True)
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
    with os.fdopen(descriptor, "w", encoding="utf-8") as stream:
        stream.write(value + "\n")
        stream.flush()
        os.fsync(stream.fileno())


def build_init(node):
    state = run_root / f"node-{node}"
    for child in ("continuity", "tls", "credentials", "observability", "distributed/keys"):
        (state / child).mkdir(parents=True, exist_ok=True)
    node_certificate = state / "tls/server-cert.pem"
    node_private_key = state / "tls/server-key.pem"
    node_trust_roots = state / "tls/root-ca.pem"
    node_certificate.write_bytes(certificate.read_bytes() + intermediate.read_bytes())
    node_private_key.write_bytes(private_key.read_bytes())
    node_private_key.chmod(0o600)
    node_trust_roots.write_bytes(trust_roots.read_bytes())
    write_secret(state / "credentials/control-public-key.hex", control_public)
    write_secret(state / "credentials/operation-public-key.hex", operation_public)
    write_secret(state / "credentials/continuity-signing-key.hex", (bytes([23]) * 32).hex())
    write_secret(state / "credentials/observatory-token.txt", observatory_token)
    write_secret(state / "credentials/acip-write-token.txt", acip_token)
    write_secret(state / "distributed/keys/local-voter-signing-key.hex", voter_keys[node][0])
    write_secret(state / "distributed/keys/local-kernel-token.txt", local_kernel_tokens[node])
    for peer in voter_names:
        if peer != node:
            write_secret(state / f"distributed/keys/voter-{peer}-public-key.hex", voter_keys[peer][1])
    text = template
    replacements = {
        'state_root = "/var/lib/adl/runtime-v3"': f'state_root = "{state}"',
        'kernel_path = "/opt/adl/bin/adl-runtime-kernel"': f'kernel_path = "{kernel_bin}"',
        'address = "127.0.0.1:20997"': f'address = "127.0.0.1:{api_ports[node]}"',
        'public_base_url = "https://runtime.dev.agent-logic.ai:20997"': f'public_base_url = "https://localhost:{api_ports[node]}"',
        'certificate_chain_path = "/var/lib/adl/runtime-v3/tls/fullchain.pem"': f'certificate_chain_path = "{node_certificate}"',
        'private_key_path = "/var/lib/adl/runtime-v3/tls/private-key.pem"': f'private_key_path = "{node_private_key}"',
        'trust_roots_path = "/var/lib/adl/runtime-v3/tls/trust-roots.pem"': f'trust_roots_path = "{node_trust_roots}"',
        'server_name = "runtime.dev.agent-logic.ai"': 'server_name = "localhost"',
        'control_public_key_path = "/var/lib/adl/runtime-v3/credentials/control-public-key.hex"': f'control_public_key_path = "{state / "credentials/control-public-key.hex"}"',
        'operation_public_key_path = "/var/lib/adl/runtime-v3/credentials/operation-public-key.hex"': f'operation_public_key_path = "{state / "credentials/operation-public-key.hex"}"',
        'continuity_signing_key_path = "/var/lib/adl/runtime-v3/credentials/continuity-signing-key.hex"': f'continuity_signing_key_path = "{state / "credentials/continuity-signing-key.hex"}"',
        'observatory_token_path = "/var/lib/adl/runtime-v3/credentials/observatory-token.txt"': f'observatory_token_path = "{state / "credentials/observatory-token.txt"}"',
        'acip_write_token_path = "/var/lib/adl/runtime-v3/credentials/acip-write-token.txt"': f'acip_write_token_path = "{state / "credentials/acip-write-token.txt"}"',
        'vector_binary_path = "/opt/adl/bin/vector"': f'vector_binary_path = "{vector_bin}"',
        'guardian_id = "guardian-process-0"': f'guardian_id = "{guardian_names[node]}"',
    }
    for old, new in replacements.items():
        text = replace_once(text, old, new)
    peers = ", ".join(
        f'"{voter_names[peer]}" = "127.0.0.1:{raft_ports[peer]}"'
        for peer in voter_names if peer != node
    )
    peer_keys = ", ".join(
        f'"{voter_names[peer]}" = "distributed/keys/voter-{peer}-public-key.hex"'
        for peer in voter_names if peer != node
    )
    distributed = f'''\n[distributed]
schema = "adl.runtime_v3.distributed.v1"
polis_id = "issue-142-local-polis"
trust_domain = "issue-142-local-domain"
local_voter_id = "{voter_names[node]}"
guardian_id = "{guardian_names[node]}"
voter_ids = ["wuji-voter-a", "wuji-voter-b", "wuji-voter-c"]
bootstrap = {str(node == 1).lower()}
listen_address = "127.0.0.1:{raft_ports[node]}"
peer_addresses = {{ {peers} }}
consensus_state_dir = "distributed/consensus"
voter_signing_key_path = "distributed/keys/local-voter-signing-key.hex"
voter_public_key_paths = {{ {peer_keys} }}
local_kernel_token_path = "distributed/keys/local-kernel-token.txt"
observatory_projection_path = "distributed/observatory-projection.json"
shepherd_agent_ref = "resident-agent:shepherd"
shepherd_identity_ref = "identity:shepherd-non-voter"
voter_model_profile = "small-local"
shepherd_model_profile = "small-local"
observatory_lease_millis = 3000

[distributed.model_profiles.small-local]
provider = "ollama_http"
endpoint = "http://127.0.0.1:11434"
model_ref = "{model_ref}"
model_artifact_sha256 = "{model_digest}"
runner_program_path = "{runner}"
runner_program_sha256 = "{runner_digest}"
context_tokens = 2048
max_in_flight = 1
max_prompt_bytes = 8192
max_output_bytes = 32768
max_memory_bytes = 4294967296
max_cpu_seconds = 120
'''
    init = state / "runtime-init.toml"
    init.write_text(text + distributed, encoding="utf-8")
    return init


init_paths = {node: build_init(node) for node in voter_names}


def start_node(node):
    stdout = open(run_root / f"guardian-{node}.stdout", "ab", buffering=0)
    stderr = open(run_root / f"guardian-{node}.stderr", "ab", buffering=0)
    process = subprocess.Popen(
        [str(guardian_bin), "--init", str(init_paths[node])],
        stdout=stdout,
        stderr=stderr,
        start_new_session=True,
    )
    processes[node] = (process, stdout, stderr)


def stop_node(node):
    entry = processes.pop(node, None)
    if entry is None:
        return
    process, stdout, stderr = entry
    if process.poll() is None:
        os.killpg(process.pid, signal.SIGTERM)
        try:
            process.wait(timeout=25)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.wait(timeout=5)
    stdout.close()
    stderr.close()
    if process.returncode not in (0, -signal.SIGTERM):
        fail(f"Guardian {node} exited unexpectedly: {process.returncode}")


def cleanup():
    failures = []
    for node in list(processes):
        try:
            stop_node(node)
        except Exception as error:
            failures.append(str(error))
    if failures:
        print("; ".join(failures), file=sys.stderr)


def tls_socket(node, timeout=5):
    tcp = socket.create_connection(("127.0.0.1", api_ports[node]), timeout=timeout)
    context = ssl.create_default_context(cafile=str(trust_roots))
    wrapped = context.wrap_socket(tcp, server_hostname="localhost")
    wrapped.settimeout(timeout)
    return wrapped


def http_get(node, token=None):
    with tls_socket(node) as stream:
        authorization = f"Authorization: Bearer {token}\r\n" if token else ""
        stream.sendall((
            "GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\n"
            + authorization + "Connection: close\r\n\r\n"
        ).encode("ascii"))
        response = bytearray()
        while True:
            chunk = stream.recv(65536)
            if not chunk:
                break
            response.extend(chunk)
            if len(response) > 2 * 1024 * 1024:
                fail("Observatory response exceeded bound")
    headers, body = bytes(response).split(b"\r\n\r\n", 1)
    status = int(headers.split(b" ", 2)[1])
    value = json.loads(body) if body else None
    return status, value


def wait_owner(previous=None, timeout=25):
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        owners = []
        for node in list(processes):
            process = processes[node][0]
            if process.poll() is not None:
                fail(f"Guardian {node} exited during readiness: {process.returncode}")
            try:
                status, feed = http_get(node, observatory_token)
            except (OSError, ssl.SSLError, ValueError, json.JSONDecodeError):
                continue
            if status == 200:
                projection = feed.get("distributed_polis") if isinstance(feed, dict) else None
                if projection and projection.get("owner_guardian_id") == guardian_names[node]:
                    owners.append((node, feed))
            elif status != 503:
                fail(f"non-owner Observatory returned unexpected status {status}")
        if len(owners) == 1 and (previous is None or owners[0][0] != previous):
            return owners[0]
        time.sleep(0.1)
    fail("exactly one leased Observatory did not become ready")


def read_exact(stream, count):
    value = bytearray()
    while len(value) < count:
        chunk = stream.recv(count - len(value))
        if not chunk:
            fail("unexpected websocket EOF")
        value.extend(chunk)
    return bytes(value)


def read_frame(stream):
    first, second = read_exact(stream, 2)
    length = second & 0x7F
    if length == 126:
        length = struct.unpack("!H", read_exact(stream, 2))[0]
    elif length == 127:
        length = struct.unpack("!Q", read_exact(stream, 8))[0]
    if length > MAX_FRAME:
        fail("websocket frame exceeded bound")
    return first & 0x0F, read_exact(stream, length)


def write_frame(stream, opcode, payload):
    mask = secrets.token_bytes(4)
    if len(payload) < 126:
        header = bytes((0x80 | opcode, 0x80 | len(payload)))
    else:
        header = bytes((0x80 | opcode, 0x80 | 126)) + struct.pack("!H", len(payload))
    masked = bytes(byte ^ mask[index % 4] for index, byte in enumerate(payload))
    stream.sendall(header + mask + masked)


def websocket(node, path, bearer_token=None):
    stream = tls_socket(node, timeout=10)
    key = base64.b64encode(secrets.token_bytes(16)).decode("ascii")
    authorization = f"Authorization: Bearer {bearer_token}\r\n" if bearer_token else ""
    stream.sendall((
        f"GET {path} HTTP/1.1\r\nHost: localhost\r\nUpgrade: websocket\r\n"
        f"Connection: Upgrade\r\nSec-WebSocket-Key: {key}\r\nSec-WebSocket-Version: 13\r\n"
        f"{authorization}\r\n"
    ).encode("ascii"))
    headers = bytearray()
    while not headers.endswith(b"\r\n\r\n"):
        headers.extend(read_exact(stream, 1))
    if not headers.startswith(b"HTTP/1.1 101"):
        fail(f"websocket upgrade failed: {bytes(headers)!r}")
    return stream


def observatory_ws(node):
    stream = websocket(node, "/v1/observatory/ws")
    stream.settimeout(0.5)
    try:
        leaked = stream.recv(1)
    except socket.timeout:
        leaked = b""
    if leaked:
        fail("Observatory websocket emitted data before authentication")
    stream.settimeout(5)
    auth = json.dumps({
        "schema": "adl.runtime_v3.observatory_ws_auth.v1",
        "bearer_token": observatory_token,
    }).encode()
    write_frame(stream, 1, auth)
    _, result = read_frame(stream)
    _, feed = read_frame(stream)
    if json.loads(result).get("status") != "authenticated":
        fail("Observatory websocket authentication failed")
    value = json.loads(feed)
    if not value.get("distributed_polis"):
        fail("Observatory websocket omitted distributed polis projection")
    stream.close()


def varint(value):
    encoded = bytearray()
    while True:
        byte = value & 0x7F
        value >>= 7
        encoded.append(byte | (0x80 if value else 0))
        if not value:
            return bytes(encoded)


def protobuf_string(tag, value):
    encoded = value.encode()
    return varint((tag << 3) | 2) + varint(len(encoded)) + encoded


def acip_envelope(message_id, source, sequence, prompt):
    payload = json.dumps({
        "schema": "adl.runtime.shepherd_request.v1",
        "correlation_id": message_id,
        "runtime_id": "issue-142-local-polis",
        "prompt": prompt,
    }, separators=(",", ":"), sort_keys=True)
    return b"".join((
        protobuf_string(1, "adl.csm.acip_carrier.protobuf_envelope.v1"),
        protobuf_string(2, message_id),
        protobuf_string(3, source),
        protobuf_string(4, "runtime-target"),
        protobuf_string(5, "shepherd"),
        protobuf_string(6, payload),
        varint(7 << 3),
        varint(sequence),
    ))


def acip(node, message_id, source, sequence, prompt):
    stream = websocket(node, "/v1/acip/ws", acip_token)
    _, hello = read_frame(stream)
    hello_value = json.loads(hello)
    if hello_value.get("event") != "authenticated":
        fail(f"ACIP authentication failed: {hello_value}")
    write_frame(stream, 2, acip_envelope(message_id, source, sequence, prompt))
    _, response = read_frame(stream)
    stream.close()
    return json.loads(response)


def snapshot_boundary(node, digest):
    body = json.dumps({
        "schema": "adl.distributed.local_snapshot_boundary.v1",
        "snapshot_sha256": digest,
    }).encode()
    request = (
        f"POST /internal/client/snapshot-boundary HTTP/1.1\r\nHost: localhost\r\n"
        f"Authorization: Bearer {local_kernel_tokens[node]}\r\nContent-Type: application/json\r\n"
        f"Content-Length: {len(body)}\r\nConnection: close\r\n\r\n"
    ).encode() + body
    with socket.create_connection(("127.0.0.1", raft_ports[node]), timeout=10) as stream:
        stream.sendall(request)
        response = bytearray()
        while True:
            chunk = stream.recv(65536)
            if not chunk:
                break
            response.extend(chunk)
    headers, payload = bytes(response).split(b"\r\n\r\n", 1)
    if not headers.startswith(b"HTTP/1.1 200"):
        fail(f"snapshot boundary was not committed: {headers!r}")
    return json.loads(payload)


try:
    start_node(2)
    start_node(3)
    start_node(1)
    owner, initial_feed = wait_owner()
    unauthenticated, _ = http_get(owner)
    if unauthenticated != 401:
        fail(f"unauthenticated Observatory returned {unauthenticated}, expected 401")
    observatory_ws(owner)
    first = acip(owner, "governed-work-1", "issue-142-client", 1, "Reply with exactly: polis-ready")
    if first.get("status") != "completed" or not first.get("sequence_reserved"):
        fail(f"governed Shepherd work failed: {first}")
    replay = acip(owner, "governed-work-replay", "issue-142-client", 1, "must not execute")
    if replay.get("status") != "rejected" or replay.get("reason") != "monotonic_sequence_must_advance":
        fail(f"ACIP replay was not rejected deterministically: {replay}")
    snapshot_digest = hashlib.sha256(json.dumps(first, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
    snapshot = snapshot_boundary(owner, snapshot_digest)
    if not snapshot.get("accepted"):
        fail("committed snapshot boundary was rejected")

    nonowners = [node for node in voter_names if node != owner]
    stop_node(nonowners[0])
    with_two = acip(owner, "governed-work-2", "issue-142-client", 2, "Reply with exactly: quorum-two")
    if with_two.get("status") != "completed":
        fail("two-of-three quorum did not continue governed work")
    start_node(nonowners[0])
    wait_owner()

    stop_node(owner)
    successor, successor_feed = wait_owner(previous=owner)
    successor_projection = successor_feed["distributed_polis"]
    if successor_projection.get("snapshot_sha256") != snapshot_digest:
        fail("successor Observatory did not retain the committed snapshot boundary")
    recovered = acip(successor, "governed-work-3", "issue-142-client", 3, "Reply with exactly: recovered-owner")
    if recovered.get("status") != "completed":
        fail("successor quorum did not continue governed work")

    remaining_peer = next(node for node in processes if node != successor)
    stop_node(remaining_peer)
    isolated = acip(successor, "governed-work-4", "issue-142-client", 4, "must not be authorized")
    if isolated.get("status") != "rejected":
        fail("one-of-three voter authorized governed mutation")

    report = {
        "schema": "adl.runtime_v3.distributed_operational_demo.v1",
        "status": "pass",
        "revision": revision,
        "phase": "local_wuji_three_voters",
        "voters": [voter_names[node] for node in voter_names],
        "quorum_size": 2,
        "initial_observatory_owner": guardian_names[owner],
        "successor_observatory_owner": guardian_names[successor],
        "authenticated_observatory_rest": True,
        "observatory_wss_no_pre_auth_frame": True,
        "governed_model": {"provider": "ollama_http", "model_ref": model_ref, "artifact_sha256": model_digest},
        "governed_result_hashes": [first.get("result_hash"), with_two.get("result_hash"), recovered.get("result_hash")],
        "replay_rejected": True,
        "snapshot_sha256": snapshot_digest,
        "snapshot_visible_after_owner_loss": True,
        "two_of_three_continued": True,
        "one_of_three_halted": True,
        "cleanup": "pending",
    }
    report_path = run_root / "local-proof.json"
    report_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
finally:
    cleanup()

for process, _, _ in processes.values():
    if process.poll() is None:
        fail("distributed Runtime process remained live after cleanup")
report = json.loads((run_root / "local-proof.json").read_text(encoding="utf-8"))
report["cleanup"] = "complete_zero_live_guardians"
(run_root / "local-proof.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
print(json.dumps({"status": "pass", "phase": "local", "report": str(run_root / "local-proof.json")}))
PY
