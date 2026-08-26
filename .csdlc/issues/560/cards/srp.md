# Structured Review Prompt

Template: 1.0.0

Issue: 560

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/560
.csdlc/prepared/issues/560
.csdlc/evidence/560
adl/.config/nextest.toml

## Prompts

- Verify the change is an exact ci-coverage timeout/profile adjustment for only the three observed runtime_v2 tests.
- Verify Runtime v2 semantics and assertions are unchanged.
- Verify hosted coverage remains the final shared-gate proof.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- OpenAI Responses API exact-head review PASS response resp_0db9cfd6d229d16d006a8f6d76679887d08a074998060a2b4d at commit a75e499a75cd4a69c27bd45c4f68f3f7279eb84c.
- The focused local run completed in about 5 seconds and therefore cannot reproduce hosted contention or prove that the 240-second ceiling is sufficient.
- Full hosted workspace coverage remains the fail-closed integration proof and must pass before merge.
- The timeout override is exact to the three fully qualified test names but is not additionally constrained by package or binary; the recorded run nevertheless selected exactly those three tests across 33 binaries.

## Review Result

Revision: Some("git-blake3:a75e499a75cd4a69c27bd45c4f68f3f7279eb84c:3ad581bb5cc9bb39b71b5093039f37d71d8150f9906c43695a45e539e4ce41c5")

Reviewer: Some("openai-responses:resp_0db9cfd6d229d16d006a8f6d76679887d08a074998060a2b4d:gpt-5.6-sol")

Result: pass
