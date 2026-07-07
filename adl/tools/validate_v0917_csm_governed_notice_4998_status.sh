#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROOF_DIR="${1:-$ROOT/docs/milestones/v0.91.7/review/runtime/csm_governed_notice_4998/live_eventbridge}"

python3 - "$PROOF_DIR" <<'PY'
import json
import pathlib
import re
import sys

proof = pathlib.Path(sys.argv[1])
summary_path = proof / "live_governed_notice_summary.json"
notice_path = proof / "state/csm_governed_notice_latest.json"
ledger_path = proof / "state/csm_governed_notices.jsonl"
safe_fail_path = proof / "state/safe_fail_bundle.json"
cloudwatch_path = proof / "logs/cloudwatch_governed_notice_events.redacted.json"
lambda_path = proof / "logs/lambda_receiver_events.redacted.json"
eventbridge_setup_path = proof / "aws_eventbridge_setup.redacted.json"

required = [
    summary_path,
    notice_path,
    ledger_path,
    safe_fail_path,
    cloudwatch_path,
    lambda_path,
    eventbridge_setup_path,
]
for path in required:
    if not path.exists():
        raise SystemExit(f"missing required proof artifact: {path}")

summary = json.loads(summary_path.read_text())
notice = json.loads(notice_path.read_text())
cloudwatch = json.loads(cloudwatch_path.read_text())
lambda_events = json.loads(lambda_path.read_text())
eventbridge_setup = json.loads(eventbridge_setup_path.read_text())

if summary.get("schema") != "adl.csm.governed_notice_live_proof.v3":
    raise SystemExit("unexpected summary schema")
if summary.get("status") != "passed":
    raise SystemExit("summary status must be passed")
if summary.get("runtime_owner") != "csm":
    raise SystemExit("runtime owner must be csm")
if not re.fullmatch(r"[0-9a-f]{16}", str(summary.get("aws_account_hash", ""))):
    raise SystemExit("aws_account_hash must be a 16-character redacted hash")

delivery = summary.get("delivery", {})
for channel in ["local_notice_ledger", "cloudwatch_logs", "acip_sns", "cloudfront_control_plane"]:
    if channel not in delivery or not isinstance(delivery[channel], dict):
        raise SystemExit(f"missing delivery channel: {channel}")

if delivery["local_notice_ledger"].get("status") != "recorded":
    raise SystemExit("local notice ledger must be recorded")
if delivery["cloudwatch_logs"].get("status") != "published_live":
    raise SystemExit("CloudWatch delivery must be live")
if delivery["acip_sns"].get("status") != "published_live":
    raise SystemExit("ACIP/SNS delivery must be live")
if not delivery["acip_sns"].get("provider_message_id"):
    raise SystemExit("ACIP/SNS provider message id must be retained")
control = delivery["cloudfront_control_plane"]
if control.get("status") != "published_live":
    raise SystemExit("control-plane delivery must be live")
if control.get("target_kind") != "eventbridge":
    raise SystemExit("control-plane target must be EventBridge")
if not control.get("provider_message_id"):
    raise SystemExit("control-plane provider message id must be retained")
if not re.fullmatch(r"[0-9a-f]{64}", str(control.get("target_sha256", ""))):
    raise SystemExit("control-plane target hash must be retained without raw target")

if eventbridge_setup.get("schema") != "adl.csm_notice_eventbridge_setup.v1":
    raise SystemExit("unexpected EventBridge setup schema")
if eventbridge_setup.get("status") != "configured":
    raise SystemExit("EventBridge setup must be configured")
if eventbridge_setup.get("routing") != "EventBridge source=adl.csm detail-type=CSM Governed Notice -> Lambda receiver":
    raise SystemExit("EventBridge setup must document the routed receiver path")

if notice.get("schema") != "adl.csm.governed_notice.v1":
    raise SystemExit("unexpected notice schema")
if notice.get("trigger") != "bounded_test_supervisor_failure":
    raise SystemExit("latest notice must prove bounded test supervisor failure")
if notice.get("notice_kind") != "shutdown":
    raise SystemExit("latest notice must be classified as shutdown")
if notice.get("severity") != "critical":
    raise SystemExit("latest notice severity must be critical")
policy = notice.get("local_first_policy", {})
if policy.get("source_of_truth") != "local_safe_fail_and_checkpoint_artifacts":
    raise SystemExit("local safe-fail artifacts must remain source of truth")
if policy.get("transport_failure_policy") != "retain_delivery_failure_and_continue_recovery":
    raise SystemExit("transport failure policy must preserve recovery")

ledger = ledger_path.read_text()
if '"trigger":"daemon_child_failed"' not in ledger:
    raise SystemExit("notice ledger must include daemon_child_failed degradation")
if '"trigger":"bounded_test_supervisor_failure"' not in ledger:
    raise SystemExit("notice ledger must include bounded_test_supervisor_failure shutdown")

notice_kinds = []
for event in cloudwatch.get("events", []):
    try:
        payload = json.loads(event.get("message", ""))
    except json.JSONDecodeError:
        continue
    if payload.get("signal_kind") == "csm_governed_notice":
        notice_kinds.append(payload.get("notice_kind"))
if {"degradation", "shutdown"} - set(notice_kinds):
    raise SystemExit("CloudWatch retained events must include degradation and shutdown notices")

observability = summary.get("observability", {})
if observability.get("primary_surface") != "CloudWatch Logs plus EventBridge routed receiver receipts":
    raise SystemExit("summary must name the primary observability surface")
if observability.get("cloudwatch_governed_notice_event_count", 0) < 2:
    raise SystemExit("summary must retain at least degradation and shutdown CloudWatch events")
if {"degradation", "shutdown"} - set(observability.get("cloudwatch_notice_kinds", [])):
    raise SystemExit("summary must classify degradation and shutdown notice kinds")
if observability.get("lambda_receiver_receipt_count", 0) < 1:
    raise SystemExit("EventBridge routed Lambda receiver receipt must be retained")
if observability.get("lambda_receiver_receipt_schema") != "adl.csm_notice_receiver.receipt.v1":
    raise SystemExit("receiver receipt schema must be retained")

receiver_receipts = [
    event.get("message", "")
    for event in lambda_events.get("events", [])
    if "adl.csm_notice_receiver.receipt.v1" in event.get("message", "")
]
if not receiver_receipts:
    raise SystemExit("EventBridge-routed Lambda receiver CloudWatch receipts must be retained")

combined = "\n".join(path.read_text(errors="replace") for path in required)
for pattern in [
    r"\b\d{12}\b",
    r"arn:aws:",
    r"AWS_SECRET",
    r"AWS_ACCESS",
    r"https://[^\s\"]+lambda-url",
    r"BEGIN .*KEY",
]:
    if re.search(pattern, combined):
        raise SystemExit(f"forbidden unredacted material matched: {pattern}")

print("PASS validate_v0917_csm_governed_notice_4998_status")
PY
