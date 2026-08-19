#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import ssl
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from typing import Any
from urllib.parse import parse_qs, urlparse


def load_json(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise SystemExit(f"{path} must contain a JSON object")
    return value


def agent_rows(roster: dict[str, Any], source_revision: str) -> list[dict[str, Any]]:
    now = int(time.time() * 1000)
    rows: list[dict[str, Any]] = []
    for agent in roster.get("agents", []):
        if not isinstance(agent, dict):
            continue
        rows.append(
            {
                "id": agent["agent_id"],
                "label": agent["agent_id"].replace("issue341-", "").replace("-", " ").title(),
                "role": agent.get("role", "runtime agent"),
                "state": "ready" if agent.get("status") == "running" else "degraded",
                "detail": "Issue #341 private ACIP TCP roster projection",
                "health": "healthy" if agent.get("acip_direct_tcp") is True else "degraded",
                "availability": "available" if agent.get("status") == "running" else "unavailable",
                "activity": "provider-neutral birthday proof",
                "capabilities": ["direct_tcp_acip", "provider_neutral_proof"],
                "location": agent.get("observed_listening_address"),
                "communication_eligible": agent.get("acip_direct_tcp") is True,
                "observed_at_unix_millis": now,
                "freshness_deadline_unix_millis": now + 10_000,
                "source_revision": source_revision,
                "provenance": "issue341_private_observatory_probe",
                "ssm_access": agent.get("ssm_access"),
            }
        )
    return rows


def observatory_feed(matrix: dict[str, Any], source_revision: str, port: int) -> dict[str, Any]:
    roster = matrix.get("observatory")
    if not isinstance(roster, dict):
        raise SystemExit("matrix does not contain an observatory roster")
    rows = agent_rows(roster, source_revision)
    now = int(time.time() * 1000)
    return {
        "schema": "adl.runtime_v3.observatory_feed.v2",
        "runtime_instance_id": "issue341-provider-neutral-private-runtime",
        "runtime_incarnation_id": f"issue341-{source_revision[:12]}",
        "runtime_process_id": 0,
        "default_runtime_changed": False,
        "runtime_selection": "runtime_v3_explicit_opt_in",
        "control": {
            "port": port,
            "public_base_url": f"https://localhost:{port}",
            "read_endpoint": "/v1/observatory",
            "websocket_endpoint": "/v1/observatory/ws",
            "websocket_full_duplex": False,
            "websocket_acip_binary_schema": "adl.csm.acip_carrier.websocket_frame.v1",
            "signed_command_endpoint": "/v1/control",
            "signed_commands_required_for_mutation": True,
            "bearer_token_required_for_read": False,
            "login_required_for_mutation": True,
            "browser_mutation_authority": False,
        },
        "health": {
            "snapshot": {
                "schema": "adl.runtime.control_snapshot.v1",
                "revision": now,
                "topology_generation": 341,
                "components": {
                    "agent_runtime": "running",
                    "acip": "running",
                    "observability": "running",
                    "provider": "running",
                    "shepherd": "running",
                },
                "restart_counts": {},
                "queues": {},
                "clock": {"status": "authoritative", "source": "local_monotonic", "unix_millis": now},
                "lifecycle": "running",
                "event_count": len(matrix.get("negative_cases", [])) + len(rows),
                "observability": {"status": "ready"},
                "observability_ready": True,
                "agent_admissions": {
                    row["id"]: {
                        "observed_at_unix_millis": row["observed_at_unix_millis"],
                        "freshness_deadline_unix_millis": row["freshness_deadline_unix_millis"],
                        "source_revision": source_revision,
                    }
                    for row in rows
                },
            },
            "observability_ready": True,
        },
        "weather": {"schema": "adl.runtime.weather_health.v1", "resource_state": "healthy", "shutdown_decision": "continue"},
        "weather_freshness": {"observed_at_unix_millis": now, "age_millis": 0, "stale_after_millis": 10_000, "stale": False},
        "continuity": {"checkpoint": {"generation": 341, "accepted_through": now}},
        "ingress": {"accepted_through": 0, "completed": {}},
        "agents": {
            "schema": "adl.runtime_v3.agent_roster_page.v1",
            "revision": now,
            "scope": "local_runtime",
            "total_count": len(rows),
            "rendered_sample_count": len(rows),
            "has_more": False,
            "next_page_token": None,
            "event_cursor": f"issue341.{now}",
            "population_complete": True,
            "sample": rows,
        },
        "proof": {
            "issue": 341,
            "default_runtime_switch_authorized": False,
            "runtime_v2_decommission_authorized": False,
            "sidecar_required": False,
            "provider_neutral_private_observatory": True,
        },
        "events": [
            {
                "sequence": index,
                "monotonic_millis": index,
                "component": row["id"],
                "event": "issue341_agent_observed",
                "correlation_id": "issue341-private-observatory",
            }
            for index, row in enumerate(rows)
        ],
    }


class Handler(BaseHTTPRequestHandler):
    matrix: dict[str, Any]
    source_revision: str
    port: int

    def _origin(self) -> str:
        origin = self.headers.get("origin")
        if origin in {"https://localhost:8766", "https://localhost:8765"}:
            return origin
        return "https://localhost:8766"

    def _send(self, status: int, payload: dict[str, Any]) -> None:
        body = json.dumps(payload, sort_keys=True).encode("utf-8")
        self.send_response(status)
        self.send_header("access-control-allow-origin", self._origin())
        self.send_header("content-type", "application/json")
        self.send_header("content-length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_OPTIONS(self) -> None:
        self.send_response(204)
        self.send_header("access-control-allow-origin", self._origin())
        self.send_header("access-control-allow-methods", "GET, OPTIONS")
        self.send_header("access-control-allow-headers", "content-type, authorization")
        self.end_headers()

    def do_GET(self) -> None:
        parsed = urlparse(self.path)
        feed = observatory_feed(self.matrix, self.source_revision, self.port)
        if parsed.path == "/v1/observatory":
            self._send(200, feed)
        elif parsed.path == "/v1/ready":
            self._send(200, {"schema": "adl.runtime.ready.v1", "ready": True, "status": "ready", "degraded_reasons": []})
        elif parsed.path == "/v1/health":
            self._send(200, {"schema": "adl.runtime.health.v1", "status": "healthy"})
        elif parsed.path == "/v1/agents":
            page_size = int(parse_qs(parsed.query).get("page_size", ["50"])[0])
            page = dict(feed["agents"])
            page["sample"] = page["sample"][:page_size]
            page["rendered_sample_count"] = len(page["sample"])
            self._send(200, page)
        else:
            self._send(404, {"error": "not_found", "path": parsed.path})

    def log_message(self, _format: str, *_args: Any) -> None:
        return


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--matrix", type=Path, default=Path("demos/v0.92/provider-neutral-birthday/proof-matrix-observatory.json"))
    parser.add_argument("--source-revision", required=True)
    parser.add_argument("--host", default="127.0.0.1")
    parser.add_argument("--port", type=int, default=20998)
    parser.add_argument("--cert", type=Path)
    parser.add_argument("--key", type=Path)
    parser.add_argument("--emit-feed", action="store_true")
    args = parser.parse_args()
    Handler.matrix = load_json(args.matrix)
    Handler.source_revision = args.source_revision
    Handler.port = args.port
    if args.emit_feed:
        print(json.dumps(observatory_feed(Handler.matrix, Handler.source_revision, args.port), indent=2, sort_keys=True))
        return 0
    if not args.cert or not args.key:
        raise SystemExit("--cert and --key are required unless --emit-feed is used")
    server = ThreadingHTTPServer((args.host, args.port), Handler)
    context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
    context.load_cert_chain(args.cert, args.key)
    server.socket = context.wrap_socket(server.socket, server_side=True)
    print(f"https://localhost:{args.port}", flush=True)
    server.serve_forever()
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
