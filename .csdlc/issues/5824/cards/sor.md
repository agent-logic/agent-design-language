# Structured Output Record

Template: 1.0.0

Issue: 5824

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Audited every restricted enum and scalar string surface in the prompt-card model, retained digest-bound dispositions for all 132 source-derived candidates, and proved that no additional finite enum conversion is currently justified.

## Artifacts

- .csdlc/evidence/5824/enum-inventory.json
- .csdlc/evidence/5824/enum-audit-decision.json
- .csdlc/prepared/issues/5824/validate-enum-inventory.rb
- csdlc-v2/tests/prompt_card_enum_typing.rs

## Execution

- Expanded the inventory validator to derive enum definitions and scalar string fields from structs and enum variants.
- Retained canonical inventory and audit-decision evidence with source, denominator, ownership, and disposition digests.
- Added focused integration coverage for enum parsing, tagged enums, all six tracked card round trips, active template parity, typed invalid-value diagnostics, and mutation safety.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5824/validate-enum-inventory.rb"
    ],
    "purpose": "Prove a complete source-derived denominator, one disposition per candidate, digest-bound ownership, and a finite-gap decision.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5824/enum-audit-decision.json"
  },
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--no-tests=fail",
      "--test",
      "prompt_card_enum_typing"
    ],
    "purpose": "Prove enum parsing and schema parity, tagged-enum rejection, all six card round trips, active template parity, typed invalid-value diagnostics, and mutation safety.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2/tests/prompt_card_enum_typing.rs"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Prove whitespace hygiene across the bounded audit, evidence, test, and lifecycle surfaces.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/5824/enum-audit-decision.json"
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
