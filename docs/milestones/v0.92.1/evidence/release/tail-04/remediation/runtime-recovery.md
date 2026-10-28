## Outcome

Repair the Runtime admission-greeting crash boundary, preserve one stable logical work identity across retries, and make the real-local Shepherd proof reject every off-host URL form.

## Parent and findings

- Parent remediation issue: #522
- Source review: #520 at `fb6cbc7f619daa54f901fd2d12f480add682ace3`
- Findings: `D520-RUNTIME-001`, `D520-RUNTIME-002`, `D520-SEC-004`

## Acceptance criteria

- A crash after the final permitted greeting-attempt claim cannot dispatch another attempt or make the durable agent store unloadable.
- The admission record, dispatch, provider work, result, transcript, and audit surfaces retain one immutable logical work/idempotency key across retries.
- The local Ollama proof parses and validates the URL structurally and rejects userinfo, off-host DNS targets, unsafe redirects, and other authority-confusion forms.
- Focused deterministic regressions cover each trigger and pass at the exact reviewed head.

## Owned paths

- `adl-runtime-kernel/src/control.rs`
- `adl-runtime/tests/shepherd_local_model.rs`
- tightly coupled Runtime tests only

## Validation

- Focused admission-greeting crash/retry tests.
- Focused Shepherd local-model URL negative fixtures.
- Runtime owner validation lane proportional to the touched surface.

## Non-goals

- Changing provider selection or expanding the Ollama deployment architecture.
- General Runtime refactoring.
