# Structured Output Record

Template: 1.0.0

Issue: 514

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented shared provider inference-profile materialization with deterministic Ollama defaults, strict bounds, last-known-good retention, and redacted projection evidence.

## Artifacts

- adl/src/provider/profiles.rs
- adl/src/provider/mod.rs
- docs/provider/inference-profiles.md
- docs/milestones/v0.92.1/evidence/provider/prov-a/README.md
- .csdlc/prepared/issues/514/validate-profile-schema.rb
- .csdlc/prepared/issues/514/validate-ollama-materialization.rb
- .csdlc/prepared/issues/514/validate-invalid-profile.rb
- .csdlc/prepared/issues/514/validate-last-known-good.rb
- .csdlc/prepared/issues/514/validate-redaction.rb

## Execution

- Added bounded inference defaults to provider profile expansion, including provider_model_id, temperature, top_p, max_output_tokens, timeout_secs, and deterministic Ollama seed/materialization policy.
- Added last-known-good profile state retention and validate-before-activation semantics during profile expansion.
- Added a redacted provider profile projection that excludes credentials, tokens, keys, prompts, secrets, auth config, and private payloads from evidence surfaces.
- Added focused Rust coverage and deterministic PROV-A validation scripts for schema, Ollama materialization, invalid profiles, last-known-good retention, and redaction.
- Documented the shared provider inference-profile contract and issue-local evidence boundary.

## Validation

[
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/514/validate-invalid-profile.rb"
    ],
    "purpose": "Validate invalid profile parameters fail before activation.",
    "outcome": "passed",
    "evidence_ref": "invalid-profile.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/514/validate-last-known-good.rb"
    ],
    "purpose": "Validate profile state retains last-known-good materialization.",
    "outcome": "passed",
    "evidence_ref": "last-known-good.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/514/validate-ollama-materialization.rb"
    ],
    "purpose": "Validate deterministic Ollama provider profile materialization.",
    "outcome": "passed",
    "evidence_ref": "ollama-materialization.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/514/validate-profile-schema.rb"
    ],
    "purpose": "Validate shared provider inference profile schema materialization.",
    "outcome": "passed",
    "evidence_ref": "profile-schema.log"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/514/validate-redaction.rb"
    ],
    "purpose": "Validate profile evidence uses redacted projection boundaries.",
    "outcome": "passed",
    "evidence_ref": "redaction.log"
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
