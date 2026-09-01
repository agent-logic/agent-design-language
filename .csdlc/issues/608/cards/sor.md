# Structured Output Record

Template: 1.0.0

Issue: 608

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented Vertex AI Gemini provider support for native global endpoints and configurable thinking parameters, then remediated exact review findings by tying trusted Vertex hosts to the configured location, failing closed on invalid thinking_budget values, and redacting credential-adjacent proof details.

## Artifacts

- adl/src/provider/http_family.rs
- adl/src/provider/http_family/tests.rs
- .csdlc/prepared/issues/608/validate-provider.sh
- .csdlc/evidence/608/live-vertex/run-live-provider-proof.sh
- .csdlc/evidence/608/live-vertex/proof-summary.md
- .csdlc/evidence/608/vertex-provider-focused.log
- .csdlc/evidence/608/vertex-provider-live.log

## Execution

- Generated regional Vertex endpoints as <location>-aiplatform.googleapis.com and global Vertex endpoints as aiplatform.googleapis.com without endpoint overrides.
- Accepted only the configured regional host or the global host for credential-bearing Vertex calls unless an explicit custom endpoint trust override is set.
- Rendered thinkingLevel, thinkingBudget, and includeThoughts into generationConfig.thinkingConfig while rejecting mutually exclusive or invalid thinking settings.
- Added focused regressions for global endpoint derivation, global trust, configured regional trust, arbitrary aiplatform-looking host refusal, thinking config rendering, mutual exclusion, and invalid thinking_budget values.
- Preserved live Vertex proof through approved key-path routing while redacting local key path and service-account identity from committed evidence.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/608/validate-provider.sh"
    ],
    "purpose": "Prove endpoint derivation, request body rendering, strict trust policy, invalid config behavior, formatting, diff hygiene, and package compile.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/608/vertex-provider-focused.log; 6/6 focused Vertex tests passed and cargo check passed."
  },
  {
    "command": [
      "bash",
      ".csdlc/evidence/608/live-vertex/run-live-provider-proof.sh"
    ],
    "purpose": "Prove native provider calls against approved company Vertex endpoints without endpoint overrides for regional Gemini 2.5 and global Gemini 3.x models.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/608/vertex-provider-live.log; regional 2.5 matrix 3/3 and global 3.x matrix 6/6 succeeded."
  },
  {
    "command": [
      "rg",
      "-n",
      "ya29\\.|private_key|BEGIN PRIVATE|refresh_token|client_secret|/Users/daniel/keys|tf-bootstrap@",
      ".csdlc/evidence/608",
      ".csdlc/prepared/issues/608",
      "adl/src/provider/http_family.rs",
      "adl/src/provider/http_family/tests.rs"
    ],
    "purpose": "Prove committed #608 source, proof scripts, and evidence do not expose provider tokens, key material, local key paths, or service-account identity.",
    "outcome": "passed",
    "evidence_ref": "Local scan returned no matches after proof redaction."
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
