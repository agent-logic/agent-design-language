# Structured Review Prompt

Template: 1.0.0

Issue: 554

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

docs/milestones/v0.92/README.md
adl/src/runtime_v2/contracts.rs
.csdlc/issues/554
.csdlc/evidence/554

## Prompts

- Verify the docs fix is truthful and bounded.
- Verify Runtime-v2 reliability improves without weakening coverage or hiding failures.
- Verify no #483 or #514 behavior changed.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted coverage remains pending as fail-closed post-publication proof; the recorded focused local tests do not replace that hosted result.
- OpenAI Responses API review artifact: response id resp_019dc5f474c188ba006a8f4d47985087d0bbfc71f2f946a7c7, model gpt-5.6-sol, publication_safe true.

## Review Result

Revision: Some("git-blake3:c61367a900955deb5408817e08e3d2eb4092a9a7:1928560984f691a91c86b20cad596ba43b360082ed657f39514754cdbb52bbea")

Reviewer: Some("openai-responses:resp_019dc5f474c188ba006a8f4d47985087d0bbfc71f2f946a7c7:gpt-5.6-sol")

Result: pass
