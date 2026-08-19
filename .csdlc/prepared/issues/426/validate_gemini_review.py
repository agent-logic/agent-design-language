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
    "reviewed_revision": "7102c5712bd73a00a6205fc72d4dfa9cf351be3d",
    "provider": "gemini",
    "provider_model_id": "gemini-3.1-pro-preview",
    "runtime_surface": "hosted_api",
    "request_id": "issue-426-gemini-exact-head-7102c571",
    "attempts": 1,
    "final_status": "ok",
    "verdict": "APPROVED",
    "actionable_findings": 0,
    "raw_result_sha256": "225d47d931e5e0c0e10818550b06799d50c7f5b1cd072fb56933d7d90ffccf33",
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
