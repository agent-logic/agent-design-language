# Structured Review Prompt

Template: 1.0.0

Issue: 602

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl-runtime-kernel/src/control.rs
.csdlc/evidence/602/live-wuji-acceptance.md

## Prompts

- Can any unauthorized or conflicting request mutate durable or live roster state?
- Can persistence and in-memory roster truth split after any modeled failure?
- Does restart reload preserve exact admission and reject corrupt state?
- Does csmctl keep credentials out of argv output errors and persisted state?
- Does the live proof preserve Shepherd and avoid init mutation or restart for first add?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live Wuji proves exact-candidate lifecycle, inference, clean shutdown, and restart; the 10-minute queue-expiry branch is proven deterministically rather than by an unnecessary live wait.

## Review Result

Revision: Some("git-blake3:a6c5bd5cf3156a7efdc64d0c5a651fde32642a30:08d0247414fbd3f0d15d3e01cc404bc4862bd0935b6a80903d68b5c31747ab0c")

Reviewer: Some("codex-subagent:issue_602_queue_review")

Result: pass
