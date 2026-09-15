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
    vector = write(
        root / "vector-fixture.sh",
        "#!/bin/sh\ntrap 'exit 0' TERM INT\nwhile :; do sleep 1; done\n",
    )
    vector.chmod(0o700)
    tls = certificates(root / "tls")
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
        install = root / "runtime-v3"
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
        ctl = root / "csmctl"
        shutil.copy2(args.csmctl, ctl)
        setup = SimpleNamespace(
            vector=vector,
            source_revision=args.source_revision,
            hosted_mode=False,
            hosted_approved=False,
        )
        init, api_port, tokens = prepare_init(root, tls, setup, clock, fixture)
        providers = json.loads((root / "providers.yaml").read_text())
        openai = providers["providers"]["openai"]
        openai["config"]["auth"] = {
            "type": "bearer",
            "env": "ISSUE990_UNUSED_DIRECT_SECRET",
            "file_env": "ISSUE990_OPENAI_KEY_FILE",
        }
        providers["providers"] = {"openai": openai}
        write(root / "providers.yaml", providers)
        credential = write(root / "credential.txt", "issue990-local-fixture-value\n", True)
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
                    "WorkingDirectory": str(args.repo),
                    "EnvironmentVariables": {
                        "PATH": "/usr/bin:/bin:/usr/sbin:/sbin",
                        "TMPDIR": str(root / "tmp"),
                        "ADL_PROVIDER_CA_FILE": str(tls["ca"]),
                        "ISSUE990_OPENAI_KEY_FILE": str(credential),
                    },
                    "KeepAlive": True,
                    "RunAtLoad": False,
                    "StandardOutPath": str(root / "guardian.stdout.log"),
                    "StandardErrorPath": str(root / "guardian.stderr.log"),
                }
            )
        )
        (root / "tmp").mkdir(mode=0o700)
        csm_args.extend(["--init", init, "--plist", plist])
        stop_args.extend(["--init", init])
        env = {
            key: value
            for key, value in os.environ.items()
            if not any(part in key for part in ("API_KEY", "ACCESS_TOKEN", "GOOGLE_APPLICATION_CREDENTIALS"))
        }

        first_start = json.loads(run(csm_args, env=env))
        require(first_start["listener_ready"], "first managed start did not reach readiness")
        context = ssl.create_default_context(cafile=str(tls["ca"]))

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
                    "credential_ref": "env:ISSUE990_UNUSED_DIRECT_SECRET",
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
        report.update(
            result="pass",
            first_start_listener_ready=True,
            second_start_listener_ready=True,
            runtime_process_replaced=True,
            persisted_agent_ready_after_restart=second_agent["communication_eligible"],
            provider_requests_observed=len(fixture.calls),
            pre_restart_reply_delivered=first_reply["status"] == "delivered",
            post_restart_reply_delivered=second_reply["status"] == "delivered",
            provider_sidecar_sha256=sha256(root / "providers.yaml"),
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
        write(root / "report.json", report)
        fixture.server.shutdown()
        fixture.resident_server.shutdown()
        clock.sock.close()
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
