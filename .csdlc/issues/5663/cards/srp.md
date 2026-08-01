# Structured Review Prompt

Template: 1.0.0

Issue: 5663

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/assembly.rs
adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
adl-runtime-kernel/src/governed_operations.rs
adl-runtime-kernel/src/operations.rs
adl-runtime-kernel/tests/assembly.rs
adl-runtime-kernel/tests/operations.rs

## Prompts

- Can any of the six local adapters still earn production success with only a generic receipt?
- Are timeout, cancellation, saturation, duplicate, restart, shutdown, checkpoint restore, and lifelog redaction behaviors real and tested?
- Does the implementation stay out of Provider, ACIP, A2A, Cloud Bridge, AWS, and WP-12 scope?
- Is the before/after physical LoC measurement truthful and net-negative?
- Do focused tests and strict Clippy prove the owned surface without relying on fixture-only credit?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Claude Opus 5 was invoked through the Rust adl-provider-adapter for issue-5663-opus-review-fb04c9fa and returned final_status=failed, failure.kind=provider_empty_text_output, http_status=200 after one bounded attempt; retained artifacts: .adl/local-artifacts/5663-opus-review/result.json and run.log.jsonl.
- Existing secondary GPT 5.5 evidence is preserved at .adl/local-artifacts/opus5-review-5663-fb04c9fa/review-guard-request-gpt55.json and states PASS/no findings after Opus unavailability, but it is not recorded as an Opus pass and its stored scope is not the current six-path assignment.

## Review Result

Revision: Some("git-blake3:fb04c9fa29c528c06a7b3c76e5f6560b7700d43e:63d9cae55cf4e1e08f8e37bbb7e766d6221a50993ae347a92c31ce62f0fad259")

Reviewer: Some("external:claude-opus-5")

Result: changes_required
