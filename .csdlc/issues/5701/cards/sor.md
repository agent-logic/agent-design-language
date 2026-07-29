# Structured Output Record

Template: 1.0.0

Issue: 5701

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented complete versioned Core and Observatory OpenAPI contracts and mounted their real authenticated Runtime v3 routes on the guardian-launched Axum/Tokio/Rustls listener.

## Artifacts

- docs/api/runtime-v3/v1/openapi.json
- docs/api/runtime-v3/v1/observatory.openapi.json
- docs/api/runtime-v3/v1/API_VERSIONING.md
- adl-runtime-kernel/tests/openapi_contract.rs
- commit 645d66a5f
- docs/api/runtime-v3/v1/openapi.json
- docs/api/runtime-v3/v1/observatory.openapi.json
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/openapi_contract.rs
- .adl/local-artifacts/5701-gemini-review/result-final.json

## Execution

- Added docs/api/runtime-v3/v1/openapi.json for the signed Runtime Core control endpoint
- Added docs/api/runtime-v3/v1/observatory.openapi.json for the authenticated Observatory snapshot and WSS endpoint
- Added docs/api/runtime-v3/v1/API_VERSIONING.md with independent Runtime Core and Observatory API versioning rules
- Added adl-runtime-kernel/tests/openapi_contract.rs to parse contracts, resolve local refs, reject phantom route claims, and check WSS frame documentation
- Serve embedded Core and Observatory OpenAPI 3.1 documents plus human-readable Swagger UI without a sidecar server or runtime CDN.
- Expose authenticated /v1/health and /v1/metrics from live runtime state.
- Expose authenticated full-duplex /v1/acip/ws using bounded Protobuf frames, replay sequencing, canonical ingress, and real ACIP adapter dispatch.
- Replace opaque payload contracts with bounded typed schemas and align shared code-generation types across both documents.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "/Volumes/FastWork/adl-wp-5701/target",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Focused #5701 lint validation",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-openapi-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--target-dir",
      "/Volumes/FastWork/adl-wp-5701/target",
      "--test",
      "openapi_contract"
    ],
    "purpose": "Focused #5701 contract validation",
    "outcome": "passed",
    "evidence_ref": "runtime-v3-openapi-contract.log"
  },
  {
    "command": [
      "cargo",
      "test-and-clippy",
      "runtime-v3-openapi-focused-suite"
    ],
    "purpose": "Prove the versioned route inventory, typed OpenAPI contracts, real Observatory and ACIP WSS behavior, strict Rust quality, clean diff, live HTTPS documentation, authenticated health and metrics, and ready Vector observability pipeline.",
    "outcome": "passed",
    "evidence_ref": "Kernel control 21/21; OpenAPI contracts 6/6; Observatory WSS 5/5; runtime API docs 2/2; runtime API WSS 2/2; ACIP 5/5; strict kernel Clippy passed; git diff --check passed; live https://localhost:20997 returned 200 for both Swagger UIs and both specs, authenticated health observability_ready=true, authenticated metrics health=ready, and zero ERROR/FATAL master-log events."
  },
  {
    "command": [
      "cargo",
      "test-and-clippy",
      "runtime-v3-openapi-review-remediation"
    ],
    "purpose": "Prove sole production OpenAPI authority, versioned route parity, canonical ACIP dispatch, failure-safe replay sequencing, first-frame Observatory authentication, guardian persistence, and strict Rust quality.",
    "outcome": "passed",
    "evidence_ref": "Kernel control 21/21; guardian soak 7/7; Observatory WSS 6/6; OpenAPI contracts 6/6; adl-runtime library 134/134; runtime API WSS 2/2; strict all-target kernel Clippy passed; strict all-target adl-runtime Clippy passed; git diff --check passed."
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
