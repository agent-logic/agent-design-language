#!/usr/bin/env python3
"""Validate packet denominator and samples against an independent source inventory."""
import argparse
import json
from pathlib import Path
from build_repo_packet import validate_packet


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('packet', type=Path)
    parser.add_argument('--source-inventory', type=Path, required=True,
                        help='Independent complete sorted path list, one path per line')
    args = parser.parse_args()
    try:
        def read(name):
            return json.loads((args.packet / name).read_text())
        validate_packet(read('lane_denominators.json'), read('evidence_index.json')['evidence'],
                        read('specialist_assignments.json')['assignments'],
                        args.source_inventory.read_text().splitlines())
    except (ValueError, KeyError, TypeError, OSError) as error:
        parser.exit(1, f'packet validation failed: {error}\n')
    print('Packet denominator and sample integrity passed; semantic review remains unproven.')
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
