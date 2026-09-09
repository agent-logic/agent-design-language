#!/usr/bin/env python3
"""Fail closed on unsafe or structurally ambiguous publication JSON."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import re
import sys
from typing import Any


SENSITIVE_FIELDS = {"provider_payload", "prompt", "output", "tool_arguments"}
SECRET_PATTERNS = (
    ("machine_local_path", re.compile(r"/Users/|/Volumes/|/private/tmp/")),
    ("private_key_block", re.compile(r"-----BEGIN [A-Z0-9 ]*PRIVATE KEY-----")),
    ("github_token", re.compile(r"gh[pousr]_[A-Za-z0-9]{20,}")),
    ("aws_access_key", re.compile(r"AKIA[A-Z0-9]{16}")),
    ("bearer_token", re.compile(r"Bearer\s+[A-Za-z0-9._~+/=-]{16,}")),
    (
        "provider_credential",
        re.compile(
            r"(OPENAI|ANTHROPIC|DEEPSEEK|GEMINI|GOOGLE|AWS|GITHUB)_"
            r"[A-Z0-9_]*(API_)?(KEY|TOKEN)\s*=\s*\S{8,}",
            re.IGNORECASE,
        ),
    ),
)


class UnsafePublication(ValueError):
    pass


def unique_object(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise UnsafePublication(f"duplicate_key:{key}")
        result[key] = value
    return result


def inspect(value: Any, pointer: str, found: set[str]) -> None:
    if isinstance(value, dict):
        for key, child in value.items():
            child_pointer = f"{pointer}/{key}"
            if key in SENSITIVE_FIELDS:
                found.add(key)
                if child != "[REDACTED]":
                    raise UnsafePublication(f"unredacted_provider_payload:{child_pointer}")
            inspect(child, child_pointer, found)
    elif isinstance(value, list):
        for index, child in enumerate(value):
            inspect(child, f"{pointer}/{index}", found)
    elif isinstance(value, str):
        for category, pattern in SECRET_PATTERNS:
            if pattern.search(value):
                raise UnsafePublication(f"{category}:{pointer}")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--expected-schema")
    parser.add_argument("--require-sensitive-fields", action="store_true")
    parser.add_argument("path")
    args = parser.parse_args()
    try:
        with Path(args.path).open(encoding="utf-8") as handle:
            value = json.load(handle, object_pairs_hook=unique_object)
        if not isinstance(value, dict):
            raise UnsafePublication("root_not_object")
        if args.expected_schema and value.get("schema") != args.expected_schema:
            raise UnsafePublication("unexpected_schema")
        found: set[str] = set()
        inspect(value, "", found)
        if args.require_sensitive_fields and found != SENSITIVE_FIELDS:
            raise UnsafePublication("missing_sensitive_fields")
    except (OSError, json.JSONDecodeError, UnsafePublication) as error:
        print(f"publication_json_rejected:{error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
