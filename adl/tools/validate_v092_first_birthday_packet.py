#!/usr/bin/env python3
import argparse
import json
import pathlib
import sys


def fail(message: str) -> None:
    raise SystemExit(f"first-birthday packet invalid: {message}")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("packet", type=pathlib.Path)
    parser.add_argument("--expect", choices=("complete", "rejected", "incomplete"), required=True)
    args = parser.parse_args()
    packet = json.loads(args.packet.read_text())
    if packet.get("schema") != "adl.first_birthday.demo_packet.v1":
        fail("unsupported schema")
    if packet.get("status") != args.expect:
        fail(f"expected {args.expect}, got {packet.get('status')}")
    if not packet.get("packet_sha256"):
        fail("missing packet digest")
    encoded = json.dumps(packet, sort_keys=True).lower()
    for forbidden in ("runtime-private-state-not-exported", "/users/", "/home/", "/private/", "github_pat_", "bearer "):
        if forbidden in encoded:
            fail(f"redaction failure: {forbidden}")
    if args.expect == "complete":
        if not packet.get("decision", {}).get("accepted"):
            fail("complete packet lacks accepted decision")
        for field in ("capability", "cognitive_profile", "witness_packet"):
            if not packet.get(field):
                fail(f"complete packet lacks {field}")
    else:
        if packet.get("capability") or packet.get("cognitive_profile") or packet.get("witness_packet"):
            fail("non-complete packet contains downstream authority")
        if not packet.get("rejections"):
            fail("non-complete packet lacks typed rejection")
    print(json.dumps({"schema": "adl.first_birthday.packet_validation.v1", "status": "pass", "packet": str(args.packet)}))


if __name__ == "__main__":
    main()
