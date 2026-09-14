#!/usr/bin/env python3
"""Qualify #901 failure and recovery through one registered Runtime session."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import plistlib
import secrets
import shutil
import signal
import ssl
import subprocess
import sys
import threading
import time
from types import SimpleNamespace
from typing import Any

sys.path.insert(0, str(Path(__file__).resolve().parent))
import run_issue901_provider_recovery_qualification as adapter_proof  # noqa: E402

lifecycle: Any = None


SCHEMA = "adl.issue901.runtime_provider_recovery.v1"
REPO = Path(__file__).resolve().parents[2]


def require(condition: bool, message: str) -> None:
    if not condition:
        raise RuntimeError(message)


def digest(value: Any) -> str:
    return hashlib.sha256(json.dumps(value, separators=(",", ":"), sort_keys=True).encode()).hexdigest()


def terminal_conversation(api_port: int, context: ssl.SSLContext, token: str, agent_id: str, marker: str):
    websocket = lifecycle.WebSocket(api_port, context, "https://localhost:8765")
    websocket.send({"schema": "adl.runtime_v3.observatory_ws_auth.v1", "bearer_token": token})
    websocket.until(lambda item: item.get("status") == "authenticated")
    turn_id = secrets.token_hex(8)
    correlation_id = secrets.token_hex(16)
    websocket.send({
        "schema": "adl.runtime_v3.observatory_conversation_intent.v1",
        "conversation_id": f"issue901-runtime-{turn_id}",
        "turn_id": turn_id,
        "recipient_id": agent_id,
        "correlation_id": correlation_id,
        "message": f"{marker}: reply with one short confirmation.",
        "message_parts": [],
    })
    websocket.until(
        lambda item: item.get("schema") == "adl.runtime_v3.observatory_conversation_result.v1"
        and item.get("turn_id") == turn_id
        and item.get("status") in {"accepted", "pending"}
    )
    return websocket, turn_id, correlation_id


def await_terminal(websocket: lifecycle.WebSocket, turn_id: str) -> dict[str, Any]:
    return websocket.until(
        lambda item: item.get("schema") == "adl.runtime_v3.observatory_conversation_result.v1"
        and item.get("turn_id") == turn_id
        and item.get("status") not in {"accepted", "pending"},
        timeout=45,
    )


def public_terminal(result: dict[str, Any]) -> dict[str, Any]:
    return {
        "status": result.get("status"),
        "failure_reason": result.get("failure_reason"),
        "reply_present": bool(result.get("reply")),
        "reply_sha256": hashlib.sha256(str(result.get("reply", "")).encode()).hexdigest()
        if result.get("reply")
        else None,
    }


def validate_report(report: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if report.get("schema") != SCHEMA:
        errors.append("schema_invalid")
    identity = report.get("runtime_identity") or {}
    if not identity.get("runtime_incarnation_id") or not isinstance(identity.get("runtime_process_id"), int):
        errors.append("runtime_identity_missing")
    if report.get("runtime_identity_after") != identity:
        errors.append("runtime_session_changed")
    provider = report.get("registered_provider") or {}
    if provider.get("provider") != "openai-compatible" or provider.get("model") != report.get("model"):
        errors.append("registered_provider_projection_invalid")
    scenarios = report.get("scenarios") or {}
    if set(scenarios) != {"loss", "interruption", "recovery"}:
        return errors + ["scenario_population_invalid"]
    if scenarios["loss"].get("terminal", {}).get("status") != "failed":
        errors.append("runtime_loss_not_failed")
    if scenarios["interruption"].get("session_interrupted") is not True:
        errors.append("runtime_session_interruption_missing")
    recovery = scenarios["recovery"].get("terminal", {})
    if recovery.get("status") != "delivered" or recovery.get("reply_present") is not True:
        errors.append("runtime_recovery_not_delivered")
    correlations = [row.get("correlation_id_sha256") for row in scenarios.values()]
    if len(set(correlations)) != 3 or any(not isinstance(value, str) or len(value) != 64 for value in correlations):
        errors.append("runtime_correlation_identity_invalid")
    pids = report.get("provider_process_ids") or []
    if len(pids) < 2 or len(set(pids)) < 2:
        errors.append("fresh_provider_incarnation_missing")
    if report.get("checkpoint", {}).get("durable") is not True or report.get("agent_removed") is not True:
        errors.append("runtime_lifecycle_incomplete")
    if report.get("paid_calls") != 0:
        errors.append("paid_call_boundary_invalid")
    if report.get("timeout_evidence_scope") != "standalone_adapter_retained":
        errors.append("timeout_evidence_scope_invalid")
    return errors


def run(args: argparse.Namespace) -> dict[str, Any]:
    global lifecycle
    import issue855_provider_lifecycle as lifecycle_module

    lifecycle = lifecycle_module
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    output.chmod(0o700)
    tls = lifecycle.certificates(output / "state/tls")
    fixture = lifecycle.Fixture(tls)
    clock = lifecycle.LocalTime()
    upstream_port = adapter_proof.reserve_port()
    proxy_port = adapter_proof.reserve_port()
    proxy_state = adapter_proof.ProxyState(upstream_port)
    proxy = adapter_proof.ThreadingHTTPServer(("127.0.0.1", proxy_port), adapter_proof.proxy_handler(proxy_state))
    proxy_thread = threading.Thread(target=proxy.serve_forever, daemon=True)
    proxy_thread.start()
    install = output / "runtime-v3"
    lifecycle.run([
        REPO / "adl/tools/install_runtime_v3_generation.sh", "install", "--root", install,
        "--generation", "issue901-runtime", "--csm", args.csm, "--guardian", args.guardian,
        "--kernel", args.kernel, "--source-revision", args.source_revision, "--build-profile", "debug",
    ])
    lifecycle.run([REPO / "adl/tools/install_runtime_v3_generation.sh", "verify", "--root", install])
    ctl = output / "csmctl"
    shutil.copy2(args.csmctl, ctl)
    init_args = SimpleNamespace(hosted_mode=False, hosted_approved=False, vector=args.vector, source_revision=args.source_revision)
    init, api_port, tokens = lifecycle.prepare_init(output, tls, init_args, clock, fixture)
    definitions_path = output / "providers.yaml"
    definitions = json.loads(definitions_path.read_text())
    selected = definitions["providers"]["openai-compatible"]
    selected["default_model"] = args.model
    selected["config"].update({
        "endpoint": f"http://127.0.0.1:{proxy_port}/v1/chat/completions",
        "timeout_secs": 2,
        "max_output_tokens": 16,
        "runtime_max_output_tokens": 16,
    })
    definitions_path.write_text(json.dumps(definitions, indent=2) + "\n")
    env = dict(os.environ)
    for key in list(env):
        if any(part in key for part in ("API_KEY", "ACCESS_TOKEN", "GOOGLE_APPLICATION_CREDENTIALS")):
            del env[key]
    env.update(ADL_PROVIDER_CA_FILE=str(tls["ca"]), ADL_PROVIDER_FIXTURE_TOKEN="issue901-local-fixture")
    service_label = f"ai.agent-logic.issue901-runtime-{os.getpid()}"
    plist = output / "fixture.plist"
    plist.write_bytes(plistlib.dumps({
        "Label": service_label,
        "ProgramArguments": [str(install / "current/bin/adl-runtime-guardian"), "--init", str(init)],
    }))
    status = subprocess.run(
        [str(output / "bin/csm"), "runtime-v3", "status", "--init", str(init),
         "--plist", str(plist), "--label", service_label, "--json"],
        env=env, capture_output=True, text=True, timeout=30,
    )
    require(status.stdout.strip(), "CSM Runtime configuration preflight produced no result")
    config_status = json.loads(status.stdout)
    require(config_status.get("config_valid") and not config_status.get("service_loaded"),
            "isolated CSM Runtime configuration preflight failed")
    (output / "csm-config-status.json").write_text(json.dumps(config_status, indent=2) + "\n")
    guardian_log = (output / "guardian.log").open("w")
    guardian = subprocess.Popen(
        [str(install / "current/bin/adl-runtime-guardian"), "--init", str(init)],
        stdout=guardian_log, stderr=subprocess.STDOUT, env=env, start_new_session=True,
    )
    provider: adapter_proof.OwnedProcess | None = None
    report: dict[str, Any] = {
        "schema": SCHEMA,
        "source_revision": args.source_revision,
        "model": args.model,
        "paid_calls": 0,
        "scenarios": {},
        "provider_process_ids": [],
    }
    context = ssl.create_default_context(cafile=str(tls["ca"]))
    try:
        deadline = time.monotonic() + 45
        snapshot = None
        readiness_error = None
        while time.monotonic() < deadline:
            require(guardian.poll() is None, "Guardian exited before Runtime readiness")
            try:
                snapshot = lifecycle.api(context, api_port, tokens["observatory"], "/v1/observatory?schema=v3")
                if snapshot.get("runtime_incarnation_id"):
                    break
            except (OSError, ValueError) as error:
                readiness_error = f"{type(error).__name__}: {error}"
            time.sleep(0.1)
        require(
            snapshot is not None and snapshot.get("runtime_incarnation_id"),
            f"Runtime readiness deadline; last API error: {readiness_error}; last snapshot: {snapshot}",
        )
        runtime_identity = lifecycle.identity(snapshot)
        report["runtime_identity"] = runtime_identity

        def csmctl(*argv: object) -> dict[str, Any]:
            return json.loads(lifecycle.run([ctl, "agent", *argv], env=env))

        provider = adapter_proof.start_provider(args.provider_binary, args.model_file, upstream_port, output / "provider-loss.log")
        report["provider_process_ids"].append(provider.pid)
        agent_id = "issue901-runtime-agent"
        config = {
            "schema": "adl.csm.agent_config.v1",
            "runtime": {"init": str(init)},
            "identity": {"id": agent_id, "name": "recovery.fixture", "display_name": "Recovery Fixture"},
            "office": "assistant",
            "provider": {"kind": "openai-compatible", "model": args.model, "required_capabilities": ["conversation"]},
        }
        config_path = lifecycle.write(output / "agent.json", config)
        require(csmctl("add", "--config", config_path).get("status") == "admitted", "Runtime admission failed")
        deadline = time.monotonic() + 20
        detail = None
        while time.monotonic() < deadline:
            detail = csmctl("get", "--init", init, "--id", agent_id)
            if detail.get("communication_eligible"):
                break
            time.sleep(0.1)
        require(detail and detail.get("communication_eligible"), "registered Runtime agent not ready")
        report["registered_provider"] = {
            "provider": detail.get("provider"), "model": detail.get("model"),
            "adapter": (detail.get("provider_binding") or {}).get("adapter"),
        }

        before = proxy_state.generate_count
        socket, turn, correlation = terminal_conversation(api_port, context, tokens["observatory"], agent_id, "issue901-runtime-loss")
        proxy_state.wait_for_generate(before)
        stopped = provider.stop()
        provider = None
        loss = await_terminal(socket, turn)
        socket.sock.close()
        report["scenarios"]["loss"] = {
            "terminal": public_terminal(loss), "correlation_id_sha256": hashlib.sha256(correlation.encode()).hexdigest(),
            "provider_signal_sent_at": stopped["signal_sent_at"], "provider_returncode": stopped["returncode"],
        }

        provider = adapter_proof.start_provider(args.provider_binary, args.model_file, upstream_port, output / "provider-recovered.log")
        report["provider_process_ids"].append(provider.pid)
        before = proxy_state.generate_count
        socket, turn, correlation = terminal_conversation(api_port, context, tokens["observatory"], agent_id, "issue901-runtime-interruption")
        proxy_state.wait_for_generate(before)
        interrupted_at = adapter_proof.utc_now()
        socket.sock.close()
        proxy_state.wait_for_completion("issue901-runtime-interruption")
        live = lifecycle.api(context, api_port, tokens["observatory"], "/v1/observatory?schema=v3")
        require(lifecycle.identity(live) == runtime_identity, "Runtime changed after session interruption")
        report["scenarios"]["interruption"] = {
            "session_interrupted": True, "interrupted_at": interrupted_at,
            "correlation_id_sha256": hashlib.sha256(correlation.encode()).hexdigest(),
        }

        before = proxy_state.generate_count
        socket, turn, correlation = terminal_conversation(api_port, context, tokens["observatory"], agent_id, "issue901-runtime-recovery")
        proxy_state.wait_for_generate(before)
        recovered = await_terminal(socket, turn)
        socket.sock.close()
        report["scenarios"]["recovery"] = {
            "terminal": public_terminal(recovered), "correlation_id_sha256": hashlib.sha256(correlation.encode()).hexdigest(),
        }
        checkpoint = output / "checkpoint.json"
        csmctl("checkpoint", "--init", init, "--id", agent_id, "--out", checkpoint)
        report["checkpoint"] = {"durable": checkpoint.is_file(), "sha256": adapter_proof.sha256_file(checkpoint)}
        csmctl("remove", "--init", init, "--id", agent_id)
        report["agent_removed"] = agent_id not in json.dumps(csmctl("list", "--init", init))
        final_snapshot = lifecycle.api(context, api_port, tokens["observatory"], "/v1/observatory?schema=v3")
        report["runtime_identity_after"] = lifecycle.identity(final_snapshot)
        report["timeout_evidence_scope"] = "standalone_adapter_retained"
        report["proxy_request_count"] = len([
            row for row in proxy_state.records if row.get("path") == "/v1/chat/completions"
        ])
        report["proxy_request_digest"] = digest([
            {key: row.get(key) for key in ("path", "body_sha256", "request_id", "upstream_status", "upstream_error")}
            for row in proxy_state.records if row.get("path") == "/v1/chat/completions"
        ])
        errors = validate_report(report)
        report["validation"] = {"status": "passed" if not errors else "failed", "errors": errors}
        (output / "runtime-qualification-report.json").write_text(json.dumps(report, indent=2) + "\n")
        if args.public_report:
            args.public_report.parent.mkdir(parents=True, exist_ok=True)
            args.public_report.write_text(json.dumps(report, indent=2) + "\n")
        return report
    finally:
        if provider is not None:
            provider.stop()
        proxy.shutdown()
        proxy.server_close()
        proxy_thread.join(timeout=5)
        fixture.server.shutdown()
        fixture.resident_server.shutdown()
        clock.sock.close()
        if guardian.poll() is None:
            os.killpg(guardian.pid, signal.SIGTERM)
            try:
                guardian.wait(timeout=20)
            except subprocess.TimeoutExpired:
                os.killpg(guardian.pid, signal.SIGKILL)
                guardian.wait(timeout=5)
        guardian_log.close()


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    for name in ("csm", "csmctl", "guardian", "kernel", "vector", "provider_binary", "model_file"):
        parser.add_argument("--" + name.replace("_", "-"), type=Path, required=True)
    parser.add_argument("--source-revision", required=True)
    parser.add_argument("--model", default="gemma:2b")
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--public-report", type=Path)
    parser.add_argument("--validate-report", type=Path)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.validate_report:
        report = json.loads(args.validate_report.read_text())
        errors = validate_report(report)
        print(json.dumps({"status": "passed" if not errors else "failed", "errors": errors}))
        return 0 if not errors else 1
    report = run(args)
    print(json.dumps(report["validation"]))
    return 0 if report["validation"]["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
