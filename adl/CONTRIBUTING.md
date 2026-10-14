# Contributing to the ADL public CLI

This package provides the portable planning and installed-product CLI for Agent Design Language (ADL). Runtime implementation belongs to the [Runtime repository](https://github.com/agent-logic/agent-logic-runtime); lifecycle tooling belongs to [C-SDLC](https://github.com/agent-logic/cognitive-sdlc).

All contribution workflow, governance, and repository-wide policies are defined in the root:

- `../CONTRIBUTING.md`

This file exists only to clarify public CLI expectations and to prevent process drift.

---

## Before You Start

Please read:

- `../CONTRIBUTING.md` (canonical workflow + governance)
- `../docs/codex_playbook.md` (card-based PR workflow)
- `../docs/design_goals.md` (stable principles)

---

## Public CLI Expectations

In addition to the root contribution rules:

- Default unit tests remain **hermetic**. Integration and live-provider proof follows the declared PVF lane and resource/authorization policy; it is not part of the default unit-test promise.
- Public CLI changes must preserve deterministic planning semantics.
- Changes affecting ADL meaning must be proposed in `/adl-spec` first.
- Coverage discipline (>=80% per file or documented exception) applies.

If unsure whether a change affects language semantics vs runtime implementation, open an issue first.

---

## Focused Local Validation (from repository root)

```bash
bash tools/public_adl/run_cli_ci.sh
```

---

The public CLI defers to the root `CONTRIBUTING.md` for all process, workflow, and governance rules.
