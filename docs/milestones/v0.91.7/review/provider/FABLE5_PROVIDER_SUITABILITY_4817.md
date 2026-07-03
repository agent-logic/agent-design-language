# Fable 5 Provider Suitability Probe

Issue: #4817
Milestone: v0.91.7
Date: 2026-07-03
Provider: Anthropic
Model ID observed: `claude-fable-5`
Credential source used for live probe: operator-approved `$HOME/keys/claude2.key`

## Summary

Claude Fable 5 was reachable through the Anthropic API and through ADL's hosted Anthropic provider-adapter path. The first multi-lane suitability probe exposed an ADL adapter limitation rather than a model reachability failure: the Anthropic adapter currently sends a fixed `max_tokens: 256`, which clipped several structured responses. A direct Anthropic rerun with `max_tokens: 1024` produced usable structured output for the tested planning/review/card-validation lanes.

This record does not claim broad Fable 5 quality, benchmark standing, release authority, merge authority, or suitability for unrestricted autonomous repository mutation.

## Probe Results

| Surface | Result | Evidence |
| --- | --- | --- |
| Anthropic model listing | PASS | `/v1/models` included `claude-fable-5` with display name `Claude Fable 5`. |
| Anthropic messages API | PASS | A direct `claude-fable-5` Messages API call returned the expected probe text: `ADL Fable 5 API probe ok`. |
| ADL provider adapter smoke | PASS | `adl-provider-adapter` completed a hosted Anthropic invocation for `claude-fable-5`; final status was `ok`, no failure was recorded, and the model returned `ADL Fable 5 provider adapter smoke ok`. |
| Five-lane mini panel through current adapter | PARTIAL | Adapter calls returned HTTP 200/final status `ok`, but planner/reviewer style outputs were clipped because the adapter used a fixed Anthropic `max_tokens: 256`. |
| Direct Anthropic mini panel at 1024 output tokens | PASS_WITH_BOUNDARIES | Planner, card-validator, and reviewer lanes produced usable structured output. A watcher prompt containing provider/credential/adapter operational wording refused; a narrower watcher-state prompt succeeded. |

## Issue Follow-Up

Issue #4819 tracks the general ADL provider-adapter fix: add a portable output-token budget field to provider invocations and map it to provider-native request fields. That issue is required before a normal Fable 5 suitability panel can run through the adapter without response clipping.

## Boundaries

- No secret value was printed, copied, committed, or stored in this artifact.
- Raw live-response scratch artifacts were local and non-durable; this tracked note records only the summarized result needed for ADL issue truth.
- The successful direct run used a larger output budget than the current adapter exposes, so it is evidence that Fable 5 can answer the tested tasks, not proof that the current adapter can run the full panel without #4819.
- The watcher refusal appears prompt-surface related and should be handled by narrower, non-operational suitability prompts rather than by weakening safety wording.

## Next Step

Implement #4819, then rerun the Fable 5 code-oriented/provider suitability test through the repo-native adapter with an explicit output-token budget.
