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
    "reviewed_revision": "097b10b89825b4968e47b97d318a34a0386ee8d6",
    "provider": "gemini",
    "provider_model_id": "gemini-3.1-pro-preview",
    "runtime_surface": "hosted_api",
    "request_id": "issue-426-gemini-exact-head-097b10b89",
    "attempts": 1,
    "final_status": "ok",
    "verdict": "APPROVED",
    "actionable_findings": 0,
    "raw_result_sha256": "4056125c72b86e9be4f47efb0bc20d506f6b5d0eb08e7ff07d6c02cab3d60150",
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
