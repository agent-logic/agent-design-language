# Structured Review Prompt

Template: 1.0.0

Issue: 5344

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5344
.csdlc/issues/5587
.csdlc/prepared/issues/5344
adl-runtime/src/guardian.rs
adl/tools/check_coverage_impact.sh
adl/tools/test_check_coverage_impact.sh

## Prompts

- Does the tracked-path scan consume Git's NUL-delimited literal filenames without reintroducing line-oriented parsing?
- Do UTF-8 paths, spaces, and embedded newlines preserve complete path components during Windows portability validation?
- Do genuine Windows-illegal characters, trailing spaces or dots, backslashes, and reserved device names still fail closed?
- Does the focused regression mirror the PR failure by retaining a portable UTF-8 baseline path while proving ordinary and newline-hidden illegal components are rejected?
- Are the recovery, focused validation, and exact-head review records scoped truthfully to the two CI path-policy files?

## Findings

[
  {
    "id": "F-5344-CI-FOLLOWUP-1",
    "severity": "p1",
    "summary": "The coverage contract still required a second deleted guardian CLI test, so the full tooling contract exited 1.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "F-5344-CI-FOLLOWUP-2",
    "severity": "p1",
    "summary": "The SRP and SOR still described the earlier path-policy repair and omitted the guardian, coverage-contract, and closed #5587 claim-release proof.",
    "actionable": true,
    "in_scope": true,
    "disposition": "accepted_risk",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:497b530553fc905e2d91b7e5397d82ec306dec4c:510594b56e9394c365ee1d3c6442cd6d84e2db83069eb2b1dc5509f987d1e6a3")

Reviewer: Some("subagent:019fac6c-4d03-74e3-90a2-3c3f07ed609d")

Result: changes_required
