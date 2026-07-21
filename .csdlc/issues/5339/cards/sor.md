# Structured Output Record

Template: 1.0.0

Issue: 5339

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented a pure independent Rust language crate for the six ADL primitives with strict parsing, generated checked schema, semantic validation, deterministic canonicalization, stable diagnostics, and mapped #5337 characterization proof.

## Artifacts

- adl-v2/crates/adl-language
- adl-v2/crates/adl-language/schema/adl-document.schema.json
- adl-v2/crates/adl-language/CHARACTERIZATION_PARITY.md

## Execution

- Added typed provider, tool, agent, task, workflow, and singular run models with strict unknown-field rejection
- Added duplicate-key-safe YAML and JSON parsing plus version, identity, reference, state, cycle, and run-target validation
- Added deterministic canonical JSON, a checked schema generator, focused tests, and an explicit #5337 corpus parity map
- Kept compiler expansion, runtime execution, provider invocation, control-plane, cloud, storage, and migration outside the crate

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/5339/validate-language.sh",
      "focused|quality|parity|budgets"
    ],
    "purpose": "Prove the six-primitives language model, strict parsing and schema alignment, semantic diagnostics, canonical ordering, #5337 corpus mapping, dependency boundary, LoC budgets, and latency budget.",
    "outcome": "passed",
    "evidence_ref": "Focused: 9 tests passed. Quality: strict Clippy passed. Parity: 3 mapped corpus tests passed. Budgets: 637 implementation lines, 254 test lines, exact five-dependency COTS set, no forbidden dependency family, warm all-target validation 0 seconds. git diff --check passed."
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
