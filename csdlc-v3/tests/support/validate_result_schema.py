#!/usr/bin/env python3
"""PVF: deterministic small local schema proof for #868/SIM-07; no effects.

Requires jsonschema==4.26.0. Input is a nonempty JSON array of objects with
unique `label` and actual command `envelope` fields. No schema-only result
proves installed behavior; the caller must retain the producer provenance.
"""
import argparse
import copy
import json
from pathlib import Path

from jsonschema import Draft202012Validator


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--schema", required=True, type=Path)
    parser.add_argument("--results", required=True, type=Path)
    args = parser.parse_args()
    schema = json.loads(args.schema.read_text())
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema)
    cases = json.loads(args.results.read_text())
    if not isinstance(cases, list) or not cases:
        raise SystemExit("nonempty labeled result array required")
    seen = set()
    failures = []
    for case in cases:
        label = case.get("label")
        if not isinstance(label, str) or not label or label in seen:
            raise SystemExit("unique nonempty case labels required")
        seen.add(label)
        errors = list(validator.iter_errors(case.get("envelope")))
        if errors:
            failures.append({"label": label, "invalid_paths": [
                "/".join(map(str, error.absolute_path)) for error in errors
            ]})
    if failures:
        print(json.dumps({"status": "failed", "cases": len(cases), "failures": failures}))
        raise SystemExit(1)

    seed = cases[0]["envelope"]
    negatives = []
    for field in schema["required"]:
        changed = copy.deepcopy(seed)
        del changed[field]
        negatives.append(("missing_" + field, changed))
    for field, value in [
        ("status", "invented_status"),
        ("effects", {"class": "observation", "outcome": "invented_effect"}),
        ("issue", {"status": "identified"}),
        ("issue", {"status": "identified", "number": 0}),
        ("issue", {"status": "not_applicable", "number": 868}),
        ("timing", {"started_unix_ms": 0, "monotonic_duration_ms": -1}),
    ]:
        changed = copy.deepcopy(seed)
        changed[field] = value
        negatives.append(("invalid_" + field, changed))
    accepted = [name for name, value in negatives if validator.is_valid(value)]
    print(json.dumps({"status": "failed" if accepted else "passed",
                      "valid_results": len(cases), "negative_cases": len(negatives),
                      "accepted_invalid_cases": accepted}))
    if accepted:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
