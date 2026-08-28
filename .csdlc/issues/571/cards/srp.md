# Structured Review Prompt

Template: 1.0.0

Issue: 571

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

docs/csdlc-v3/CONTRACT.md
docs/csdlc-v3/predecessor-coverage.json
docs/csdlc-v3/proportional-lifecycle.json
.csdlc/prepared/issues/500/validate-implementation.rb
.csdlc/prepared/issues/571/validate-v3a-followup.rb
.csdlc/issues/571
.csdlc/prepared/issues/571

## Prompts

- Does every retained #161-#163 predecessor row have exactly useful owner issue and proof-lane data?
- Does CONTRACT.md bind the V3-A construction decision to measured #162 evidence and #163/Decision 11 approval evidence?
- Can the default lifecycle path still omit retained bind, publication, finish, or cleanup gates?
- Does diff hygiene validation use an exact PR base/head range?
- Does the patch preserve v2 live authority until V3-F/#505 and avoid widening into later v3 slices?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Publication was not present during review, so the PR body closing keyword remains a publication-time check; the PR must include Closes #571.

## Review Result

Revision: Some("git-blake3:b7771eb24811acc123a02f060f44428143a76396:78ec3804511f6ec4182e77adb23b547400639f37b5b85b360c98671d3a14c836")

Reviewer: Some("subagent:issue_571_pre_pr_review")

Result: pass
