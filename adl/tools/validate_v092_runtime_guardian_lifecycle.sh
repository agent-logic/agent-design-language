#!/usr/bin/env bash
set -euo pipefail

repo_root=$(cd "$(dirname "$0")/../.." && pwd -P)
qualification_root=${ADL_RUNTIME_GUARDIAN_EVIDENCE_ROOT:-$repo_root/.adl/runtime-v3/qualification}
target_dir=${CARGO_TARGET_DIR:-$repo_root/.adl/target/5820-runtime}
target_root=${ADL_RUNTIME_GUARDIAN_TARGET_ROOT:-$repo_root}
lifecycle_suite=${ADL_RUNTIME_GUARDIAN_SUITE:-preflight}

while (( $# > 0 )); do
  case "$1" in
    --suite)
      (( $# >= 2 )) || { echo "--suite requires a value" >&2; exit 64; }
      lifecycle_suite=$2
      shift 2
      ;;
    -h|--help)
      echo "Usage: $0 [--suite preflight_1x|lifecycle_10000|stress_100x10s|endurance_10x600s|six_hour_qualification]"
      exit 0
      ;;
    *)
      echo "unsupported argument: $1" >&2
      exit 64
      ;;
  esac
done

case "$lifecycle_suite" in
  preflight|preflight_1x|lifecycle_10000|stress_100x10s|endurance_10x600s|six_hour|six_hour_qualification) ;;
  *)
    echo "unsupported ADL_RUNTIME_GUARDIAN_SUITE: $lifecycle_suite" >&2
    exit 64
    ;;
esac

validate_contained_path() {
  python3 - "$1" "$2" "$3" <<'PY'
import os
import pathlib
import sys

root_arg, candidate_arg, required_prefix = sys.argv[1:]


def fail(message):
    raise SystemExit(message)


root_path = pathlib.Path(root_arg)
candidate_path = pathlib.Path(candidate_arg)
if not root_path.is_absolute():
    fail("containment root must be absolute")
if not root_path.is_dir():
    fail("containment root must exist")
if root_path.is_symlink():
    fail("containment root must not be a symlink")
if not candidate_path.is_absolute():
    fail("candidate path must be absolute")
if ".." in candidate_path.parts:
    fail("candidate path must not contain traversal")

root = os.path.realpath(root_arg)
candidate = os.path.abspath(candidate_arg)
try:
    relative = os.path.relpath(candidate, root)
except ValueError:
    fail("candidate path escapes containment root")
if relative == os.pardir or relative.startswith(os.pardir + os.sep):
    fail("candidate path escapes containment root")
if relative == os.curdir:
    fail("candidate path escapes containment root")
if required_prefix and not relative.startswith(required_prefix):
    fail("candidate path is outside the required prefix")

current = root
for part in pathlib.PurePath(relative).parts:
    current = os.path.join(current, part)
    if os.path.islink(current):
        fail("candidate path traverses a symlink")
print(candidate)
PY
}

qualification_root=$(validate_contained_path "$repo_root" "$qualification_root" ".adl/") || exit 64
target_dir=$(validate_contained_path "$target_root" "$target_dir" "") || exit 64
mkdir -p "$qualification_root" "$target_dir"
[[ "$(cd "$qualification_root" && pwd -P)" == "$qualification_root" ]] || {
  echo "evidence root canonicalization changed after creation" >&2
  exit 64
}
[[ "$(cd "$target_dir" && pwd -P)" == "$target_dir" ]] || {
  echo "target directory canonicalization changed after creation" >&2
  exit 64
}
export CARGO_TARGET_DIR="$target_dir"

vector_bin=${ADL_RUNTIME_VECTOR_BIN:-}
if [[ -z "$vector_bin" ]]; then
  vector_bin=$(command -v vector || true)
fi
if [[ -z "$vector_bin" || ! -x "$vector_bin" ]]; then
  echo "ADL_RUNTIME_VECTOR_BIN must name an executable Vector binary" >&2
  exit 69
fi
vector_bin=$(cd "$(dirname "$vector_bin")" && pwd -P)/$(basename "$vector_bin")

cargo build --locked --manifest-path "$repo_root/adl-runtime-kernel/Cargo.toml" \
  --bin adl-runtime-kernel
cargo build --locked --manifest-path "$repo_root/adl-runtime/Cargo.toml" \
  --bin adl-runtime-guardian --bin adl-runtime-lifecycle-soak

revision=$(git -C "$repo_root" rev-parse HEAD)
run_root=$(mktemp -d "$qualification_root/5820-run.XXXXXX")
state_root="$run_root/state"
report="$run_root/report.json"
wss_proof="$run_root/wss-proof.json"
wss_transcript="$run_root/wss-transcript.json"
https_transcript="$run_root/https-transcript.json"
wss_stderr="$run_root/wss-proof.stderr"
probe_ready="$run_root/pre-restart.ready"
probe_ack="$run_root/pre-restart.ack"
api_port=$(python3 - <<'PY'
import socket

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as listener:
    listener.bind(("127.0.0.1", 0))
    print(listener.getsockname()[1])
PY
)
continuity_port=$(python3 - "$api_port" <<'PY'
import socket
import sys

api_port = int(sys.argv[1])
while True:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as listener:
        listener.bind(("127.0.0.1", 0))
        port = listener.getsockname()[1]
    if port != api_port:
        print(port)
        break
PY
)
init_template="$qualification_root/5820-runtime-init-$api_port.toml"
mkdir -p "$(dirname "$init_template")"
python3 - "$repo_root/infra/runtime-v3/runtime-init.toml" "$init_template" "$api_port" \
  "$continuity_port" "$state_root" <<'PY'
import hashlib
import os
import pathlib
import re
import shutil
import subprocess
import sys

source, destination, port, continuity_port, state_root = sys.argv[1:]
source_path = pathlib.Path(source)
destination_path = pathlib.Path(destination)
state_root_path = pathlib.Path(state_root)
repo_root = source_path.parents[2]
fixture_root = repo_root / "adl-runtime" / "tests" / "support" / "tls-fixtures"
tls_root = destination_path.parent / "tls"
tls_root.mkdir(mode=0o700, exist_ok=True)
certificate = tls_root / "server-cert.pem"
private_key = tls_root / "server-key.pem"
trust_roots = tls_root / "root-ca.pem"
guardian_certificate = tls_root / "client-cert.pem"
guardian_private_key = tls_root / "client-key.pem"
certificate.write_bytes(
    (fixture_root / "server-cert.pem").read_bytes()
    + (fixture_root / "intermediate-ca.pem").read_bytes()
)
shutil.copyfile(fixture_root / "server-key.pem", private_key)
shutil.copyfile(fixture_root / "root-ca.pem", trust_roots)
shutil.copyfile(fixture_root / "client-cert.pem", guardian_certificate)
shutil.copyfile(fixture_root / "client-key.pem", guardian_private_key)
os.chmod(certificate, 0o600)
os.chmod(private_key, 0o600)
os.chmod(trust_roots, 0o600)
os.chmod(guardian_certificate, 0o600)
os.chmod(guardian_private_key, 0o600)

def spki_sha256(path):
    public_key = subprocess.run(
        ["openssl", "x509", "-in", str(path), "-pubkey", "-noout"],
        check=True,
        capture_output=True,
    ).stdout
    der = subprocess.run(
        ["openssl", "pkey", "-pubin", "-outform", "DER"],
        input=public_key,
        check=True,
        capture_output=True,
    ).stdout
    return hashlib.sha256(der).hexdigest()

text = source_path.read_text(encoding="utf-8")
address = 'address = "127.0.0.1:20997"'
public_url_pattern = re.compile(r'^public_base_url\s*=\s*"https://[^"]+"$', re.MULTILINE)
server_name_pattern = re.compile(r'^server_name\s*=\s*"[^"]+"$', re.MULTILINE)
observatory_origin = '  "https://observatory.dev.agent-logic.ai",'
readiness_timeout = "readiness_timeout_millis = 10000"
tls_fields = {
    'certificate_chain_path = "/var/lib/adl/runtime-v3/tls/fullchain.pem"':
        f'certificate_chain_path = "{certificate}"',
    'private_key_path = "/var/lib/adl/runtime-v3/tls/private-key.pem"':
        f'private_key_path = "{private_key}"',
    'trust_roots_path = "/var/lib/adl/runtime-v3/tls/trust-roots.pem"':
        f'trust_roots_path = "{trust_roots}"',
}
if text.count(address) != 1:
    raise SystemExit("canonical API address missing")
if len(public_url_pattern.findall(text)) != 1:
    raise SystemExit("canonical public URL field missing or ambiguous")
if len(server_name_pattern.findall(text)) != 1:
    raise SystemExit("canonical TLS server name field missing or ambiguous")
if text.count(observatory_origin) != 1:
    raise SystemExit("canonical Observatory origin missing or ambiguous")
if text.count(readiness_timeout) != 1:
    raise SystemExit("canonical qualification readiness timeout missing or ambiguous")
text = text.replace(address, f'address = "127.0.0.1:{port}"', 1)
text = public_url_pattern.sub(f'public_base_url = "https://localhost:{port}"', text, count=1)
text = server_name_pattern.sub('server_name = "localhost"', text, count=1)
text = text.replace(observatory_origin, '  "https://observatory.example.test",', 1)
text = text.replace(readiness_timeout, "readiness_timeout_millis = 120000", 1)
for canonical, localized in tls_fields.items():
    if text.count(canonical) != 1:
        raise SystemExit("canonical TLS configuration field missing or ambiguous")
    text = text.replace(canonical, localized, 1)
text += f'''\n[continuity_control]
address = "127.0.0.1:{continuity_port}"
guardian_state_dir = "{state_root_path / 'guardian-continuity'}"
state_dir = "{state_root_path / 'kernel-continuity'}"
staging_dir = "{state_root_path / 'continuity-staging'}"
trust_domain = "agent-logic.lifecycle"
polis = "lifecycle-polis"
source_node = "lifecycle-source"
target_node = "lifecycle-target"
guardian_id = "lifecycle-guardian"
kernel_control_id = "lifecycle-kernel-control"
channel_epoch = 1

[continuity_control.tls]
server_certificate_chain_path = "{certificate}"
server_private_key_path = "{private_key}"
server_trust_roots_path = "{trust_roots}"
server_name = "localhost"
guardian_certificate_chain_path = "{guardian_certificate}"
guardian_private_key_path = "{guardian_private_key}"
guardian_trust_roots_path = "{trust_roots}"
guardian_spki_sha256 = "{spki_sha256(guardian_certificate)}"
server_spki_sha256 = "{spki_sha256(certificate)}"
certificate_generation = 1

[continuity_control.bounds]
max_frame_bytes = 65536
max_blob_bytes = 65536
max_total_bytes = 524288
max_services = 5
max_journal_entries = 64
max_open_handles = 8
'''
destination_path.write_text(text, encoding="utf-8")
PY

python3 - "$state_root" "$wss_proof" "$https_transcript" "$wss_transcript" \
  "$probe_ready" "$probe_ack" <<'PY' 2>"$wss_stderr" &
import base64
import hashlib
import json
import os
import pathlib
import re
import secrets
import socket
import ssl
import struct
import sys
import time

(
    state_root_arg,
    proof_path,
    https_transcript_path,
    wss_transcript_path,
    probe_ready_path,
    probe_ack_path,
) = sys.argv[1:]
deadline = time.monotonic() + 180.0
MAX_FRAME_BYTES = 65_536
MAX_HTTP_BYTES = 1_048_576
WEBSOCKET_GUID = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"


def read_exact(stream, count):
    value = bytearray()
    while len(value) < count:
        chunk = stream.recv(count - len(value))
        if not chunk:
            raise RuntimeError("unexpected EOF")
        value.extend(chunk)
    return bytes(value)


def is_beneath(path, root):
    try:
        return os.path.commonpath((path, root)) == root and path != root
    except ValueError:
        return False


def safe_existing(path, root, label):
    candidate = pathlib.Path(path)
    if ".." in candidate.parts:
        raise RuntimeError(f"{label} contains traversal")
    root = os.path.realpath(root)
    expanded = os.path.abspath(path)
    if not is_beneath(expanded, root):
        raise RuntimeError(f"{label} escapes state root")
    current = root
    for part in pathlib.PurePath(os.path.relpath(expanded, root)).parts:
        current = os.path.join(current, part)
        if os.path.islink(current):
            raise RuntimeError(f"{label} traverses a symlink")
    resolved = os.path.realpath(expanded)
    if not is_beneath(resolved, root):
        raise RuntimeError(f"{label} resolves outside state root")
    if not os.path.exists(resolved):
        raise RuntimeError(f"{label} does not exist")
    return resolved


def write_json(path, value):
    temporary = path + ".tmp"
    with open(temporary, "w", encoding="utf-8") as stream:
        stream.write(json.dumps(value, indent=2))
    os.replace(temporary, path)


def sha256_bytes(value):
    return hashlib.sha256(value).hexdigest()


def sha256_file(path):
    digest = hashlib.sha256()
    with open(path, "rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def tls_socket(address, certificate):
    host, port = address.rsplit(":", 1)
    remaining = max(0.1, deadline - time.monotonic())
    tcp = socket.create_connection((host, int(port)), timeout=remaining)
    context = ssl.create_default_context(cafile=certificate)
    context.check_hostname = True
    context.verify_mode = ssl.CERT_REQUIRED
    try:
        tls = context.wrap_socket(tcp, server_hostname="localhost")
    except BaseException:
        tcp.close()
        raise
    tls.settimeout(max(0.1, deadline - time.monotonic()))
    return tls


def authenticated_https(address, certificate, token):
    with tls_socket(address, certificate) as tls:
        request = (
            "GET /v1/observatory HTTP/1.1\r\n"
            "Host: localhost\r\n"
            f"Authorization: Bearer {token}\r\n"
            "Connection: close\r\n\r\n"
        ).encode("ascii")
        tls.sendall(request)
        response = bytearray()
        while True:
            chunk = tls.recv(65_536)
            if not chunk:
                break
            response.extend(chunk)
            if len(response) > MAX_HTTP_BYTES:
                raise RuntimeError("authenticated HTTPS response exceeds configured bound")
    raw_response = bytes(response)
    try:
        status, body = raw_response.split(b"\r\n\r\n", 1)
    except ValueError as error:
        raise RuntimeError("authenticated HTTPS response is incomplete") from error
    if not status.startswith(b"HTTP/1.1 200 OK"):
        raise RuntimeError("authenticated HTTPS did not return 200")
    value = json.loads(body.decode("utf-8"))
    if value.get("schema") != "adl.runtime_v3.observatory_feed.v2":
        raise RuntimeError("wrong Observatory schema")
    return value, sha256_bytes(raw_response)


def read_frame(stream):
    first, second = read_exact(stream, 2)
    if not first & 0x80:
        raise RuntimeError("fragmented WebSocket frame")
    if second & 0x80:
        raise RuntimeError("masked server WebSocket frame")
    length = second & 0x7F
    if length == 126:
        length = struct.unpack("!H", read_exact(stream, 2))[0]
    elif length == 127:
        length = struct.unpack("!Q", read_exact(stream, 8))[0]
    if length > MAX_FRAME_BYTES:
        raise RuntimeError("WebSocket frame exceeds configured bound")
    return first & 0x0F, read_exact(stream, length)


def write_frame(stream, opcode, payload):
    if len(payload) > MAX_FRAME_BYTES:
        raise RuntimeError("WebSocket request exceeds configured bound")
    mask = secrets.token_bytes(4)
    length = len(payload)
    if length < 126:
        prefix = bytes((0x80 | opcode, 0x80 | length))
    elif length <= 65_535:
        prefix = bytes((0x80 | opcode, 0x80 | 126)) + struct.pack("!H", length)
    else:
        prefix = bytes((0x80 | opcode, 0x80 | 127)) + struct.pack("!Q", length)
    masked = bytes(byte ^ mask[index % 4] for index, byte in enumerate(payload))
    stream.sendall(prefix + mask + masked)


def authenticated_wss(address, certificate, token):
    with tls_socket(address, certificate) as tls:
        key = base64.b64encode(secrets.token_bytes(16)).decode("ascii")
        upgrade_request = (
            "GET /v1/observatory/ws HTTP/1.1\r\n"
            "Host: localhost\r\n"
            "Origin: https://observatory.example.test\r\n"
            "Upgrade: websocket\r\n"
            "Connection: Upgrade\r\n"
            f"Sec-WebSocket-Key: {key}\r\n"
            "Sec-WebSocket-Version: 13\r\n"
            "\r\n"
        ).encode("ascii")
        tls.sendall(upgrade_request)
        headers = bytearray()
        while not headers.endswith(b"\r\n\r\n"):
            headers.extend(read_exact(tls, 1))
            if len(headers) > MAX_FRAME_BYTES:
                raise RuntimeError("WSS upgrade headers exceed configured bound")
        raw_headers = bytes(headers)
        if not raw_headers.startswith(b"HTTP/1.1 101 Switching Protocols"):
            raise RuntimeError("authenticated WSS did not return 101")
        expected = base64.b64encode(hashlib.sha1((key + WEBSOCKET_GUID).encode("ascii")).digest()).decode("ascii")
        accept = None
        for line in raw_headers.decode("iso-8859-1").splitlines():
            if line.lower().startswith("sec-websocket-accept:"):
                accept = line.split(":", 1)[1].strip()
                break
        if accept != expected:
            raise RuntimeError("WSS upgrade accept digest mismatch")
        request = json.dumps({
            "schema": "adl.runtime_v3.observatory_ws_auth.v1",
            "bearer_token": token,
        }, separators=(",", ":")).encode("utf-8")
        write_frame(tls, 1, request)
        feed = None
        response = None
        for _ in range(4):
            opcode, payload = read_frame(tls)
            if opcode != 1:
                continue
            value = json.loads(payload.decode("utf-8"))
            if value.get("schema") == "adl.runtime_v3.observatory_feed.v2":
                feed = payload
            if (
                value.get("schema") == "adl.runtime_v3.observatory_ws_control_result.v1"
                and value.get("status") == "authenticated"
            ):
                response = payload
            if feed is not None and response is not None:
                break
        if feed is None or response is None:
            raise RuntimeError("WSS feed and authenticated control result were not both observed")
    return raw_headers, feed, request, response


while not os.path.isfile(probe_ready_path):
    if time.monotonic() >= deadline:
        raise RuntimeError("lifecycle harness did not publish pre-restart readiness")
    time.sleep(0.01)
probe_nonce = pathlib.Path(probe_ready_path).read_text(encoding="utf-8").strip()
if not probe_nonce:
    raise RuntimeError("lifecycle harness published an empty pre-restart nonce")

while True:
    try:
        if not os.path.isdir(state_root_arg):
            raise RuntimeError("state root is not ready")
        if os.path.islink(state_root_arg):
            raise RuntimeError("state root is a symlink")
        state_root = os.path.realpath(state_root_arg)
        init = safe_existing(os.path.join(state_root, "runtime-init.toml"), state_root, "runtime init")
        text = pathlib.Path(init).read_text(encoding="utf-8")
        address_match = re.search(r'^address = "([^"]+)"$', text, re.MULTILINE)
        certificate_match = re.search(r'^certificate_chain_path = "([^"]+)"$', text, re.MULTILINE)
        token_match = re.search(r'^observatory_token_path = "([^"]+)"$', text, re.MULTILINE)
        if not address_match:
            raise RuntimeError("API address missing")
        if not certificate_match:
            raise RuntimeError("certificate path missing")
        if not token_match:
            raise RuntimeError("token path missing")
        certificate = safe_existing(certificate_match.group(1), state_root, "certificate")
        token_file = safe_existing(token_match.group(1), state_root, "observatory token")
        token = pathlib.Path(token_file).read_text(encoding="utf-8").strip()
        observatory, https_sha256 = authenticated_https(address_match.group(1), certificate, token)
        headers, hello, request, response = authenticated_wss(address_match.group(1), certificate, token)
        https_value = {
            "schema": "adl.runtime_v3.guardian_https_transcript.v1",
            "request": {"method": "GET", "path": "/v1/observatory", "authentication": "bearer_redacted"},
            "response": {"status": 200, "sha256": https_sha256, "schema": observatory["schema"]},
            "runtime_instance_id": observatory["runtime_instance_id"],
            "runtime_process_id": observatory["runtime_process_id"],
        }
        hello_value = json.loads(hello.decode("utf-8"))
        response_value = json.loads(response.decode("utf-8"))
        wss_value = {
            "schema": "adl.runtime_v3.guardian_wss_transcript.v1",
            "request": {"method": "GET", "path": "/v1/observatory/ws", "authentication": "in_band_bearer_redacted", "upgrade": "websocket"},
            "upgrade": {"status": 101, "sha256": sha256_bytes(headers)},
            "hello": hello_value,
            "bounded_request": {"opcode": "text", "bytes": len(request), "sha256": sha256_bytes(request)},
            "response": response_value,
        }
        write_json(https_transcript_path, https_value)
        write_json(wss_transcript_path, wss_value)
        proof = {
            "schema": "adl.runtime_v3.guardian_wss_proof.v1",
            "status": "pass",
            "runtime_instance_id": observatory["runtime_instance_id"],
            "runtime_process_id": observatory["runtime_process_id"],
            "authenticated_https": True,
            "https_transcript_path": https_transcript_path,
            "https_transcript_sha256": sha256_file(https_transcript_path),
            "authenticated_wss": True,
            "wss_transcript_path": wss_transcript_path,
            "wss_transcript_sha256": sha256_file(wss_transcript_path),
            "wss_upgrade_sha256": sha256_bytes(headers),
            "wss_hello_sha256": sha256_bytes(hello),
            "wss_request_sha256": sha256_bytes(request),
            "wss_response_sha256": sha256_bytes(response),
            "bounded_request_response": True,
        }
        write_json(proof_path, proof)
        probe_ack_temporary = probe_ack_path + ".tmp"
        pathlib.Path(probe_ack_temporary).write_text(probe_nonce + "\n", encoding="utf-8")
        os.replace(probe_ack_temporary, probe_ack_path)
        break
    except Exception as error:
        if os.environ.get("ADL_RUNTIME_WSS_DEBUG") == "1":
            print(error, file=sys.stderr)
        if time.monotonic() >= deadline:
            raise
        time.sleep(0.01)
PY
wss_probe_pid=$!
trap 'kill "$wss_probe_pid" 2>/dev/null || true' EXIT

soak_status=0
"$target_dir/debug/adl-runtime-lifecycle-soak" \
  --guardian "$target_dir/debug/adl-runtime-guardian" \
  --kernel "$target_dir/debug/adl-runtime-kernel" \
  --vector "$vector_bin" \
  --init-template "$init_template" \
  --state-root "$state_root" \
  --report "$report" \
  --revision "$revision" \
  --pre-restart-ready-file "$probe_ready" \
  --pre-restart-ack-file "$probe_ack" \
  --suite "$lifecycle_suite" || soak_status=$?

probe_status=0
wait "$wss_probe_pid" || probe_status=$?
trap - EXIT
if (( probe_status != 0 )); then
  echo "authenticated HTTPS/WSS probe failed; retained diagnostic follows" >&2
  cat "$wss_stderr" >&2 || true
fi
if (( soak_status != 0 || probe_status != 0 )); then
  echo "Guardian lifecycle validation failed: soak_status=$soak_status probe_status=$probe_status" >&2
  exit 1
fi

python3 - "$report" "$wss_proof" "$run_root/issue-proof.json" "$revision" \
  "$target_dir/debug/adl-runtime-guardian" "$target_dir/debug/adl-runtime-kernel" \
  "$repo_root/infra/runtime-v3/runtime-init.toml" "$https_transcript" "$wss_transcript" <<'PY'
import hashlib
import json
import os
import sys

(
    report_path,
    wss_path,
    proof_path,
    revision,
    guardian_path,
    kernel_path,
    canonical_init_path,
    https_transcript_path,
    wss_transcript_path,
) = sys.argv[1:]


def fail(message):
    raise SystemExit(message)


def sha256_file(path):
    digest = hashlib.sha256()
    with open(path, "rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


with open(report_path, encoding="utf-8") as stream:
    report = json.load(stream)
with open(wss_path, encoding="utf-8") as stream:
    wss = json.load(stream)

if report.get("schema") != "adl.runtime_v3.lifecycle_soak.v1":
    fail("wrong lifecycle report schema")
if report.get("status") != "pass":
    fail("lifecycle preflight failed")
runtime_soak = report.get("runtime_v3_soak") or {}
if runtime_soak.get("status") != "pass":
    fail("bounded Runtime soak evaluation did not pass")
evaluation = (runtime_soak.get("evidence") or {}).get("evaluation") or {}
if evaluation.get("status") != "pass" or evaluation.get("violations"):
    fail("bounded Runtime soak evidence remained fail-closed")
if report.get("revision") != revision:
    fail("lifecycle revision drifted")
if report.get("suite") == "six_hour_qualification":
    if int(report.get("minimum_exposure_seconds", 0)) != 21600:
        fail("six-hour minimum exposure denominator drifted")
    measured = int(report.get("measured_exposure_seconds", 0))
    overshoot = int(report.get("overshoot_seconds", -1))
    if measured < 21600 or overshoot != measured - 21600:
        fail("six-hour measured exposure did not reconcile")
    if int(report.get("maximum_overshoot_seconds", 0)) != 600 or overshoot > 600:
        fail("six-hour final-cycle overshoot exceeded its fixed cap")
completed_cycles = int(report.get("completed_cycles", 0))
total_restarts = int(report.get("total_restarts", 0))
if completed_cycles < 1:
    fail("no lifecycle cycle completed")
if int(report.get("guardian_launch_count", 0)) != completed_cycles:
    fail("Guardian launch denominator drifted")
if int(report.get("runtime_start_count", 0)) != completed_cycles + total_restarts:
    fail("kernel start denominator did not reconcile with restarts")
if report.get("restart_budget_exercised") is not True:
    fail("kernel restart was not exercised")
if total_restarts < 1:
    fail("kernel restart count was empty")
if int(report.get("continuity_generation", 0)) != completed_cycles:
    fail("durable continuity was not retained")
expected_acceptance = report.get("suite") != "preflight_1x"
if report.get("acceptance_eligible") is not expected_acceptance:
    fail("suite acceptance eligibility drifted")
if expected_acceptance and report.get("anti_rollback_minimum_enforced") is not True:
    fail("acceptance suite did not enforce anti-rollback continuity")
if report.get("logging_complete") is not True or report.get("master_log_status") != "clean":
    fail("clean log proof is missing")
if wss.get("status") != "pass" or wss.get("authenticated_https") is not True:
    fail("real authenticated HTTPS proof is missing")
if wss.get("authenticated_wss") is not True or wss.get("bounded_request_response") is not True:
    fail("real authenticated WSS proof is missing")
if wss.get("https_transcript_path") != https_transcript_path or wss.get("https_transcript_sha256") != sha256_file(https_transcript_path):
    fail("HTTPS transcript digest mismatch")
if wss.get("wss_transcript_path") != wss_transcript_path or wss.get("wss_transcript_sha256") != sha256_file(wss_transcript_path):
    fail("WSS transcript digest mismatch")

proof = {
    "schema": "adl.runtime_v3.guardian_lifecycle_proof.v1",
    "status": "pass",
    "source_revision": revision,
    "acceptance_eligible": report.get("acceptance_eligible") is True,
    "lifecycle_component_suite": report.get("suite"),
    "lifecycle_component_acceptance_eligible": report.get("acceptance_eligible"),
    "lifecycle_report_path": report_path,
    "lifecycle_report_sha256": sha256_file(report_path),
    "wss_proof_path": wss_path,
    "wss_proof_sha256": sha256_file(wss_path),
    "guardian_binary_path": guardian_path,
    "guardian_binary_sha256": sha256_file(guardian_path),
    "kernel_binary_path": kernel_path,
    "kernel_binary_sha256": sha256_file(kernel_path),
    "canonical_init_path": canonical_init_path,
    "canonical_init_sha256": sha256_file(canonical_init_path),
    "https_transcript_path": https_transcript_path,
    "https_transcript_sha256": sha256_file(https_transcript_path),
    "wss_transcript_path": wss_transcript_path,
    "wss_transcript_sha256": sha256_file(wss_transcript_path),
    "assertions": {
        "guardian_launched": True,
        "kernel_ready": True,
        "authenticated_https": True,
        "authenticated_wss": True,
        "child_killed": True,
        "bounded_restart": True,
        "state_preserved": True,
        "clean_shutdown": True,
        "clean_logs": True,
    },
}
temporary = proof_path + ".tmp"
with open(temporary, "w", encoding="utf-8") as stream:
    stream.write(json.dumps(proof, indent=2))
os.replace(temporary, proof_path)
PY

printf 'PASS: production Guardian lifecycle proof=%s revision=%s\n' "$run_root/issue-proof.json" "$revision"
if [[ "$lifecycle_suite" == "six_hour" || "$lifecycle_suite" == "six_hour_qualification" ]]; then
  printf 'ADL_ISSUE268_REPORT_BEGIN\n'
  cat "$report"
  printf '\nADL_ISSUE268_REPORT_END\n'
  printf 'ADL_ISSUE268_PROOF_BEGIN\n'
  cat "$run_root/issue-proof.json"
  printf '\nADL_ISSUE268_PROOF_END\n'
fi
