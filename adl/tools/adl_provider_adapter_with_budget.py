#!/usr/bin/env python3
"""Run adl-provider-adapter after adding a portable output-token budget.

The UTS hosted ADL bridge owns temporary provider request files. This wrapper
keeps the upstream runner unchanged while allowing an issue or operator to
require a larger `max_output_tokens` budget for models that need it.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    parser.add_argument("--adapter", required=True)
    parser.add_argument("--max-output-tokens", type=int, required=True)
    parser.add_argument("adapter_args", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    if args.max_output_tokens <= 0:
        parser.error("--max-output-tokens must be greater than zero")
    if args.adapter_args and args.adapter_args[0] == "--":
        args.adapter_args = args.adapter_args[1:]
    if "--request" not in args.adapter_args:
        parser.error("adapter args must include --request <path>")
    return args


def request_path(adapter_args: list[str]) -> Path:
    for index, arg in enumerate(adapter_args):
        if arg == "--request" and index + 1 < len(adapter_args):
            return Path(adapter_args[index + 1])
    raise SystemExit("missing --request <path>")


def update_request(path: Path, max_output_tokens: int) -> None:
    payload = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(payload, dict):
        raise SystemExit("provider request must be a JSON object")
    payload["max_output_tokens"] = max_output_tokens
    fingerprint = payload.get("inference_parameter_fingerprint")
    budget_note = f"max_output_tokens={max_output_tokens}"
    if isinstance(fingerprint, str) and fingerprint:
        if "max_output_tokens=" not in fingerprint:
            payload["inference_parameter_fingerprint"] = f"{fingerprint};{budget_note}"
    else:
        payload["inference_parameter_fingerprint"] = budget_note
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    update_request(request_path(args.adapter_args), args.max_output_tokens)
    completed = subprocess.run([args.adapter, *args.adapter_args], check=False, env=os.environ.copy())
    return int(completed.returncode)


if __name__ == "__main__":
    sys.exit(main())
