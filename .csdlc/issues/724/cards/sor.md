# Structured Output Record

Template: 1.0.0

Issue: 724

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented one GitHub-like C-SDLC v3 issue-create command that constructs the existing typed dispatch and preserves authority, intent, readback, receipt, and reconciliation behavior.

## Artifacts

- csdlc-v3/src/main.rs
- csdlc-v3/tests/operational_cli_commands.rs
- docs/csdlc-v3/CONTRACT.md
- .csdlc/prepared/issues/724/validate-simple-issue-create.sh

## Execution

- Added strict title, inline-body/body-file, repeatable label/assignee, milestone, repository, credential-name, exact-head, and execute parsing.
- Projected the simple flags into OperationalRemoteDispatchRequest and the existing GithubMutation::IssueCreate path without an alternate transport.
- Added positive, adversarial, and inactive-authority CLI tests while retaining assigned-number and idempotent reconciliation unit proof.
- Documented the simple command first and the request-file form as the advanced audit interface.

## Validation

[
  {
    "command": [
      "/bin/bash",
      ".csdlc/prepared/issues/724/validate-simple-issue-create.sh",
      "."
    ],
    "purpose": "Issue 724 focused implementation validation",
    "outcome": "passed",
    "evidence_ref": "issue-724-focused.log"
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
