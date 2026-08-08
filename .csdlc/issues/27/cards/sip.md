# Structured Intent Prompt

Template: 1.0.0

Issue: 27

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make native Runtime v3 receipt validation order-independent while preserving fail-closed revision and digest checks.

## Required Outcome

Valid role sets pass regardless of order, duplicate roles fail, verifier-only post-proof changes are accepted, and product changes are rejected.

## Scope

- adl/tools/validate_v092_runtime_native_receipts.rb
- adl/tools/test_validate_v092_runtime_native_receipts.sh
- .csdlc/issues/27
- .csdlc/prepared/issues/27

## Authority

- Git revision and changed-path evidence
- Runtime v3 native receipt packet schema and digests

## Assumptions

- none

## Operator Constraints

- Do not modify or rerun Runtime v3 product soak work
- Do not touch the active WP-03 worktree
- Use only focused validation and FastWork storage
