#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import importlib.util
import json
import os
import sys
import time
import urllib.parse
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
PACKET = ROOT / "docs/reviews/v0.92/internal-review-5846"
ADAPTER_PATH = ROOT / "adl/tools/real_chatgpt_gemini_provider_adapter.py"
OUTPUT = PACKET / "independent-api-review"
MODEL = "gemini-3.1-pro-preview"
TARGET = "c6792e54df1db5969fa28c59b6dfe4c714ed5559"


def load_adapter_module():
    spec = importlib.util.spec_from_file_location("adl_live_adapter", ADAPTER_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("cannot load ADL provider adapter")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def build_prompt() -> tuple[str, dict[str, str]]:
    sources = [
        "SYNTHESIS.md",
        "FINDINGS_REGISTER.md",
        "LIVE_STATE.md",
        "specialists/architecture.md",
        "specialists/code.md",
        "specialists/tests.md",
        "specialists/security.md",
        "specialists/dependencies.md",
        "specialists/docs.md",
        "specialists/lifecycle.md",
        "specialists/demos.md",
        "specialists/release_publication.md",
    ]
    sections = []
    source_digests = {}
    for rel in sources:
        body = (PACKET / rel).read_text(encoding="utf-8")
        source_digests[rel] = hashlib.sha256(body.encode("utf-8")).hexdigest()
        sections.append(f"\n===== {rel} =====\n{body}")
    prompt = """You are the independent external API meta-reviewer for the Agent Design Language v0.92 WP-25 internal review. Review only the supplied packet text, which targets exact Git revision TARGET_SHA. Do not invent repository facts. Assess: (1) whether synthesis preserves material specialist findings, (2) duplicate or unsupported findings, (3) severity calibration, (4) missing cross-cutting findings visible in the supplied reports, (5) whether evidence and non-claims are sufficient, and (6) whether the packet is ready as an internal findings-first review while product release remains blocked on open remediation. Return Markdown with: Verdict, Actionable Findings ordered P0-P3, Finding Reconciliation, Evidence Limits, and Recommended Next Gate. If there are no actionable packet defects, explicitly say so; do not treat open product findings as defects in the review packet merely because they remain open. End the response with exactly `PACKET_ACTIONABLE_FINDINGS=0` when there are none, otherwise end with the exact positive integer count.

TARGET_SHA: """ + TARGET + "\n" + "".join(sections)
    return prompt, source_digests


def verify_receipt() -> int:
    prompt, source_digests = build_prompt()
    prompt_sha = hashlib.sha256(prompt.encode("utf-8")).hexdigest()
    receipt = json.loads((OUTPUT / "receipt.json").read_text(encoding="utf-8"))
    metadata = json.loads((OUTPUT / "provider-invocation.json").read_text(encoding="utf-8"))
    review = (OUTPUT / "gemini-meta-review.md").read_bytes()
    if receipt.get("prompt_sha256") != prompt_sha or receipt.get("source_sha256") != source_digests:
        raise RuntimeError("Gemini prompt/source digest mismatch")
    if receipt.get("response_sha256") != hashlib.sha256(review).hexdigest():
        raise RuntimeError("Gemini response digest mismatch")
    calls = metadata.get("invocations")
    if not isinstance(calls, list) or len(calls) != 1:
        raise RuntimeError("Gemini invocation count mismatch")
    call = calls[0]
    required_digest_fields = ("provider_response_id_sha256", "response_payload_sha256")
    if any(not isinstance(call.get(field), str) or len(call[field]) != 64 for field in required_digest_fields):
        raise RuntimeError("Gemini provider provenance digest missing")
    if call.get("family") != "gemini" or call.get("model") != receipt.get("model") or call.get("prompt_sha256") != prompt_sha:
        raise RuntimeError("Gemini model/prompt identity mismatch")
    if call.get("http_status") != 200 or call.get("request_id_present") is not True:
        raise RuntimeError("Gemini provider success identity missing")
    if not isinstance(call.get("model_version"), str) or not call["model_version"]:
        raise RuntimeError("Gemini model version missing")
    if call.get("finish_reasons") != ["STOP"]:
        raise RuntimeError("Gemini response did not finish normally")
    if receipt.get("verdict") != "pass" or receipt.get("actionable_finding_count") != 0:
        raise RuntimeError("Gemini meta-review did not pass")
    if review.decode("utf-8").rstrip().splitlines()[-1] != "PACKET_ACTIONABLE_FINDINGS=0":
        raise RuntimeError("Gemini actionable-finding marker mismatch")
    print("PASS: reconstructed Gemini prompt and verified provider response provenance")
    return 0


def main() -> int:
    if sys.argv[1:] == ["--verify-receipt"]:
        return verify_receipt()
    if sys.argv[1:]:
        raise RuntimeError(f"unexpected arguments: {' '.join(sys.argv[1:])}")
    prompt, source_digests = build_prompt()
    prompt_sha = hashlib.sha256(prompt.encode("utf-8")).hexdigest()

    module = load_adapter_module()
    OUTPUT.mkdir(parents=True, exist_ok=True)
    metadata_path = OUTPUT / "provider-invocation.json"
    started = time.time()
    endpoint = module.GEMINI_GENERATE_URL.format(
        model=urllib.parse.quote(MODEL, safe="")
    )
    status, headers, payload = module._post_json(
        endpoint,
        {
            "x-goog-api-key": os.environ["GEMINI_API_KEY"],
            "Content-Type": "application/json",
        },
        {
            "contents": [{"role": "user", "parts": [{"text": prompt}]}],
            "generationConfig": {
                "maxOutputTokens": 8192,
                "thinkingConfig": {"thinkingBudget": 512},
            },
        },
        180,
    )
    response = module._extract_gemini_text(payload)
    if not 200 <= status < 300 or not response:
        raise RuntimeError(f"Gemini review failed with HTTP {status}")
    provider_response_id = payload.get("responseId")
    model_version = payload.get("modelVersion")
    if not isinstance(provider_response_id, str) or not provider_response_id:
        raise RuntimeError("Gemini response did not contain a provider response ID")
    if not isinstance(model_version, str) or not model_version:
        raise RuntimeError("Gemini response did not contain a model version")
    finish_reasons = [
        candidate.get("finishReason")
        for candidate in payload.get("candidates", [])
        if isinstance(candidate, dict) and isinstance(candidate.get("finishReason"), str)
    ]
    response_payload_sha256 = hashlib.sha256(
        json.dumps(payload, sort_keys=True, separators=(",", ":")).encode("utf-8")
    ).hexdigest()
    metadata_path.write_text(
        json.dumps(
            {
                "schema_version": "adl.live_provider_invocations.v1",
                "credential_policy": "operator_env_or_home_keys_no_secret_material_recorded",
                "providers": [{"family": "gemini", "model": MODEL}],
                "invocations": [
                    {
                        "family": "gemini",
                        "model": MODEL,
                        "http_status": status,
                        "request_id_present": bool(
                            headers.get("x-goog-request-id")
                            or headers.get("x-request-id")
                            or provider_response_id
                        ),
                        "provider_response_id_sha256": hashlib.sha256(
                            provider_response_id.encode("utf-8")
                        ).hexdigest(),
                        "model_version": model_version,
                        "finish_reasons": finish_reasons,
                        "response_payload_sha256": response_payload_sha256,
                        "prompt_sha256": prompt_sha,
                        "latency_ms": int((time.time() - started) * 1000),
                        "completed_at_unix_ms": int(time.time() * 1000),
                        "prompt_chars": len(prompt),
                        "output_chars": len(response),
                    }
                ],
            },
            indent=2,
            sort_keys=True,
        )
        + "\n",
        encoding="utf-8",
    )
    review_path = OUTPUT / "gemini-meta-review.md"
    normalized_review = "\n".join(line.rstrip() for line in response.rstrip().splitlines()) + "\n"
    review_path.write_text(normalized_review, encoding="utf-8")
    if response.rstrip().splitlines()[-1] != "PACKET_ACTIONABLE_FINDINGS=0":
        raise RuntimeError("Gemini response did not provide a zero-actionable-finding verdict")
    receipt = {
        "schema_version": "adl.internal_review_api_meta_review.v1",
        "provider_family": "gemini",
        "model": MODEL,
        "target_revision": TARGET,
        "prompt_sha256": prompt_sha,
        "response_sha256": hashlib.sha256(review_path.read_bytes()).hexdigest(),
        "source_sha256": source_digests,
        "verdict": "pass",
        "actionable_finding_count": 0,
        "credential_material_retained": False,
    }
    (OUTPUT / "receipt.json").write_text(
        json.dumps(receipt, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
