# Structured Output Record

Template: 1.0.0

Issue: 5701

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented canonical Runtime Core API v1 and Observatory API v1 OpenAPI 3.1 contracts plus route-parity validation for the currently reachable Runtime v3 Axum routes. Discovery serving remains a typed protected-path gate because active #5344 owns the router/config surfaces.

## Artifacts

- docs/api/runtime-v3/v1/openapi.json
- docs/api/runtime-v3/v1/observatory.openapi.json
- docs/api/runtime-v3/v1/API_VERSIONING.md
- adl-runtime-kernel/tests/openapi_contract.rs
- commit 645d66a5f

## Execution

- Added docs/api/runtime-v3/v1/openapi.json for the signed Runtime Core control endpoint
- Added docs/api/runtime-v3/v1/observatory.openapi.json for the authenticated Observatory snapshot and WSS endpoint
- Added docs/api/runtime-v3/v1/API_VERSIONING.md with independent Runtime Core and Observatory API versioning rules
- Added adl-runtime-kernel/tests/openapi_contract.rs to parse contracts, resolve local refs, reject phantom route claims, and check WSS frame documentation

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
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
