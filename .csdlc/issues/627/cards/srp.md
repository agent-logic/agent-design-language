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

- Remote PR #635 branch remains stale until typed publication pushes this reviewed local candidate.

## Review Result

Revision: Some("git-blake3:295c2319b95622c82360d3f4c794efed44fc5d3c:b7ab776004cc5ef7b184fc5a3e39ca8db1ebe527b9e7d1ea1e953f77900e4e3a")

Reviewer: Some("codex-reviewer:review_627_r5_exact_head")

Result: pass
