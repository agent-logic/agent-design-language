# Structured Intent Prompt

Template: 1.0.0

Issue: 341

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement and prove the WP-18B provider-neutral multi-agent scenario with at least two approved real providers, redacted proof artifacts, negative cases, and a private Observatory demonstration with several agents running.

## Required Outcome

A reviewable PR for #341 that truthfully proves provider-neutral multi-agent execution across at least two approved real providers, retains redacted artifact digests and negative cases, and demonstrates several private agents running in the Observatory without widening into public exposure.

## Scope

- current-repository successor for legacy danielbaustin/agent-design-language#5838
- provider-neutral WP-18B proof over landed ACIP/Runtime prerequisites
- two-provider positive matrix with identical scenario semantics
- negative proof for malformed, denied, interrupted, unavailable, loss, and substitution cases
- redacted artifacts and digest-backed validator
- private Observatory demonstration with several agents running after proof success

## Authority

- #341 consumes #256 and #414 terminal Runtime prerequisites; it does not own their implementation paths.
- #341 owns provider-neutral proof harness, validator, redacted receipts, failure matrix, and feature projection only.
- Observatory public exposure, production CloudFront/API Gateway/load-balancer ingress, GPU quota work, and broader Runtime architecture are separate lanes.
- Provider credentials are execution inputs only and must never be printed, committed, or published.
- C-SDLC v2 typed records are lifecycle authority; raw GitHub writes are not used for covered lifecycle state.

## Assumptions

- none

## Operator Constraints

- Use the #341 FastWork worktree and do not write tracked product changes on main.
- Use typed C-SDLC v2 for lifecycle and covered GitHub writes.
- Do not leave AWS instances running; networks/subnets may remain, instances must be stopped/terminated after any cloud run.
- Prefer private/local Runtime proof first; only use cloud/provider resources when required and record teardown truth.
- Do not absorb Observatory public exposure or production ingress work.
