# Structured Output Record

Template: 1.0.0

Issue: 608

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented native Vertex Gemini global endpoint routing, first-party global endpoint trust, and config-backed thinking controls for #608.

## Artifacts

- adl/src/provider/http_family.rs
- adl/src/provider/http_family/tests.rs
- .csdlc/prepared/issues/608/validate-provider.sh
- .csdlc/evidence/608/live-vertex/run-live-provider-proof.sh
- .csdlc/evidence/608/live-vertex/vertex_2_5_model_matrix_native_endpoint.adl.yaml
- .csdlc/evidence/608/live-vertex/vertex_global_3x_model_matrix_native_endpoint.adl.yaml
- .csdlc/evidence/608/live-vertex/proof-summary.md

## Execution

- Added global endpoint derivation so location global uses aiplatform.googleapis.com while regional locations keep <region>-aiplatform.googleapis.com.
- Accepted aiplatform.googleapis.com as a trusted first-party Vertex endpoint for credentialed calls.
- Added config-backed thinking_level, thinking_budget, and include_thoughts request-body rendering.
- Rejected simultaneous thinking_level and thinking_budget to avoid mixing model-family contracts.
- Added focused unit tests for endpoint derivation and thinking config behavior.
- Added issue-owned focused and live Vertex proof scripts.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/608/validate-provider.sh"
    ],
    "purpose": "Run the issue-owned #608 local validator.",
    "outcome": "passed",
    "evidence_ref": "vertex-provider-focused.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/evidence/608/live-vertex/run-live-provider-proof.sh"
    ],
    "purpose": "Run the issue-owned #608 live Vertex provider proof.",
    "outcome": "passed",
    "evidence_ref": "vertex-provider-live.log"
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
