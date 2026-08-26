# Structured Review Prompt

Template: 1.0.0

Issue: 259

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/distributed/authority_protocol.rs
adl-runtime/src/distributed/authority_reconciliation.rs
adl-runtime/src/distributed/transport/core.rs
adl-runtime/src/distributed/transport/governed/learner_transport/tests.rs
adl-runtime/src/distributed/transport/governed/polis_runtime.rs
adl-runtime/tests/distributed_discovery.rs
adl-runtime/tests/distributed_runtime_transport.rs
adl-runtime/tests/distributed_transport.rs
.csdlc/issues/259
.csdlc/prepared/issues/259
.csdlc/evidence/259

## Prompts

- Review every acceptance criterion AC-1 through AC-5 against the assigned immutable revision, approved design, exact changed scope, and retained typed validation evidence.
- Review code correctness and authorization security: production TransportAuthorization and PolisRuntimeAuthorityBootstrap must consume #258 AuthorityBoundCertificateStore; any direct raw-store transport seam must be cfg(test), crate-private, and unavailable to dependent crates; all adapter errors must fail closed.
- Review test and evidence sufficiency: dependent integration proof must publish reconciliation authority and obtain its handle through AuthorityStoreAdapterRegistry, focused positive and negative transport behavior must be covered, and retained SOR/VPP evidence must truthfully match execution.
- Review scope boundaries: #259 must not absorb #260 non-transport caller migration, #203 parent integration, #205 serving eligibility, or mutate preserved #203 worktrees.
- Report findings first in P0 through P3 order with repository-relative file and line evidence, explicitly state review and validation limitations, remain read-only without editing Git, lifecycle, PR, or GitHub state, and return PASS only when no actionable finding remains.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Reviewer independently reran distributed_runtime_transport 14/14; retained exact-head evidence and owner reruns prove distributed_transport 15/15, distributed_discovery 13/13, focused strict Clippy, and diff-check, while GitHub CI remains the remote integration authority.

## Review Result

Revision: Some("git-blake3:375a149b567bb3b87958d9305624a495e3cc4ee7:dd570abc64883edbca3be8c448be9933202cf467c2eb46916eaee01c99d814f4")

Reviewer: Some("fresh-session:8c75e3e0-50b7-4e58-b0dc-fec5d9f1b493")

Result: pass
