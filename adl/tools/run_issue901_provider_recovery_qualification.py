#!/usr/bin/env python3
"""Run #901 provider loss, timeout, interruption, and recovery qualification."""

from __future__ import annotations

import argparse
import copy
import hashlib
import http.client
import json
import os
from pathlib import Path
import signal
import socket
import subprocess
import sys
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from typing import Any
from urllib.request import urlopen


SCHEMA = "adl.issue901.provider_recovery_qualification.v1"
SCENARIOS = ("loss", "timeout", "interruption", "recovery")


def utc_now() -> str:
    from datetime import datetime, timezone

    return datetime.now(timezone.utc).isoformat()


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def reserve_port() -> int:
    with socket.socket() as sock:
        sock.bind(("127.0.0.1", 0))
        return int(sock.getsockname()[1])


class ProxyState:
    def __init__(self, upstream_port: int) -> None:
        self.upstream_port = upstream_port
        self.condition = threading.Condition()
        self.generate_count = 0
        self.records: list[dict[str, Any]] = []

    def record_forwarded(self, record: dict[str, Any]) -> None:
        with self.condition:
            self.records.append(record)
            if record["path"] == "/v1/responses":
                self.generate_count += 1
            self.condition.notify_all()

    def wait_for_generate(self, previous: int, timeout: float = 30.0) -> int:
        deadline = time.monotonic() + timeout
        with self.condition:
            while self.generate_count <= previous:
                remaining = deadline - time.monotonic()
                if remaining <= 0:
                    raise RuntimeError("provider generate request was not forwarded")
                self.condition.wait(remaining)
            return self.generate_count


def proxy_handler(state: ProxyState) -> type[BaseHTTPRequestHandler]:
    class Handler(BaseHTTPRequestHandler):
        protocol_version = "HTTP/1.1"

        def do_GET(self) -> None:  # noqa: N802
            self._forward()

        def do_POST(self) -> None:  # noqa: N802
            self._forward()

        def _forward(self) -> None:
            length = int(self.headers.get("Content-Length", "0"))
            body = self.rfile.read(length) if length else b""
            started = utc_now()
            record: dict[str, Any] = {
                "method": self.command,
                "path": self.path,
                "body_sha256": hashlib.sha256(body).hexdigest(),
                "body_bytes": len(body),
                "forwarded_at": started,
                "upstream_port": state.upstream_port,
                "completed_at": None,
                "upstream_status": None,
                "upstream_error": None,
                "request_id": None,
            }
            if self.path == "/v1/responses":
                try:
                    input_text = json.loads(body).get("input", "")
                    if isinstance(input_text, str):
                        record["request_id"] = input_text.split(":", 1)[0]
                except (json.JSONDecodeError, AttributeError):
                    pass
            connection = http.client.HTTPConnection("127.0.0.1", state.upstream_port, timeout=180)
            try:
                headers = {
                    key: value
                    for key, value in self.headers.items()
                    if key.lower() not in {"host", "connection", "content-length"}
                }
                headers["Content-Length"] = str(len(body))
                connection.request(self.command, self.path, body=body, headers=headers)
                state.record_forwarded(record)
                response = connection.getresponse()
                payload = response.read()
                record["upstream_status"] = response.status
                self.send_response(response.status)
                for key, value in response.getheaders():
                    if key.lower() not in {"connection", "transfer-encoding", "content-length"}:
                        self.send_header(key, value)
                self.send_header("Content-Length", str(len(payload)))
                self.end_headers()
                self.wfile.write(payload)
            except Exception as error:  # transport outcome is evidence
                record["upstream_error"] = type(error).__name__
                try:
                    self.close_connection = True
                    self.connection.shutdown(socket.SHUT_RDWR)
                    self.connection.close()
                except OSError:
                    pass
            finally:
                record["completed_at"] = utc_now()
                connection.close()

        def log_message(self, _format: str, *_args: object) -> None:
            return

    return Handler


class OwnedProcess:
    def __init__(self, process: subprocess.Popen[str], log_handle: Any, started_at: str) -> None:
        self.process = process
        self.log_handle = log_handle
        self.started_at = started_at

    @property
    def pid(self) -> int:
        return self.process.pid

    def stop(self) -> dict[str, Any]:
        sent_at = utc_now()
        if self.process.poll() is None:
            os.killpg(self.process.pid, signal.SIGTERM)
            try:
                self.process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                os.killpg(self.process.pid, signal.SIGKILL)
                self.process.wait(timeout=10)
        self.log_handle.close()
        return {"signal_sent_at": sent_at, "returncode": self.process.returncode}


def start_provider(binary: Path, model_file: Path, port: int, log_path: Path) -> OwnedProcess:
    log_handle = log_path.open("a", encoding="utf-8")
    started_at = utc_now()
    process = subprocess.Popen(
        [
            str(binary),
            "--model", str(model_file),
            "--host", "127.0.0.1",
            "--port", str(port),
            "--device", "none",
            "--gpu-layers", "0",
            "--fit", "off",
            "--offline",
            "--no-webui",
            "--ctx-size", "2048",
            "--threads", "4",
            "--parallel", "1",
        ],
        stdout=log_handle,
        stderr=subprocess.STDOUT,
        text=True,
        start_new_session=True,
    )
    deadline = time.monotonic() + 30
    url = f"http://127.0.0.1:{port}/health"
    while time.monotonic() < deadline:
        if process.poll() is not None:
            log_handle.close()
            raise RuntimeError(f"owned provider exited during startup: {process.returncode}")
        try:
            with urlopen(url, timeout=1) as response:
                if response.status == 200:
                    return OwnedProcess(process, log_handle, started_at)
        except Exception:
            time.sleep(0.1)
    os.killpg(process.pid, signal.SIGKILL)
    process.wait(timeout=5)
    log_handle.close()
    raise RuntimeError("owned provider did not become ready")


def request_payload(model: str, endpoint: str, scenario: str, timeout_ms: int, tokens: int) -> dict[str, Any]:
    request_id = f"issue901-{scenario}"
    return {
        "route": {
            "provider_kind": "local",
            "provider": "openai",
            "runtime_surface": "hosted_api",
            "provider_model_id": model,
            "endpoint_ref": endpoint,
            "credential_ref": "env:ADL_ISSUE901_LOCAL_KEY",
            "source_registry": "issue901.task_owned_local",
        },
        "model_identity": {
            "provider_kind": "local",
            "provider": "openai",
            "model_ref": model,
            "provider_model_id": model,
            "runtime_surface": "hosted_api",
            "identity_strength": "tag_only",
            "observed_at": "issue901-preflight",
            "source_registry": "issue901.task_owned_local",
        },
        "prompt_contract_ref": "issue901.provider_recovery.v1",
        "lane_ref": "provider",
        "run_id": request_id,
        "request_id": request_id,
        "attempt_policy": {"max_attempts": 1, "timeout_ms": timeout_ms, "retry_backoff_ms": 1},
        "input_text": f"{request_id}: reply with a concise confirmation and continue until complete.",
        "max_output_tokens": tokens,
        "context_window_tokens": 2048,
        "local_keep_alive": "0",
    }


def launch_adapter(adapter: Path, output: Path, endpoint: str, model: str, scenario: str, timeout_ms: int, tokens: int) -> tuple[subprocess.Popen[str], dict[str, Path], float]:
    paths = {
        "request": output / f"{scenario}-request.json",
        "result": output / f"{scenario}-result.json",
        "log": output / f"{scenario}-adapter.jsonl",
        "stdout": output / f"{scenario}-stdout.log",
        "stderr": output / f"{scenario}-stderr.log",
    }
    paths["request"].write_text(json.dumps(request_payload(model, endpoint, scenario, timeout_ms, tokens), indent=2) + "\n")
    stdout = paths["stdout"].open("w", encoding="utf-8")
    stderr = paths["stderr"].open("w", encoding="utf-8")
    started = time.monotonic()
    process = subprocess.Popen(
        [str(adapter), "--request", str(paths["request"]), "--out", str(paths["result"]), "--log", str(paths["log"])],
        stdout=stdout,
        stderr=stderr,
        text=True,
        env=os.environ | {"ADL_ISSUE901_LOCAL_KEY": "task-owned-local-provider"},
        start_new_session=True,
    )
    process._issue901_handles = (stdout, stderr)  # type: ignore[attr-defined]
    return process, paths, started


def finish_adapter(process: subprocess.Popen[str], started: float, timeout: float = 190) -> tuple[int, int]:
    returncode = process.wait(timeout=timeout)
    elapsed_ms = int((time.monotonic() - started) * 1000)
    for handle in process._issue901_handles:  # type: ignore[attr-defined]
        handle.close()
    return returncode, elapsed_ms


def result_summary(paths: dict[str, Path], returncode: int, elapsed_ms: int) -> dict[str, Any]:
    result = json.loads(paths["result"].read_text()) if paths["result"].exists() else None
    artifacts = {}
    for kind, path in paths.items():
        if path.exists():
            artifacts[kind] = {
                "ref": path.name,
                "sha256": sha256_file(path),
                "bytes": path.stat().st_size,
            }
    return {
        "request_ref": paths["request"].name,
        "result_ref": paths["result"].name if result else None,
        "run_log_ref": paths["log"].name if paths["log"].exists() else None,
        "stdout_ref": paths["stdout"].name,
        "stderr_ref": paths["stderr"].name,
        "adapter_returncode": returncode,
        "wall_elapsed_ms": elapsed_ms,
        "result": result,
        "artifacts": artifacts,
    }


def is_sha256(value: Any) -> bool:
    return isinstance(value, str) and len(value) == 64 and all(char in "0123456789abcdef" for char in value)


def execution_observation(report: dict[str, Any]) -> dict[str, Any]:
    """Project execution claims into the separately hashed raw observation artifact."""
    scenarios = report.get("scenarios", {})
    return {
        "schema": "adl.issue901.execution_observations.v1",
        "provider_incarnations": report.get("provider_incarnations"),
        "proxy_records": report.get("proxy_records"),
        "inference_request_body_sha256": report.get("inference_request_body_sha256"),
        "scenarios": {
            name: {
                key: value
                for key, value in scenarios.get(name, {}).items()
                if key not in {"result", "artifacts"}
            }
            for name in SCENARIOS
        },
    }


def validate_report(report: dict[str, Any], artifact_root: Path | None = None) -> list[str]:
    errors: list[str] = []
    if report.get("schema") != SCHEMA:
        errors.append("schema_invalid")
    scenarios = report.get("scenarios")
    if not isinstance(scenarios, dict) or any(name not in scenarios for name in SCENARIOS):
        return errors + ["scenario_population_incomplete"]
    if report.get("caller_failure_flags") is not False:
        errors.append("caller_failure_flags_not_proof")
    if report.get("reference_trace_only") is not False:
        errors.append("reference_trace_not_execution")

    proxy_records = report.get("proxy_records")
    if not isinstance(proxy_records, list):
        proxy_records = []
        errors.append("proxy_records_missing")
    incarnations = report.get("provider_incarnations")
    if not isinstance(incarnations, list):
        incarnations = []
        errors.append("provider_incarnations_missing")
    incarnation_by_label = {row.get("label"): row for row in incarnations if isinstance(row, dict)}

    observation_artifact = report.get("execution_observation_artifact")
    if (
        not isinstance(observation_artifact, dict)
        or observation_artifact.get("ref") != "execution-observations.json"
        or not is_sha256(observation_artifact.get("sha256"))
        or not isinstance(observation_artifact.get("bytes"), int)
        or observation_artifact["bytes"] <= 0
    ):
        errors.append("execution_observation_artifact_invalid")
    elif artifact_root is not None:
        observation_path = artifact_root / "execution-observations.json"
        try:
            if (
                sha256_file(observation_path) != observation_artifact["sha256"]
                or observation_path.stat().st_size != observation_artifact["bytes"]
            ):
                errors.append("execution_observation_artifact_content_mismatch")
            elif json.loads(observation_path.read_text()) != execution_observation(report):
                errors.append("execution_observation_summary_mismatch")
        except (OSError, json.JSONDecodeError):
            errors.append("execution_observation_artifact_content_mismatch")

    request_ids: set[str] = set()
    provider_pids: list[int] = []
    for name in SCENARIOS:
        row = scenarios[name]
        request_id = row.get("request_id")
        if request_id != f"issue901-{name}" or request_id in request_ids:
            errors.append(f"{name}_request_identity_invalid")
        request_ids.add(request_id)
        if not row.get("generate_forwarded"):
            errors.append(f"{name}_provider_invocation_missing")
        if row.get("run_log_ref") != f"{name}-adapter.jsonl":
            errors.append(f"{name}_execution_log_missing")

        provider_pid = row.get("owned_provider_pid")
        if not isinstance(provider_pid, int) or provider_pid <= 0:
            errors.append(f"{name}_provider_process_identity_invalid")
        else:
            provider_pids.append(provider_pid)
        incarnation = incarnation_by_label.get(name)
        if not isinstance(incarnation, dict) or incarnation.get("pid") != provider_pid or not incarnation.get("started_at"):
            errors.append(f"{name}_provider_incarnation_mismatch")

        matching_proxy = [
            record
            for record in proxy_records
            if isinstance(record, dict)
            and record.get("path") == "/v1/responses"
            and record.get("request_id") == request_id
        ]
        if len(matching_proxy) != 1 or not matching_proxy[0].get("forwarded_at"):
            errors.append(f"{name}_proxy_evidence_invalid")

        result = row.get("result")
        if name != "interruption" and (not isinstance(result, dict) or result.get("request_id") != request_id):
            errors.append(f"{name}_result_identity_invalid")

        artifacts = row.get("artifacts")
        required = {"request", "log", "stdout", "stderr"} | ({"result"} if name != "interruption" else set())
        if not isinstance(artifacts, dict):
            errors.append(f"{name}_artifact_manifest_missing")
            continue
        for kind in required:
            artifact = artifacts.get(kind)
            suffix = "jsonl" if kind == "log" else "json" if kind in {"request", "result"} else "log"
            label = "adapter" if kind == "log" else kind
            expected_ref = f"{name}-{label}.{suffix}"
            if (
                not isinstance(artifact, dict)
                or artifact.get("ref") != expected_ref
                or not is_sha256(artifact.get("sha256"))
                or not isinstance(artifact.get("bytes"), int)
                or artifact["bytes"] < (1 if kind in {"request", "log", "result"} else 0)
            ):
                errors.append(f"{name}_{kind}_artifact_invalid")
                continue
            if artifact_root is not None:
                artifact_path = artifact_root / expected_ref
                if (
                    not artifact_path.is_file()
                    or sha256_file(artifact_path) != artifact["sha256"]
                    or artifact_path.stat().st_size != artifact["bytes"]
                ):
                    errors.append(f"{name}_{kind}_artifact_content_mismatch")
        if artifact_root is not None:
            request_path = artifact_root / f"{name}-request.json"
            try:
                request = json.loads(request_path.read_text())
                if request.get("request_id") != request_id:
                    errors.append(f"{name}_request_artifact_identity_mismatch")
            except (OSError, json.JSONDecodeError):
                errors.append(f"{name}_request_artifact_invalid")
            if name != "interruption":
                result_path = artifact_root / f"{name}-result.json"
                try:
                    if json.loads(result_path.read_text()) != result:
                        errors.append(f"{name}_result_artifact_summary_mismatch")
                except (OSError, json.JSONDecodeError):
                    errors.append(f"{name}_result_artifact_invalid")
            log_path = artifact_root / f"{name}-adapter.jsonl"
            try:
                events = [json.loads(line) for line in log_path.read_text().splitlines() if line.strip()]
                event_types = {event.get("event_type") for event in events}
                if (
                    not events
                    or any(event.get("request_id") != request_id for event in events)
                    or not {"run_start", "attempt_start"}.issubset(event_types)
                ):
                    errors.append(f"{name}_run_log_content_invalid")
            except (OSError, json.JSONDecodeError):
                errors.append(f"{name}_run_log_content_invalid")

    timeout = scenarios["timeout"]
    timeout_result = timeout.get("result") or {}
    timeout_failure = timeout_result.get("failure") or {}
    timeout_budget = timeout.get("timeout_ms")
    if (
        timeout_failure.get("kind") != "provider_timeout"
        or not isinstance(timeout.get("wall_elapsed_ms"), int)
        or not isinstance(timeout_budget, int)
        or timeout["wall_elapsed_ms"] < timeout_budget
        or not isinstance(timeout_result.get("duration_ms"), int)
        or timeout_result["duration_ms"] < timeout_budget
    ):
        errors.append("timeout_elapsed_deadline_evidence_invalid")

    loss = scenarios["loss"]
    if (
        not isinstance(loss.get("owned_provider_returncode"), int)
        or loss["owned_provider_returncode"] >= 0
        or (loss.get("result") or {}).get("final_status") != "failed"
    ):
        errors.append("actual_process_loss_invalid")
    loss_proxy = next(
        (record for record in proxy_records if isinstance(record, dict) and record.get("request_id") == "issue901-loss"),
        {},
    )
    if loss_proxy.get("upstream_status") is not None or not loss_proxy.get("upstream_error") or not loss.get("provider_signal_sent_at"):
        errors.append("actual_process_loss_transport_invalid")

    interruption = scenarios["interruption"]
    if interruption.get("adapter_returncode") != -signal.SIGTERM or interruption.get("result") is not None:
        errors.append("actual_interruption_invalid")
    if not interruption.get("adapter_signal_sent_at"):
        errors.append("actual_interruption_signal_missing")

    recovery = scenarios["recovery"]
    recovery_result = recovery.get("result") or {}
    if recovery_result.get("final_status") != "ok" or not (
        recovery_result.get("output_text") or recovery_result.get("output_text_present") is True
    ):
        errors.append("healthy_recovery_invalid")
    recovery_proxy = next(
        (record for record in proxy_records if isinstance(record, dict) and record.get("request_id") == "issue901-recovery"),
        {},
    )
    if recovery_proxy.get("upstream_status") != 200 or recovery_proxy.get("upstream_error") is not None:
        errors.append("healthy_recovery_transport_invalid")

    if len(provider_pids) != 4 or len(set(provider_pids)) != 4:
        errors.append("provider_process_identity_not_unique")
    if recovery.get("owned_provider_pid") in {loss.get("owned_provider_pid"), None}:
        errors.append("stale_provider_identity_reused")

    inference_hashes = report.get("inference_request_body_sha256")
    observed_hashes = [
        record.get("body_sha256")
        for record in proxy_records
        if isinstance(record, dict) and record.get("path") == "/v1/responses"
    ]
    if (
        not isinstance(inference_hashes, list)
        or len(inference_hashes) != 4
        or len(set(inference_hashes)) != 4
        or any(not is_sha256(value) for value in inference_hashes)
        or inference_hashes != observed_hashes
    ):
        errors.append("duplicate_or_missing_provider_work")
    return errors

def public_report(report: dict[str, Any]) -> dict[str, Any]:
    public = copy.deepcopy(report)
    public["adapter"]["path"] = "adl/target/debug/adl-provider-adapter"
    public["provider"]["binary"] = Path(public["provider"]["binary"]).name
    for row in public["scenarios"].values():
        result = row.get("result")
        if not isinstance(result, dict):
            continue
        output = result.pop("output_text", None)
        result["output_text_present"] = bool(output)
        result["output_text_sha256"] = hashlib.sha256(output.encode()).hexdigest() if output else None
        failure = result.get("failure")
        if isinstance(failure, dict):
            failure.pop("message", None)
            failure.pop("provider_error_excerpt", None)
    public["publication_boundary"] = "Portable receipt: prompts and model output omitted; private raw run retained in the issue worktree."
    return public


def run(args: argparse.Namespace) -> dict[str, Any]:
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    upstream_port = reserve_port()
    proxy_port = reserve_port()
    state = ProxyState(upstream_port)
    server = ThreadingHTTPServer(("127.0.0.1", proxy_port), proxy_handler(state))
    proxy_thread = threading.Thread(target=server.serve_forever, daemon=True)
    proxy_thread.start()
    endpoint = f"http://127.0.0.1:{proxy_port}/v1/responses"
    owned: OwnedProcess | None = None
    scenarios: dict[str, Any] = {}
    provider_incarnations: list[dict[str, Any]] = []

    def start_incarnation(label: str) -> OwnedProcess:
        process = start_provider(args.provider_binary, args.model_file, upstream_port, output / f"provider-{label}.log")
        provider_incarnations.append({"label": label, "pid": process.pid, "started_at": process.started_at})
        return process

    try:
        owned = start_incarnation("loss")
        before = state.generate_count
        process, paths, started = launch_adapter(args.adapter, output, endpoint, args.model, "loss", 30_000, 1024)
        state.wait_for_generate(before)
        time.sleep(0.1)
        stopped = owned.stop()
        loss_pid = owned.pid
        owned = None
        returncode, elapsed = finish_adapter(process, started)
        scenarios["loss"] = result_summary(paths, returncode, elapsed) | {
            "request_id": "issue901-loss",
            "generate_forwarded": True,
            "owned_provider_pid": loss_pid,
            "owned_provider_returncode": stopped["returncode"],
            "provider_signal_sent_at": stopped["signal_sent_at"],
        }

        owned = start_incarnation("timeout")
        before = state.generate_count
        process, paths, started = launch_adapter(args.adapter, output, endpoint, args.model, "timeout", 150, 512)
        state.wait_for_generate(before)
        returncode, elapsed = finish_adapter(process, started)
        scenarios["timeout"] = result_summary(paths, returncode, elapsed) | {
            "request_id": "issue901-timeout",
            "generate_forwarded": True,
            "timeout_ms": 150,
            "owned_provider_pid": owned.pid,
        }
        owned.stop()
        owned = None

        owned = start_incarnation("interruption")
        before = state.generate_count
        process, paths, started = launch_adapter(args.adapter, output, endpoint, args.model, "interruption", 30_000, 1024)
        state.wait_for_generate(before)
        interrupted_at = utc_now()
        os.killpg(process.pid, signal.SIGTERM)
        returncode, elapsed = finish_adapter(process, started)
        scenarios["interruption"] = result_summary(paths, returncode, elapsed) | {
            "request_id": "issue901-interruption",
            "generate_forwarded": True,
            "owned_provider_pid": owned.pid,
            "adapter_signal_sent_at": interrupted_at,
        }
        owned.stop()
        owned = None

        owned = start_incarnation("recovery")
        before = state.generate_count
        process, paths, started = launch_adapter(args.adapter, output, endpoint, args.model, "recovery", 120_000, 8)
        state.wait_for_generate(before)
        returncode, elapsed = finish_adapter(process, started)
        scenarios["recovery"] = result_summary(paths, returncode, elapsed) | {
            "request_id": "issue901-recovery",
            "generate_forwarded": True,
            "owned_provider_pid": owned.pid,
        }
        owned.stop()
        owned = None
    finally:
        if owned is not None:
            owned.stop()
        server.shutdown()
        server.server_close()
        proxy_thread.join(timeout=5)

    generate_records = [record for record in state.records if record["path"] == "/v1/responses"]
    report: dict[str, Any] = {
        "schema": SCHEMA,
        "generated_at": utc_now(),
        "source_revision": subprocess.check_output(["git", "rev-parse", "HEAD"], text=True).strip(),
        "adapter": {"path": str(args.adapter), "sha256": sha256_file(args.adapter)},
        "provider": {
            "kind": "task_owned_local_openai_compatible_llama_server",
            "binary": str(args.provider_binary),
            "sha256": sha256_file(args.provider_binary),
            "version": args.provider_version,
            "model": args.model,
            "model_file_sha256": sha256_file(args.model_file),
        },
        "resource_authority": "operator-approved task-owned local macOS; no paid/cloud calls or shared service mutation",
        "provider_incarnations": provider_incarnations,
        "scenarios": scenarios,
        "proxy_records": state.records,
        "inference_request_body_sha256": [record["body_sha256"] for record in generate_records],
        "caller_failure_flags": False,
        "reference_trace_only": False,
        "test_classification": {
            "lane": "provider",
            "proof_role": "production-adapter effect and recovery qualification",
            "determinism": "bounded live local provider effects; deterministic report validation negatives",
            "resource_profile": "task-owned CPU llama-server and adapter process groups, loopback, local disk",
            "release_gate": "required issue gate",
        },
    }
    observation_path = output / "execution-observations.json"
    observation_path.write_text(json.dumps(execution_observation(report), indent=2) + "\n")
    report["execution_observation_artifact"] = {
        "ref": observation_path.name,
        "sha256": sha256_file(observation_path),
        "bytes": observation_path.stat().st_size,
    }
    errors = validate_report(report)
    report["validation"] = {"status": "passed" if not errors else "failed", "errors": errors, "scenario_count": len(scenarios)}
    (output / "qualification-report.json").write_text(json.dumps(report, indent=2) + "\n")
    if args.public_report:
        args.public_report.parent.mkdir(parents=True, exist_ok=True)
        args.public_report.write_text(json.dumps(public_report(report), indent=2) + "\n")
    return report


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--adapter", type=Path, required=True)
    parser.add_argument("--provider-binary", type=Path, required=True)
    parser.add_argument("--model", default="gemma:2b")
    parser.add_argument("--model-file", type=Path, required=True)
    parser.add_argument("--provider-version", required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--public-report", type=Path)
    parser.add_argument("--validate-report", type=Path)
    parser.add_argument("--artifact-root", type=Path)
    args = parser.parse_args()
    return args


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
    sys.exit(main())
