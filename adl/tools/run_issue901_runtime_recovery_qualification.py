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


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def is_sha256(value: Any) -> bool:
    return isinstance(value, str) and len(value) == 64 and all(character in "0123456789abcdef" for character in value)


def parse_timestamp(value: Any) -> float | None:
    if not isinstance(value, str):
        return None
    try:
        return __import__("datetime").datetime.fromisoformat(value).timestamp()
    except ValueError:
        return None


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


def public_terminal(result: dict[str, Any], correlation_id: str) -> dict[str, Any]:
    return {
        "status": result.get("status"),
        "failure_reason": result.get("failure_reason"),
        "correlation_matched": result.get("correlation_id") == correlation_id,
        "reply_present": bool(result.get("reply")),
        "reply_sha256": hashlib.sha256(str(result.get("reply", "")).encode()).hexdigest()
        if result.get("reply")
        else None,
    }


def checkpoint_binding(checkpoint_value: dict[str, Any]) -> dict[str, Any]:
    turns = [
        turn
        for conversation in checkpoint_value.get("conversation_history", [])
        for turn in conversation.get("turns", [])
    ]
    return {
        "schema": checkpoint_value.get("schema"),
        "checkpoint_digest": checkpoint_value.get("checkpoint_digest"),
        "agent_id": (checkpoint_value.get("declaration") or {}).get("id"),
        "provider": (checkpoint_value.get("declaration") or {}).get("provider"),
        "model": (checkpoint_value.get("declaration") or {}).get("model"),
        "terminal_statuses": [turn.get("terminal_status") for turn in turns],
        "correlation_sha256": [
            hashlib.sha256(str(turn.get("correlation_id", "")).encode()).hexdigest()
            for turn in turns
        ],
    }


def validate_raw_artifacts(report: dict[str, Any], artifact_root: Path | None) -> list[str]:
    if artifact_root is None or not artifact_root.is_dir():
        return ["runtime_raw_artifacts_missing"]
    expected = {
        "install_receipt": "runtime-v3/generations/issue901-runtime/receipt.json",
        "config_status": "csm-config-status.json",
        "runtime_observations": "runtime-observations.json",
        "checkpoint": "checkpoint.json",
        "proxy_requests": "proxy-requests.json",
        "guardian_log": "guardian.log",
    }
    declared = report.get("raw_artifacts") or {}
    paths = {name: artifact_root / relative for name, relative in expected.items()}
    if set(declared) != set(expected) or any(
        not path.is_file() or declared.get(name) != sha256_file(path)
        for name, path in paths.items()
    ):
        return ["runtime_raw_artifact_binding_invalid"]
    try:
        install = json.loads(paths["install_receipt"].read_text())
        config_status = json.loads(paths["config_status"].read_text())
        observations = json.loads(paths["runtime_observations"].read_text())
        checkpoint_value = json.loads(paths["checkpoint"].read_text())
        proxy_records = json.loads(paths["proxy_requests"].read_text())
    except (OSError, ValueError, TypeError):
        return ["runtime_raw_artifact_parse_invalid"]
    errors: list[str] = []
    if (install.get("schema") != "adl.runtime_v3.install_generation.v1"
            or install.get("generation") != "issue901-runtime"
            or install.get("source_revision") != report.get("source_revision")
            or set((install.get("artifacts") or {})) != {"csm", "guardian", "kernel"}
            or any(not is_sha256(row.get("sha256")) for row in (install.get("artifacts") or {}).values())):
        errors.append("runtime_install_receipt_invalid")
    if (config_status.get("schema") != "adl.csm.runtime_v3_service_status.v1"
            or config_status.get("config_valid") is not True
            or config_status.get("service_loaded") is not False):
        errors.append("runtime_config_preflight_invalid")
    expected_observation = {
        "source_revision": report.get("source_revision"),
        "runtime_identity": report.get("runtime_identity"),
        "runtime_identity_after": report.get("runtime_identity_after"),
        "registered_provider": report.get("registered_provider"),
        "provider_process_ids": report.get("provider_process_ids"),
        "scenarios": report.get("scenarios"),
        "agent_removed": report.get("agent_removed"),
        "paid_calls": report.get("paid_calls"),
    }
    if observations != expected_observation:
        errors.append("runtime_observation_projection_mismatch")
    if (report.get("checkpoint", {}).get("sha256") != sha256_file(paths["checkpoint"])
            or report.get("checkpoint", {}).get("binding") != checkpoint_binding(checkpoint_value)):
        errors.append("runtime_checkpoint_projection_mismatch")
    projected_proxy = [
        {
            "request_id": row.get("request_id"),
            "body_sha256": row.get("body_sha256"),
            "upstream_status": row.get("upstream_status"),
            "upstream_error": row.get("upstream_error"),
            "completed": bool(row.get("completed_at")),
        }
        for row in proxy_records if row.get("path") == "/v1/chat/completions"
    ]
    if projected_proxy != report.get("proxy_requests"):
        errors.append("runtime_proxy_projection_mismatch")
    if paths["guardian_log"].stat().st_size == 0:
        errors.append("runtime_guardian_log_empty")
    return errors


def validate_report(report: dict[str, Any], artifact_root: Path | None = None) -> list[str]:
    errors: list[str] = []
    if report.get("schema") != SCHEMA:
        errors.append("schema_invalid")
    if not isinstance(report.get("source_revision"), str) or len(report["source_revision"]) != 40 or any(
        character not in "0123456789abcdef" for character in report["source_revision"]
    ):
        errors.append("source_revision_invalid")
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
    if scenarios["loss"].get("terminal", {}).get("correlation_matched") is not True:
        errors.append("runtime_loss_correlation_unbound")
    if scenarios["interruption"].get("session_interrupted") is not True:
        errors.append("runtime_session_interruption_missing")
    recovery = scenarios["recovery"].get("terminal", {})
    if recovery.get("status") != "delivered" or recovery.get("reply_present") is not True:
        errors.append("runtime_recovery_not_delivered")
    if recovery.get("correlation_matched") is not True:
        errors.append("runtime_recovery_correlation_unbound")
    correlations = [row.get("correlation_id_sha256") for row in scenarios.values()]
    if len(set(correlations)) != 3 or any(not isinstance(value, str) or len(value) != 64 for value in correlations):
        errors.append("runtime_correlation_identity_invalid")
    pids = report.get("provider_process_ids") or []
    if len(pids) < 2 or len(set(pids)) < 2:
        errors.append("fresh_provider_incarnation_missing")
    loss = scenarios["loss"]
    if (loss.get("provider_pid") != pids[0] or not isinstance(loss.get("provider_returncode"), int)
            or loss["provider_returncode"] >= 0 or parse_timestamp(loss.get("provider_signal_sent_at")) is None):
        errors.append("runtime_loss_process_evidence_invalid")
    interruption = scenarios["interruption"]
    if (interruption.get("provider_completed_after_disconnect") is not True
            or parse_timestamp(interruption.get("interrupted_at")) is None):
        errors.append("runtime_interruption_completion_unbound")
    checkpoint = report.get("checkpoint") or {}
    binding = checkpoint.get("binding") or {}
    if (checkpoint.get("durable") is not True or not is_sha256(checkpoint.get("sha256"))
            or binding.get("schema") != "adl.runtime_v3.agent_checkpoint.v1"
            or binding.get("agent_id") != "issue901-runtime-agent"
            or binding.get("provider") != "openai-compatible"
            or binding.get("model") != report.get("model")
            or binding.get("terminal_statuses") != ["failed", "delivered"]
            or binding.get("correlation_sha256") != [
                scenarios["loss"].get("correlation_id_sha256"),
                scenarios["recovery"].get("correlation_id_sha256"),
            ]
            or not is_sha256(binding.get("checkpoint_digest"))
            or report.get("agent_removed") is not True):
        errors.append("runtime_lifecycle_incomplete")
    if report.get("paid_calls") != 0:
        errors.append("paid_call_boundary_invalid")
    if report.get("timeout_evidence_scope") != "standalone_adapter_retained":
        errors.append("timeout_evidence_scope_invalid")
    proxy_requests = report.get("proxy_requests") or []
    expected_request_ids = ["issue901-runtime-loss", "issue901-runtime-recovery", "issue901-runtime-interruption"]
    if (report.get("proxy_request_count") != 3 or len(proxy_requests) != 3
            or [row.get("request_id") for row in proxy_requests] != expected_request_ids
            or any(not is_sha256(row.get("body_sha256")) or row.get("completed") is not True for row in proxy_requests)
            or len({row.get("body_sha256") for row in proxy_requests}) != 3
            or proxy_requests[0].get("upstream_status") is not None
            or not proxy_requests[0].get("upstream_error")
            or any(row.get("upstream_status") != 200 or row.get("upstream_error") is not None for row in proxy_requests[1:])
            or report.get("proxy_request_digest") != digest(proxy_requests)):
        errors.append("runtime_proxy_evidence_invalid")
    errors.extend(validate_raw_artifacts(report, artifact_root))
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
    raw_observations: dict[str, Any] = {"source_revision": args.source_revision, "paid_calls": 0}
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
        raw_observations["runtime_identity"] = runtime_identity

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
        raw_observations["registered_provider"] = report["registered_provider"]

        before = proxy_state.generate_count
        socket, turn, correlation = terminal_conversation(api_port, context, tokens["observatory"], agent_id, "issue901-runtime-loss")
        proxy_state.wait_for_generate(before)
        stopped = provider.stop()
        provider = None
        loss = await_terminal(socket, turn)
        socket.sock.close()
        report["scenarios"]["loss"] = {
            "terminal": public_terminal(loss, correlation), "correlation_id_sha256": hashlib.sha256(correlation.encode()).hexdigest(),
            "provider_pid": report["provider_process_ids"][0],
            "provider_signal_sent_at": stopped["signal_sent_at"], "provider_returncode": stopped["returncode"],
        }

        provider = adapter_proof.start_provider(args.provider_binary, args.model_file, upstream_port, output / "provider-recovered.log")
        report["provider_process_ids"].append(provider.pid)
        before = proxy_state.generate_count
        socket, turn, correlation = terminal_conversation(api_port, context, tokens["observatory"], agent_id, "issue901-runtime-recovery")
        proxy_state.wait_for_generate(before)
        recovered = await_terminal(socket, turn)
        socket.sock.close()
        report["scenarios"]["recovery"] = {
            "terminal": public_terminal(recovered, correlation), "correlation_id_sha256": hashlib.sha256(correlation.encode()).hexdigest(),
        }

        interruption_agent_id = "issue901-interruption-agent"
        interruption_config = {
            **config,
            "identity": {
                "id": interruption_agent_id,
                "name": "interruption.fixture",
                "display_name": "Interruption Fixture",
            },
        }
        interruption_config_path = lifecycle.write(output / "interruption-agent.json", interruption_config)
        require(csmctl("add", "--config", interruption_config_path).get("status") == "admitted",
                "Runtime interruption-agent admission failed")
        deadline = time.monotonic() + 20
        interruption_detail = None
        while time.monotonic() < deadline:
            interruption_detail = csmctl("get", "--init", init, "--id", interruption_agent_id)
            if interruption_detail.get("communication_eligible"):
                break
            time.sleep(0.1)
        require(interruption_detail and interruption_detail.get("communication_eligible"),
                "registered Runtime interruption agent not ready")
        before = proxy_state.generate_count
        socket, turn, correlation = terminal_conversation(
            api_port, context, tokens["observatory"], interruption_agent_id, "issue901-runtime-interruption"
        )
        proxy_state.wait_for_generate(before)
        interrupted_at = adapter_proof.utc_now()
        socket.sock.close()
        interruption_proxy = proxy_state.wait_for_completion("issue901-runtime-interruption")
        interruption_completed_at = parse_timestamp(interruption_proxy.get("completed_at"))
        interruption_started_at = parse_timestamp(interrupted_at)
        live = lifecycle.api(context, api_port, tokens["observatory"], "/v1/observatory?schema=v3")
        require(lifecycle.identity(live) == runtime_identity, "Runtime changed after session interruption")
        report["scenarios"]["interruption"] = {
            "session_interrupted": True, "interrupted_at": interrupted_at,
            "provider_completed_after_disconnect": (
                interruption_proxy.get("upstream_status") == 200
                and interruption_proxy.get("upstream_error") is None
                and interruption_completed_at is not None
                and interruption_started_at is not None
                and interruption_completed_at > interruption_started_at
            ),
            "correlation_id_sha256": hashlib.sha256(correlation.encode()).hexdigest(),
        }

        checkpoint = output / "checkpoint.json"
        csmctl("checkpoint", "--init", init, "--id", agent_id, "--out", checkpoint)
        checkpoint_value = json.loads(checkpoint.read_text())
        report["checkpoint"] = {
            "durable": checkpoint.is_file(),
            "sha256": adapter_proof.sha256_file(checkpoint),
            "binding": checkpoint_binding(checkpoint_value),
        }
        csmctl("remove", "--init", init, "--id", agent_id)
        csmctl("remove", "--init", init, "--id", interruption_agent_id)
        remaining_agents = json.dumps(csmctl("list", "--init", init))
        report["agent_removed"] = all(
            removed not in remaining_agents for removed in (agent_id, interruption_agent_id)
        )
        final_snapshot = lifecycle.api(context, api_port, tokens["observatory"], "/v1/observatory?schema=v3")
        report["runtime_identity_after"] = lifecycle.identity(final_snapshot)
        report["timeout_evidence_scope"] = "standalone_adapter_retained"
        report["proxy_requests"] = [
            {
                "request_id": row.get("request_id"),
                "body_sha256": row.get("body_sha256"),
                "upstream_status": row.get("upstream_status"),
                "upstream_error": row.get("upstream_error"),
                "completed": bool(row.get("completed_at")),
            }
            for row in proxy_state.records if row.get("path") == "/v1/chat/completions"
        ]
        report["proxy_request_count"] = len(report["proxy_requests"])
        report["proxy_request_digest"] = digest(report["proxy_requests"])
        raw_observations.update({
            "runtime_identity_after": report["runtime_identity_after"],
            "provider_process_ids": report["provider_process_ids"],
            "scenarios": report["scenarios"],
            "agent_removed": report["agent_removed"],
        })
        (output / "runtime-observations.json").write_text(json.dumps(raw_observations, indent=2) + "\n")
        (output / "proxy-requests.json").write_text(json.dumps(proxy_state.records, indent=2) + "\n")
        if guardian.poll() is None:
            os.killpg(guardian.pid, signal.SIGTERM)
            guardian.wait(timeout=20)
        guardian_log.flush()
        report["raw_artifacts"] = {
            "install_receipt": sha256_file(output / "runtime-v3/generations/issue901-runtime/receipt.json"),
            "config_status": sha256_file(output / "csm-config-status.json"),
            "runtime_observations": sha256_file(output / "runtime-observations.json"),
            "checkpoint": sha256_file(output / "checkpoint.json"),
            "proxy_requests": sha256_file(output / "proxy-requests.json"),
            "guardian_log": sha256_file(output / "guardian.log"),
        }
        errors = validate_report(report, output)
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
    parser.add_argument("--artifact-root", type=Path)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.validate_report:
        report = json.loads(args.validate_report.read_text())
        errors = validate_report(report, args.artifact_root)
        print(json.dumps({"status": "passed" if not errors else "failed", "errors": errors}))
        return 0 if not errors else 1
    report = run(args)
    print(json.dumps(report["validation"]))
    return 0 if report["validation"]["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
