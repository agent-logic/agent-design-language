# Structured Intent Prompt

Template: 1.0.0

Issue: 5895

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the active installer and coexistence authority exactly match the claim-free v1-sunset C-SDLC v2 binary set.

## Required Outcome

Current-main installation succeeds without csdlc-migrate, selector and provenance match the exact source revision, and an installed claim-free lifecycle canary passes; if current main already satisfies this, close with exact evidence and no speculative code.

## Scope

- csdlc-v2/Cargo.toml
- csdlc-v2/operator/coexistence.json
- csdlc-v2/operator/generation-selector.json
- csdlc-v2/src/proof.rs
- csdlc-v2/tests/gate10a.rs
- adl/tools/install_owner_binaries.sh
- docs/tooling

## Authority

- Current installer/coexistence/selector/provenance surfaces only
- Historical evidence remains immutable
- Issue authority remains danielbaustin/agent-design-language#5895
- Code PR publication targets agent-logic/agent-design-language
- PR body must use Closes danielbaustin/agent-design-language#5895
- This is split issue/code publication authority, not repository cutover or issue migration

## Assumptions

- none

## Operator Constraints

- No AWS
- No broad product suite
- Do not restore csdlc-migrate or any wrapper
- Prefer no-code closure when current-main evidence already proves acceptance
