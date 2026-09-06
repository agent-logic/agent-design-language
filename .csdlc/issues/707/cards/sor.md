# Structured Output Record

Template: 1.0.0

Issue: 707

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Repaired dependency-independent Runtime configuration identity, installed one coherent reviewed CSM/Guardian/Kernel generation under the single canonical Wuji service, and delivered two distinct live Beacon-to-Ember A2A conversations.

## Artifacts

- adl-runtime-kernel/src/config_generation.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/layer8_authority/mod.rs
- adl-runtime/src/bin/adl-runtime-guardian.rs
- adl/src/cli/csm_runtime_v3_cmd.rs
- adl/tools/test_runtime_v3_cross_binary_generation.sh
- .csdlc/evidence/707/live-a2a-results.json

## Execution

- Made configuration receipt identity canonical across independently resolved CSM, Guardian, and Kernel manifests while retaining fail-closed mismatch validation.
- Unified resident agent conversation execution so Shepherd is a role and every admitted communication-capable agent uses the same governed A2A path.
- Added Runtime-delegated signed identity, carried ACIP integrity, per-sender monotonic replay protection, roster eligibility, and durable Layer 8 audit.
- Made Contact and Continue an explicit all-to-all invariant for admitted communication-capable agents.
- Retired the duplicate Wuji launchd path and deployed the reviewed generation through the single canonical com.agentlogic.adl-runtime-v3 service.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/707/validate-publication-proof.sh"
    ],
    "purpose": "Aggregate retained proof: A2A tests 10/10, configuration tests 37/37, real CSM-Guardian-Kernel generation parity, cargo check, diff hygiene, two fresh live Beacon-to-Ember deliveries, and publication-head review PASS.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/707/live-a2a-results.json"
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
