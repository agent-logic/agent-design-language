# Structured Intent Prompt

Template: 1.0.0

Issue: 5337

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Implement and prove an independent, versioned characterization and determinism corpus for pinned ADL v1 behavior.

## Required Outcome

A complete adl-characterization crate, versioned corpus, narrow normalizer contract, at least three retained v1 observations per case, coverage map, deterministic reports, focused and full tests, exact-revision review, and typed publication all pass with no deferred criteria.

## Scope

- independent adl-characterization Rust crate and CLI
- versioned positive and negative corpus fixtures and schema
- pinned-v1 repeated raw and normalized observation evidence
- normalizer contract, coverage map, deterministic reports, tests, and documentation
- issue-local typed lifecycle, validation, review, and publication records

## Authority

- Issue #5337 and the operator instruction authorize the complete WP-03 implementation
- Pinned revision 19c2b6e2ad18bddc75db9231643a54b2a446ce72 is behavioral evidence only
- The independent harness invokes a caller-supplied v1 binary and does not depend on incumbent ADL Rust code
- Typed C-SDLC v2 binaries and records are lifecycle authority
- No credentialed, network, remote, or AWS provider execution is authorized

## Assumptions

- the pinned v1 revision can be built locally with Cargo output on /Volumes/FastWork
- ADL_OBSERVABILITY=0 suppresses ordinary observability noise for black-box capture
- fixed local fixtures and mock providers are sufficient to exercise the declared behavioral surface

## Operator Constraints

- Use installed typed C-SDLC v2 binaries and card-editor semantics only
- Implement every acceptance criterion in this issue; do not defer product work
- Do not edit incumbent adl, Runtime v2, main, sibling worktrees, or shared milestone files
- Use /Volumes/FastWork for Cargo output
- Do not use AWS, raw gh, credentials, or network providers
- Use COTS crates for parsing, schema validation, assertions, and temporary files
