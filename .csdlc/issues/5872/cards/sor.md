# Structured Output Record

Template: 1.0.0

Issue: 5872

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented bounded authenticated resource-weather advertisements with deterministic normalization, trust-domain and certificate binding, durable replay protection, signed withdrawal, explicit no-data semantics, and fail-closed validation.

## Artifacts

- adl-runtime/src/distributed/resource_weather.rs
- adl-runtime/tests/distributed_resource_weather.rs
- .csdlc/evidence/5872/execution-proof.json
- .csdlc/evidence/5872/negative-cases.json

## Execution

- Added the issue-owned unregistered resource-weather module with deterministic fixed-width projection and explicit unavailable values.
- Added AdvertisementSigning authorization, domain-separated canonical claims, freshness and expiry enforcement, and durable per-holder generation and sequence replay state.
- Added signed withdrawal and restart-safe no-data semantics without widening scheduling or authority.
- Added the temporary issue-owned #[path] integration harness with positive, negative, replay, resource-bound, corruption, and redaction coverage.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "distributed_resource_weather",
      "--no-tests=fail"
    ],
    "purpose": "Prove bounded deterministic normalization, certificate authorization, freshness, replay, withdrawal, resource bounds, corruption handling, and redaction for #5872.",
    "outcome": "passed",
    "evidence_ref": "exact-child-tests.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/5872/validate-proof-receipt.rb"
    ],
    "purpose": "Prove source and evidence revisions, exact artifact digests, nonzero native test receipt, negative-case corpus, and bounded runner identity for #5872.",
    "outcome": "passed",
    "evidence_ref": "exact-revision-proof-receipt.log"
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
