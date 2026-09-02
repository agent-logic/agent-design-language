# Structured Review Prompt

Template: 1.0.0

Issue: 515

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/provider/mod.rs
adl/src/execute/tests.rs
adl/src/execute/runner.rs
adl/src/remote_exec.rs
adl/tests/provider_shadow_isolation.rs
adl/tests/provider_shadow_comparison.rs
adl/tests/provider_shadow_fallback.rs
adl/tests/provider_shadow_open_pr_review.rs
.csdlc/prepared/issues/515/validate-provider-shadow-readiness.sh
.csdlc/prepared/issues/515/validate-provider-shadow-redaction.sh
docs/milestones/v0.92.1/evidence/provider/prov-b/local-model-shadow-comparison.json
docs/milestones/v0.92.1/evidence/provider/prov-b/open-pr-shadow-review-smoke.json
.csdlc/prepared/issues/515

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

- The local Ollama smoke test uses hard-coded real PR review prompts for open PR #618 and #614 rather than fetching live PR body or diff material at test runtime.
- The reviewer did not rerun Cargo or local Ollama because the exact-head review was read-only; the record relies on the already captured passing validation and redacted smoke evidence.

## Review Result

Revision: Some("git-blake3:b41443094d38109cd9d87c4462ad95b0bd992c79:df29f83e8b6abe1e57136c16872e28348f7370d81288e7c107d7d88f497d9442")

Reviewer: Some("fresh-session:4af50d8b-e397-4cab-a8fa-3aae6c92c822")

Result: pass
