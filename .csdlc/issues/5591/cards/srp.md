# Structured Review Prompt

Template: 1.0.0

Issue: 5591

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime/src/guardian.rs
.csdlc/issues/5591/cards/sor.values.json
.csdlc/issues/5591/cards/srp.values.json

## Prompts

- Does every acceptance criterion require guardian-launched live production behavior rather than fixture or library evidence?
- Are checkpoint/replay/resume determinism, authenticity, duplicate prevention, and corrupt-state negatives fully specified?
- Does pressure handling prove admission quiescence, durable serialization, terminal Observatory output, bounded shutdown, and correct restart?
- Are local/remote access and Observatory configuration-driven, TLS-authenticated, redacted, and free of hard-coded addresses?
- Are Runtime v2, AWS, cutover, deletion, provider deployment, and later-parity ownership boundaries explicit?
- Do plan steps and validation lanes cover AC-1 through AC-8 bidirectionally with no deferred acceptance?
- Is #5336 integration the sole current stop while the future claim-scope proposal remains non-authoritative?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The fixed-port Horust SIGTERM process test remains ignored and was intentionally not run; dynamic-port pressure and signed terminal paths share the reviewed serialization helper and pass.
- Runtime v3 is 12,683 physical source lines, an explicitly reviewed +474 exception over the pinned #5336 baseline of 12,209; exact review found the delta necessary, non-duplicative, and functionally required.

## Review Result

Revision: Some("git-blake3:a3fd76181cf3b922eeccd555fccb846d2ad8447b:275fdc0c21d309343bca0b3e68a8f0207b6707fc6305f9a50e0cfadbba203eb8")

Reviewer: Some("subagent:/root/review_5591_exact_head")

Result: pass
