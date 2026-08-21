# Structured Review Prompt

Template: 1.0.0

Issue: 450

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/450/audit.jsonl
.csdlc/issues/450/cards/sip.md
.csdlc/issues/450/cards/sip.values.json
.csdlc/issues/450/cards/sor.md
.csdlc/issues/450/cards/sor.values.json
.csdlc/issues/450/cards/spp.md
.csdlc/issues/450/cards/spp.values.json
.csdlc/issues/450/cards/srp.md
.csdlc/issues/450/cards/srp.values.json
.csdlc/issues/450/cards/stp.md
.csdlc/issues/450/cards/stp.values.json
.csdlc/issues/450/cards/vpp.md
.csdlc/issues/450/cards/vpp.values.json
.csdlc/issues/450/index.json
.csdlc/evidence/450/adl_memory_palace_projection.log
.csdlc/evidence/450/csm_memory_palace_readiness.log
.csdlc/evidence/450/kernel_memory_palace_packet.log
.csdlc/evidence/450/runtime_broad_lib_regression.log
.csdlc/evidence/450/runtime_memory_palace_service.log
.csdlc/prepared/issues/450/design.md
.csdlc/prepared/issues/450/diagram.mmd
adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/governed_operations.rs
adl-runtime-kernel/src/lib.rs
adl-runtime-kernel/src/memory_palace.rs
adl-runtime-kernel/src/memory_palace_authority.rs
adl-runtime-kernel/tests/assembly.rs
adl-runtime-kernel/tests/memory_palace.rs
adl-runtime/src/lib.rs
adl-runtime/src/memory_palace.rs
adl-runtime/src/supervision.rs
adl-runtime/src/topology.rs
adl/Cargo.lock
adl/Cargo.toml
adl/src/csm_runtime_api.rs
adl/src/memory_palace.rs
adl/tests/memory_palace_tests.rs

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

- Post-fix review was read-only and relied on scoped inspection plus local passing evidence, not a fresh broad coverage rerun.
- CI workspace coverage failure reproduced before the fix and the specific failing test passed locally after the fix.
- Worktree retained one untracked local lock file: .csdlc/locks/450.lock.

## Review Result

Revision: Some("git-blake3:758fab9892e9b1ee1613de4a9169f0425b274525:6508bd581fbd7c732da25e43c0e778c2b73699b37fb47296dfb8df3e66ead7e5")

Reviewer: Some("fresh-session:1d8d12b7-620a-4bc7-815a-bebcc3c5f23c")

Result: pass
