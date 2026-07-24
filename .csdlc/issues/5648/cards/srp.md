# Structured Review Prompt

Template: 1.0.0

Issue: 5648

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

csdlc-v2/src/lifecycle.rs
csdlc-v2/src/bin/csdlc-bind.rs
csdlc-v2/src/schema.rs
csdlc-v2/src/lib.rs
csdlc-v2/tests/gate2.rs
.csdlc/issues/5648

## Prompts

- Check operator authority and CAS boundaries
- Check phase/protected-path truth
- Check no direct state or secret leakage

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The operator authority marker is validated as explicit non-empty provenance; its external authorization remains an operator responsibility.

## Review Result

Revision: Some("git-blake3:ce7165c2429c70c1f8ee0741dba5f7e2f9066e47:cd76969c26526f6a08099f3b17e2124ce9b62c4c0c08dfe5e1128abd3e315f99")

Reviewer: Some("bounded-subagent-review-5648")

Result: pass
