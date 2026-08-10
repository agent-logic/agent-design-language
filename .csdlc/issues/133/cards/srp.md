# Structured Review Prompt

Template: 1.0.0

Issue: 133

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/133
.csdlc/prepared/issues/133
.csdlc/evidence/133
adl-runtime/src/distributed/certificates.rs
adl-runtime/src/distributed/failure_detection.rs
adl-runtime/src/distributed/lease.rs
adl-runtime/src/distributed/fencing.rs
adl-runtime/src/distributed/placement.rs
adl-runtime/src/distributed/migration.rs
adl-runtime/src/distributed/recovery.rs
adl-runtime/src/distributed/membership.rs
adl-runtime/tests/distributed_authority_snapshots.rs

## Prompts

- Can any caller synthesize rows or snapshots without the owning authority?
- Does every mutation, removal, replacement, and restore update or preserve revision truth correctly?
- Are rows complete, deterministic, bounded, and explicit about unavailable state?
- Can any private key, raw probe, signature, migration payload, or recovery payload escape through the snapshot APIs?
- Do focused tests prove N/N+1 drift and restart parity across all five authorities?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Production module registration and integration remain owned by #5878.

## Review Result

Revision: Some("git-blake3:edbe72499835fba8cd67b0793555ac5dbb709fee:55e843686ee2b949badbc799ca879a2c3166abba1c56fe968f10d2bc327f998f")

Reviewer: Some("/root/prepare_5875_release")

Result: pass
