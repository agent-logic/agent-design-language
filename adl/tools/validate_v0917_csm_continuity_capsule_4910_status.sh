#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PROOF="${1:-$ROOT/docs/milestones/v0.91.7/review/runtime/csm_continuity_capsule_4910}"

python3 - "$PROOF" <<'PY'
import hashlib
import json
import os
import pathlib
import base64
import subprocess
import sys
import tempfile
import textwrap

proof = pathlib.Path(sys.argv[1])
required = [
    "README.md",
    "proof_summary.json",
    "negative_results.json",
    "logs/daemon_stdout.json",
    "logs/capture_stdout.json",
    "logs/stage_stdout.json",
    "logs/ec2_stage_stdout.json",
    "logs/restore_stdout.json",
    "logs/restored_daemon_stdout.json",
    "logs/observability.log",
    "logs/otel.jsonl",
    "logs/otel_status.json",
    "capsule/continuity_capsule_manifest.json",
    "capsule/custody_manifest.json",
    "capsule/state/agent_spec.locked.json",
    "capsule/state/status.json",
    "capsule/state/daemon_status.json",
    "capsule/state/continuity.json",
    "capsule/state/continuity_checkpoint.json",
    "capsule/state/continuity_replay_manifest.json",
    "capsule/state/cycle_ledger.jsonl",
    "capsule/state/memory_index.json",
    "capsule/state/provider_binding_history.jsonl",
    "capsule/state/operator_events.jsonl",
    "ec2_staged/stage_report.json",
    "ec2_blocked/stage_report.json",
    "ec2_restored/restore_report.json",
    "ec2_restored/agent.yaml",
    "ec2_restored/state/continuity_checkpoint.json",
]
missing = [path for path in required if not (proof / path).exists()]
if missing:
    raise SystemExit(f"missing proof artifacts: {missing}")

summary = json.loads((proof / "proof_summary.json").read_text(encoding="utf-8"))
if summary.get("schema") != "adl.csm.continuity_capsule_4910_proof_summary.v1":
    raise SystemExit("proof summary schema drift")
if summary.get("runtime_owner") != "csm":
    raise SystemExit("runtime owner must be csm")
if summary.get("command_surface") != "csm continuity capture|stage|restore":
    raise SystemExit("command surface drift")
allowed_classifications = {
    "proving_with_restore_fire_up_and_blocked_live_ec2_transfer",
    "proving_with_local_restore_fire_up_and_aws_ec2_restore_fire_up",
}
if summary.get("proof_classification") not in allowed_classifications:
    raise SystemExit("proof classification drift")
if summary.get("live_ec2_status") != "blocked":
    raise SystemExit("live EC2 transfer must remain explicitly blocked")
if summary.get("restore_status") != "restored":
    raise SystemExit("restore status must be restored")
if summary.get("restored_daemon_fire_up") != "passed":
    raise SystemExit("restored daemon fire-up must pass")
if summary.get("aws_profile_policy") != "agent-logic-admin":
    raise SystemExit("AWS profile policy must name agent-logic-admin")

aws_fire_up = summary.get("aws_ec2_restore_fire_up")
readme_text = (proof / "README.md").read_text(encoding="utf-8")
aws_summary_exists = (proof / "aws_remote_restore_fireup_summary.json").exists()
aws_claimed = "aws_remote_restore_fireup_summary.json" in readme_text or summary.get("proof_classification") == "proving_with_local_restore_fire_up_and_aws_ec2_restore_fire_up"
if (aws_summary_exists or aws_claimed) and aws_fire_up is None:
    raise SystemExit("AWS EC2 restore/fire-up proof is claimed or retained but missing from proof summary")
if aws_fire_up is not None:
    if aws_fire_up.get("status") != "passed":
        raise SystemExit("AWS EC2 restore/fire-up status must pass")
    summary_ref = aws_fire_up.get("summary_ref")
    if summary_ref != "aws_remote_restore_fireup_summary.json":
        raise SystemExit("AWS EC2 restore/fire-up summary ref drift")
    aws_summary_path = proof / summary_ref
    if not aws_summary_path.exists():
        raise SystemExit("AWS EC2 restore/fire-up summary missing")
    aws_summary_text = aws_summary_path.read_text(encoding="utf-8")
    for forbidden in ['"account_id"', '"arn"', '"user_id"', '"instance_id"', '"volume_id"', '"vpc_id"', '"subnet_id"', '"security_group_id"', '"role_name"', '"instance_profile_name"', '"security_group_name"', '"command_id"', '"ami_id"', '"ssh_allowed_cidr"', "arn:", "ADLAwsRemoteValidation"]:
        if forbidden in aws_summary_text:
            raise SystemExit(f"AWS retained summary contains unsanitized field: {forbidden}")
    aws_summary = json.loads(aws_summary_text)
    if aws_summary.get("schema") != "adl.csm.continuity_capsule_4910_aws_remote_restore_fireup.v1":
        raise SystemExit("AWS retained summary schema drift")
    if aws_summary.get("status") != "passed":
        raise SystemExit("AWS retained summary must pass")
    if aws_summary.get("command_status") != "Success" or aws_summary.get("command_response_code") != 0:
        raise SystemExit("AWS retained command status must be successful")
    validation = aws_summary.get("validation", {})
    if validation.get("validator_pass_count") != 2:
        raise SystemExit("AWS retained summary must record both validator passes")
    aws_surface = aws_summary.get("aws_surface", {})
    cleanup = aws_surface.get("cleanup", {})
    launch_cleanup = aws_surface.get("launch_surface_cleanup", {})
    if cleanup.get("final_instance_state") != "terminated":
        raise SystemExit("AWS retained cleanup must terminate instance")
    if launch_cleanup.get("security_group_deleted") is not True:
        raise SystemExit("AWS retained cleanup must delete temporary security group")
    if launch_cleanup.get("instance_profile_deleted") is not True or launch_cleanup.get("role_deleted") is not True:
        raise SystemExit("AWS retained cleanup must delete temporary IAM surface")

manifest = json.loads((proof / "capsule" / "continuity_capsule_manifest.json").read_text(encoding="utf-8"))
if manifest.get("schema") != "adl.csm.continuity_capsule.v1":
    raise SystemExit("manifest schema drift")
if manifest.get("format_version") != "csm.continuity-capsule.v1":
    raise SystemExit("format version drift")
if manifest.get("runtime_owner") != "csm":
    raise SystemExit("manifest runtime_owner drift")
if manifest.get("source_host") != "wuji":
    raise SystemExit("source host must remain wuji")
if manifest.get("target_host") != "ec2-staging":
    raise SystemExit("target host must remain ec2-staging")
if manifest.get("rebind_policy", {}).get("aws", {}).get("default_profile") != "agent-logic-admin":
    raise SystemExit("manifest AWS rebind policy drift")
if manifest.get("rebind_policy", {}).get("provider_auth") != "excluded_from_bundle_rebind_from_target_provider_environment":
    raise SystemExit("provider auth exclusion drift")
if manifest.get("custody_manifest_ref") != "custody_manifest.json":
    raise SystemExit("custody manifest ref drift")

artifacts = manifest.get("artifacts")
if not isinstance(artifacts, list) or len(artifacts) < 10:
    raise SystemExit("manifest must retain runtime artifacts")
roles = {artifact.get("role") for artifact in artifacts}
for role in [
    "runtime_identity",
    "recoverable_status",
    "daemon_status",
    "continuity",
    "continuity_checkpoint",
    "continuity_replay_manifest",
    "dag_run_ledger",
    "memory_index",
    "provider_binding_history",
    "observability_tail",
]:
    if role not in roles:
        raise SystemExit(f"missing retained role: {role}")

for artifact in artifacts:
    bundle_ref = artifact.get("bundle_ref")
    source_ref = artifact.get("source_ref")
    if not isinstance(bundle_ref, str) or bundle_ref.startswith("/") or ".." in pathlib.PurePosixPath(bundle_ref).parts:
        raise SystemExit(f"unsafe bundle_ref: {bundle_ref}")
    if not isinstance(source_ref, str) or source_ref.startswith("/") or ".." in pathlib.PurePosixPath(source_ref).parts:
        raise SystemExit(f"unsafe source_ref: {source_ref}")
    path = proof / "capsule" / bundle_ref
    if not path.exists():
        raise SystemExit(f"artifact missing from capsule: {bundle_ref}")
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    if digest != artifact.get("sha256"):
        raise SystemExit(f"artifact hash mismatch: {bundle_ref}")

custody = json.loads((proof / "capsule" / "custody_manifest.json").read_text(encoding="utf-8"))
if custody.get("schema") != "adl.csm.polis_artifact_custody_manifest.v1":
    raise SystemExit("custody manifest schema drift")
if custody.get("format_version") != "csm.polis-custody.v1":
    raise SystemExit("custody manifest format drift")
if custody.get("runtime_owner") != "csm":
    raise SystemExit("custody manifest runtime_owner drift")
if custody.get("capsule_manifest_ref") != "continuity_capsule_manifest.json":
    raise SystemExit("custody capsule manifest ref drift")
signature = custody.get("signature") or {}
if signature.get("schema") != "adl.csm.polis_artifact_custody_signature.v1":
    raise SystemExit("custody signature schema drift")
if signature.get("alg") != "ecdsa-p256-sha256":
    raise SystemExit("custody signature algorithm drift")
if not signature.get("key_id"):
    raise SystemExit("custody signature key id missing")
if not isinstance(signature.get("public_key_b64"), str) or len(signature["public_key_b64"]) < 40:
    raise SystemExit("custody signature public key missing")
if not isinstance(signature.get("sig_b64"), str) or len(signature["sig_b64"]) < 80:
    raise SystemExit("custody signature bytes missing")
signed_payload = signature.get("signed_payload") or {}
if signed_payload.get("canonical_json_profile") != "adl.csm.polis_custody.canonical_json.sorted_serde_json.v1":
    raise SystemExit("custody canonical JSON profile drift")
if signed_payload.get("excluded_fields") != ["signature"]:
    raise SystemExit("custody signature excluded fields drift")
if not isinstance(signed_payload.get("payload_sha256"), str) or not signed_payload["payload_sha256"].startswith("sha256:"):
    raise SystemExit("custody signed payload digest missing")
unsigned_custody = dict(custody)
unsigned_custody["signature"] = None
canonical_custody = json.dumps(unsigned_custody, sort_keys=True, separators=(",", ":")).encode("utf-8")
expected_payload_sha256 = "sha256:" + hashlib.sha256(canonical_custody).hexdigest()
if signed_payload["payload_sha256"] != expected_payload_sha256:
    raise SystemExit("custody signed payload digest mismatch")
retained_proof_trusted_keys = {
    "csm-continuity-4910-proof-key": "BHE1+k/ZOgnc6Yu/aBtL/PUOfA1jVOYq+wv/KjQpYXhl7UwfAt25Aj7lalV+UV1qncZsEfIglg3llDNN9Yh3ZyQ=",
}
trusted_public_key = (
    os.environ.get("ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64")
    or os.environ.get("CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64")
    or retained_proof_trusted_keys.get(signature.get("key_id"))
)
if not trusted_public_key:
    raise SystemExit("trusted custody public key env missing")
if signature.get("public_key_b64") != trusted_public_key:
    raise SystemExit("custody signature public key does not match trusted verification key")
try:
    public_key_bytes = base64.b64decode(trusted_public_key, validate=True)
    signature_bytes = base64.b64decode(signature["sig_b64"], validate=True)
except Exception as exc:
    raise SystemExit(f"custody signature base64 decode failed: {exc}")
if len(public_key_bytes) != 65 or public_key_bytes[0] != 4:
    raise SystemExit("trusted custody public key must be uncompressed P-256 SEC1 bytes")
spki_prefix = bytes.fromhex("3059301306072a8648ce3d020106082a8648ce3d030107034200")
public_key_der = spki_prefix + public_key_bytes
public_key_pem = (
    "-----BEGIN PUBLIC KEY-----\n"
    + "\n".join(textwrap.wrap(base64.b64encode(public_key_der).decode("ascii"), 64))
    + "\n-----END PUBLIC KEY-----\n"
)
with tempfile.TemporaryDirectory() as td:
    td_path = pathlib.Path(td)
    pub_path = td_path / "custody_pub.pem"
    sig_path = td_path / "custody_sig.der"
    payload_path = td_path / "custody_payload.json"
    pub_path.write_text(public_key_pem, encoding="ascii")
    sig_path.write_bytes(signature_bytes)
    payload_path.write_bytes(canonical_custody)
    verify = subprocess.run(
        [
            "openssl",
            "dgst",
            "-sha256",
            "-verify",
            str(pub_path),
            "-signature",
            str(sig_path),
            str(payload_path),
        ],
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
if verify.returncode != 0:
    raise SystemExit(f"custody ECDSA signature verification failed: {verify.stderr or verify.stdout}")
custody_artifacts = custody.get("artifacts")
if not isinstance(custody_artifacts, list) or len(custody_artifacts) < len(artifacts) + 1:
    raise SystemExit("custody artifacts incomplete")
custody_by_id = {entry.get("artifact_id"): entry for entry in custody_artifacts}
if "continuity-capsule-manifest" not in custody_by_id:
    raise SystemExit("custody root artifact missing")
for artifact in artifacts:
    artifact_id = f"artifact:{artifact.get('role')}:{artifact.get('source_ref')}"
    custody_artifact = custody_by_id.get(artifact_id)
    if custody_artifact is None:
        raise SystemExit(f"custody artifact missing: {artifact_id}")
    if custody_artifact.get("sha256") != artifact.get("sha256"):
        raise SystemExit(f"custody artifact hash mismatch: {artifact_id}")
    if custody_artifact.get("storage_location") != artifact.get("bundle_ref"):
        raise SystemExit(f"custody artifact storage drift: {artifact_id}")
    redaction = custody_artifact.get("redaction") or {}
    if redaction.get("sensitive_material_present") is not False or redaction.get("contains_host_private_paths") is not False:
        raise SystemExit(f"custody redaction posture drift: {artifact_id}")
    if redaction.get("redacted_fields") != []:
        raise SystemExit(f"custody redacted fields must be empty: {artifact_id}")

stage_report = json.loads((proof / "ec2_staged" / "stage_report.json").read_text(encoding="utf-8"))
if stage_report.get("schema") != "adl.csm.continuity_capsule_stage_report.v1":
    raise SystemExit("stage report schema drift")
if stage_report.get("status") != "staged":
    raise SystemExit("ec2-staging report must be staged")
if not (proof / "ec2_staged" / "staged_state" / "continuity_checkpoint.json").exists():
    raise SystemExit("staged continuity checkpoint missing")

ec2_report = json.loads((proof / "ec2_blocked" / "stage_report.json").read_text(encoding="utf-8"))
if ec2_report.get("status") != "blocked":
    raise SystemExit("live EC2 stage must be blocked")
if ec2_report.get("rebind_policy", {}).get("target_host") != "ec2":
    raise SystemExit("live EC2 rebind policy target host drift")
if ec2_report.get("blockers", [{}])[0].get("required_profile") != "agent-logic-admin":
    raise SystemExit("live EC2 blocker must require agent-logic-admin")

restore_report = json.loads((proof / "ec2_restored" / "restore_report.json").read_text(encoding="utf-8"))
if restore_report.get("schema") != "adl.csm.continuity_capsule_restore_report.v1":
    raise SystemExit("restore report schema drift")
if restore_report.get("status") != "restored":
    raise SystemExit("restore report must be restored")
if restore_report.get("runtime_owner") != "csm":
    raise SystemExit("restore report runtime_owner drift")
restore_observability = restore_report.get("observability", {})
if "continuity_capsule_restore" not in restore_observability.get("event_stages", []):
    raise SystemExit("restore report must retain restore observability event stage")

negative = json.loads((proof / "negative_results.json").read_text(encoding="utf-8"))
if negative.get("schema") != "adl.csm.continuity_capsule_4910_negative_results.v1":
    raise SystemExit("negative result schema drift")
expected_cases = {
    "version_mismatch",
    "missing_file",
    "missing_custody_manifest",
    "custody_signature_tamper",
    "custody_untrusted_public_key",
    "path_leakage",
    "credential_leakage",
    "corrupted_manifest",
    "unsupported_target_host",
}
records = {case.get("name"): case for case in negative.get("cases", [])}
if set(records) != expected_cases:
    raise SystemExit(f"negative cases drift: {sorted(records)}")
for name, record in records.items():
    if record.get("status") != "failed_as_expected" or record.get("returncode") == 0 or record.get("stderr_matched") is not True:
        raise SystemExit(f"negative case did not fail as expected: {name}")

capture_stdout = json.loads((proof / "logs" / "capture_stdout.json").read_text(encoding="utf-8"))
stage_stdout = json.loads((proof / "logs" / "stage_stdout.json").read_text(encoding="utf-8"))
restore_stdout = json.loads((proof / "logs" / "restore_stdout.json").read_text(encoding="utf-8"))
restored_daemon_stdout = json.loads((proof / "logs" / "restored_daemon_stdout.json").read_text(encoding="utf-8"))
if capture_stdout.get("schema") != "adl.csm.continuity_capsule_command_result.v1":
    raise SystemExit("capture stdout schema drift")
if capture_stdout.get("operation") != "capture" or capture_stdout.get("status") != "captured":
    raise SystemExit("capture stdout status drift")
if stage_stdout.get("operation") != "stage" or stage_stdout.get("status") != "staged":
    raise SystemExit("stage stdout status drift")
if restore_stdout.get("operation") != "restore" or restore_stdout.get("status") != "restored":
    raise SystemExit("restore stdout status drift")
if restored_daemon_stdout.get("schema") != "adl.long_lived_agent_daemon_status.v1":
    raise SystemExit("restored daemon stdout schema drift")
if restored_daemon_stdout.get("state") != "completed":
    raise SystemExit("restored daemon must complete")

observability = (proof / "logs" / "observability.log").read_text(encoding="utf-8")
for marker in [
    "command=csm",
    "stage=daemon_started",
    "stage=checkpoint_write",
    "stage=continuity_capsule_capture",
    "stage=continuity_capsule_stage",
    "stage=continuity_capsule_restore",
    "otel_service_name=csm-runtime-daemon",
]:
    if marker not in observability:
        raise SystemExit(f"missing observability marker: {marker}")
otel_status = json.loads((proof / "logs" / "otel_status.json").read_text(encoding="utf-8"))
if otel_status.get("schema") != "adl.otel.monitor_status.v1":
    raise SystemExit("otel status schema drift")
otel_events = (proof / "logs" / "otel.jsonl").read_text(encoding="utf-8")
if (
    "csm.continuity_capsule_capture" not in otel_events
    or "csm.continuity_capsule_stage" not in otel_events
    or "csm.continuity_capsule_restore" not in otel_events
):
    raise SystemExit("missing continuity capsule OTel events")

bad_markers = ["/Users/", "/private/tmp/", "/var/folders/", "/tmp/", "api_key", "password"]
for path in [
    proof / "capsule" / "continuity_capsule_manifest.json",
    proof / "capsule" / "custody_manifest.json",
    proof / "ec2_staged" / "stage_report.json",
    proof / "ec2_blocked" / "stage_report.json",
]:
    text = path.read_text(encoding="utf-8")
    for marker in bad_markers:
        if marker in text:
            raise SystemExit(f"portable artifact hygiene marker {marker!r} found in {path.relative_to(proof)}")

print("validate_v0917_csm_continuity_capsule_4910_status: PASS")
PY
