# Structured Output Record

Template: 1.0.0

Issue: 283

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Reconciled ADR 0065 evidence for #207, remediated r1 P3 diff-check hygiene, recovered stale review after main resync, and remediated r3 P2 validator artifact-binding truth: #209 / PR #215 is the current replacement terminal authority; #5832 is retained as historical/superseded evidence; #283 does not accept the ADR or edit shared #288-owned ADR documents.

## Artifacts

- .csdlc/evidence/283/ADR0065_ACIP_EVIDENCE_RECONCILIATION.md sha256=6e1b8dd4fedc8fe5191828a2990c3fb8327baabfea50d695c2f9e5f3c8d2914c
- .csdlc/evidence/283/evidence-manifest.json sha256=dcfe6d9882f5e99ffb2a051730d41fcb0b4a3176f6430afd9282106813918b6e
- .csdlc/evidence/283/adr0065-evidence-reconciliation.log sha256=623644f28d5c513d017d283e4dcc5f7e6fd0c9959ae5539c872691b77de2e8c0
- .csdlc/prepared/issues/283/validate-adr0065-evidence.sh sha256=f1fbf834696059b22fc8082d1a6a86dcf9f05fd373fccd5976be7893ca6e4e98
- .csdlc/prepared/issues/283/design.md sha256=37d2c15d3cc59b15b428fa5c2e2e91ab3d7c4aa93ea2c03719eb65d0960f00b1
- .csdlc/prepared/issues/283/diagram.mmd sha256=826962955865e356c3b0b5b539014a48294607299c298748a38b53faa6c5c73f
- .git/csdlc-v2/derived-terminal/209.json sha256=2db7585030569dbf7350e1ce2cedc70e8c6f90ca7d7d08476f2be0ecac9cc59a
- .csdlc/evidence/209/local-validation-manifest.json sha256=33b6d90ba1330aec3ca9ff228bb997c7fd8cbf062208119669991cc846dd1c74
- .csdlc/evidence/209/native-validation-manifest.json sha256=c85fc5f007e2e091f2fa91ddec1dad2f5602a15861039e5d600886f49ce10987
- .csdlc/evidence/5832/acip-native-receipts.json sha256=eb69f742c8074ea96d3bfb9a6d846001a9a4abfe9caf25bdb237b1bac4d11f4c

## Execution

- Added issue-local reconciliation packet .csdlc/evidence/283/ADR0065_ACIP_EVIDENCE_RECONCILIATION.md.
- Added machine-readable evidence manifest .csdlc/evidence/283/evidence-manifest.json.
- Added issue-owned focused validator .csdlc/prepared/issues/283/validate-adr0065-evidence.sh.
- Captured typed finalize PVF output in .csdlc/evidence/283/adr0065-evidence-reconciliation.log.
- Verified #209 live/typed terminal authority and #209/#5832 evidence inputs without editing shared ADR docs.
- Recovered the self-routed design approval, reapproved the EOF-clean design/diagram with canonical fresh-session UUID fresh-session:ce78ec0d-4168-494b-b604-73181fb20ecd, and removed blank EOF lines flagged by r1 review.
- Merged current origin/main, recovered stale exact-head review truth after review_publication_dead_end, and reran the focused validator/diff guard at the resynced head.
- Remediated r3 P2 by deriving #209/#5832 native artifact paths and expected SHA-256 values from their source manifests and failing closed on any missing, empty, invalid, or byte-drifted artifact.

## Validation

[
  {
    "command": [
      ".csdlc/prepared/issues/283/validate-adr0065-evidence.sh"
    ],
    "purpose": "Focused issue-owned ADR 0065 evidence reconciliation validator after r3 artifact-binding remediation.",
    "outcome": "passed",
    "evidence_ref": "terminal output: PASS: ADR 0065 evidence inputs are present, non-empty, and classified for #283 reconciliation"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-283-adr0065-acip-evidence-reconciliation",
      "issue",
      "--issue",
      "283"
    ],
    "purpose": "Typed #283 issue validation after r3 artifact-binding remediation.",
    "outcome": "passed",
    "evidence_ref": "terminal output: status pass, phase implemented, generation 12, findings []"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      "/Volumes/FastWork/adl-worktrees/adl-issue-283-adr0065-acip-evidence-reconciliation",
      "--issue",
      "283"
    ],
    "purpose": "Typed #283 doctor after r3 artifact-binding remediation.",
    "outcome": "passed",
    "evidence_ref": "terminal output: status pass, phase implemented, generation 12, findings []"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "purpose": "Whitespace/diff hygiene after r3 artifact-binding remediation.",
    "outcome": "passed",
    "evidence_ref": "terminal output: exit 0"
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
