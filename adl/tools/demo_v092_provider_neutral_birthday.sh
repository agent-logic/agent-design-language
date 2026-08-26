#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DEMO_DIR="${ADL_ISSUE341_DEMO_DIR:-$ROOT_DIR/demos/v0.92/provider-neutral-birthday}"
EVIDENCE_DIR="${ADL_ISSUE341_EVIDENCE_DIR:-$ROOT_DIR/.csdlc/evidence/341}"
MODE="all"
HOSTED_KEYS_FILE="${ADL_HOSTED_PROVIDER_KEYS_FILE:-$ROOT_DIR/adl/tools/benchmark/hosted_provider_key_files.json}"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --mode) MODE="${2:?missing --mode value}"; shift 2 ;;
    -h|--help)
      echo "Usage: $0 [--mode all|local-proof|positive|negative|observatory]"
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done

mkdir -p "$DEMO_DIR" "$EVIDENCE_DIR"

resolve_key_file() {
  local env_name="$1"
  local explicit_path="${2:-}"
  if [[ -n "$explicit_path" ]]; then
    printf '%s\n' "$explicit_path"
    return 0
  fi
  python3 - "$HOSTED_KEYS_FILE" "$env_name" <<'PY' || true
import json
import os
import sys
from pathlib import Path

path = Path(sys.argv[1])
env_name = sys.argv[2]
if not path.is_file():
    raise SystemExit(1)
doc = json.loads(path.read_text(encoding="utf-8"))
entry = (doc.get("keys") or {}).get(env_name, "")
if isinstance(entry, dict):
    file_env = entry.get("file_env_var", "")
    entry = os.environ.get(file_env, "") if file_env else ""
print(entry, end="")
PY
}

load_optional_key() {
  local env_name="$1"
  local key_file="$2"
  if [[ -n "${!env_name:-}" || -z "$key_file" || ! -s "$key_file" ]]; then
    return 0
  fi
  local key_value
  key_value="$(python3 - "$env_name" "$key_file" <<'PY'
import sys

env_name, path = sys.argv[1:3]
raw = open(path, encoding="utf-8").read().strip()
value = ""
for line in raw.splitlines():
    stripped = line.strip()
    if not stripped or stripped.startswith("#"):
        continue
    if stripped.startswith(env_name + "="):
        value = stripped.split("=", 1)[1].strip().strip("'\"")
        break
    value = stripped.strip("'\"")
    break
print(value, end="")
PY
)"
  if [[ -n "$key_value" ]]; then
    export "$env_name=$key_value"
  fi
}

load_optional_key OPENAI_API_KEY "$(resolve_key_file OPENAI_API_KEY "${ADL_OPENAI_API_KEY_FILE:-${ADL_OPENAI_KEY_FILE:-}}")"
load_optional_key GEMINI_API_KEY "$(resolve_key_file GEMINI_API_KEY "${ADL_GEMINI_API_KEY_FILE:-${ADL_GEMINI_KEY_FILE:-}}")"
load_optional_key ANTHROPIC_API_KEY "$(resolve_key_file ANTHROPIC_API_KEY "${ADL_ANTHROPIC_API_KEY_FILE:-${ADL_ANTHROPIC_KEY_FILE:-}}")"

python3 - "$ROOT_DIR" "$MODE" "$DEMO_DIR" "$EVIDENCE_DIR" <<'PY'
from __future__ import annotations

import hashlib
import json
import os
import pathlib
import socket
import socketserver
import sys
import threading
import time
import urllib.error
import urllib.parse
import urllib.request
from typing import Any

root = pathlib.Path(sys.argv[1])
mode = sys.argv[2]
demo_dir = pathlib.Path(sys.argv[3])
evidence_dir = pathlib.Path(sys.argv[4])

prompt = (
    "ADL v0.92 WP-18B provider-neutral birthday proof. Do not include secrets. "
    "Return exactly four short lines with these labels and words: "
    "identity: governed identity boundary; continuity: governed continuity boundary; "
    "witness: bounded witness boundary; startup: Startup is not a birthday."
)
scenario = {
    "id": "v0.92.wp18b.provider-neutral-birthday.v1",
    "prompt_sha256": hashlib.sha256(prompt.encode()).hexdigest(),
    "acip_operations": [
        {"op": "load_birthday_packet", "version": "v0.92"},
        {"op": "evaluate_identity_continuity", "version": "v0.92"},
        {"op": "emit_bounded_witness_summary", "version": "v0.92"},
    ],
}


def digest(value: Any) -> str:
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def write(path: pathlib.Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


class Trace:
    def __init__(self) -> None:
        self.lock = threading.Lock()
        self.events: list[dict[str, Any]] = []

    def add(self, **event: Any) -> None:
        with self.lock:
            event.setdefault("sequence", len(self.events) + 1)
            event.setdefault("observed_at", f"monotonic:{time.monotonic_ns()}")
            self.events.append(event)


trace = Trace()


def post_json(url: str, headers: dict[str, str], payload: dict[str, Any]) -> tuple[int, dict[str, str], dict[str, Any]]:
    request = urllib.request.Request(url, data=json.dumps(payload).encode(), headers=headers, method="POST")
    try:
        with urllib.request.urlopen(request, timeout=240) as response:
            return response.status, dict(response.headers.items()), json.loads(response.read().decode())
    except urllib.error.HTTPError as exc:
        body = exc.read().decode(errors="replace")
        try:
            parsed = json.loads(body)
        except json.JSONDecodeError:
            parsed = {"error": body[:240]}
        return exc.code, dict(exc.headers.items()), parsed


def extract_openai(payload: dict[str, Any]) -> str:
    if isinstance(payload.get("output_text"), str):
        return payload["output_text"]
    chunks: list[str] = []
    for item in payload.get("output", []):
        for content in item.get("content", []) if isinstance(item, dict) else []:
            if isinstance(content, dict) and isinstance(content.get("text"), str):
                chunks.append(content["text"])
    return "\n".join(chunks)


def extract_gemini(payload: dict[str, Any]) -> str:
    chunks: list[str] = []
    for candidate in payload.get("candidates", []):
        content = candidate.get("content", {}) if isinstance(candidate, dict) else {}
        for part in content.get("parts", []) if isinstance(content, dict) else []:
            if isinstance(part, dict) and isinstance(part.get("text"), str):
                chunks.append(part["text"])
    return "\n".join(chunks)


def extract_anthropic(payload: dict[str, Any]) -> str:
    chunks: list[str] = []
    for item in payload.get("content", []):
        if isinstance(item, dict) and item.get("type") == "text" and isinstance(item.get("text"), str):
            chunks.append(item["text"])
    return "\n".join(chunks)


def live_provider_text(provider: str, model: str) -> tuple[str, bool]:
    if provider == "openai":
        status, headers, payload = post_json(
            "https://api.openai.com/v1/responses",
            {"Authorization": f"Bearer {os.environ['OPENAI_API_KEY']}", "Content-Type": "application/json"},
            {"model": model, "input": prompt, "max_output_tokens": 800},
        )
        text = extract_openai(payload)
        if status != 200 or not text.strip():
            raise RuntimeError(f"OpenAI provider failed with status {status}")
        return text, bool(headers.get("x-request-id") or headers.get("request-id"))
    if provider == "gemini":
        endpoint = "https://generativelanguage.googleapis.com/v1beta/models/" + urllib.parse.quote(model, safe="") + ":generateContent"
        status, headers, payload = post_json(
            endpoint,
            {"x-goog-api-key": os.environ["GEMINI_API_KEY"], "Content-Type": "application/json"},
            {"contents": [{"role": "user", "parts": [{"text": prompt}]}], "generationConfig": {"maxOutputTokens": 300}},
        )
        text = extract_gemini(payload)
        if status != 200 or not text.strip():
            raise RuntimeError(f"Gemini provider failed with status {status}")
        return text, bool(headers.get("x-goog-request-id") or headers.get("x-request-id"))
    if provider == "anthropic":
        status, headers, payload = post_json(
            "https://api.anthropic.com/v1/messages",
            {
                "x-api-key": os.environ["ANTHROPIC_API_KEY"],
                "anthropic-version": "2023-06-01",
                "Content-Type": "application/json",
            },
            {"model": model, "messages": [{"role": "user", "content": prompt}], "max_tokens": 300},
        )
        text = extract_anthropic(payload)
        if status != 200 or not text.strip():
            raise RuntimeError(f"Anthropic provider failed with status {status}")
        return text, bool(headers.get("request-id") or headers.get("x-request-id"))
    raise RuntimeError(f"unsupported provider {provider}")


def semantic_assertions(text: str) -> dict[str, bool]:
    folded = text.lower()
    return {
        "names_identity_boundary": "identity" in folded,
        "names_continuity_boundary": "continuity" in folded,
        "retains_witness_boundary": "witness" in folded,
        "rejects_startup_as_birthday": "startup is not a birthday" in folded or "not-a-birthday" in folded or "not a birthday" in folded,
    }


class AgentServer(socketserver.ThreadingTCPServer):
    allow_reuse_address = True

    def __init__(self, address: tuple[str, int], handler: type[socketserver.BaseRequestHandler], agent: "ProviderAgent"):
        super().__init__(address, handler)
        self.agent = agent


class AcipHandler(socketserver.StreamRequestHandler):
    def handle(self) -> None:
        raw = self.rfile.readline(1_000_000)
        agent: ProviderAgent = self.server.agent  # type: ignore[attr-defined]
        try:
            envelope = json.loads(raw.decode("utf-8"))
        except json.JSONDecodeError:
            self._reply(agent.reject("malformed_acip", "invalid json", raw_sha256=hashlib.sha256(raw).hexdigest()))
            return
        trace.add(
            event="acip_request_received",
            agent_id=agent.agent_id,
            provider=agent.provider,
            local_address=f"127.0.0.1:{agent.port}",
            envelope_sha256=digest(envelope),
            operation_count=len(envelope.get("operations", [])) if isinstance(envelope, dict) else 0,
        )
        self._reply(agent.handle_envelope(envelope))

    def _reply(self, receipt: dict[str, Any]) -> None:
        self.wfile.write((json.dumps(receipt, sort_keys=True) + "\n").encode("utf-8"))


class ProviderAgent:
    def __init__(self, agent_id: str, provider: str, model: str, execution_mode: str) -> None:
        self.agent_id = agent_id
        self.provider = provider
        self.model = model
        self.execution_mode = execution_mode
        self.server = AgentServer(("127.0.0.1", 0), AcipHandler, self)
        self.port = int(self.server.server_address[1])
        self.thread = threading.Thread(target=self.server.serve_forever, name=agent_id, daemon=True)
        self.thread.start()
        trace.add(event="agent_listening", agent_id=agent_id, provider=provider, local_address=f"127.0.0.1:{self.port}")

    def close(self) -> None:
        self.server.shutdown()
        self.server.server_close()
        self.thread.join(timeout=5)
        trace.add(event="agent_stopped", agent_id=self.agent_id, provider=self.provider)

    def reject(self, case: str, reason: str, **extra: Any) -> dict[str, Any]:
        receipt = {
            "schema": "adl.issue341.acip_receipt.v1",
            "agent_id": self.agent_id,
            "provider": self.provider,
            "model": self.model,
            "execution_mode": self.execution_mode,
            "outcome": "non_pass",
            "case": case,
            "reason": reason,
            "credential_material_recorded": False,
            "raw_prompt_recorded": False,
            "raw_output_recorded": False,
            **extra,
        }
        trace.add(event="acip_rejected", agent_id=self.agent_id, provider=self.provider, case=case, receipt_sha256=digest(receipt))
        return receipt

    def handle_envelope(self, envelope: dict[str, Any]) -> dict[str, Any]:
        if envelope.get("schema") != "adl.acip.envelope.v1":
            return self.reject("malformed_acip", "missing versioned ACIP envelope")
        if envelope.get("authority") != "authorized":
            return self.reject("denied_authority", "request lacks authorized execution context")
        if envelope.get("interrupt_after_accept") is True:
            return self.reject("interrupted_provider", "partial response cannot be completed proof")
        if envelope.get("provider_available") is False:
            return self.reject("provider_unavailable", "unavailable provider marks only its column failed")
        if envelope.get("required_state_generation") != "current":
            return self.reject("provider_loss", "lost state cannot be reconstructed from cache")
        if envelope.get("cached_substitution") is True:
            return self.reject("substitution_attempt", "fixture/cached output cannot satisfy live positive proof")
        if envelope.get("scenario_id") != scenario["id"] or envelope.get("prompt_sha256") != scenario["prompt_sha256"]:
            return self.reject("malformed_acip", "scenario binding mismatch")
        if envelope.get("operations") != scenario["acip_operations"]:
            return self.reject("malformed_acip", "operation sequence mismatch")

        if self.execution_mode == "live_provider":
            text, request_id_present = live_provider_text(self.provider, self.model)
        else:
            text, request_id_present = "identity continuity witness startup is not a birthday", False
        assertions = semantic_assertions(text)
        summary = {"chars": len(text), **assertions}
        receipt = {
            "schema": "adl.issue341.acip_receipt.v1",
            "agent_id": self.agent_id,
            "provider": self.provider,
            "model": self.model,
            "execution_mode": self.execution_mode,
            "outcome": "pass",
            "credential_material_recorded": False,
            "raw_prompt_recorded": False,
            "raw_output_recorded": False,
            "request_id_present": request_id_present,
            "scenario_id": scenario["id"],
            "acip_operations": scenario["acip_operations"],
            "capabilities": ["bounded_summary", "acip_compatible_completion", "direct_tcp_agent"],
            "output_sha256": digest(summary),
            "semantic_assertions": assertions,
        }
        receipt_sha = digest(receipt)
        trace.add(event="acip_response_sent", agent_id=self.agent_id, provider=self.provider, receipt_sha256=receipt_sha)
        receipt["receipt_sha256"] = receipt_sha
        return receipt


def envelope(**overrides: Any) -> dict[str, Any]:
    value = {
        "schema": "adl.acip.envelope.v1",
        "authority": "authorized",
        "scenario_id": scenario["id"],
        "prompt_sha256": scenario["prompt_sha256"],
        "operations": scenario["acip_operations"],
        "required_state_generation": "current",
        "provider_available": True,
        "raw_prompt_recorded": False,
    }
    value.update(overrides)
    return value


def call_agent(agent: ProviderAgent, packet: dict[str, Any]) -> dict[str, Any]:
    trace.add(event="acip_request_sent", agent_id=agent.agent_id, provider=agent.provider, local_address=f"127.0.0.1:{agent.port}", envelope_sha256=digest(packet))
    client_timeout = float(os.environ.get("ADL_ISSUE341_ACIP_CLIENT_TIMEOUT_SECONDS", "180"))
    with socket.create_connection(("127.0.0.1", agent.port), timeout=client_timeout) as sock:
        sock.sendall((json.dumps(packet, sort_keys=True) + "\n").encode("utf-8"))
        sock.settimeout(client_timeout)
        with sock.makefile("r", encoding="utf-8") as reader:
            return json.loads(reader.readline())


def build_agents() -> list[ProviderAgent]:
    if mode == "positive":
        candidates: list[tuple[str, str, str]] = []
        if os.environ.get("OPENAI_API_KEY"):
            candidates.append(("issue341-openai-agent", "openai", os.environ.get("ADL_ISSUE341_OPENAI_MODEL", "gpt-4.1-mini")))
        if os.environ.get("GEMINI_API_KEY"):
            candidates.append(("issue341-gemini-agent", "gemini", os.environ.get("ADL_ISSUE341_GEMINI_MODEL", "gemini-2.5-flash")))
        if os.environ.get("ANTHROPIC_API_KEY"):
            candidates.append(("issue341-anthropic-agent", "anthropic", os.environ.get("ADL_ISSUE341_ANTHROPIC_MODEL", "claude-opus-5")))
        if len(candidates) < 2:
            raise SystemExit("positive mode requires at least two live provider credentials")
        return [ProviderAgent(*item, execution_mode="live_provider") for item in candidates[:2]]
    return [
        ProviderAgent("issue341-openai-reference-agent", "openai-reference", "reference-gpt", "local_reference"),
        ProviderAgent("issue341-gemini-reference-agent", "gemini-reference", "reference-gemini", "local_reference"),
        ProviderAgent("wuji-shepherd", "wuji-shepherd-reference", "reference-shepherd", "local_reference"),
    ]


def run_positive(agents: list[ProviderAgent]) -> list[dict[str, Any]]:
    columns: list[dict[str, Any]] = []
    for agent in agents:
        receipt = call_agent(agent, envelope())
        if receipt.get("outcome") != "pass":
            raise SystemExit(f"{agent.provider} positive ACIP path failed: {receipt.get('reason')}")
        columns.append({**receipt, "positive": True})
    return columns


def run_negatives(agent: ProviderAgent) -> list[dict[str, Any]]:
    cases = [
        ("malformed_acip", {"schema": "wrong.schema"}),
        ("denied_authority", envelope(authority="denied")),
        ("interrupted_provider", envelope(interrupt_after_accept=True)),
        ("provider_unavailable", envelope(provider_available=False)),
        ("provider_loss", envelope(required_state_generation="stale")),
        ("substitution_attempt", envelope(cached_substitution=True)),
    ]
    receipts: list[dict[str, Any]] = []
    for expected, packet in cases:
        receipt = call_agent(agent, packet)
        if receipt.get("outcome") == "pass" or receipt.get("case") != expected:
            raise SystemExit(f"negative case {expected} did not fail visibly")
        receipts.append({"case": expected, "outcome": "non_pass", "visible": True, "reason": receipt["reason"], "receipt_sha256": digest(receipt)})
    return receipts


def observe_roster(agents: list[ProviderAgent]) -> dict[str, Any]:
    rows: list[dict[str, Any]] = []
    for agent in agents:
        reachable = False
        try:
            with socket.create_connection(("127.0.0.1", agent.port), timeout=2) as sock:
                sock.sendall((json.dumps(envelope(interrupt_after_accept=True), sort_keys=True) + "\n").encode("utf-8"))
                reachable = bool(sock.recv(4096))
        except OSError:
            reachable = False
        rows.append({
            "agent_id": agent.agent_id,
            "role": "shepherd" if agent.agent_id == "wuji-shepherd" else "provider-column",
            "status": "running" if agent.thread.is_alive() else "stopped",
            "observed_listening_address": f"127.0.0.1:{agent.port}",
            "acip_direct_tcp": reachable,
            "ssm_access": "maintenance_only" if agent.agent_id == "wuji-shepherd" else "none",
        })
    return {
        "schema": "adl.issue341.private_observatory_roster.v1",
        "visibility": "private",
        "public_exposure": "not_claimed",
        "observation_method": "live_localhost_tcp_listener_probe",
        "agents": rows,
        "summary": {
            "running_agents": sum(1 for item in rows if item["status"] == "running"),
            "all_voters_direct_tcp": all(item["acip_direct_tcp"] for item in rows),
            "ordinary_agent_ssm_access": any(item["ssm_access"] != "none" for item in rows if item["role"] != "shepherd"),
        },
    }


agents = build_agents()
try:
    columns = run_positive(agents)
    negatives = run_negatives(agents[0])
    roster = observe_roster(agents) if mode in {"all", "local-proof", "observatory"} else None
finally:
    for candidate in agents:
        candidate.close()

trace_packet = {
    "schema": "adl.issue341.acip_trace.v1",
    "scenario_id": scenario["id"],
    "transport": "localhost_tcp",
    "raw_prompt_recorded": False,
    "raw_output_recorded": False,
    "credential_material_recorded": False,
    "events": trace.events,
}
trace_name = f"acip-trace-{mode}.json"
write(demo_dir / trace_name, trace_packet)
write(evidence_dir / trace_name, trace_packet)

matrix = {
    "schema": "adl.issue341.provider_neutral_birthday_matrix.v1",
    "issue": 341,
    "mode": mode,
    "scenario": scenario,
    "provider_columns": [{**column, "trace_ref": trace_name} for column in columns],
    "negative_cases": negatives,
    "observatory": roster,
    "claims": {
        "real_provider_positive_claimed": mode == "positive",
        "local_reference_only": mode in {"all", "local-proof", "observatory"},
        "private_observatory_only": True,
        "public_exposure_claimed": False,
        "credential_material_recorded": False,
        "raw_payloads_recorded": False,
        "acip_tcp_trace_observed": True,
    },
}
write(demo_dir / "proof-matrix.json", matrix)
write(demo_dir / f"proof-matrix-{mode}.json", matrix)
write(evidence_dir / "proof-matrix.json", matrix)
write(evidence_dir / f"proof-matrix-{mode}.json", matrix)
if roster:
    write(evidence_dir / "private-observatory-roster.json", roster)
try:
    print((demo_dir / "proof-matrix.json").relative_to(root))
except ValueError:
    print(demo_dir / "proof-matrix.json")
PY
