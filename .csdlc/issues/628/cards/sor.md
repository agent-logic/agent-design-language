# Structured Output Record

Template: 1.0.0

Issue: 628

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the V3-H.2 local lifecycle routes under the single operationally non-authoritative csdlc v3 binary, preserving #505 as the cutover authority while allowing the issue route to write explicit v3 construction state only after stale/missing/existing-state lifecycle-digest checks pass.

## Artifacts

- csdlc-v3/src/commands/local/mod.rs
- csdlc-v3/src/main.rs
- csdlc-v3/tests/command_manifest.rs
- csdlc-v3/tests/local_commands.rs
- csdlc-v3/tests/real_issue_canary.rs
- docs/csdlc-v3/v3-command-manifest.json
- .csdlc/prepared/issues/628/design.md
- .csdlc/prepared/issues/628/diagram.mmd
- .csdlc/prepared/issues/628/validate-v3-h2-local-lifecycle.sh
- .csdlc/evidence/628/627-derived-terminal.json
- .csdlc/issues/628

## Execution

- Implemented the #628-owned local routes issue, bind, edit, validate, doctor, schedule, shepherd, and eligibility as one-binary v3 commands.
- Kept all local routes operationally non-authoritative before #505 cutover; the issue route is the bounded exception that writes explicit v3 construction state only when --v3-state-root is supplied, genuinely missing state is being initialized, or existing state is protected by an expected lifecycle digest.
- Added explicit local lifecycle state inspection so missing local state reports missing_local_lifecycle_state with repair guidance.
- Updated the v3 command manifest so #628 local routes are implemented but not live authority; GitHub, publication, finish, clean, and cutover routes remain fail-closed before #505.
- Added focused tests for local route help/dispatch, eight-command contract coverage, exact registered-worktree binding, unsafe-primary-checkout denial, issue-route construction-state writes, write-free stale/missing digest rejection, existing-state no-digest rejection, and missing lifecycle state diagnostics.
- Recorded #627 predecessor terminal evidence as a derived-terminal cache artifact under #628 evidence, because the tracked #627 issue projection remains published while live GitHub terminal truth is closed by merged PR #635.
- Recorded setup defects for #632, including bind prep/exec friction and prepared-validator projection gaps.

## Validation

[
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict Clippy across csdlc-v3 targets.",
    "outcome": "passed",
    "evidence_ref": "628-clippy.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check",
      "origin/main..HEAD"
    ],
    "purpose": "Run exact-range diff hygiene; success is represented by exit code 0 and quiet output.",
    "outcome": "passed",
    "evidence_ref": "628-diff-hygiene.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml"
    ],
    "purpose": "Run all C-SDLC v3 tests.",
    "outcome": "passed",
    "evidence_ref": "628-full-v3-regression.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/628/validate-v3-h2-local-lifecycle.sh",
      "all"
    ],
    "purpose": "Run the #628 issue-owned validator.",
    "outcome": "passed",
    "evidence_ref": "628-issue-validator.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "local_commands"
    ],
    "purpose": "Run focused local command tests.",
    "outcome": "passed",
    "evidence_ref": "628-local-route-tests.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "command_manifest"
    ],
    "purpose": "Run focused v3 command manifest tests.",
    "outcome": "passed",
    "evidence_ref": "628-command-manifest-tests.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "real_issue_canary"
    ],
    "purpose": "Run real issue canary tests.",
    "outcome": "passed",
    "evidence_ref": "628-real-issue-canary.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--check",
      "--",
      "--verbose"
    ],
    "purpose": "Verify rustfmt and retain non-empty formatter evidence.",
    "outcome": "passed",
    "evidence_ref": "628-rustfmt.log"
  },
  {
    "command": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-628-v3-h2-local-lifecycle-exec",
      "issue",
      "--issue",
      "628"
    ],
    "purpose": "Verify #628 C-SDLC issue state at current generation.",
    "outcome": "passed",
    "evidence_ref": "628-typed-issue-validation.log"
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
