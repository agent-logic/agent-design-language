#!/usr/bin/env python3
"""Validate the v0.91.7 HTML Observatory integrated proof surface."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import textwrap
from pathlib import Path
from typing import Any


PACKET_REF = (
    "../../../docs/milestones/v0.91.7/review/runtime/soak2_4682/"
    "agent_lifecycle/runtime_v2/observatory/visibility_packet.json"
)
REPORT_REF = (
    "../../../docs/milestones/v0.91.7/review/runtime/soak2_4682/"
    "agent_lifecycle/runtime_v2/observatory/operator_report.md"
)
CSM_SERVICE_REF = "../../../docs/milestones/v0.91.7/review/runtime/csm_service_4903/service/service_manifest.json"
CSM_API_REF = "../../../docs/milestones/v0.91.7/review/runtime/CSM_RUNTIME_API_4929.md"
CLOUDWATCH_REF = "../../../docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684/live_heartbeat_summary.json"
CLOUDWATCH_EVENTS_REF = "../../../docs/milestones/v0.91.7/review/runtime/csm_liveness_4976/published/aws/cloudwatch_recent_events.redacted.json"


def fail(message: str) -> None:
    raise SystemExit(f"FAIL: {message}")


def read_json(path: Path) -> dict[str, Any]:
    with path.open("r", encoding="utf-8") as handle:
        value = json.load(handle)
    if not isinstance(value, dict):
        fail(f"{path} must contain a JSON object")
    return value


def assert_contains(label: str, haystack: str, needle: str) -> None:
    if needle not in haystack:
        fail(f"{label} missing {needle!r}")


def run_js_view_model(
    js_path: Path,
    packet_path: Path,
    report_path: Path,
    service_path: Path,
    api_path: Path,
    cloudwatch_path: Path,
    cloudwatch_events_path: Path,
) -> dict[str, Any]:
    node_program = textwrap.dedent(
        f"""
        const fs = require("fs");
        const vm = require("vm");
        const source = fs.readFileSync({json.dumps(str(js_path))}, "utf8");
        const packet = JSON.parse(fs.readFileSync({json.dumps(str(packet_path))}, "utf8"));
        const reportText = fs.readFileSync({json.dumps(str(report_path))}, "utf8");
        const serviceManifest = JSON.parse(fs.readFileSync({json.dumps(str(service_path))}, "utf8"));
        const apiText = fs.readFileSync({json.dumps(str(api_path))}, "utf8");
        const cloudwatchSummary = JSON.parse(fs.readFileSync({json.dumps(str(cloudwatch_path))}, "utf8"));
        const cloudwatchEvents = JSON.parse(fs.readFileSync({json.dumps(str(cloudwatch_events_path))}, "utf8"));
        const context = {{ console, URL, globalThis: {{}} }};
        context.globalThis = context;
        vm.runInNewContext(source, context);
        const viewModel = context.AdlHtmlObservatory.buildViewModel(packet, reportText);
        const integrationViewModel = context.AdlHtmlObservatory.buildIntegrationViewModel({{
          serviceManifest,
          apiText,
          cloudwatchSummary,
          cloudwatchEvents
        }});
        const operatorEnvelope = context.AdlHtmlObservatory.buildOperatorEnvelope({{
          channel: "events",
          message: "Request current CSM event tail and runtime readiness.",
          packetId: packet.packet_id
        }});
        process.stdout.write(JSON.stringify({{
          packetId: viewModel.packet.packet_id,
          evidenceLevel: viewModel.packet.source.evidence_level,
          manifoldState: viewModel.packet.manifold.state,
          citizenCount: viewModel.citizens.length,
          serviceCount: viewModel.services.length,
          decisionCounts: viewModel.decisionCounts,
          invariantCount: viewModel.invariants.length,
          latestEvent: viewModel.latestEvent,
          actionCount: viewModel.availableActions.length + viewModel.disabledActions.length,
          reportLoaded: viewModel.reportText.includes("CSM Observatory Operator Report"),
          serviceRows: integrationViewModel.serviceRows,
          cloudwatchRows: integrationViewModel.cloudwatchRows,
          parsedCloudWatchEventCount: integrationViewModel.parsedEvents.length,
          latestCloudWatchTarget: integrationViewModel.latestEvent.transport?.target_kind || "",
          awsLinkageCount: context.AdlHtmlObservatory.AWS_LINKAGES.length,
          openAwsLinkageCount: context.AdlHtmlObservatory.AWS_LINKAGES.filter((item) => item.state === "open").length,
          operatorEnvelope,
          loopbackPolicy: {{
            localHttp: context.AdlHtmlObservatory.isLoopbackApiBase("http://127.0.0.1:49210"),
            localhostHttp: context.AdlHtmlObservatory.isLoopbackApiBase("http://localhost:49210"),
            ipv6Http: context.AdlHtmlObservatory.isLoopbackApiBase("http://[::1]:49210"),
            remoteHttp: context.AdlHtmlObservatory.isLoopbackApiBase("https://example.com"),
            malformed: context.AdlHtmlObservatory.isLoopbackApiBase("not a url")
          }},
          closedAwsIssues: context.AdlHtmlObservatory.AWS_LINKAGES.filter((item) => item.state === "closed").map((item) => item.issue),
          openAwsIssues: context.AdlHtmlObservatory.AWS_LINKAGES.filter((item) => item.state === "open").map((item) => item.issue)
        }}));
        """
    )
    try:
        completed = subprocess.run(
            ["node", "-e", node_program],
            check=True,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
    except FileNotFoundError:
        fail("node is required for HTML Observatory JS validation")
    except subprocess.CalledProcessError as exc:
        fail(f"HTML Observatory JS validation failed: {exc.stderr.strip()}")
    return json.loads(completed.stdout)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--html", type=Path, required=True)
    parser.add_argument("--css", type=Path, required=True)
    parser.add_argument("--js", type=Path, required=True)
    parser.add_argument("--packet", type=Path, required=True)
    parser.add_argument("--report", type=Path, required=True)
    parser.add_argument("--csm-service", type=Path, required=True)
    parser.add_argument("--csm-api", type=Path, required=True)
    parser.add_argument("--cloudwatch", type=Path, required=True)
    parser.add_argument("--cloudwatch-events", type=Path, required=True)
    args = parser.parse_args()

    html = args.html.read_text(encoding="utf-8")
    css = args.css.read_text(encoding="utf-8")
    js = args.js.read_text(encoding="utf-8")
    packet = read_json(args.packet)
    report = args.report.read_text(encoding="utf-8")
    service = read_json(args.csm_service)
    api_text = args.csm_api.read_text(encoding="utf-8")
    cloudwatch = read_json(args.cloudwatch)
    cloudwatch_events = read_json(args.cloudwatch_events)
    smoke = run_js_view_model(
        args.js,
        args.packet,
        args.report,
        args.csm_service,
        args.csm_api,
        args.cloudwatch,
        args.cloudwatch_events,
    )

    assert_contains("HTML packet ref", html, f'data-packet-ref="{PACKET_REF}"')
    assert_contains("HTML report ref", html, f'data-report-ref="{REPORT_REF}"')
    assert_contains("HTML CSM service ref", html, f'data-csm-service-ref="{CSM_SERVICE_REF}"')
    assert_contains("HTML CSM API ref", html, f'data-csm-api-ref="{CSM_API_REF}"')
    assert_contains("HTML CloudWatch ref", html, f'data-cloudwatch-ref="{CLOUDWATCH_REF}"')
    assert_contains("HTML CloudWatch events ref", html, f'data-cloudwatch-events-ref="{CLOUDWATCH_EVENTS_REF}"')
    assert_contains("HTML title", html, "ADL HTML Observatory - Runtime Proof")
    assert_contains("HTML integrated proof copy", html, "HTML Observatory integrated proof")
    assert_contains("HTML CSM API section", html, "CSM local control plane")
    assert_contains("HTML CloudWatch section", html, "CloudWatch heartbeat")
    assert_contains("HTML AWS linkages section", html, "AWS runtime linkages")
    assert_contains("HTML communication section", html, "CSM event channel")
    assert_contains("HTML communication input", html, 'id="runtime-api-base"')
    assert_contains("HTML governance section", html, "Freedom gate")
    assert_contains("HTML evidence section", html, "Same packet, same report, same boundary.")
    assert_contains("CSS responsive layout", css, "@media (max-width: 980px)")
    assert_contains("CSS orbit visualization", css, ".orbit-map")
    assert_contains("CSS Magic UI inspired card styling", css, ".proof-card")
    assert_contains("JS packet loader", js, "loadJson(packetRef)")
    assert_contains("JS report loader", js, "loadText(reportRef)")
    assert_contains("JS view model", js, "buildViewModel")
    assert_contains("JS CSM integration view model", js, "buildIntegrationViewModel")
    assert_contains("JS AWS linkage state", js, "AWS_LINKAGES")
    assert_contains("JS communication envelope", js, "buildOperatorEnvelope")
    assert_contains("JS events endpoint check", js, "checkEventsEndpoint")
    assert_contains("JS loopback API policy", js, "isLoopbackApiBase")

    if packet.get("packet_id") != "v0916-runtime-soak-observatory-packet-0001":
      fail("unexpected runtime packet id")
    if packet.get("source", {}).get("evidence_level") != "bounded_local_runtime_capture":
      fail("runtime packet is not the retained bounded local runtime capture")
    if "CSM Observatory Operator Report" not in report:
      fail("operator report identity missing")
    if service.get("schema") != "adl.csm.service_manifest.v1":
      fail("CSM service manifest schema mismatch")
    if service.get("runtime_owner") != "csm":
      fail("CSM service manifest does not record csm runtime ownership")
    for endpoint in ("csm api serve --spec <agent-spec.yaml>", "/status", "/health", "/ready", "/metrics", "/events"):
      if endpoint not in api_text:
        fail(f"CSM API proof missing {endpoint}")
    if cloudwatch.get("schema") != "adl.wp08.heartbeat_live_proof.v1":
      fail("CloudWatch heartbeat proof schema mismatch")
    if cloudwatch.get("status") != "passed":
      fail("CloudWatch heartbeat proof did not pass")
    if cloudwatch.get("cloudwatch", {}).get("event_count", 0) < 1:
      fail("CloudWatch heartbeat proof has no retained events")
    if cloudwatch.get("heartbeat", {}).get("target_kind") != "cloudwatch_logs":
      fail("CloudWatch heartbeat target is not cloudwatch_logs")
    redaction = cloudwatch.get("redaction", {})
    if redaction.get("credentials_recorded") is not False or redaction.get("raw_account_id_recorded") is not False:
      fail("CloudWatch proof redaction posture is not operations safe")
    if len(cloudwatch_events.get("events", [])) < 1:
      fail("CloudWatch event tail is empty")
    if smoke["packetId"] != packet["packet_id"]:
      fail("JS view model did not consume the retained packet")
    if smoke["evidenceLevel"] != "bounded_local_runtime_capture":
      fail("JS view model evidence level mismatch")
    if smoke["manifoldState"] != packet["manifold"]["state"]:
      fail("JS view model manifold state mismatch")
    if smoke["citizenCount"] < 3:
      fail("expected three runtime lanes in HTML Observatory view model")
    if smoke["serviceCount"] < 4:
      fail("expected runtime services in HTML Observatory view model")
    if smoke["decisionCounts"] != {"allow": 1, "defer": 1, "refuse": 1}:
      fail(f"unexpected decision counts: {smoke['decisionCounts']!r}")
    if smoke["invariantCount"] < 3:
      fail("expected retained invariants in HTML Observatory view model")
    if smoke["latestEvent"] < 5:
      fail("expected retained trace tail through event 5")
    if smoke["actionCount"] < 5:
      fail("expected available and disabled operator actions")
    if not smoke["reportLoaded"]:
      fail("JS view model did not receive the operator report text")
    if len(smoke["serviceRows"]) < 3:
      fail("JS integration view model did not build CSM service rows")
    if len(smoke["cloudwatchRows"]) < 3:
      fail("JS integration view model did not build CloudWatch rows")
    if smoke["parsedCloudWatchEventCount"] < 1:
      fail("JS integration view model did not parse CloudWatch events")
    if smoke["latestCloudWatchTarget"] != "cloudwatch_logs":
      fail("latest CloudWatch event is not a cloudwatch_logs signal")
    if smoke["awsLinkageCount"] != 5 or smoke["openAwsLinkageCount"] != 2:
      fail("AWS linkage lane did not preserve open WP-08 work truth")
    if smoke["closedAwsIssues"] != [4684, 4685, 4687]:
      fail(f"closed AWS linkage issues mismatch: {smoke['closedAwsIssues']!r}")
    if smoke["openAwsIssues"] != [4686, 4688]:
      fail(f"open AWS linkage issues mismatch: {smoke['openAwsIssues']!r}")
    envelope = smoke["operatorEnvelope"]
    if envelope.get("schema") != "adl.html_observatory.operator_message.v1":
      fail("operator communication envelope schema mismatch")
    if envelope.get("runtime_mutation_claimed") is not False:
      fail("operator communication envelope overclaims runtime mutation")
    if envelope.get("allowed_live_check") != "/events":
      fail("operator communication envelope does not route to /events")
    loopback_policy = smoke["loopbackPolicy"]
    if not all(loopback_policy[key] for key in ("localHttp", "localhostHttp", "ipv6Http")):
      fail(f"loopback CSM API bases were not accepted: {loopback_policy!r}")
    if loopback_policy["remoteHttp"] or loopback_policy["malformed"]:
      fail(f"non-loopback or malformed API base was accepted: {loopback_policy!r}")

    secret_pattern = re.compile(
        r"/Users/|/private/var/|localhost:[0-9]|192\\.168\\.|"
        r"bearer\\s+[A-Za-z0-9._-]{8,}|"
        r"(api[_-]?key|secret|token)\\s*[:=]\\s*[A-Za-z0-9._-]{8,}",
        re.IGNORECASE,
    )
    for label, content in {"html": html, "css": css, "js": js}.items():
      if secret_pattern.search(content):
        fail(f"{label} contains private path, endpoint, or secret-like text")

    print("PASS: v0.91.7 HTML Observatory integrated proof validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
