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
adl-runtime-kernel/src/control.rs
adl/src/cli/csmctl_cmd.rs
bounded PR #646 CI janitor fixes for rustfmt and csmctl private import

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

- Review passed for local exact head 92e8d07d7d6ab73159264a25e94f5bb44a0bbf04; PR #646 must be republished/updated and CI must rerun before remote merge readiness can be claimed.
- Reviewer did not rerun cargo tests or clippy; local fmt/clippy proof was run and recorded by the implementation session before review.

## Review Result

Revision: Some("git-blake3:92e8d07d7d6ab73159264a25e94f5bb44a0bbf04:96b78dcb8eb9a2dd269618910a6676a43c62dbbd763a18463b6b5138100115a5")

Reviewer: Some("codex-subagent:/root/review_622_ci_janitor_exact_head")

Result: pass
