#!/usr/bin/env python3
"""Probe the already-running issue-607 Guardian over authenticated HTTPS and WSS."""

import argparse
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
import time

MAX_FRAME_BYTES = 65_536
MAX_HTTP_BYTES = 1_048_576
WEBSOCKET_GUID = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"


def sha256(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def read_exact(stream, count: int) -> bytes:
    value = bytearray()
    while len(value) < count:
        chunk = stream.recv(count - len(value))
        if not chunk:
            raise RuntimeError("unexpected EOF")
        value.extend(chunk)
    return bytes(value)


def read_frame(stream):
    first, second = read_exact(stream, 2)
    if not first & 0x80 or second & 0x80:
        raise RuntimeError("invalid server WebSocket frame")
    length = second & 0x7F
    if length == 126:
        length = struct.unpack("!H", read_exact(stream, 2))[0]
    elif length == 127:
        length = struct.unpack("!Q", read_exact(stream, 8))[0]
    if length > MAX_FRAME_BYTES:
        raise RuntimeError("WebSocket frame exceeds configured bound")
    return first & 0x0F, read_exact(stream, length)


def write_frame(stream, payload: bytes):
    mask = secrets.token_bytes(4)
    length = len(payload)
    if length < 126:
        prefix = bytes((0x81, 0x80 | length))
    elif length <= 65_535:
        prefix = bytes((0x81, 0x80 | 126)) + struct.pack("!H", length)
    else:
        prefix = bytes((0x81, 0x80 | 127)) + struct.pack("!Q", length)
    masked = bytes(byte ^ mask[index % 4] for index, byte in enumerate(payload))
    stream.sendall(prefix + mask + masked)


def tls_socket(address: str, roots: str, deadline: float):
    host, port = address.rsplit(":", 1)
    tcp = socket.create_connection((host, int(port)), timeout=max(0.1, deadline - time.monotonic()))
    context = ssl.create_default_context(cafile=roots)
    context.check_hostname = True
    context.verify_mode = ssl.CERT_REQUIRED
    try:
        tls = context.wrap_socket(tcp, server_hostname="localhost")
    except BaseException:
        tcp.close()
        raise
    tls.settimeout(max(0.1, deadline - time.monotonic()))
    return tls


def probe_https(address: str, roots: str, token: str, deadline: float):
    with tls_socket(address, roots, deadline) as tls:
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
                raise RuntimeError("HTTPS response exceeds configured bound")
    headers, body = bytes(response).split(b"\r\n\r\n", 1)
    if not headers.startswith(b"HTTP/1.1 200 OK"):
        raise RuntimeError("authenticated HTTPS did not return 200")
    value = json.loads(body)
    if value.get("schema") != "adl.runtime_v3.observatory_feed.v2":
        raise RuntimeError("wrong Observatory schema")
    return value, sha256(bytes(response))


def probe_wss(address: str, roots: str, token: str, deadline: float):
    with tls_socket(address, roots, deadline) as tls:
        key = base64.b64encode(secrets.token_bytes(16)).decode("ascii")
        request = (
            "GET /v1/observatory/ws HTTP/1.1\r\n"
            "Host: localhost\r\n"
            "Origin: https://observatory.example.test\r\n"
            "Upgrade: websocket\r\n"
            "Connection: Upgrade\r\n"
            f"Sec-WebSocket-Key: {key}\r\n"
            "Sec-WebSocket-Version: 13\r\n\r\n"
        ).encode("ascii")
        tls.sendall(request)
        headers = bytearray()
        while not headers.endswith(b"\r\n\r\n"):
            headers.extend(read_exact(tls, 1))
            if len(headers) > MAX_FRAME_BYTES:
                raise RuntimeError("WSS headers exceed configured bound")
        raw_headers = bytes(headers)
        if not raw_headers.startswith(b"HTTP/1.1 101 Switching Protocols"):
            raise RuntimeError("authenticated WSS did not return 101")
        expected = base64.b64encode(
            hashlib.sha1((key + WEBSOCKET_GUID).encode("ascii")).digest()
        ).decode("ascii")
        accept = next(
            (
                line.split(":", 1)[1].strip()
                for line in raw_headers.decode("iso-8859-1").splitlines()
                if line.lower().startswith("sec-websocket-accept:")
            ),
            None,
        )
        if accept != expected:
            raise RuntimeError("WSS accept digest mismatch")
        auth = json.dumps(
            {"schema": "adl.runtime_v3.observatory_ws_auth.v1", "bearer_token": token},
            separators=(",", ":"),
        ).encode()
        write_frame(tls, auth)
        feed = control = None
        for _ in range(6):
            opcode, payload = read_frame(tls)
            if opcode != 1:
                continue
            value = json.loads(payload)
            if value.get("schema") == "adl.runtime_v3.observatory_feed.v2":
                feed = payload
            if (
                value.get("schema") == "adl.runtime_v3.observatory_ws_control_result.v1"
                and value.get("status") == "authenticated"
            ):
                control = payload
            if feed is not None and control is not None:
                break
        if feed is None or control is None:
            raise RuntimeError("WSS feed and authenticated control result were not both observed")
    return raw_headers, feed, auth, control


def contained_file(path: str, root: pathlib.Path) -> pathlib.Path:
    candidate = pathlib.Path(path)
    resolved = candidate.resolve(strict=True)
    if resolved == root or root not in resolved.parents or candidate.is_symlink():
        raise RuntimeError("runtime credential path escapes state root")
    return resolved


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--init", required=True)
    parser.add_argument("--state-root", required=True)
    parser.add_argument("--output", required=True)
    parser.add_argument("--timeout-seconds", type=float, default=15.0)
    args = parser.parse_args()
    state_root = pathlib.Path(args.state_root).resolve(strict=True)
    init = contained_file(args.init, state_root)
    text = init.read_text(encoding="utf-8")
    address = re.search(r'^address = "([^"]+)"$', text, re.MULTILINE)
    roots = re.search(r'^trust_roots_path = "([^"]+)"$', text, re.MULTILINE)
    token = re.search(r'^observatory_token_path = "([^"]+)"$', text, re.MULTILINE)
    if not address or not roots or not token:
        raise RuntimeError("runtime init is missing probe inputs")
    roots_path = contained_file(roots.group(1), state_root)
    token_path = contained_file(token.group(1), state_root)
    deadline = time.monotonic() + args.timeout_seconds
    observed = []
    while True:
        try:
            bearer = token_path.read_text(encoding="utf-8").strip()
            observatory, https_digest = probe_https(
                address.group(1), str(roots_path), bearer, deadline
            )
            headers, hello, request, response = probe_wss(
                address.group(1), str(roots_path), bearer, deadline
            )
            result = {
                "schema": "adl.issue607.running_runtime_transport_proof.v1",
                "status": "pass",
                "runtime_instance_id": observatory["runtime_instance_id"],
                "runtime_process_id": observatory["runtime_process_id"],
                "authenticated_https": True,
                "authenticated_wss": True,
                "https_response_sha256": https_digest,
                "wss_upgrade_sha256": sha256(headers),
                "wss_hello_sha256": sha256(hello),
                "wss_request_sha256": sha256(request),
                "wss_response_sha256": sha256(response),
            }
            output = pathlib.Path(args.output)
            temporary = output.with_suffix(output.suffix + ".tmp")
            temporary.write_text(json.dumps(result, sort_keys=True) + "\n", encoding="utf-8")
            os.replace(temporary, output)
            return
        except Exception as error:  # bounded retry records only exception classes
            if type(error).__name__ not in observed:
                observed.append(type(error).__name__)
            if time.monotonic() >= deadline:
                raise RuntimeError("runtime transport probe timed out: " + ",".join(observed)) from error
            time.sleep(0.05)


if __name__ == "__main__":
    main()
