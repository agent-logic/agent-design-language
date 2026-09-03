#!/usr/bin/env python3
"""Validate issue #624 corporate sidecar hardening artifacts."""

from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]

REGISTER = ROOT / "docs/operations/corporate/control-transfer/operational-control-hardening-sidecar.md"
RECEIPT = ROOT / "docs/milestones/v0.92.1/evidence/corporate/corp-sidecar-624/operational-control-hardening.v1.json"

REQUIRED_CATEGORIES = {
    "github_ci",
    "dns_certificate",
    "aws_guardrails",
    "private_custody",
    "deployment_rollback",
}

REQUIRED_ROW_IDS = {
    "GH-ORG-RECOVERY",
    "GH-CI-GUARDRAILS",
    "DNS-DELEGATION",
    "CERT-RENEWAL",
    "AWS-AUDIT-GUARDRAILS",
    "DEPLOY-ROLLBACK",
    "PRIVATE-CUSTODY",
}

FORBIDDEN_PATTERNS = [
    re.compile(r"AKIA[0-9A-Z]{16}"),
    re.compile(r"ASIA[0-9A-Z]{16}"),
    re.compile(r"(?i)aws_secret_access_key"),
    re.compile(r"(?i)private[_ -]?key"),
    re.compile(r"(?i)password\s*[:=]"),
    re.compile(r"(?i)token\s*[:=]"),
    re.compile(r"\b\d{12}\b"),
]


def fail(message: str) -> None:
    print(f"validation_failed: {message}", file=sys.stderr)
    raise SystemExit(1)


def load_json(path: Path) -> dict:
    if not path.exists():
        fail(f"missing JSON artifact: {path}")
    try:
        return json.loads(path.read_text())
    except json.JSONDecodeError as exc:
        fail(f"invalid JSON {path}: {exc}")


def require_no_sensitive_text(path: Path) -> None:
    text = path.read_text() if path.exists() else ""
    for pattern in FORBIDDEN_PATTERNS:
        if pattern.search(text):
            fail(f"secret-like or account-id-like material in {path}: {pattern.pattern}")


def require_repo_relative(path_text: str) -> None:
    if path_text.startswith("/") or path_text.startswith("~"):
        fail(f"non-portable evidence path: {path_text}")
    if ".." in Path(path_text).parts:
        fail(f"parent traversal in evidence path: {path_text}")


def main() -> None:
    if not REGISTER.exists():
        fail(f"missing register: {REGISTER}")
    receipt = load_json(RECEIPT)

    if receipt.get("schema") != "adl.corporate.operational_control_hardening.v1":
        fail("unexpected receipt schema")
    if receipt.get("issue") != 624:
        fail("receipt is not bound to issue 624")
    if receipt.get("parent_issue") != 497:
        fail("receipt does not preserve #497 sidecar boundary")
    if receipt.get("live_mutation_performed") is not False:
        fail("receipt must state no live mutation was performed")
    if receipt.get("ip_transfer_acceptance_reopened") is not False:
        fail("receipt must not reopen #497")

    rows = receipt.get("rows")
    if not isinstance(rows, list) or len(rows) != len(REQUIRED_ROW_IDS):
        fail(f"rows must contain exactly {len(REQUIRED_ROW_IDS)} denominator entries")

    seen_ids: set[str] = set()
    seen_categories: set[str] = set()
    for row in rows:
        row_id = row.get("id")
        if not isinstance(row_id, str) or not row_id:
            fail("row missing id")
        if row_id in seen_ids:
            fail(f"duplicate row id: {row_id}")
        seen_ids.add(row_id)

        category = row.get("category")
        if category not in REQUIRED_CATEGORIES:
            fail(f"row {row_id} has unknown category {category!r}")
        seen_categories.add(category)

        status = row.get("status")
        if status not in {"proven", "follow_on_required"}:
            fail(f"row {row_id} has invalid status {status!r}")

        for field in ["owner_role", "action", "authority_gate", "closeout_condition"]:
            if not row.get(field):
                fail(f"row {row_id} missing {field}")

        refs = row.get("evidence_refs")
        if not isinstance(refs, list) or not refs:
            fail(f"row {row_id} missing evidence refs")
        for ref in refs:
            require_repo_relative(ref)
            if not (ROOT / ref).exists():
                fail(f"row {row_id} references missing evidence {ref}")

        if status == "proven" and row.get("authority_gate") != "read_only_retained_evidence":
            fail(f"row {row_id} overclaims proven status without read-only retained evidence")
        if status == "follow_on_required" and "explicit_operator_authorization" not in row.get("authority_gate", ""):
            fail(f"row {row_id} follow-on lacks explicit authorization gate")

    missing_categories = REQUIRED_CATEGORIES - seen_categories
    if missing_categories:
        fail(f"missing denominator categories: {sorted(missing_categories)}")

    missing_row_ids = REQUIRED_ROW_IDS - seen_ids
    unexpected_row_ids = seen_ids - REQUIRED_ROW_IDS
    if missing_row_ids or unexpected_row_ids:
        fail(
            "row denominator mismatch: "
            f"missing={sorted(missing_row_ids)} unexpected={sorted(unexpected_row_ids)}"
        )

    non_claims = receipt.get("non_claims")
    if not isinstance(non_claims, list) or len(non_claims) < 5:
        fail("non_claims must enumerate external-action and privacy boundaries")
    required_non_claim_words = ["credential", "account", "DNS", "GitHub", "AWS", "custody", "deployment"]
    joined_non_claims = "\n".join(non_claims)
    for word in required_non_claim_words:
        if word not in joined_non_claims:
            fail(f"non_claims missing boundary word: {word}")

    for path in [REGISTER, RECEIPT]:
        require_no_sensitive_text(path)

    head = subprocess.check_output(
        ["git", "rev-parse", "HEAD"],
        cwd=ROOT,
        text=True,
    ).strip()

    print(json.dumps({
        "schema": "adl.validation_receipt.v1",
        "issue": 624,
        "head": head,
        "validator": ".csdlc/prepared/issues/624/validate-corp-sidecar-hardening.py",
        "status": "passed",
        "rows": len(rows),
        "categories": sorted(seen_categories),
    }, sort_keys=True))


if __name__ == "__main__":
    main()
