# Structured Review Prompt

Template: 1.0.0

Issue: 622

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/provider/reload.rs
adl/src/provider/mod.rs
adl/src/provider/local.rs
adl/src/execute/runner.rs
adl/src/execute/tests.rs
adl-runtime-kernel/src/config_reload.rs
adl/src/long_lived_agent.rs
adl/src/long_lived_agent/tests.rs
.csdlc/prepared/issues/622/validate-provider-profile-hotload.sh
docs/providers/provider-profile-hot-loading.md
post-review fixes for production startup, credential-value rejection, provider reload generation, validation truth, and PR-readiness

## Prompts

- Does a real production execution path consume the reload owner rather than only helper tests?
- Does every inference call retain exactly one immutable starting snapshot?
- Can malformed unsupported or secret-bearing candidates ever replace last-known-good state?
- Does the implementation reuse the existing watcher and provider registry?
- Are accepted and rejected diagnostics bounded and redacted?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Focused local validation passed for the #622 production and safety lanes; broader unrelated binary compile surfaces remain outside this issue's proof claim.
- Review was anchored to immutable commit 76d588b0e6ca3446794d7487ce46facfb3d06cde; later lifecycle metadata must remain metadata-only before publication.

## Review Result

Revision: Some("git-blake3:76d588b0e6ca3446794d7487ce46facfb3d06cde:2c2a710e8f40160c503b340ef5170052ac06aefb5dbc69f8d24ebc22baf450a3")

Reviewer: Some("codex-subagent:/root/review_622_exact_head_2")

Result: pass
