# Structured Intent Prompt

Template: 1.0.0

Issue: 360

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Add a test-only authentic builder for distinct sealed Observatory transitions.

## Required Outcome

Authentic tests can construct distinct Acquire Renew Transfer and Revoke projections through the real #350/#358 verifier without any production authority change.

## Scope

- Test-only distinct Observatory transition descriptor
- Real PublishedAuthorityResult artifact cut cross-binding
- Authentic positive and negative transition fixture proof

## Authority

- Every helper is cfg(test) or internal-test-fixtures only
- Production verifier schema and sealed construction remain unchanged
- No raw quorum membership OwnerCommit lease or artifact authority is exposed

## Assumptions

- none

## Operator Constraints

- Base is exact origin/main cd0feef31240b95d344c5ae9b774325506586a5d
- Own exactly authority_protocol.rs serving_authority.rs and existing projection integration test
- No #274 production module or mod.rs edit
