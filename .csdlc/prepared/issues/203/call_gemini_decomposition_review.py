#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
import os
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path


ROOT = Path.cwd()
PROMPT = ROOT / ".csdlc/evidence/203/provider-reviews/gemini-3.1-pro-decomposition-prompt.md"
RESULT = ROOT / ".csdlc/evidence/203/provider-reviews/gemini-3.1-pro-decomposition-result.json"
MODEL = os.environ.get("ADL_203_GEMINI_REVIEW_MODEL", "gemini-3.1-pro-preview")
KEY_FILE = Path(os.environ.get("ADL_203_GEMINI_API_KEY_FILE", str(Path.home() / "keys/gcp-ace-2023.key")))
TIMEOUT_SECONDS = int(os.environ.get("ADL_203_GEMINI_REVIEW_TIMEOUT_SECONDS", "600"))


def read_key() -> str:
    direct = os.environ.get("GEMINI_API_KEY")
    if direct:
        return direct.strip()
    return KEY_FILE.read_text(encoding="utf-8").strip()


def main() -> int:
    prompt = PROMPT.read_text(encoding="utf-8")
    payload = {
        "contents": [{"role": "user", "parts": [{"text": prompt}]}],
        "generationConfig": {"maxOutputTokens": 4096, "temperature": 0.2},
    }
    url = "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent".format(
        urllib.parse.quote(MODEL, safe="")
    )
    started = time.time()
    result: dict[str, object]
    try:
        request = urllib.request.Request(
            url,
            data=json.dumps(payload).encode("utf-8"),
            headers={
                "Content-Type": "application/json",
                "x-goog-api-key": read_key(),
            },
            method="POST",
        )
        with urllib.request.urlopen(request, timeout=TIMEOUT_SECONDS) as response:
            parsed = json.loads(response.read().decode("utf-8"))
            chunks: list[str] = []
            for candidate in parsed.get("candidates", []):
                content = candidate.get("content", {}) if isinstance(candidate, dict) else {}
                for part in content.get("parts", []) if isinstance(content, dict) else []:
                    text = part.get("text") if isinstance(part, dict) else None
                    if isinstance(text, str):
                        chunks.append(text)
            review_text = "\n".join(chunks).strip()
            result = {
                "schema": "adl.issue203.gemini_decomposition_result.v1",
                "status": "received" if review_text else "failed",
                "provider": "google",
                "model": MODEL,
                "http_status": response.status,
                "latency_ms": int((time.time() - started) * 1000),
                "prompt_path": str(PROMPT.relative_to(ROOT)),
                "prompt_sha256": hashlib.sha256(prompt.encode("utf-8")).hexdigest(),
                "output_chars": len(review_text),
                "finish_reasons": [
                    candidate.get("finishReason")
                    for candidate in parsed.get("candidates", [])
                    if isinstance(candidate, dict)
                ],
                "review_sha256": hashlib.sha256(review_text.encode("utf-8")).hexdigest(),
                "review_text": review_text,
            }
    except urllib.error.HTTPError as exc:
        result = {
            "schema": "adl.issue203.gemini_decomposition_result.v1",
            "status": "failed",
            "provider": "google",
            "model": MODEL,
            "http_status": exc.code,
            "latency_ms": int((time.time() - started) * 1000),
            "prompt_path": str(PROMPT.relative_to(ROOT)),
            "prompt_sha256": hashlib.sha256(prompt.encode("utf-8")).hexdigest(),
            "error": exc.read().decode("utf-8", errors="replace")[:2000],
        }
    except Exception as exc:
        result = {
            "schema": "adl.issue203.gemini_decomposition_result.v1",
            "status": "failed",
            "provider": "google",
            "model": MODEL,
            "latency_ms": int((time.time() - started) * 1000),
            "prompt_path": str(PROMPT.relative_to(ROOT)),
            "prompt_sha256": hashlib.sha256(prompt.encode("utf-8")).hexdigest(),
            "error": str(exc),
        }
    RESULT.write_text(json.dumps(result, indent=2) + "\n", encoding="utf-8")
    print(json.dumps({k: v for k, v in result.items() if k != "review_text"}, indent=2))
    return 0 if result.get("status") == "received" else 1


if __name__ == "__main__":
    raise SystemExit(main())
