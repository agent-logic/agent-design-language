# Structured Task Prompt

Template: 1.0.0

Issue: 341

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Bring #341 to reviewable publication-ready PR state for provider-neutral proof and private Observatory demonstration; stop before merge unless separately authorized.

## Deliverables

- provider-neutral scenario harness
- proof validator
- focused test runner
- redacted provider receipts and artifact digests under .csdlc/evidence/341
- feature projection document
- private Observatory demo evidence showing several agents running

## Acceptance

1. AC-1: At least two approved real providers complete the identical versioned scenario through equivalent ACIP operations.
2. AC-2: Provider identity/capability truth and bounded semantic differences are retained without credentials, private prompts, or private payloads.
3. AC-3: Malformed, denied, interrupted, unavailable, provider-loss, and substitution cases have visible non-pass outcomes.
4. AC-4: One provider failure leaves Runtime and unrelated agents available, with macOS/Linux tooling posture recorded.
5. AC-5: Exact-head review has no unresolved actionable finding.
6. AC-6: A private Observatory demonstration shows several agents running after proof success, without claiming public exposure or production deployment.

## Dependencies

- #256 closed completed and ancestral to current origin/main
- #414 closed completed and ancestral to current origin/main
- two approved real-provider credential sources available at execution time
- existing ACIP and birthday scenario contracts

## Inputs

- https://github.com/agent-logic/agent-design-language/issues/341
- https://github.com/agent-logic/agent-design-language/issues/256
- https://github.com/agent-logic/agent-design-language/issues/414
- existing ACIP contracts
- existing birthday scenario proof artifacts
- existing Runtime/Observatory local demo tooling

## Non Goals

- identical prose or token usage across providers
- every possible provider
- changing ACIP or the birthday scenario
- publishing credentials, private prompts, or raw provider payloads
- production CloudFront/API Gateway/load-balancer ingress
- GPU quota remediation
- new Runtime recovery or snapshot implementation
