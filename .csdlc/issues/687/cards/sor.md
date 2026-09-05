# Structured Output Record

Template: 1.0.0

Issue: 687

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented a provider/model inference-readiness taxonomy and routed resident Shepherd, dynamic-agent, roster, and control projections through it so component liveness no longer grants inference readiness.

## Artifacts

- adl-runtime-kernel/src/agent_roster.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/resident_shepherd.rs
- adl-runtime-kernel/tests/agent_roster.rs
- adl-runtime-kernel/tests/shepherd.rs
- adl-runtime-kernel/tests/assembly.rs
- adl-runtime-kernel/tests/control.rs
- .csdlc/evidence/687/validation-summary.json

## Execution

- Added explicit inference readiness states for unimplemented, unavailable, model_loading, failed, and ready with one projection denominator for presence, health, availability, activity, and communication eligibility.
- Changed resident Shepherd and dynamic provider-backed agent projection so only verified ready inference state is communication-eligible.
- Classified resident Shepherd recovery failures into unsupported adapter, unavailable provider/model, failed probe, loading, and ready states without adding live provider or cloud authority.
- Added deterministic no-cloud tests for roster taxonomy projection, resident Shepherd recovery, production provider placeholder rejection, and control projection compatibility.

## Validation

[
  {
    "command": [
      "see",
      ".csdlc/evidence/687/validation-summary.json"
    ],
    "purpose": "Retain exact local formatting, diff hygiene, compile, roster, shepherd, assembly, and control validation results without cloud/provider execution.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/687/validation-summary.json"
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
