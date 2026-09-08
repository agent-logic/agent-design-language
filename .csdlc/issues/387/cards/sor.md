# Structured Output Record

Template: 1.0.0

Issue: 387

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Remediated PR #389 csdlc-v2-standalone CI failure by narrowing implemented-phase card-truth recovery guards: assignment-only review recovery no longer authorizes implemented card repair, SIP required-outcome repair remains immediate-only after recorded-review recovery, and SPP summary repair closes after one correction.

## Artifacts

- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs

## Execution

- csdlc-v2/src/store.rs: require implemented pre-publication recovery to follow an actual recorded review before guarded card-truth repairs.
- csdlc-v2/src/store.rs: keep SIP required-outcome recovery immediate-generation only.
- csdlc-v2/src/store.rs: make SPP plan-summary correction one-shot within the recovery epoch.
- csdlc-v2/tests/gate5.rs: add/repair regressions for assignment-only recovery rejection, one-shot SPP summary repair, SIP immediate-only behavior, and the #387 comprehensive repair path.

## Validation

[
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--check"
    ],
    "purpose": "Reject formatting drift after CI-red remediation.",
    "outcome": "passed",
    "evidence_ref": "terminal output: cargo fmt --check exited 0"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5"
    ],
    "purpose": "Reproduce and fix PR #389 csdlc-v2-standalone failures plus full Gate 5 lifecycle guard coverage.",
    "outcome": "passed",
    "evidence_ref": "terminal output: 60 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Reject warning regressions across C-SDLC v2 after CI-red remediation.",
    "outcome": "passed",
    "evidence_ref": "terminal output: Finished dev profile, exited 0"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
