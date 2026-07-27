# Structured Review Prompt

Template: 1.0.0

Issue: 5692

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

AGENTS.md
csdlc-v2/src/bin/csdlc-publish.rs
csdlc-v2/src/publication.rs
csdlc-v2/tests/gate6.rs

## Prompts

- Review only AGENTS.md closing-keyword policy wording, csdlc-v2 publication body validation, and focused tests. Findings first; no workflow rewrite.

## Findings

[
  {
    "id": "5692-P1-qualified-cross-repo-closing-ref",
    "severity": "p1",
    "summary": "Repository-qualified closing references must match the governed repository; wrong/repo#5692 cannot satisfy ADL issue auto-close linkage.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:b23a8f87e76536fdbf65a6be78ca64b2a3918b19:a7e4c35e19734c21bc504eb8d84721230270f247278c7a764d1d1f3dc238a4a9",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No live PR body was observed before publication; csdlc-publish will validate the publication request and remote PR body again during publication.

## Review Result

Revision: Some("git-blake3:b23a8f87e76536fdbf65a6be78ca64b2a3918b19:a7e4c35e19734c21bc504eb8d84721230270f247278c7a764d1d1f3dc238a4a9")

Reviewer: Some("bounded-subagent:019fa471-1206-7ab2-b567-b31f2cb1a3c3")

Result: pass
