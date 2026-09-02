# Structured Output Record

Template: 1.0.0

Issue: 592

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the governed Polis Vertex AI configuration path without live paid GCP calls. Runtime init now supports an explicit `polis.vertex_ai` block with provider, GCP project, Vertex location, model, and redacted credential-source selection; the deployed runtime init file names the Vertex AI route explicitly; operator documentation records credential boundaries and failure classifications; focused tests prove parsing, rejection of ambient/unsafe shapes, and deterministic failure buckets.

## Artifacts

- adl-runtime-kernel/src/config.rs
- adl-runtime-kernel/tests/configuration.rs
- infra/runtime-v3/runtime-init.toml
- docs/runtime/VERTEX_AI_POLIS_CONFIGURATION.md
- .csdlc/prepared/issues/592/bind-request.json
- .csdlc/prepared/issues/592/validate-528-merge.sh
- .csdlc/prepared/issues/592/validate-vertex-config-docs.sh
- .csdlc/prepared/issues/592/validate-runtime-vertex-ai.sh
- .csdlc/prepared/issues/592/validate-tooling-canary.sh

## Execution

- Added typed Runtime init support for `polis.vertex_ai` with explicit `vertex_ai` provider selection, GCP project, Vertex location, model, and credential-source fields.
- Added validation that rejects missing ambient GCP project selection, non-Vertex provider selection, unsafe labels, and relative service-account credential paths.
- Added deterministic Vertex AI provider failure classification for missing credentials, disabled Vertex APIs, project/location mismatch, quota/auth, model/request, and transport failures.
- Configured `infra/runtime-v3/runtime-init.toml` to select Vertex AI for the Agent Logic development Polis using Application Default Credentials without storing secret material.
- Added `docs/runtime/VERTEX_AI_POLIS_CONFIGURATION.md` documenting the GCP project/location/model, credential boundary, redaction rules, failure buckets, and deferred paid-call boundary.
- Strengthened the issue-owned validators so they check the implemented runtime config, docs, failure classifier, retained bind request, and secret-marker hygiene.

## Validation

[]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
