# Structured Task Prompt

Template: 1.0.0

Issue: 27

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Repair and prove only the native receipt validator policy.

## Deliverables

- Order-independent role denominator validation
- Duplicate-role and post-proof drift regression proof

## Acceptance

1. Canonical and observed artifact role denominators compare order-independently
2. Artifact roles remain unique and duplicates fail closed
3. Only an explicit verifier and issue-finalization allowlist may differ after the proof revision
4. Any runtime or product change, rename bypass, dirty worktree, or non-ancestor verifier revision is rejected
5. Digest recomputation and platform denominator checks remain intact

## Dependencies

- WP-03 validator introduction at committed revision 93641db996f2409baf94be2e9e6f27bb1ec9039b

## Inputs

- adl/tools/validate_v092_runtime_native_receipts.rb
- adl/tools/test_validate_v092_runtime_native_receipts.sh
- .csdlc/prepared/issues/27/design.md
- .csdlc/prepared/issues/27/diagram.mmd

## Non Goals

- Runtime, Guardian, kernel, TLS, or lifecycle behavior changes
- Native proof regeneration
- WP-03 branch mutation
