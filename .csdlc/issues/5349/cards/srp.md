# Structured Review Prompt

Template: 1.0.0

Issue: 5349

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/5349
.csdlc/locks/5349.lock
.csdlc/prepared/issues/5349
.csdlc/evidence/5349
adl-v2/crates/adl-adapters

## Prompts

- Do the direct dependencies exactly match the canonical WP-06 and WP-08 wave, with WP-07/#5591 transitive through #5341 and #5526 correctly downstream?
- Are preparation and future product protected paths disjoint from every active claim, with no shared-manifest or Runtime source write?
- Does every adapter have explicit preconditions, postconditions, stable errors, bounds, cancellation, and no hidden retry/scheduling/policy/signing authority?
- Can URL parsing, redirects, proxies, DNS/endpoint authority, oversized bodies, malformed JSON, header values, or cancellation bypass the HTTPS contract?
- Can any governed-tool input mint or widen authority, bypass Freedom Gate, suppress denial, execute a shell, alter evidence, or invoke a different tool?
- Do compatibility mappings reject unknown, ambiguous, lossy, extra-field, and alias-drift inputs without incumbent source reuse or silent fallback?
- Do exact COTS versions/features, LoC/module/test/time budgets, secret-canary proof, no-deferral matrix, rollback, and no-credential live-claim gate cover every acceptance criterion?

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
