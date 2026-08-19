#!/usr/bin/env python3
"""Validate the redacted exact-head Gemini review receipt for issue 426."""

import json
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
RECEIPT = ROOT / ".csdlc/evidence/426/gemini-exact-head-review.json"
EXPECTED = {
    "schema": "adl.issue426.gemini_exact_head_review.v1",
    "issue": 426,
    "repository": "agent-logic/agent-design-language",
    "reviewed_revision": "8d4455b4983cffdc8f8091deb66d39bbe5b8f79f",
    "provider": "gemini",
    "provider_model_id": "gemini-3.1-pro-preview",
    "runtime_surface": "hosted_api",
    "request_id": "issue-426-gemini-exact-head-8d4455b4",
    "attempts": 1,
    "final_status": "ok",
    "verdict": "APPROVED",
    "actionable_findings": 0,
    "raw_result_sha256": "b1aad4a1daeb2b48e62c055d295fd0d2452897b91b06073272c54d7fd6176cd3",
    "credential_material_retained": False,
    "authority": "review_evidence_only",
}


def main() -> None:
    receipt = json.loads(RECEIPT.read_text(encoding="utf-8"))
    if receipt != EXPECTED:
        raise SystemExit("issue 426 Gemini receipt does not match the reviewed exact head")
    print("PASS: issue 426 Gemini exact-head review receipt")


if __name__ == "__main__":
    main()
