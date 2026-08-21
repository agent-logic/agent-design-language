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

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
