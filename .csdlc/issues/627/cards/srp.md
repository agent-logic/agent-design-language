# Structured Review Prompt

Template: 1.0.0

Issue: 627

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

csdlc-v3/src/main.rs
csdlc-v3/tests/command_manifest.rs
docs/csdlc-v3/v3-command-manifest.json
.csdlc/prepared/issues/627
.csdlc/evidence/627

## Prompts

- Does the manifest reconcile exactly with the installed v2 binary denominator and the operator-confirmed 19-route sprint target?
- Does the single `csdlc` binary expose a stable command surface without separate v3 helper binaries?
- Do not-yet-implemented live-authority routes fail closed clearly and without fallback?
- Can later child issues consume this manifest without command-name ambiguity?
- Did the issue avoid all C-SDLC v2 source changes?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:b9ad9881d81171625267d00e0921190437534420:3e6fa8662d07984b7428f33ccb1eb72d86f9a4af507de58a119b051b0e191d6b")

Reviewer: Some("codex-reviewer:review_627_r3_evidence")

Result: pass
