# Structured Output Record

Template: 1.0.0

Issue: 341

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the bounded v0.92 WP-18B provider-neutral multi-agent proof slice with real-provider positive evidence, negative-case validation, redacted ACIP traces, and a temporary issue-local Runtime-v3 Observatory overlay for private roster display.

## Artifacts

- adl/tools/demo_v092_provider_neutral_birthday.sh
- adl/tools/validate_v092_provider_neutral_proof.py
- adl/tools/test_v092_provider_neutral_proof.sh
- adl/tools/serve_v092_provider_neutral_observatory_api.py
- demos/v0.92/provider-neutral-birthday
- .csdlc/evidence/341
- demos/html-observatory/app.js
- adl/tools/test_html_observatory.sh

## Execution

- Added the provider-neutral birthday proof harness, validator, local proof test, and redacted retained proof matrices/traces.
- Captured live-provider positive proof for OpenAI and Anthropic columns through the same scenario and ACIP operation contract without retaining raw prompts, outputs, credentials, or private payloads.
- Added negative-case proof for malformed ACIP, denied authority, interrupted provider, provider unavailable, provider loss, and provider substitution outcomes.
- Added a temporary issue-local Runtime-v3 Observatory overlay feed that projects three actual direct-TCP ACIP agents: OpenAI reference, Gemini reference, and Wuji shepherd; ordinary agents expose no SSM access and shepherd is maintenance-only.
- Repaired the HTML Observatory live Runtime-v3 badge so an authoritative live feed shows CSM Runtime instead of stale fallback shell state.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_v092_provider_neutral_proof.sh"
    ],
    "purpose": "Run focused provider-neutral proof tests including validator denial cases and three-agent Runtime-v3 overlay feed generation from the current Git HEAD.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/341/local-test/validator-pass.log"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_html_observatory.sh"
    ],
    "purpose": "Run the HTML Observatory Runtime-v3 contract test including the live CSM Runtime badge projection.",
    "outcome": "passed",
    "evidence_ref": "stdout"
  },
  {
    "command": [
      "python3",
      "adl/tools/validate_v092_provider_neutral_proof.py",
      "demos/v0.92/provider-neutral-birthday/proof-matrix-positive.json",
      "--require-live"
    ],
    "purpose": "Validate the retained live-provider positive matrix for provider parity, same-agent/same-provider response linkage, and redaction.",
    "outcome": "passed",
    "evidence_ref": "stdout"
  },
  {
    "command": [
      "python3",
      "adl/tools/validate_v092_provider_neutral_proof.py",
      "demos/v0.92/provider-neutral-birthday/proof-matrix-observatory.json",
      "--require-observatory"
    ],
    "purpose": "Validate the retained private Observatory matrix with three real TCP agents and SSM boundary truth.",
    "outcome": "passed",
    "evidence_ref": "stdout"
  },
  {
    "command": [
      "python3",
      "-m",
      "py_compile",
      "adl/tools/validate_v092_provider_neutral_proof.py",
      "adl/tools/serve_v092_provider_neutral_observatory_api.py"
    ],
    "purpose": "Compile Python proof and temporary overlay helper scripts.",
    "outcome": "passed",
    "evidence_ref": "stdout"
  },
  {
    "command": [
      "bash",
      "-n",
      "adl/tools/demo_v092_provider_neutral_birthday.sh",
      "adl/tools/test_v092_provider_neutral_proof.sh"
    ],
    "purpose": "Check shell syntax for the proof harness and focused test lane.",
    "outcome": "passed",
    "evidence_ref": "stdout"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Reject whitespace and patch hygiene errors.",
    "outcome": "passed",
    "evidence_ref": "stdout"
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

- The issue-local Runtime-v3 overlay is a temporary #341 demonstration bridge only; v0.92.1 Observatory work should serve the worktree HTML/runtime path directly against the real runtime API.
