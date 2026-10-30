# Contributing to ADL Runtime (ADL Reference Runtime)

ADL Runtime is the **reference runtime + CLI** for Agent Design Language (ADL).

All contribution workflow, governance, and repository-wide policies are defined in the root:

- `../CONTRIBUTING.md`

This file exists only to clarify ADL Runtime-specific expectations and to prevent process drift.

---

## Before You Start

Please read:

- `../CONTRIBUTING.md` (canonical workflow + governance)
- `../docs/codex_playbook.md` (card-based PR workflow)
- `../docs/design_goals.md` (stable principles)

---

## ADL Runtime-Specific Expectations

In addition to the root contribution rules:

- Default unit tests remain **hermetic**. Integration and live-provider proof follows the declared PVF lane and resource/authorization policy; it is not part of the default unit-test promise.
- Runtime changes must not alter deterministic planning semantics.
- Changes affecting ADL meaning must be proposed in `/adl-spec` first.
- Coverage discipline (>=80% per file or documented exception) applies.

If unsure whether a change affects language semantics vs runtime implementation, open an issue first.

---

## Quick Local Validation (from `adl/`)

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

---

ADL Runtime defers to the root `CONTRIBUTING.md` for all process, workflow, and governance rules.
