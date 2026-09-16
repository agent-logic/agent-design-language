#!/usr/bin/env python3
"""Run #905's bounded baseline/speculative comparison through Runtime v3.

The target and speculative Ollama model aliases must refer to the same resident
model bytes. Their only intentional difference is ``draft_num_predict``.  The
script starts an isolated Runtime v3, admits one agent for each alias, sends the
same deterministic prompt corpus through the Observatory conversation route,
and records only reply digests and timing/accounting metadata.
"""

from __future__ import annotations

import argparse
import hashlib
import http.server
import json
import os
from pathlib import Path
import plistlib
import re
import signal
import ssl
import statistics
import subprocess
import time
import urllib.request

import issue855_provider_lifecycle as lifecycle


PROMPTS = (
    ("Reply with exactly TOKENIZER-COMPATIBLE and nothing else.", "TOKENIZER-COMPATIBLE"),
    ("Reply with exactly FALLBACK-HEALTHY and nothing else.", "FALLBACK-HEALTHY"),
)


class OllamaProxy:
    """Force Runtime's supported chat-to-generate compatibility fallback."""

    def __init__(self) -> None:
        self.calls: list[dict] = []
        self.context: dict[str, object] = {}
        owner = self

        class Handler(http.server.BaseHTTPRequestHandler):
            def log_message(self, *_args: object) -> None:
                pass

            def send_json(self, status: int, body: dict) -> None:
                data = json.dumps(body).encode()
                self.send_response(status)
                self.send_header("Content-Type", "application/json")
                self.send_header("Content-Length", str(len(data)))
                self.end_headers()
                self.wfile.write(data)

            def do_GET(self) -> None:
                with urllib.request.urlopen("http://127.0.0.1:11434" + self.path, timeout=60) as response:
                    self.send_json(response.status, json.load(response))

            def do_POST(self) -> None:
                size = int(self.headers.get("Content-Length", "0"))
                require(0 < size <= 4_194_304, "invalid proxied request size")
                body = json.loads(self.rfile.read(size))
                if self.path == "/api/chat":
                    owner.calls.append({"path": self.path, "model": body.get("model"), "status": 400})
                    self.send_json(400, {"error": "issue905_force_generate_compatibility_path"})
                    return
                # Runtime's generic Ollama request predates the explicit thinking
                # switch.  Pin it at the bounded provider boundary so reasoning
                # tokens cannot consume the whole measured output allowance.
                body["think"] = False
                request = urllib.request.Request(
                    "http://127.0.0.1:11434" + self.path,
                    data=json.dumps(body).encode(),
                    headers={"Content-Type": "application/json"},
                )
                started = time.perf_counter()
                with urllib.request.urlopen(request, timeout=900) as response:
                    result = json.load(response)
                elapsed = time.perf_counter() - started
                owner.calls.append({
                    **owner.context,
                    "path": self.path,
                    "model": body.get("model"),
                    "status": 200,
                    "elapsed_seconds": elapsed,
                    "total_duration_ns": result.get("total_duration"),
                    "load_duration_ns": result.get("load_duration"),
                    "prompt_eval_count": result.get("prompt_eval_count"),
                    "prompt_eval_duration_ns": result.get("prompt_eval_duration"),
                    "eval_count": result.get("eval_count"),
                    "eval_duration_ns": result.get("eval_duration"),
                    "response_sha256": hashlib.sha256(result.get("response", "").encode()).hexdigest(),
                })
                self.send_json(response.status, result)

        self.server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        import threading
        threading.Thread(target=self.server.serve_forever, daemon=True).start()
        self.url = f"http://127.0.0.1:{self.server.server_port}"


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def ollama_json(path: str, body: dict | None = None) -> dict:
    request = urllib.request.Request(
        "http://127.0.0.1:11434" + path,
        data=None if body is None else json.dumps(body).encode(),
        headers={"Content-Type": "application/json"},
    )
    with urllib.request.urlopen(request, timeout=60) as response:
        return json.load(response)


def model_identity(name: str) -> dict:
    shown = ollama_json("/api/show", {"model": name})
    tags = ollama_json("/api/tags")["models"]
    tag = next(item for item in tags if item["name"].split(":", 1)[0] == name.split(":", 1)[0])
    mtp = [item["name"] for item in shown.get("tensors", []) if item["name"].startswith("mtp.")]
    parameters = {}
    for line in shown.get("parameters", "").splitlines():
        fields = line.split()
        if len(fields) >= 2:
            parameters[fields[0]] = " ".join(fields[1:])
    modelfile = shown.get("modelfile", "")
    base_match = re.search(r"^FROM .*/sha256-([0-9a-f]{64})$", modelfile, re.MULTILINE)
    model_info = shown.get("model_info", {})
    tokenizer_info = {
        key: value for key, value in model_info.items() if key.startswith("tokenizer.")
    }
    stable_model_info = json.dumps(model_info, sort_keys=True, separators=(",", ":")).encode()
    stable_tokenizer_info = json.dumps(tokenizer_info, sort_keys=True, separators=(",", ":")).encode()
    return {
        "name": name,
        "manifest_digest": tag["digest"],
        "base_blob_sha256": base_match.group(1) if base_match else None,
        "size_bytes": tag["size"],
        "details": shown.get("details"),
        "model_info_sha256": hashlib.sha256(stable_model_info).hexdigest(),
        "tokenizer_info_sha256": hashlib.sha256(stable_tokenizer_info).hexdigest(),
        "tokenizer_info_keys": len(tokenizer_info),
        "template_sha256": hashlib.sha256(shown.get("template", "").encode()).hexdigest(),
        "system_sha256": hashlib.sha256(shown.get("system", "").encode()).hexdigest(),
        "mtp_tensor_count": len(mtp),
        "parameters": parameters,
    }


def create_model(name: str, modelfile: Path, allow_failure: bool = False) -> tuple[int, str]:
    completed = subprocess.run(
        ["ollama", "create", name, "-f", str(modelfile)],
        capture_output=True,
        text=True,
        timeout=120,
    )
    stderr = completed.stderr[-2000:]
    if completed.returncode and not allow_failure:
        raise RuntimeError(f"ollama create failed ({completed.returncode}): {stderr[-500:]}")
    return completed.returncode, stderr


def run_json(argv: list[object], env: dict[str, str], allow_failure: bool = False) -> tuple[int, dict | None, str]:
    completed = subprocess.run(
        [str(item) for item in argv], capture_output=True, text=True, timeout=90, env=env
    )
    payload = None
    if completed.stdout.strip():
        try:
            payload = json.loads(completed.stdout)
        except json.JSONDecodeError:
            pass
    if completed.returncode and not allow_failure:
        raise RuntimeError(
            f"{Path(str(argv[0])).name} failed ({completed.returncode}): "
            + completed.stderr[-500:].replace("\n", " ")
        )
    return completed.returncode, payload, completed.stderr[-1000:]


def main() -> int:
    parser = argparse.ArgumentParser()
    for name in ("csm", "csmctl", "guardian", "kernel", "vector"):
        parser.add_argument("--" + name, type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--source-revision", required=True)
    parser.add_argument("--source-model", default="Qwen3.5:9b")
    parser.add_argument("--baseline-model", default="adl-905-arm-a:latest")
    parser.add_argument("--speculative-model", default="adl-905-arm-b:latest")
    parser.add_argument("--repeats", type=int, default=2)
    args = parser.parse_args()
    args.hosted_mode = False
    args.hosted_approved = False
    require(args.repeats > 0, "repeats must be positive")
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    root.chmod(0o700)
    for name in ("csm", "csmctl", "guardian", "kernel", "vector"):
        path = getattr(args, name).resolve()
        require(path.is_file(), f"missing {name} binary")
        setattr(args, name, path)

    model_dir = root / "models"
    model_dir.mkdir()
    common = (
        f"FROM {args.source_model}\nSYSTEM /no_think\n"
        "PARAMETER temperature 0\nPARAMETER seed 905\nPARAMETER num_predict 64\n"
    )
    baseline_modelfile = model_dir / "baseline.Modelfile"
    speculative_modelfile = model_dir / "speculative.Modelfile"
    invalid_modelfile = model_dir / "invalid-draft.Modelfile"
    baseline_modelfile.write_text(common + "PARAMETER draft_num_predict 0\n")
    speculative_modelfile.write_text(common + "PARAMETER draft_num_predict 4\n")
    invalid_modelfile.write_text(common + "PARAMETER draft_num_predict invalid\n")
    create_model(args.baseline_model, baseline_modelfile)
    create_model(args.speculative_model, speculative_modelfile)
    invalid_model = "adl-905-invalid-draft:latest"
    invalid_code, invalid_stderr = create_model(invalid_model, invalid_modelfile, True)
    require(invalid_code != 0, "invalid speculative draft configuration was accepted")

    baseline_identity = model_identity(args.baseline_model)
    speculative_identity = model_identity(args.speculative_model)
    require(baseline_identity["mtp_tensor_count"] > 0, "baseline model has no embedded MTP tensors")
    require(speculative_identity["mtp_tensor_count"] == baseline_identity["mtp_tensor_count"], "MTP tensor mismatch")
    require(baseline_identity["parameters"].get("draft_num_predict") == "0", "baseline drafting is not disabled")
    require(speculative_identity["parameters"].get("draft_num_predict") not in (None, "0"), "speculative drafting is not enabled")
    for field in (
        "base_blob_sha256", "details", "model_info_sha256", "tokenizer_info_sha256",
        "tokenizer_info_keys", "template_sha256", "system_sha256", "size_bytes",
    ):
        require(
            baseline_identity[field] == speculative_identity[field],
            f"baseline/speculative model identity differs at {field}",
        )
    baseline_parameters = dict(baseline_identity["parameters"])
    speculative_parameters = dict(speculative_identity["parameters"])
    baseline_parameters.pop("draft_num_predict")
    speculative_parameters.pop("draft_num_predict")
    require(
        baseline_parameters == speculative_parameters,
        "baseline/speculative parameters differ beyond draft_num_predict",
    )

    tls = lifecycle.certificates(root / "state/tls")
    fixture = lifecycle.Fixture(tls)
    proxy = OllamaProxy()
    clock = lifecycle.LocalTime()
    env = dict(os.environ)
    for key in list(env):
        if any(part in key for part in ("API_KEY", "ACCESS_TOKEN", "GOOGLE_APPLICATION_CREDENTIALS")):
            del env[key]
    env.update(ADL_PROVIDER_CA_FILE=str(tls["ca"]), ADL_PROVIDER_FIXTURE_TOKEN="issue905-local-fixture")
    install = root / "runtime-v3"
    lifecycle.run([
        Path(__file__).resolve().parents[1] / "tools/install_runtime_v3_generation.sh",
        "install", "--root", install, "--generation", "issue905-local",
        "--csm", args.csm, "--guardian", args.guardian, "--kernel", args.kernel,
        "--source-revision", args.source_revision, "--build-profile", "debug",
    ])
    lifecycle.run([Path(__file__).resolve().parents[1] / "tools/install_runtime_v3_generation.sh", "verify", "--root", install])
    import shutil
    ctl = root / "csmctl"
    shutil.copy2(args.csmctl, ctl)
    init, api_port, tokens = lifecycle.prepare_init(root, tls, args, clock, fixture)
    init.write_text(
        init.read_text().replace('model = "fixture-model"', f'model = {json.dumps(args.baseline_model)}', 1)
    )
    lifecycle.write(root / "providers.yaml", {
        "schema": "adl.provider_reload_sidecar.v1",
        "version": "0.5",
        "providers": {
            "ollama": {
                "type": "ollama",
                "default_model": args.baseline_model,
                "config": {
                    "endpoint": proxy.url,
                    "runtime_max_attempts": 1,
                    "runtime_max_output_tokens": 64,
                    "max_tokens": 64,
                    "max_output_tokens": 64,
                },
            }
        },
    })
    plist = root / "fixture.plist"
    label = "ai.agent-logic.issue905-" + str(os.getpid())
    plist.write_bytes(plistlib.dumps({"Label": label, "ProgramArguments": [str(install / "current/bin/adl-runtime-guardian"), "--init", str(init)]}))
    guardian_log = (root / "guardian.log").open("w")
    guardian = subprocess.Popen(
        [str(install / "current/bin/adl-runtime-guardian"), "--init", str(init)],
        stdout=guardian_log, stderr=subprocess.STDOUT, env=env, start_new_session=True,
    )
    ctx = ssl.create_default_context(cafile=str(tls["ca"]))
    report = {
        "schema": "adl.issue905.runtime_speculative_retest.v1",
        "source_revision": args.source_revision,
        "runtime_route": "Runtime v3 Observatory conversation -> provider registry -> Ollama /api/chat",
        "ollama_version": subprocess.run(["ollama", "--version"], capture_output=True, text=True).stdout.strip(),
        "hardware": {"system": os.uname().sysname, "machine": os.uname().machine},
        "models": {"baseline": baseline_identity, "speculative": speculative_identity},
        "model_identity_proof": {
            "same_base_blob": True,
            "same_model_info": True,
            "same_tokenizer_info": True,
            "only_parameter_difference": "draft_num_predict",
        },
        "sampling": {"temperature": 0, "seed": 905, "num_predict": 64},
        "provider_boundary": {"force_generate_fallback": True, "think": False},
        "prompts": len(PROMPTS),
        "repeats": args.repeats,
        "block_preloads": [],
        "runs": [],
        "result": "running",
    }
    try:
        deadline = time.monotonic() + 60
        snapshot = None
        while time.monotonic() < deadline:
            require(guardian.poll() is None, "Guardian exited before Runtime readiness")
            try:
                snapshot = lifecycle.api(ctx, api_port, tokens["observatory"], "/v1/observatory?schema=v3")
                if snapshot.get("runtime_incarnation_id"):
                    break
            except (OSError, ValueError):
                pass
            time.sleep(0.1)
        require(snapshot and snapshot.get("runtime_incarnation_id"), "Runtime readiness deadline")
        report["runtime_identity"] = lifecycle.identity(snapshot)

        def admit(agent_id: str, model: str, allow_failure: bool = False) -> tuple[int, dict | None, str]:
            agent_name = "ember." + agent_id.replace("-", "")
            config = {
                "schema": "adl.csm.agent_config.v1",
                "runtime": {"init": str(init)},
                "identity": {"id": agent_id, "name": agent_name, "display_name": agent_name},
                "office": "benchmark",
                "provider": {"kind": "ollama", "model": model, "required_capabilities": ["conversation"]},
            }
            path = lifecycle.write(root / f"{agent_id}.json", config)
            return run_json([ctl, "agent", "add", "--config", path], env, allow_failure)

        def await_agent(agent_id: str) -> dict:
            deadline = time.monotonic() + 60
            last = None
            while time.monotonic() < deadline:
                _, last, _ = run_json([ctl, "agent", "get", "--init", init, "--id", agent_id], env)
                if last and last.get("communication_eligible"):
                    return last
                time.sleep(0.2)
            raise AssertionError(f"agent did not become ready: {agent_id}: {last}")

        # Each repeat is a counterbalanced block. Prewarm the selected arm after
        # every alias switch, exclude that call, then measure the fixed corpus.
        # Even repeats are A/B and odd repeats B/A so cache/order effects do not
        # consistently favor either arm.
        for repeat in range(args.repeats):
            order = ("baseline", "speculative") if repeat % 2 == 0 else ("speculative", "baseline")
            for order_index, mode in enumerate(order):
                # Reuse the identical Runtime identity for both arms so agent
                # metadata cannot alter the provider prompt denominator.
                agent_id = "issue905-arm"
                model = args.baseline_model if mode == "baseline" else args.speculative_model
                code, payload, _ = admit(agent_id, model)
                require(code == 0 and payload and payload.get("status") == "admitted", f"{mode} admission failed")
                await_agent(agent_id)
                preload_started = time.perf_counter()
                preload = ollama_json("/api/generate", {
                    "model": model,
                    "prompt": PROMPTS[0][0],
                    "stream": False,
                    "think": False,
                })
                report["block_preloads"].append({
                    "mode": mode,
                    "repeat": repeat,
                    "elapsed_seconds": time.perf_counter() - preload_started,
                    "response_sha256": hashlib.sha256(preload.get("response", "").encode()).hexdigest(),
                })
                require(preload.get("response", "").strip() == PROMPTS[0][1], f"{mode} direct preload correctness marker mismatch")
                proxy.context = {"mode": mode, "repeat": repeat, "phase": "prewarm"}
                warmup = lifecycle.conversation(api_port, ctx, tokens["observatory"], agent_id, PROMPTS[0][0])
                require(warmup["reply"].strip() == PROMPTS[0][1], f"{mode} prewarm correctness marker mismatch")
                for prompt_index, (prompt, expected) in enumerate(PROMPTS):
                    proxy.context = {"mode": mode, "repeat": repeat, "phase": "measured", "prompt_index": prompt_index}
                    started = time.perf_counter()
                    result = lifecycle.conversation(api_port, ctx, tokens["observatory"], agent_id, prompt)
                    elapsed = time.perf_counter() - started
                    reply = result["reply"]
                    require(reply.strip() == expected, f"{mode} correctness marker mismatch")
                    report["runs"].append({
                        "mode": mode,
                        "repeat": repeat,
                        "order_index": order_index,
                        "prompt_index": prompt_index,
                        "elapsed_seconds": elapsed,
                        "reply_sha256": hashlib.sha256(reply.encode()).hexdigest(),
                        "reply_bytes": len(reply.encode()),
                    })
                run_json([ctl, "agent", "remove", "--init", init, "--id", agent_id], env)

        report["failure_probe"] = {
            "kind": "invalid_draft_configuration",
            "source_model": args.source_model,
            "draft_num_predict": "invalid",
            "exit_code": invalid_code,
            "stderr_sha256": hashlib.sha256(invalid_stderr.encode()).hexdigest(),
            "rejected_before_runtime_admission": True,
        }
        code, payload, _ = admit("issue905-fallback", args.baseline_model)
        require(code == 0 and payload and payload.get("status") == "admitted", "baseline fallback admission failed")
        await_agent("issue905-fallback")
        proxy.context = {"mode": "baseline", "phase": "fallback"}
        fallback = lifecycle.conversation(api_port, ctx, tokens["observatory"], "issue905-fallback", PROMPTS[0][0])
        require(fallback["reply"].strip() == PROMPTS[0][1], "fallback correctness marker mismatch")
        report["fallback"] = {
            "kind": "operator_selected_ordinary_generation_after_invalid_draft_rejection",
            "automatic": False,
            "status": fallback["status"],
            "reply_sha256": hashlib.sha256(fallback["reply"].encode()).hexdigest(),
        }

        baseline = {(r["repeat"], r["prompt_index"]): r for r in report["runs"] if r["mode"] == "baseline"}
        speculative = {(r["repeat"], r["prompt_index"]): r for r in report["runs"] if r["mode"] == "speculative"}
        require(baseline.keys() == speculative.keys(), "comparison grid mismatch")
        exact = sum(baseline[key]["reply_sha256"] == speculative[key]["reply_sha256"] for key in baseline)
        baseline_seconds = sum(item["elapsed_seconds"] for item in baseline.values())
        speculative_seconds = sum(item["elapsed_seconds"] for item in speculative.values())
        report["comparison"] = {
            "pairs": len(baseline),
            "exact_reply_pairs": exact,
            "output_equivalence": exact == len(baseline),
            "baseline_total_seconds": baseline_seconds,
            "speculative_total_seconds": speculative_seconds,
            "speedup_ratio": baseline_seconds / speculative_seconds,
            "benefit_percent": (baseline_seconds / speculative_seconds - 1.0) * 100.0,
            "measurement": "counterbalanced_prewarmed",
        }
        measured_calls = [
            item for item in proxy.calls
            if item.get("phase") == "measured" and item.get("path") == "/api/generate"
        ]
        decode_rates = {}
        for mode in ("baseline", "speculative"):
            calls = [item for item in measured_calls if item.get("mode") == mode]
            eval_count = sum(item.get("eval_count") or 0 for item in calls)
            eval_duration_ns = sum(item.get("eval_duration_ns") or 0 for item in calls)
            require(eval_count > 0 and eval_duration_ns > 0, f"missing {mode} decode accounting")
            decode_rates[mode] = eval_count / (eval_duration_ns / 1_000_000_000)
        report["comparison"]["decode_tokens_per_second"] = decode_rates
        blocks = []
        for repeat in range(args.repeats):
            block_runs = [item for item in report["runs"] if item["repeat"] == repeat]
            block_baseline = sum(item["elapsed_seconds"] for item in block_runs if item["mode"] == "baseline")
            block_speculative = sum(item["elapsed_seconds"] for item in block_runs if item["mode"] == "speculative")
            block_decode = {}
            for mode in ("baseline", "speculative"):
                calls = [
                    item for item in measured_calls
                    if item.get("repeat") == repeat and item.get("mode") == mode
                ]
                block_decode[mode] = sum(item.get("eval_count") or 0 for item in calls) / (
                    sum(item.get("eval_duration_ns") or 0 for item in calls) / 1_000_000_000
                )
            blocks.append({
                "repeat": repeat,
                "baseline_seconds": block_baseline,
                "speculative_seconds": block_speculative,
                "end_to_end_benefit_percent": (block_baseline / block_speculative - 1.0) * 100.0,
                "baseline_decode_tokens_per_second": block_decode["baseline"],
                "speculative_decode_tokens_per_second": block_decode["speculative"],
                "decode_benefit_percent": (block_decode["speculative"] / block_decode["baseline"] - 1.0) * 100.0,
            })
        end_benefits = [item["end_to_end_benefit_percent"] for item in blocks]
        decode_benefits = [item["decode_benefit_percent"] for item in blocks]
        required_wins = max(1, (3 * len(blocks) + 3) // 4)
        end_wins = sum(value > 0 for value in end_benefits)
        decode_wins = sum(value > 0 for value in decode_benefits)
        robust_keep = (
            end_wins >= required_wins
            and decode_wins >= required_wins
            and statistics.median(end_benefits) > 5.0
            and statistics.median(decode_benefits) > 0.0
        )
        robust_retire = (
            len(blocks) - end_wins >= required_wins
            and len(blocks) - decode_wins >= required_wins
            and statistics.median(end_benefits) < -5.0
            and statistics.median(decode_benefits) < 0.0
        )
        report["comparison"]["per_block"] = blocks
        report["comparison"]["robustness"] = {
            "required_wins": required_wins,
            "end_to_end_wins": end_wins,
            "decode_wins": decode_wins,
            "median_end_to_end_benefit_percent": statistics.median(end_benefits),
            "median_decode_benefit_percent": statistics.median(decode_benefits),
            "classification": "keep" if robust_keep else ("retire" if robust_retire else "repair_inconclusive"),
        }
        require(report["comparison"]["output_equivalence"], "speculative output differs from baseline")
        report["runtime_provider_calls"] = proxy.calls
        report["result"] = "pass"
    except Exception as error:
        report["result"] = "failed"
        report["error_class"] = type(error).__name__
        raise
    finally:
        report["runtime_provider_calls"] = proxy.calls
        lifecycle.write(root / "report.json", report)
        try:
            os.killpg(guardian.pid, signal.SIGTERM)
            guardian.wait(timeout=20)
        except (ProcessLookupError, subprocess.TimeoutExpired):
            try:
                os.killpg(guardian.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
        guardian_log.close()
        fixture.server.shutdown()
        fixture.resident_server.shutdown()
        proxy.server.shutdown()
        clock.sock.close()
        for model in (args.baseline_model, args.speculative_model, invalid_model):
            subprocess.run(["ollama", "rm", model], capture_output=True, text=True, timeout=60)
    print(json.dumps({"result": report["result"], "report": str(root / "report.json"), "comparison": report.get("comparison")}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
