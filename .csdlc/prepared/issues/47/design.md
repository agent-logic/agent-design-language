# Issue 47 design: exact Rust validation-lane selectors

## Problem

A VPP lane declared `cargo test --manifest-path csdlc-v2/Cargo.toml schema`. Cargo interprets the final token as a test-name substring across every selected test target. It therefore entered the unrelated `estimation_contracts` integration binary instead of remaining inside the intended schema unit-test surface.

## Design

Classify typed Rust test commands by selector intent before accepting a VPP lane:

1. **Exact target lane** — the command names a Cargo target boundary such as `--lib`, `--test <name>`, `--bin <name>`, or another unambiguous target selector. Optional test-name filters operate only inside that declared target.
2. **Intentional broad lane** — the command intentionally selects the crate's normal broad test surface and carries no trailing substring that masquerades as a named lane.
3. **Ambiguous named lane** — a trailing free test-name substring is presented as if it were a target boundary without an exact Cargo target selector. Planning or validation rejects it with an actionable diagnostic.

The canonical schema proof becomes:

```text
cargo test --manifest-path csdlc-v2/Cargo.toml --lib schema::tests
```

This is target-exact at the Cargo level and selects a nonzero set of schema unit tests within the library. The implementation must not rewrite or reject truthful broad commands such as `cargo test --manifest-path csdlc-v2/Cargo.toml`; those intentionally exercise all normal targets.

## Acceptance model

| Input shape | Expected classification |
|---|---|
| `cargo test ... --lib schema::tests` | accept exact library lane; nonzero schema tests; no integration binaries |
| `cargo test ... --test gate2` | accept exact integration target |
| `cargo test ...` | accept intentional broad crate lane |
| `cargo test ... schema` | reject ambiguous substring selector |
| `cargo test ... --test` | reject missing target name |
| conflicting target selectors | reject ambiguous target boundary |

## Implementation implications

- Put selector classification in the typed VPP/validation planning boundary, not inside ordinary tests.
- Prefer a small typed enum for exact, broad, and invalid selector posture.
- Emit diagnostics that show the corrected command shape.
- Update active VPP/editor/planning guidance and examples that use Rust test selectors.
- Do not modify the unrelated estimation test or #5881 claim-removal behavior.

## Validation

Focused tests should prove classification and command selection without depending on a broad expensive build. A subprocess can enumerate the selected tests and assert the exact schema lane reports a nonzero schema set and never launches `estimation_contracts`. Separate fixtures prove the intentionally broad command remains accepted and ambiguous shapes fail before execution.

## Estimates

- Construction: 2-5 hours and approximately 10,000-25,000 model tokens.
- Focused local proof: up to 15 minutes and approximately 5,000 tokens.
- Review and bounded remediation: 30-60 minutes and approximately 8,000 tokens.

These estimates are reviewable planning inputs, not arbitrary limits on useful implementation or proof.
