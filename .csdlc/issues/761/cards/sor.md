# Structured Output Record

Template: 1.0.0

Issue: 761

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Complete lane denominators and deterministic samples implemented; six focused Python tests pass. Real exact #520 packet validates against independently reconstructed 5481 paths. Independent implementation review passed. Hosted CI and publication remain pending.

## Artifacts

- adl/tools/skills/repo-packet-builder/tests/fixtures/issue-520-changed-paths.txt
- adl/tools/skills/repo-packet-builder/tests/test_denominators.py

## Execution

- Classify every scoped path before per-lane category-balanced sampling.
- Record source/selected digests and exclusions and reject inconsistent packets.
- Honor diff scope and preserve whitespace/Unicode Git filenames.
- Retain exact #520 fixture and distinguish deterministic routing from semantic review.

## Validation

[
  {
    "command": [
      "python3",
      "adl/tools/skills/repo-packet-builder/tests/test_denominators.py",
      "-v"
    ],
    "purpose": "Exact #520 inventory, category samples, integrity negatives and CLI scope tests",
    "outcome": "passed",
    "evidence_ref": "packet-denominator-contract.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
