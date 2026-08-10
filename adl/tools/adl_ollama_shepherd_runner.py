#!/usr/bin/python3
"""Bounded loopback Ollama adapter for the Runtime shepherd executor."""

from __future__ import annotations

import ipaddress
import json
import os
import sys
import urllib.error
import urllib.parse
import urllib.request

REQUEST_SCHEMA = "adl.runtime.shepherd_runner_request.v1"
RESPONSE_SCHEMA = "adl.runtime.shepherd_runner_response.v1"
MAX_REQUEST_BYTES = 2 * 1024 * 1024
MAX_OLLAMA_RESPONSE_BYTES = 2 * 1024 * 1024
MAX_AGENT_REF_BYTES = 128
MAX_CONTEXT_TOKENS = 1_048_576


def fail(message: str) -> None:
    print(message, file=sys.stderr)
    raise SystemExit(1)


def loopback_endpoint(value: str) -> str:
    parsed = urllib.parse.urlsplit(value)
    if (
        parsed.scheme != "http"
        or parsed.query
        or parsed.fragment
        or parsed.username
        or parsed.password
    ):
        fail("invalid loopback Ollama endpoint")
    try:
        address = ipaddress.ip_address(parsed.hostname or "")
    except ValueError:
        fail("invalid loopback Ollama endpoint")
    if not address.is_loopback or parsed.path not in ("", "/"):
        fail("invalid loopback Ollama endpoint")
    if parsed.port is None:
        fail("loopback Ollama endpoint requires an explicit port")
    return value.rstrip("/")


def read_json() -> dict[str, object]:
    body = sys.stdin.buffer.read(MAX_REQUEST_BYTES + 1)
    if not body or len(body) > MAX_REQUEST_BYTES:
        fail("invalid shepherd request size")
    try:
        value = json.loads(body)
    except (UnicodeDecodeError, json.JSONDecodeError):
        fail("invalid shepherd request JSON")
    required = {
        "schema",
        "correlation_id",
        "runtime_id",
        "nonce",
        "backend_identity",
        "model_identity",
        "model_artifact_sha256",
        "prompt",
    }
    if not isinstance(value, dict) or set(value) != required:
        fail("invalid shepherd request fields")
    if value["schema"] != REQUEST_SCHEMA or any(
        not isinstance(value[name], str) or not value[name]
        for name in required - {"schema"}
    ):
        fail("invalid shepherd request values")
    return value


def main() -> None:
    request = read_json()
    endpoint = loopback_endpoint(os.environ.get("ADL_OLLAMA_ENDPOINT", ""))
    agent_ref = os.environ.get("ADL_SHEPHERD_AGENT_REF", "")
    shepherd_identity = os.environ.get("ADL_SHEPHERD_IDENTITY_REF", "")
    try:
        context_tokens = int(os.environ.get("ADL_MODEL_CONTEXT_TOKENS", "0"))
    except ValueError:
        fail("invalid model context bound")
    if not agent_ref or len(agent_ref.encode()) > MAX_AGENT_REF_BYTES:
        fail("invalid shepherd agent reference")
    if not shepherd_identity or len(shepherd_identity.encode()) > MAX_AGENT_REF_BYTES:
        fail("invalid shepherd identity reference")
    if not 256 <= context_tokens <= MAX_CONTEXT_TOKENS:
        fail("invalid model context bound")
    if request["backend_identity"] != "ollama_http":
        fail("unexpected shepherd backend")

    try:
        with urllib.request.urlopen(endpoint + "/api/tags", timeout=10) as response:
            tags_body = response.read(MAX_OLLAMA_RESPONSE_BYTES + 1)
    except (OSError, urllib.error.URLError, TimeoutError):
        fail("local model inventory failed")
    if not tags_body or len(tags_body) > MAX_OLLAMA_RESPONSE_BYTES:
        fail("invalid local model inventory size")
    try:
        inventory = json.loads(tags_body)
    except (UnicodeDecodeError, json.JSONDecodeError):
        fail("invalid local model inventory JSON")
    models = inventory.get("models") if isinstance(inventory, dict) else None
    if not isinstance(models, list):
        fail("invalid local model inventory")
    selected = next(
        (
            model
            for model in models
            if isinstance(model, dict)
            and request["model_identity"] in (model.get("name"), model.get("model"))
        ),
        None,
    )
    digest = selected.get("digest") if isinstance(selected, dict) else None
    if isinstance(digest, str) and digest.startswith("sha256:"):
        digest = digest.removeprefix("sha256:")
    if digest != request["model_artifact_sha256"]:
        fail("local model artifact attestation failed")

    prompt = (
        f"Shepherd agent {agent_ref} with governed identity {shepherd_identity}\n\n"
        f"{request['prompt']}"
    )
    payload = json.dumps(
        {
            "model": request["model_identity"],
            "prompt": prompt,
            "stream": False,
            "options": {"num_ctx": context_tokens},
        },
        separators=(",", ":"),
    ).encode()
    http_request = urllib.request.Request(
        endpoint + "/api/generate",
        data=payload,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(http_request, timeout=110) as response:
            body = response.read(MAX_OLLAMA_RESPONSE_BYTES + 1)
    except (OSError, urllib.error.URLError, TimeoutError):
        fail("local model request failed")
    if not body or len(body) > MAX_OLLAMA_RESPONSE_BYTES:
        fail("invalid local model response size")
    try:
        model_response = json.loads(body)
    except (UnicodeDecodeError, json.JSONDecodeError):
        fail("invalid local model response JSON")
    text = model_response.get("response") if isinstance(model_response, dict) else None
    returned_model = model_response.get("model") if isinstance(model_response, dict) else None
    if returned_model != request["model_identity"]:
        fail("local model identity changed during inference")
    if not isinstance(text, str) or not text.strip():
        fail("empty local model response")

    output = {
        "schema": RESPONSE_SCHEMA,
        "correlation_id": request["correlation_id"],
        "runtime_id": request["runtime_id"],
        "nonce": request["nonce"],
        "backend_identity": request["backend_identity"],
        "model_identity": request["model_identity"],
        "model_artifact_sha256": request["model_artifact_sha256"],
        "response": text,
    }
    sys.stdout.write(json.dumps(output, separators=(",", ":"), sort_keys=True))


if __name__ == "__main__":
    main()
