# Structured Task Prompt

Template: 1.0.0

Issue: 608

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Patch the existing Vertex provider to support global endpoint derivation and thinking config, then prove it locally and live.

## Deliverables

- Provider endpoint/trust/thinking-config implementation
- Focused provider unit tests
- Live Vertex provider proof packet
- Fresh exact-head review and PR with Closes #608

## Acceptance

1. AC-1: location global derives the aiplatform.googleapis.com global Vertex generateContent endpoint
2. AC-2: regional locations continue to derive <region>-aiplatform.googleapis.com endpoints
3. AC-3: global and regional first-party Vertex hosts are trusted without trust_custom_endpoint
4. AC-4: thinking_level renders generationConfig.thinkingConfig.thinkingLevel
5. AC-5: thinking_budget renders generationConfig.thinkingConfig.thinkingBudget
6. AC-6: include_thoughts renders generationConfig.thinkingConfig.includeThoughts
7. AC-7: simultaneous thinking_level and thinking_budget are rejected
8. AC-8: focused provider tests pass
9. AC-9: live regional Gemini 2.5 provider proof passes in us-west1
10. AC-10: live global Gemini 3.x provider proof passes with native location global and no endpoint override

## Dependencies

- #528 native Vertex AI Gemini provider transport

## Inputs

- agent-logic/agent-design-language#608
- adl/src/provider/http_family.rs
- adl/src/provider/http_family/tests.rs

## Non Goals

- #592 Polis integration
- Broad provider architecture redesign
- New provider dependencies
- Credential material in repository artifacts
