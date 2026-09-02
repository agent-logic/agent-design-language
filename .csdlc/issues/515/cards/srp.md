# Structured Review Prompt

Template: 1.0.0

Issue: 515

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/515/publish-after-production-shadow-wiring.json
.csdlc/prepared/issues/515/review-record-production-shadow-wiring-pass.json
.csdlc/prepared/issues/515/review-recover-after-production-shadow-review-record.json
adl/src/provider/mod.rs
adl/src/execute/tests.rs
adl/tests/provider_shadow_open_pr_review.rs
docs/milestones/v0.92.1/evidence/provider/prov-b/open-pr-shadow-review-smoke.json

## Prompts

- Can any shadow result mutate or replace the authoritative result?
- Are authority and shadow paths represented distinctly enough for reviewers and validators?
- Are comparison inputs and rules exact and deterministic?
- Do shadow failures preserve authoritative outputs and state?
- Does evidence redact credentials, private payloads, prompts, and host-local paths?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The local Ollama smoke test uses hard-coded real PR review prompts for PR #618 and #614 rather than dynamically fetching live PR body or diff material at test runtime.
- The metadata-head reviewer did not rerun tests or mutate GitHub; validation truth relies on the previously captured passing local commands and substantive exact-head review.

## Review Result

Revision: Some("git-blake3:9cb2a07564e1e73dd5089a08230745be69f56099:0960f1cb8183cfe62065c5dc029ead21a23a4f3e34867eaeac8904e98b43092d")

Reviewer: Some("fresh-session:4e8a6026-e6db-4245-90ad-a8e334534494")

Result: pass
