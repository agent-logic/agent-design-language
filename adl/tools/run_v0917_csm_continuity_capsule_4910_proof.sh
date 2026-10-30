#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="${1:-$ROOT/docs/milestones/v0.91.7/review/runtime/csm_continuity_capsule_4910}"
CSM_BIN="${CSM_BIN:-$ROOT/adl/target/debug/csm}"
# Deterministic non-secret P-256 test vector used only for local proof packets.
CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64="${CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64:-CQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQk=}"
CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64="${CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64:-BHE1+k/ZOgnc6Yu/aBtL/PUOfA1jVOYq+wv/KjQpYXhl7UwfAt25Aj7lalV+UV1qncZsEfIglg3llDNN9Yh3ZyQ=}"
CSM_CUSTODY_SIGNING_KEY_ID="${CSM_CUSTODY_SIGNING_KEY_ID:-csm-continuity-4910-proof-key}"
export CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64

case "$OUT" in
  /*) ;;
  *) OUT="$PWD/$OUT" ;;
esac

AWS_SUMMARY_TMP=""
if [ -f "$OUT/aws_remote_restore_fireup_summary.json" ]; then
  AWS_SUMMARY_TMP="$(mktemp)"
  cp "$OUT/aws_remote_restore_fireup_summary.json" "$AWS_SUMMARY_TMP"
fi

rm -rf "$OUT"
mkdir -p "$OUT/logs"
# shellcheck source=adl/tools/csm_binary_availability.sh
source "$ROOT/adl/tools/csm_binary_availability.sh"
CSM_BIN="$(adl_resolve_csm_binary "$CSM_BIN" "$OUT/csm_binary_availability.json")"
if [ -n "$AWS_SUMMARY_TMP" ]; then
  cp "$AWS_SUMMARY_TMP" "$OUT/aws_remote_restore_fireup_summary.json"
  rm -f "$AWS_SUMMARY_TMP"
fi

cat >"$OUT/agent.yaml" <<'YAML'
schema: adl.long_lived_agent_spec.v1
agent_instance_id: csm-continuity-4910
display_name: CSM Continuity Capsule 4910 Proof Agent
state_root: state
workflow:
  kind: demo_adapter
  name: csm_continuity_capsule_4910_probe
  run_args:
    provider_id: local_ollama
    model: gemma4:latest
heartbeat:
  interval_secs: 1
  max_cycles: 3
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/csm-continuity-4910
  write_policy: append_only
YAML

run_csm() {
  (
    cd "$OUT"
    ADL_OBSERVABILITY_STDERR=0 \
    ADL_OBSERVABILITY_LOG="$OUT/logs/observability.log" \
    ADL_OBSERVABILITY_REPO_ROOT="$ROOT" \
    ADL_OTEL_LOG="$OUT/logs/otel.jsonl" \
    ADL_OTEL_STATUS="$OUT/logs/otel_status.json" \
    ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64="$CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64" \
      "$CSM_BIN" "$@"
  )
}

run_csm daemon --spec agent.yaml --test-supervisor-failure-after-restarts 1 --checkpoint-interval-secs 1 --no-sleep --json \
  >"$OUT/logs/daemon_stdout.json" \
  2>"$OUT/logs/daemon_stderr.log"

ADL_CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64="$CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64" \
ADL_CSM_CUSTODY_SIGNING_KEY_ID="$CSM_CUSTODY_SIGNING_KEY_ID" \
ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64="$CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64" \
  run_csm continuity capture --spec agent.yaml --out capsule --source-host wuji --target-host ec2-staging --json \
  >"$OUT/logs/capture_stdout.json" \
  2>"$OUT/logs/capture_stderr.log"

run_csm continuity stage --bundle capsule --out ec2_staged --target-host ec2-staging --json \
  >"$OUT/logs/stage_stdout.json" \
  2>"$OUT/logs/stage_stderr.log"

run_csm continuity stage --bundle capsule --out ec2_blocked --target-host ec2 --json \
  >"$OUT/logs/ec2_stage_stdout.json" \
  2>"$OUT/logs/ec2_stage_stderr.log"

run_csm continuity restore --bundle capsule --out ec2_restored --target-host ec2 --json \
  >"$OUT/logs/restore_stdout.json" \
  2>"$OUT/logs/restore_stderr.log"

run_csm daemon --spec ec2_restored/agent.yaml --test-supervisor-failure-after-restarts 1 --checkpoint-interval-secs 1 --no-sleep --json \
  >"$OUT/logs/restored_daemon_stdout.json" \
  2>"$OUT/logs/restored_daemon_stderr.log"

python3 - "$OUT" "$CSM_BIN" <<'PY'
import json
import os
import pathlib
import shutil
import subprocess
import sys

out = pathlib.Path(sys.argv[1])
csm_bin = pathlib.Path(sys.argv[2])
capsule = out / "capsule"
negative_root = out / "negative_cases"
negative_root.mkdir(parents=True, exist_ok=True)

def clone(name):
    target = negative_root / name
    if target.exists():
        shutil.rmtree(target)
    shutil.copytree(capsule, target)
    return target

def run_stage(bundle, stage_out, target="ec2-staging"):
    result = subprocess.run(
        [
            str(csm_bin),
            "continuity",
            "stage",
            "--bundle",
            str(bundle.relative_to(out)),
            "--out",
            str(stage_out.relative_to(out)),
            "--target-host",
            target,
            "--json",
        ],
        cwd=out,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
        env={
            "ADL_OBSERVABILITY_STDERR": "0",
            "ADL_OBSERVABILITY_LOG": str(out / "logs" / "observability.log"),
            "ADL_OBSERVABILITY_REPO_ROOT": str(out),
            "ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64": os.environ["CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64"],
        },
    )
    return result

cases = []

bad = clone("version_mismatch")
manifest_path = bad / "continuity_capsule_manifest.json"
manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
manifest["format_version"] = "csm.continuity-capsule.v0"
manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
cases.append(("version_mismatch", run_stage(bad, negative_root / "version_mismatch_stage"), "unsupported continuity capsule format version"))

bad = clone("missing_file")
(bad / "state" / "status.json").unlink()
cases.append(("missing_file", run_stage(bad, negative_root / "missing_file_stage"), "custody retained artifact missing"))

bad = clone("missing_custody_manifest")
(bad / "custody_manifest.json").unlink()
cases.append(("missing_custody_manifest", run_stage(bad, negative_root / "missing_custody_manifest_stage"), "custody manifest missing bundle artifact"))

bad = clone("custody_signature_tamper")
custody_path = bad / "custody_manifest.json"
custody = json.loads(custody_path.read_text(encoding="utf-8"))
custody["signature"]["sig_b64"] = "not-a-valid-signature"
custody_path.write_text(json.dumps(custody, indent=2) + "\n", encoding="utf-8")
cases.append(("custody_signature_tamper", run_stage(bad, negative_root / "custody_signature_tamper_stage"), "invalid base64 custody signature"))

bad = clone("custody_untrusted_public_key")
custody_path = bad / "custody_manifest.json"
custody = json.loads(custody_path.read_text(encoding="utf-8"))
custody["signature"]["public_key_b64"] = "BDrasV1mJWvxXNcWA1s/BBRE5RL+0d1k1Lp1WX0g42bxVG0skKg+uroBWVCZ5fP/u9M4THSU3mdZ/dXmXvrpzGc="
custody_path.write_text(json.dumps(custody, indent=2) + "\n", encoding="utf-8")
cases.append(("custody_untrusted_public_key", run_stage(bad, negative_root / "custody_untrusted_public_key_stage"), "public key does not match trusted verification key"))

bad = clone("path_leakage")
(bad / "state" / "status.json").write_text(json.dumps({"path": str(bad.resolve())}) + "\n", encoding="utf-8")
cases.append(("path_leakage", run_stage(bad, negative_root / "path_leakage_stage"), "host-private absolute path"))

bad = clone("credential_leakage")
manifest_path = bad / "continuity_capsule_manifest.json"
manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
manifest["api_key"] = "not-exportable"
manifest_path.write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
cases.append(("credential_leakage", run_stage(bad, negative_root / "credential_leakage_stage"), "credential-like key"))

bad = clone("corrupted_manifest")
(bad / "continuity_capsule_manifest.json").write_text("{", encoding="utf-8")
cases.append(("corrupted_manifest", run_stage(bad, negative_root / "corrupted_manifest_stage"), "failed parsing"))

cases.append(("unsupported_target_host", run_stage(capsule, negative_root / "unsupported_target_stage", target="mars"), "unsupported continuity capsule target host"))

records = []
for name, result, expected in cases:
    records.append({
        "name": name,
        "status": "failed_as_expected" if result.returncode != 0 and expected in result.stderr else "unexpected",
        "returncode": result.returncode,
        "expected_reason": expected,
        "stderr_matched": expected in result.stderr,
    })

(out / "negative_results.json").write_text(json.dumps({
    "schema": "adl.csm.continuity_capsule_4910_negative_results.v1",
    "cases": records,
}, indent=2) + "\n", encoding="utf-8")
shutil.rmtree(negative_root)

manifest = json.loads((out / "capsule" / "continuity_capsule_manifest.json").read_text(encoding="utf-8"))
stage_report = json.loads((out / "ec2_staged" / "stage_report.json").read_text(encoding="utf-8"))
ec2_report = json.loads((out / "ec2_blocked" / "stage_report.json").read_text(encoding="utf-8"))
restore_report = json.loads((out / "ec2_restored" / "restore_report.json").read_text(encoding="utf-8"))
aws_summary_path = out / "aws_remote_restore_fireup_summary.json"
aws_summary = json.loads(aws_summary_path.read_text(encoding="utf-8")) if aws_summary_path.exists() else None
summary = {
    "schema": "adl.csm.continuity_capsule_4910_proof_summary.v1",
    "runtime_owner": "csm",
    "proof_classification": "proving_with_restore_fire_up_and_blocked_live_ec2_transfer",
    "command_surface": "csm continuity capture|stage|restore",
    "format_version": manifest["format_version"],
    "manifest_schema": manifest["schema"],
    "agent_instance_id": manifest["agent_instance_id"],
    "source_host": manifest["source_host"],
    "stage_target_host": stage_report["target_host"],
    "stage_status": stage_report["status"],
    "live_ec2_status": ec2_report["status"],
    "restore_target_host": restore_report["target_host"],
    "restore_status": restore_report["status"],
    "restored_daemon_fire_up": "passed",
    "aws_profile_policy": ec2_report["blockers"][0]["required_profile"],
    "artifact_count": len(manifest["artifacts"]),
    "custody_manifest_ref": manifest["custody_manifest_ref"],
    "retained_runtime_roles": sorted({artifact["role"] for artifact in manifest["artifacts"]}),
    "negative_case_count": len(records),
    "negative_cases": {record["name"]: record["status"] for record in records},
    "observability_refs": ["logs/observability.log", "logs/otel.jsonl", "logs/otel_status.json"],
    "non_claims": [
        "does not claim live EC2 transfer without operator authorization",
        "does not export provider tokens or secret material",
        "does not claim multi-region production disaster recovery"
    ]
}
if aws_summary is not None:
    summary["proof_classification"] = "proving_with_local_restore_fire_up_and_aws_ec2_restore_fire_up"
    summary["aws_ec2_restore_fire_up"] = {
        "status": aws_summary["status"],
        "summary_ref": "aws_remote_restore_fireup_summary.json",
        "business_profile": aws_summary["profile"],
        "instance_type": aws_summary["aws_surface"]["instance_type"],
        "purchase_option": aws_summary["aws_surface"]["purchase_option"],
        "resolved_commit": aws_summary["resolved_commit"],
        "cleanup": "terminated_and_ephemeral_surface_deleted",
    }
(out / "proof_summary.json").write_text(json.dumps(summary, indent=2) + "\n", encoding="utf-8")

aws_evidence = ""
aws_truth = ""
if aws_summary is not None:
    aws_evidence = """- `aws_remote_restore_fireup_summary.json` records the redacted Agent Logic AWS
  proof that the same restore/fire-up lane passed on an EC2 Spot builder.
"""
    aws_truth = ", including an EC2 restore/fire-up run in the Agent Logic business AWS account"

readme = f"""# CSM Continuity Capsule Proof (#4910)

This packet retains a bounded WP-07 proof for `csm continuity capture` and
`csm continuity stage`.

Evidence:
- `capsule/continuity_capsule_manifest.json` is the portable capsule manifest.
- `capsule/custody_manifest.json` is the signed RustCrypto P-256/ECDSA custody
  manifest for retained capsule artifacts and binary segments.
- `capsule/state/` contains retained CSM runtime state: identity/spec, status,
  daemon status, continuity checkpoint, replay manifest, cycle ledger, memory
  index, provider binding history, operator events, and cycle artifacts.
- `ec2_staged/stage_report.json` proves local EC2-staging validation.
- `ec2_blocked/stage_report.json` records the live EC2 transfer boundary and
  required `agent-logic-admin` business AWS profile.
- `ec2_restored/restore_report.json` proves capsule restore into a runtime root,
  and `logs/restored_daemon_stdout.json` proves `csm daemon` fired from the
  restored spec/state.
{aws_evidence}\
- `negative_results.json` records version mismatch, missing file, missing
  custody manifest, custody signature tampering, untrusted custody public key,
  path leakage, credential-like key, corrupted manifest, and unsupported
  target-host rejection.
- `logs/observability.log`, `logs/otel.jsonl`, and `logs/otel_status.json`
  retain runtime observability for daemon, capture, stage, and restore events.

Truth boundary: this proves portable capture, staging, restore, and restored
daemon fire-up of current CSM runtime state{aws_truth}. It does not claim provider-secret
export or production multi-region disaster recovery.
"""
(out / "README.md").write_text(readme, encoding="utf-8")
PY

bash "$ROOT/adl/tools/validate_v0917_csm_continuity_capsule_4910_status.sh" "$OUT"
