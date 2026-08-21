# Structured Review Prompt

Template: 1.0.0

Issue: 450

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/450/audit.jsonl
.csdlc/issues/450/cards/sip.values.json
.csdlc/issues/450/cards/sor.md
.csdlc/issues/450/cards/sor.values.json
.csdlc/issues/450/cards/spp.values.json
.csdlc/issues/450/cards/srp.md
.csdlc/issues/450/cards/srp.values.json
.csdlc/issues/450/cards/stp.values.json
.csdlc/issues/450/cards/vpp.values.json
.csdlc/issues/450/index.json
adl/tests/memory_palace_tests.rs
docs/planning/ADL_FEATURE_LIST.md
docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md
docs/milestones/v0.92/README.md
docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md

## Prompts

- Verify that the kernel is the only topology and working-set authority.
- Verify exact identity continuity generation source-reference and digest agreement across the adapter.
- Verify restart rejects rollback gaps duplicates forgery and incompatible schema.
- Verify existing resident behavior and no-configuration behavior remain intact.
- Verify #446 and unrelated ObsMem/Runtime redesign remain out of scope.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review was scoped to the #450 AC8 docs/assertion repair and lifecycle metadata only.
- Reviewer reported PASS with no actionable P1/P2 findings.
- Reviewer explicitly did not inspect PR #455 or the protected #446 paths.
- Local validation passed: cargo fmt --manifest-path adl/Cargo.toml --all -- --check; cargo test --manifest-path adl/Cargo.toml --test memory_palace_tests -- --nocapture.

## Review Result

Revision: Some("git-blake3:c33e65009d105b27f7f1dc557f5d95b3aca305af:4c74b3c9dad4579ebb7a47b1c6170e1ea93b03f8ed106009a42d41976ed7c379")

Reviewer: Some("fresh-session:e14a6aa9-a471-4d29-9120-75cd4c0e96f9")

Result: pass
