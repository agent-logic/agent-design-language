# Validation Planning Prompt

Template: 1.0.0

Issue: 550

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/550/design.md

Diagram: .csdlc/prepared/issues/550/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csm-origin-generation",
    "proof_role": "Execute empty, localhost-only, public-only, combined, and invalid origin generation cases.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/550/validate-csm-origin-generation.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-api-wss",
    "proof_role": "Prove Runtime API and WSS configuration contract remains valid.",
    "acceptance_ids": [
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "runtime_api_wss"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-health-cors",
    "proof_role": "Execute exact allowed and forbidden Origin cases against the Runtime health endpoint.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "control",
      "observatory_cors_allows_only_configured_origins_and_reports_canonical_port"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "html-observatory-trust",
    "proof_role": "Prove the HTML Observatory accepts the configured Wuji Runtime API host and rejects arbitrary hosts.",
    "acceptance_ids": [
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "node",
      "demos/html-observatory/tests/security_privacy_adversarial.test.mjs"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "shell-and-diff",
    "proof_role": "Prove shell syntax and branch-range diff hygiene.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/550/validate-shell-and-diff.sh"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  },
  {
    "lane": "live-wuji",
    "proof_role": "Prove both Observatory pages, trusted Let's Encrypt TLS, all three Runtime browser reads for both exact origins, and a trusted WSS connection.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/550/validate-live-wuji.sh"
    ],
    "parallel_group": "live",
    "defer_reason": null
  },
  {
    "lane": "typed-review-publication",
    "proof_role": "Prove current typed issue integrity before exact-head review and closing publication.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "run",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--quiet",
      "--bin",
      "csdlc-validate",
      "--",
      "--root",
      ".",
      "issue",
      "--issue",
      "550"
    ],
    "parallel_group": "lifecycle",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/550/validate-csm-origin-generation.sh`
- `cargo test --manifest-path adl-runtime/Cargo.toml --test runtime_api_wss`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test control observatory_cors_allows_only_configured_origins_and_reports_canonical_port`
- `node demos/html-observatory/tests/security_privacy_adversarial.test.mjs`
- `bash .csdlc/prepared/issues/550/validate-shell-and-diff.sh`
- `bash .csdlc/prepared/issues/550/validate-live-wuji.sh`
- `cargo run --locked --manifest-path csdlc-v2/Cargo.toml --quiet --bin csdlc-validate -- --root . issue --issue 550`

## Failure Semantics

Fail closed on unsafe origin syntax, invalid port, generated-config replacement after invalid input, untrusted HTML Runtime API host, self-signed TLS, live endpoint failure, scope drift, stale review, or red CI.

## Handoff

Retain typed evidence before convergence.
