# Validation Planning Prompt

Template: 1.0.0

Issue: 66

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/66/design.md

Diagram: .csdlc/prepared/issues/66/diagram.mmd

## Selected Lanes

[
  {
    "lane": "deepgram-provider-contract",
    "proof_role": "Exercise canonical profiles, typed requests, loopback HTTP construction, response parsing, media validation, redaction, timeout, malformed response, and stable error mapping.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 420,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "provider_tests",
      "deepgram_"
    ],
    "parallel_group": "provider",
    "defer_reason": null
  },
  {
    "lane": "deepgram-live-canary",
    "proof_role": "Run only deepgram_pluto_nova3_round_trip and retain .csdlc/evidence/66/deepgram-live-receipt.json containing redacted request identity, provider/model/voice, media, latency, usage, and cost fields with no source text, audio, or credentials.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "provider_tests",
      "deepgram_pluto_nova3_round_trip",
      "--",
      "--ignored"
    ],
    "parallel_group": "live-provider",
    "defer_reason": null
  },
  {
    "lane": "deepgram-focused-clippy",
    "proof_role": "Reject type, ownership, error, and dead-code regressions in the bounded library surface.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 420,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl/Cargo.toml",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "provider",
    "defer_reason": null
  },
  {
    "lane": "issue-diff-hygiene",
    "proof_role": "Reject malformed whitespace and patch artifacts before exact-head review.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "provider",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl/Cargo.toml --test provider_tests deepgram_`
- `cargo test --manifest-path adl/Cargo.toml --test provider_tests deepgram_pluto_nova3_round_trip -- --ignored`
- `cargo clippy --manifest-path adl/Cargo.toml --lib -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on credential exposure, unapproved endpoints, unsupported media, malformed provider responses, stale lifecycle truth, failed focused proof, or unresolved exact-head findings.

## Handoff

Retain typed evidence before convergence.
