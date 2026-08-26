# Structured Review Prompt

Template: 1.0.0

Issue: 544

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v2/src/lifecycle.rs
csdlc-v2/tests/primary_checkout_bootstrap_guard.rs
docs/onboarding.md
csdlc-v2/README.md
.csdlc/issues/544
.csdlc/prepared/issues/544/design.md
.csdlc/prepared/issues/544/diagram.mmd

## Prompts

- Does the guard use Git topology authority rather than branch heuristics?
- Does the guard run before all initialization writes, including design, diagram, issue, prepared, and lock surfaces?
- Does every topology-listing, parsing, canonicalization, and common-dir ambiguity fail before writes?
- Does it preserve non-primary staging checkout bootstrap and idempotent reconciliation?
- Does it leave csdlc-bind FastWork policy unchanged?
- Are tests isolated from the real repository and do they prove zero residue?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Review was bounded to issue #544 scope and did not assess merge or finish readiness.

## Review Result

Revision: Some("git-blake3:3d1529fb8e19475b34d14ccee5341d3085bde5ad:b6300c24095bf7c565cd33530567d56aae891232b462eafd79dd6838ef9ab015")

Reviewer: Some("fresh-session:01a03f42-ba25-79b2-bec5-5604c8f53592")

Result: pass
