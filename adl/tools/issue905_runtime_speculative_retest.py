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
import json
import os
from pathlib import Path
import plistlib
import signal
import ssl
import subprocess
import time
import urllib.request

import issue855_provider_lifecycle as lifecycle


PROMPTS = (
    "Explain in two sentences why tokenizer compatibility matters for speculative decoding.",
    "List three checks to perform after a provider benchmark fails.",
    "Write a one-sentence definition of deterministic fallback.",
)


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
    return {
        "name": name,
        "manifest_digest": tag["digest"],
        "size_bytes": tag["size"],
        "details": shown.get("details"),
        "mtp_tensor_count": len(mtp),
        "parameters": parameters,
    }


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
    parser.add_argument("--baseline-model", default="adl-905-baseline")
    parser.add_argument("--speculative-model", default="adl-905-speculative")
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

    baseline_identity = model_identity(args.baseline_model)
    speculative_identity = model_identity(args.speculative_model)
    require(baseline_identity["mtp_tensor_count"] > 0, "baseline model has no embedded MTP tensors")
    require(speculative_identity["mtp_tensor_count"] == baseline_identity["mtp_tensor_count"], "MTP tensor mismatch")
    require(baseline_identity["parameters"].get("draft_num_predict") == "0", "baseline drafting is not disabled")
    require(speculative_identity["parameters"].get("draft_num_predict") not in (None, "0"), "speculative drafting is not enabled")

    tls = lifecycle.certificates(root / "state/tls")
    fixture = lifecycle.Fixture(tls)
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
                    "endpoint": "http://127.0.0.1:11434",
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
        "sampling": {"temperature": 0, "seed": 905, "num_predict": 64},
        "prompts": len(PROMPTS),
        "repeats": args.repeats,
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

        for mode, model in (("baseline", args.baseline_model), ("speculative", args.speculative_model)):
            agent_id = "issue905-" + mode
            code, payload, _ = admit(agent_id, model)
            require(code == 0 and payload and payload.get("status") == "admitted", f"{mode} admission failed")
            await_agent(agent_id)
            for repeat in range(args.repeats):
                for prompt_index, prompt in enumerate(PROMPTS):
                    started = time.perf_counter()
                    result = lifecycle.conversation(api_port, ctx, tokens["observatory"], agent_id, prompt)
                    elapsed = time.perf_counter() - started
                    reply = result["reply"]
                    report["runs"].append({
                        "mode": mode,
                        "repeat": repeat,
                        "prompt_index": prompt_index,
                        "elapsed_seconds": elapsed,
                        "reply_sha256": hashlib.sha256(reply.encode()).hexdigest(),
                        "reply_bytes": len(reply.encode()),
                    })
            run_json([ctl, "agent", "remove", "--init", init, "--id", agent_id], env)

        bad_code, bad_payload, bad_stderr = admit("issue905-invalid-draft", "adl-905-missing-draft", True)
        report["failure_probe"] = {
            "model": "adl-905-missing-draft",
            "exit_code": bad_code,
            "payload": bad_payload,
            "stderr_sha256": hashlib.sha256(bad_stderr.encode()).hexdigest(),
        }
        require(bad_code != 0 or not (bad_payload or {}).get("communication_eligible"), "missing draft model was accepted as healthy")
        code, payload, _ = admit("issue905-fallback", args.baseline_model)
        require(code == 0 and payload and payload.get("status") == "admitted", "baseline fallback admission failed")
        await_agent("issue905-fallback")
        fallback = lifecycle.conversation(api_port, ctx, tokens["observatory"], "issue905-fallback", PROMPTS[0])
        report["fallback"] = {"status": fallback["status"], "reply_sha256": hashlib.sha256(fallback["reply"].encode()).hexdigest()}

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
        }
        require(report["comparison"]["output_equivalence"], "speculative output differs from baseline")
        report["result"] = "pass"
    except Exception as error:
        report["result"] = "failed"
        report["error_class"] = type(error).__name__
        raise
    finally:
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
        clock.sock.close()
    print(json.dumps({"result": report["result"], "report": str(root / "report.json"), "comparison": report.get("comparison")}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
