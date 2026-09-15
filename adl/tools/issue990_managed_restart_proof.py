#!/usr/bin/env python3
"""Run #990's bounded macOS launchd credential-survival proof."""

import argparse
import hashlib
import json
import os
import plistlib
import shutil
import ssl
import time
from pathlib import Path
from types import SimpleNamespace

from issue855_provider_lifecycle import (
    Fixture,
    LocalTime,
    api,
    certificates,
    conversation,
    prepare_init,
    require,
    run,
    write,
)


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def execute(args):
    root = args.output.resolve()
    root.mkdir(parents=True, exist_ok=False)
    root.chmod(0o700)
    service_root = Path.home() / "Library/Application Support/AgentLogic" / f"issue990-{os.getpid()}"
    service_root.mkdir(parents=True, mode=0o700, exist_ok=False)
    vector = write(
        service_root / "vector-fixture.sh",
        "#!/bin/sh\n"
        "if [ \"$1\" = --version ]; then echo 'vector 0.56.0 issue990'; exit 0; fi\n"
        "if [ \"$1\" = validate ]; then exit 0; fi\n"
        "while [ \"$#\" -gt 0 ]; do if [ \"$1\" = --config-json ]; then config=$2; break; fi; shift; done\n"
        "ingress=$(/usr/bin/awk '/\"include\": \\[/ {getline; gsub(/^[[:space:]]*\"|\",?$/, \"\"); print; exit}' \"$config\")\n"
        "master=$(/usr/bin/awk '/\"runtime_v3_master_log\":/ {sink=1} sink && /\"path\":/ {gsub(/^[^:]*:[[:space:]]*\"|\",?$/, \"\"); print; exit}' \"$config\")\n"
        "/bin/mkdir -p \"$(/usr/bin/dirname \"$master\")\"\n"
        "/bin/cat \"$ingress\" >> \"$master\"\n"
        "trap 'exit 0' TERM INT\n"
        "while :; do sleep 1; done\n",
    )
    vector.chmod(0o700)
    tls = certificates(service_root / "state/tls")
    fixture = Fixture(tls)
    clock = LocalTime()
    label = f"com.agentlogic.adl-runtime-v3.issue990.{os.getpid()}"
    installed_plist = Path.home() / "Library/LaunchAgents" / f"{label}.plist"
    report = {
        "schema": "adl.issue990.managed_restart_proof.v1",
        "result": "running",
        "service_manager": "launchd",
        "label": label,
        "paid_provider_calls": 0,
        "secret_values_retained": False,
        "credential_transport": "file_env_path_only",
        "managed_install_scope": "ephemeral_user_application_support",
    }
    csm_args = [
        args.csm,
        "runtime-v3",
        "start",
        "--label",
        label,
        "--json",
    ]
    stop_args = [
        args.csm,
        "runtime-v3",
        "stop",
        "--label",
        label,
        "--json",
    ]
    try:
        install = service_root / "runtime-v3"
        run(
            [
                args.repo / "adl/tools/install_runtime_v3_generation.sh",
                "install",
                "--root",
                install,
                "--generation",
                "issue990-local",
                "--csm",
                args.csm,
                "--guardian",
                args.guardian,
                "--kernel",
                args.kernel,
                "--source-revision",
                args.source_revision,
                "--build-profile",
                "debug",
            ]
        )
        run([args.repo / "adl/tools/install_runtime_v3_generation.sh", "verify", "--root", install])
        ctl = service_root / "csmctl"
        shutil.copy2(args.csmctl, ctl)
        setup = SimpleNamespace(
            vector=vector,
            source_revision=args.source_revision,
            hosted_mode=False,
            hosted_approved=False,
        )
        init, api_port, tokens = prepare_init(service_root, tls, setup, clock, fixture)
        providers = json.loads((service_root / "providers.yaml").read_text())
        openai = providers["providers"]["openai"]
        openai["config"]["auth"] = {
            "type": "bearer",
            "env": "ISSUE990_UNUSED_DIRECT_SECRET",
            "file_env": "ISSUE990_OPENAI_KEY_FILE",
        }
        write(service_root / "providers.yaml", providers)
        credential = write(
            service_root / "credential.txt", "issue990-local-fixture-value\n", True
        )
        fixture.expected_authorization = "Bearer issue990-local-fixture-value"
        write(service_root / "guardian.stdout.log", "")
        write(service_root / "guardian.stderr.log", "")
        plist = root / "service.plist"
        plist.write_bytes(
            plistlib.dumps(
                {
                    "Label": label,
                    "ProgramArguments": [
                        str(install / "current/bin/adl-runtime-guardian"),
                        "--init",
                        str(init),
                    ],
                    "WorkingDirectory": str(service_root),
                    "EnvironmentVariables": {
                        "HOME": str(Path.home()),
                        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
                        "TMPDIR": str(service_root / "tmp"),
                        "ADL_PROVIDER_CA_FILE": str(tls["ca"]),
                        "ISSUE990_OPENAI_KEY_FILE": str(credential),
                    },
                    "KeepAlive": True,
                    "RunAtLoad": False,
                    "StandardOutPath": str(service_root / "guardian.stdout.log"),
                    "StandardErrorPath": str(service_root / "guardian.stderr.log"),
                }
            )
        )
        (service_root / "tmp").mkdir(mode=0o700)
        csm_args.extend(["--init", init, "--plist", plist])
        stop_args.extend(["--init", init])
        env = {
            key: value
            for key, value in os.environ.items()
            if not any(part in key for part in ("API_KEY", "ACCESS_TOKEN", "GOOGLE_APPLICATION_CREDENTIALS"))
        }

        context = ssl.create_default_context(cafile=str(tls["ca"]))
        first_start = json.loads(run(csm_args, env=env))
        require(first_start["listener_ready"], "first managed start did not reach readiness")

        def csmctl(*argv):
            return json.loads(run([ctl, "agent", *argv], env=env))

        config = write(
            root / "openai-agent.json",
            {
                "schema": "adl.csm.agent_config.v1",
                "runtime": {"init": str(init)},
                "identity": {
                    "id": "issue990-openai",
                    "name": "ember.issue990",
                    "display_name": "ember.issue990",
                },
                "office": "assistant",
                "provider": {
                    "kind": "openai",
                    "model": "fixture-model",
                    "required_capabilities": ["conversation", "agent_to_agent"],
                },
            },
        )
        require(csmctl("add", "--config", config)["status"] == "admitted", "agent admission failed")
        deadline = time.monotonic() + 20
        while time.monotonic() < deadline:
            first_agent = csmctl("get", "--init", init, "--id", "issue990-openai")
            if first_agent.get("communication_eligible"):
                break
            time.sleep(0.1)
        require(first_agent.get("communication_eligible"), "first-start provider did not become ready")
        first_reply = conversation(
            api_port,
            context,
            tokens["observatory"],
            "issue990-openai",
            "Return one bounded fixture response before restart.",
        )
        first_snapshot = api(context, api_port, tokens["observatory"], "/v1/observatory?schema=v3")
        json.loads(run(stop_args, env=env))

        second_start = json.loads(run(csm_args, env=env))
        require(second_start["listener_ready"], "second managed start did not reach readiness")
        deadline = time.monotonic() + 20
        while time.monotonic() < deadline:
            second_agent = csmctl("get", "--init", init, "--id", "issue990-openai")
            if second_agent.get("communication_eligible"):
                break
            time.sleep(0.1)
        require(second_agent.get("communication_eligible"), "post-restart provider did not become ready")
        second_reply = conversation(
            api_port,
            context,
            tokens["observatory"],
            "issue990-openai",
            "Return one bounded fixture response after restart.",
        )
        second_snapshot = api(context, api_port, tokens["observatory"], "/v1/observatory?schema=v3")
        require(
            first_snapshot["runtime_process_id"] != second_snapshot["runtime_process_id"],
            "managed restart did not replace the Runtime process",
        )
        require(len(fixture.calls) >= 2, "provider fixture did not receive both requests")
        hosted_calls = [call for call in fixture.calls if call["path"].endswith("/responses")]
        require(
            len(hosted_calls) == 2
            and all(call["authorization_matches"] for call in hosted_calls),
            "file-based provider authorization was not observed before and after restart",
        )
        report.update(
            result="pass",
            first_start_listener_ready=True,
            second_start_listener_ready=True,
            runtime_process_replaced=True,
            persisted_agent_ready_after_restart=second_agent["communication_eligible"],
            provider_requests_observed=len(fixture.calls),
            pre_restart_reply_delivered=first_reply["status"] == "delivered",
            post_restart_reply_delivered=second_reply["status"] == "delivered",
            file_authorization_observed_pre_and_post_restart=True,
            provider_sidecar_sha256=sha256(service_root / "providers.yaml"),
            init_sha256=sha256(init),
            binary_sha256={
                "csm": sha256(args.csm),
                "csmctl": sha256(args.csmctl),
                "guardian": sha256(args.guardian),
                "kernel": sha256(args.kernel),
            },
            source_revision=args.source_revision,
        )
    except Exception as error:
        report.update(result="failed", error=str(error))
        raise
    finally:
        try:
            run(stop_args)
        except Exception:
            pass
        installed_plist.unlink(missing_ok=True)
        fixture.server.shutdown()
        fixture.resident_server.shutdown()
        clock.sock.close()
        shutil.rmtree(service_root, ignore_errors=True)
        report["managed_install_removed"] = not service_root.exists()
        write(root / "report.json", report)
    print(json.dumps({"result": report["result"], "report": str(root / "report.json")}))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--source-revision", required=True)
    for name in ("csm", "csmctl", "guardian", "kernel"):
        parser.add_argument(f"--{name}", type=Path, required=True)
    args = parser.parse_args()
    args.repo = args.repo.resolve()
    for name in ("csm", "csmctl", "guardian", "kernel"):
        path = getattr(args, name).resolve()
        require(path.is_file(), f"missing {name} binary")
        setattr(args, name, path)
    execute(args)


if __name__ == "__main__":
    main()
