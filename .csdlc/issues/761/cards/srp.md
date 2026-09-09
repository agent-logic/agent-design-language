# Structured Review Prompt

Template: 1.0.0

Issue: 761

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

adl/tools/skills/repo-packet-builder/references/output-contract.md
adl/tools/skills/repo-packet-builder/scripts/build_repo_packet.py
adl/tools/skills/repo-packet-builder/scripts/validate_repo_packet.py
adl/tools/skills/repo-packet-builder/tests/README.md
adl/tools/skills/repo-packet-builder/tests/fixtures/issue-520-changed-paths.txt
adl/tools/skills/repo-packet-builder/tests/fixtures/issue-520-fixture-provenance.json
adl/tools/skills/repo-packet-builder/tests/test_denominators.py
adl/tools/skills/repo-review-synthesis/references/output-contract.md

## Prompts

- Verify full inventory precedes selection and samples are category-valid.
- Verify exact fixture, integrity negatives and honest coverage claims.

## Findings

[
  {
    "id": "R761-001",
    "severity": "p2",
    "summary": "NUL Git filenames were trimmed; fixed and regression-tested for whitespace and Unicode.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:e08a060104cd46a09b01cc31ff9c744f94c6cbcc:6755825fde8fc5be7d0e5561a2172eb0da4ca3b22aeb65b4f864825864262f59",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Path classification and samples do not prove semantic review completion.

## Review Result

Revision: Some("git-blake3:e08a060104cd46a09b01cc31ff9c744f94c6cbcc:6755825fde8fc5be7d0e5561a2172eb0da4ca3b22aeb65b4f864825864262f59")

Reviewer: Some("codex:review_761")

Result: pass
