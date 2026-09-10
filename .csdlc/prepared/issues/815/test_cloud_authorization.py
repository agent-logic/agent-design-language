#!/usr/bin/env python3
"""Deterministic no-cloud negative matrix for issue #815."""

from __future__ import annotations

import copy
from datetime import datetime, timezone
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile


REPO = Path(__file__).resolve().parents[4]
VERIFIER = REPO / ".csdlc/prepared/issues/815/verify-cloud-authorization.py"
spec = importlib.util.spec_from_file_location("cloud_authorization", VERIFIER)
module = importlib.util.module_from_spec(spec)
assert spec.loader is not None
spec.loader.exec_module(module)


def sha_bytes(value: bytes) -> str:
    return hashlib.sha256(value).hexdigest()


def json_digest(value: object) -> str:
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def utc(epoch: int) -> str:
    return datetime.fromtimestamp(epoch, tz=timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def write_json(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


class Fixture:
    def __init__(self, root: Path) -> None:
        self.root = root
        self.key = root / "operator-ed25519"
        subprocess.run(["ssh-keygen", "-q", "-t", "ed25519", "-N", "", "-f", str(self.key)], check=True)
        public = self.key.with_suffix(".pub").read_text(encoding="utf-8").strip()
        self.signers = root / "allowed_signers"
        self.signers.write_text(f"operator {public}\n", encoding="utf-8")
        self.signers.chmod(0o600)

    def sign(self, packet: dict) -> dict:
        signed = copy.deepcopy(packet)
        signed.setdefault("operator_authorization", {}).pop("signature", None)
        payload = self.root / "payload.json"
        payload.write_bytes(module.canonical_payload(signed))
        signature = payload.with_suffix(payload.suffix + ".sig")
        if signature.exists():
            signature.unlink()
        subprocess.run(
            ["ssh-keygen", "-Y", "sign", "-f", str(self.key), "-n", module.NAMESPACE, str(payload)],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            check=True,
        )
        signed["operator_authorization"]["signature"] = {
            "algorithm": "ssh-ed25519",
            "namespace": module.NAMESPACE,
            "value": signature.read_text(encoding="utf-8"),
        }
        return signed

    def run(self, provider: str, packet: dict, *, expect: bool, fragment: str = "", extra: list[str] | None = None, env: dict[str, str] | None = None) -> None:
        packet_path = self.root / f"{provider}-packet.json"
        write_json(packet_path, packet)
        argv = [
            sys.executable,
            str(VERIFIER),
            provider,
            "--repo-root",
            str(REPO),
            "--packet",
            str(packet_path),
            "--now-epoch",
            str(NOW),
            "--test-mode",
            "--trusted-signers",
            str(self.signers),
        ]
        if extra:
            argv.extend(extra)
        result = subprocess.run(argv, text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=env)
        if (result.returncode == 0) != expect:
            raise AssertionError(f"{provider} expected success={expect}, rc={result.returncode}, stdout={result.stdout!r}, stderr={result.stderr!r}")
        if fragment and fragment not in result.stderr:
            raise AssertionError(f"{provider} rejection did not contain {fragment!r}: {result.stderr!r}")


NOW = 1_800_000_000


def gcp_packet(fixture: Fixture) -> tuple[dict, Path, Path, Path, Path]:
    plan = fixture.root / "gcp.tfplan"
    projection = fixture.root / "gcp-plan.json"
    plan.write_bytes(b"synthetic exact GCP plan bytes\n")
    plan_value = {"format_version": "1.2", "resource_changes": []}
    write_json(projection, plan_value)
    derived = fixture.root / "gcp-terraform-derived.json"
    write_json(derived, plan_value)
    mock = fixture.root / "gcp-terraform"
    mock.write_text(
        "#!/usr/bin/env bash\nset -euo pipefail\ncat \"${ADL_TEST_GCP_DERIVED_PLAN_JSON:?}\"\n",
        encoding="utf-8",
    )
    mock.chmod(0o700)
    packet = {
        "schema": "adl.gcp_d1.mutation_authorization_request.v2",
        "issue": 731,
        "repository": module.EXPECTED_REPOSITORY,
        "project": module.EXPECTED_GCP_PROJECT,
        "region": "us-west2",
        "zone": "us-west2-a",
        "network": "axioma-dev-csm-private",
        "subnet": "axioma-dev-csm-private-us-west2",
        "plan_digest": sha_bytes(plan.read_bytes()),
        "plan_json_digest": json_digest(plan_value),
        "plan_file": str(plan.relative_to(REPO)),
        "plan_json": str(projection.relative_to(REPO)),
        "run_id": "gcp-d1-20270115t080000z-localguard",
        "instance_name": "adl-gcp-d1-localguard",
        "cleanup_deadline_utc": utc(NOW + 600),
        "impersonated_identity": f"axioma-dev-workload@{module.EXPECTED_GCP_PROJECT}.iam.gserviceaccount.com",
        "proof_run_spend_cap_usd": 1,
        "foundation_steady_state_30d_cap_usd": 5,
        "rollback_commands": ["terraform destroy guarded", "gcloud delete guarded"],
        "operator_authorization": {
            "status": "approved",
            "approved_by": "operator",
            "approved_at_utc": utc(NOW - 60),
            "authorization_text": "synthetic local proof; no cloud authority",
            "required_before": ["any live GCP mutation or spend"],
        },
    }
    return packet, plan, projection, derived, mock


def aws_packet(fixture: Fixture) -> tuple[dict, Path, Path, Path, Path]:
    plan = fixture.root / "aws.tfplan"
    projection = fixture.root / "aws-plan.json"
    sidecar = fixture.root / "aws.tfplan.sha256"
    plan.write_bytes(b"synthetic exact AWS plan bytes\n")
    changes = [
        {"address": address, "change": {"actions": ["create"]}}
        for address in sorted(module.EXPECTED_AWS_RESOURCES)
    ]
    plan_value = {"format_version": "1.2", "resource_changes": changes}
    write_json(projection, plan_value)
    sidecar.write_text(sha_bytes(plan.read_bytes()) + "  aws.tfplan\n", encoding="utf-8")
    derived = fixture.root / "terraform-derived.json"
    write_json(derived, plan_value)
    mock = fixture.root / "terraform"
    mock.write_text("#!/usr/bin/env bash\nset -euo pipefail\ncat \"${ADL_TEST_DERIVED_PLAN_JSON:?}\"\n", encoding="utf-8")
    mock.chmod(0o700)
    packet = {
        "schema": "adl.issue_727.operator_authorization.v2",
        "issue": 727,
        "repository": module.EXPECTED_REPOSITORY,
        "purpose": "synthetic local proof; no cloud authority",
        "account_id": "123456789012",
        "account_identity_verified": True,
        "region": "us-west-2",
        "terraform_root": "infra/aws/account-foundation",
        "terraform_workspace": "aws-d-account-foundation-live",
        "state_key": "v0.92.1/aws-d/account-foundation/account-foundation.tfstate",
        "saved_plan_digest": sha_bytes(plan.read_bytes()),
        "saved_plan_path": str(plan.relative_to(REPO)),
        "saved_plan_digest_path": str(sidecar.relative_to(REPO)),
        "saved_plan_json_path": str(projection.relative_to(REPO)),
        "saved_plan_json_digest": json_digest(plan_value),
        "permitted_resources": sorted(module.EXPECTED_AWS_RESOURCES),
        "mutation_deadline": utc(NOW + 600),
        "cost_ceiling": {"currency": "USD", "maximum_monthly_usd": 50},
        "rollback_destroy_disposition": "second explicit authorization required",
        "operator_authorization": {
            "status": "approved",
            "approved_by": "operator",
            "approved_at_utc": utc(NOW - 60),
            "statement": "authorize only these synthetic exact bytes",
        },
    }
    return packet, plan, projection, sidecar, mock


def main() -> int:
    evidence_root = REPO / ".csdlc/evidence/815"
    evidence_root.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="cloud-auth-test-", dir=evidence_root) as raw:
        fixture = Fixture(Path(raw))

        gcp, gcp_plan, gcp_projection, gcp_derived, gcp_mock = gcp_packet(fixture)
        gcp_env = os.environ.copy()
        gcp_env["ADL_TEST_GCP_DERIVED_PLAN_JSON"] = str(gcp_derived)
        gcp_args = ["--terraform-bin", str(gcp_mock)]
        signed_gcp = fixture.sign(gcp)
        fixture.run("gcp", signed_gcp, expect=True, extra=gcp_args, env=gcp_env)
        fixture.run("gcp", gcp, expect=False, fragment="no detached signature", extra=gcp_args, env=gcp_env)
        forged = copy.deepcopy(signed_gcp)
        forged["operator_authorization"]["authorization_text"] = "forged by repo writer"
        fixture.run("gcp", forged, expect=False, fragment="forged", extra=gcp_args, env=gcp_env)
        expired = copy.deepcopy(gcp)
        expired["operator_authorization"]["approved_at_utc"] = utc(NOW - 9000)
        expired["cleanup_deadline_utc"] = utc(NOW - 1)
        fixture.run("gcp", fixture.sign(expired), expect=False, fragment="older than 120 minutes", extra=gcp_args, env=gcp_env)
        replay = copy.deepcopy(gcp)
        replay["project"] = "attacker-project"
        fixture.run("gcp", fixture.sign(replay), expect=False, fragment="project is outside", extra=gcp_args, env=gcp_env)
        original_gcp_plan = gcp_plan.read_bytes()
        gcp_plan.write_bytes(original_gcp_plan + b"tamper")
        fixture.run("gcp", signed_gcp, expect=False, fragment="actual saved tfplan bytes", extra=gcp_args, env=gcp_env)
        gcp_plan.write_bytes(original_gcp_plan)
        original_gcp_json = gcp_projection.read_text(encoding="utf-8")
        gcp_projection.write_text('{"tampered":true}\n', encoding="utf-8")
        fixture.run("gcp", signed_gcp, expect=False, fragment="signed canonical JSON digest", extra=gcp_args, env=gcp_env)
        gcp_projection.write_text(original_gcp_json, encoding="utf-8")
        write_json(gcp_derived, {"format_version": "1.2", "resource_changes": [{"address": "drift"}]})
        fixture.run("gcp", signed_gcp, expect=False, fragment="not derived from the exact", extra=gcp_args, env=gcp_env)
        write_json(gcp_derived, {"format_version": "1.2", "resource_changes": []})

        verified_dir = fixture.root / "verified-gcp"
        fixture.run(
            "gcp",
            signed_gcp,
            expect=True,
            extra=gcp_args + ["--verified-dir", str(verified_dir.relative_to(REPO))],
            env=gcp_env,
        )
        receipt_path = verified_dir / "receipt.json"
        receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
        attacker_packet = copy.deepcopy(signed_gcp)
        attacker_packet["project"] = "attacker-project"
        write_json(fixture.root / "gcp-packet.json", attacker_packet)
        gcp_plan.write_bytes(b"post-verification attacker replacement\n")
        if sha_bytes((verified_dir / "verified.tfplan").read_bytes()) != receipt["plan_digest"]:
            raise AssertionError("verified GCP plan snapshot changed with mutable source plan")
        durable_receipt = json.loads(receipt_path.read_text(encoding="utf-8"))
        if durable_receipt["project"] != module.EXPECTED_GCP_PROJECT:
            raise AssertionError("verified GCP receipt did not preserve signed project")

        aws, aws_plan, aws_projection, sidecar, mock = aws_packet(fixture)
        signed_aws = fixture.sign(aws)
        derived = fixture.root / "terraform-derived.json"
        aws_env = os.environ.copy()
        aws_env["ADL_TEST_DERIVED_PLAN_JSON"] = str(derived)
        aws_args = ["--observed-account-id", "123456789012", "--terraform-bin", str(mock)]
        fixture.run("aws", signed_aws, expect=True, extra=aws_args, env=aws_env)
        fixture.run("aws", aws, expect=False, fragment="no detached signature", extra=aws_args, env=aws_env)
        forged_aws = copy.deepcopy(signed_aws)
        forged_aws["operator_authorization"]["statement"] = "forged"
        fixture.run("aws", forged_aws, expect=False, fragment="forged", extra=aws_args, env=aws_env)
        expired_aws = copy.deepcopy(aws)
        expired_aws["operator_authorization"]["approved_at_utc"] = utc(NOW - 9000)
        expired_aws["mutation_deadline"] = utc(NOW - 1)
        fixture.run("aws", fixture.sign(expired_aws), expect=False, fragment="older than 120 minutes", extra=aws_args, env=aws_env)
        fixture.run("aws", signed_aws, expect=False, fragment="different account", extra=["--observed-account-id", "210987654321", "--terraform-bin", str(mock)], env=aws_env)
        original_sidecar = sidecar.read_text(encoding="utf-8")
        sidecar.write_text("0" * 64 + "\n", encoding="utf-8")
        fixture.run("aws", signed_aws, expect=False, fragment="digest sidecar", extra=aws_args, env=aws_env)
        sidecar.write_text(original_sidecar, encoding="utf-8")
        original_aws_plan = aws_plan.read_bytes()
        aws_plan.write_bytes(original_aws_plan + b"tamper")
        fixture.run("aws", signed_aws, expect=False, fragment="actual saved tfplan bytes", extra=aws_args, env=aws_env)
        aws_plan.write_bytes(original_aws_plan)
        original_projection = aws_projection.read_text(encoding="utf-8")
        aws_projection.write_text('{"tampered":true}\n', encoding="utf-8")
        fixture.run("aws", signed_aws, expect=False, fragment="signed canonical JSON digest", extra=aws_args, env=aws_env)
        aws_projection.write_text(original_projection, encoding="utf-8")
        write_json(derived, {"format_version": "1.2", "resource_changes": []})
        fixture.run("aws", signed_aws, expect=False, fragment="not derived from the exact", extra=aws_args, env=aws_env)

    print("PASS: #815 rejected unsigned/forged approval, plan and sidecar mutation, expiry, projection drift, and account/project replay")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
