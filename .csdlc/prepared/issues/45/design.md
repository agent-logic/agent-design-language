# Issue 45 design: split repository authority in `csdlc-doctor`

## Problem

The lifecycle record currently carries one repository identity and doctor compares it with the bound worktree's `origin`. That is correct for a same-repository issue and code path, but it rejects the supported split-authority topology where the source issue remains in `danielbaustin/agent-design-language` and code plus pull requests live in `agent-logic/agent-design-language`.

## Design

Keep both identities explicit. The issue record retains the issue repository as tracker authority. Bound Git topology provides the code repository, and an explicit publication/lifecycle contract records the code repository when it differs. Doctor validates the pair rather than overwriting or guessing either identity.

The decision table is deliberately small:

| Case | Issue repository | Code repository | Explicit split contract | Result |
|---|---|---|---|---|
| Same repository | `A` | `A` | not required | accept |
| Supported split | `A` | `B` | names both `A` and `B` | accept |
| Accidental drift | `A` | `B` | absent, partial, or mismatched | fail closed |

No remote name is itself authority. Doctor normalizes and compares repository identities derived from typed fields and effective Git remote evidence. It must not silently infer a split route merely because two remotes differ.

## Implementation surface

- Extend or reuse the smallest typed lifecycle/publication identity structure needed to expose issue and code repositories to doctor.
- Update doctor diagnostics so mismatch messages name the specific authority that drifted.
- Preserve same-repository compatibility.
- Add focused Rust tests for same-repository acceptance, explicitly valid split acceptance, and invalid drift rejection.
- Update active C-SDLC v2 skills and operator runbooks that describe repository identity or doctor readiness.

## Invariants

- Issue authority and code authority are never collapsed implicitly.
- A split route is accepted only when both repository identities are explicit and consistent.
- `origin` is evidence for code topology, not issue-tracker authority.
- Ambiguity and substitution remain fail-closed.
- Historical evidence is not rewritten.

## Validation strategy

Use a focused doctor/identity Rust lane with deterministic fixtures for all three cases, then validate schemas/operator contracts and run strict Clippy for the touched crate. Hosted integration proof may be deferred to the implementation PR because preparation does not change behavior.

## Estimates

- Construction: 2-4 hours, approximately 8,000-20,000 model tokens.
- Focused local validation: up to 15 minutes and approximately 4,000 tokens.
- Exact-head review and fixes: 30-60 minutes and approximately 8,000 tokens.

These are reviewable estimates, not execution limits; needed code and proof take precedence over arbitrary line or token counts.
