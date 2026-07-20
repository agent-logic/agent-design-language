# #5337 Current-Template Provenance Blocker

## Result

`blocked`

The installed typed C-SDLC v2 initialization route generated all six #5337
cards with `Template: 1.0.0` and `identity.template_version: 1.0.0`, while the
repository's active `docs/templates/prompts/current.json` registry selects
template set `1.0.3` for SIP, STP, SPP, VPP, SRP, and SOR.

Because the operator explicitly requires all six cards to come from the current
registry, the generated cards cannot be accepted as current-template-derived.
No rendered Markdown or values JSON was hand-edited.

## Evidence

- `docs/templates/prompts/current.json` records
  `csdlc_prompt_template_set: 1.0.3` and maps all six card kinds to
  `docs/templates/prompts/1.0.3/`.
- `.csdlc/issues/5337/cards/*.md` each record `Template: 1.0.0`.
- `.csdlc/issues/5337/cards/*.values.json` each record
  `identity.template_version: 1.0.0`.
- `csdlc-v2/src/cards.rs` constructs and validates card identities against
  literal template version `1.0.0`; the typed editor schema exposes no template
  registry/version selection field.
- `csdlc-doctor --repo . --issue 5337` passes canonical v2 record consistency,
  but that does not prove parity with the active 1.0.3 prompt-template registry.

## Boundary

This packet records a local lifecycle-tooling blocker only. It does not repair
the C-SDLC v2 renderer, modify prompt templates, create a follow-on issue,
implement the #5337 corpus, or claim readiness/publication approval.

## Bounded Review Findings

The required read-only subagent review also found:

- The typed `ValidationLane` model has no dedicated release-gate field. This was
  repaired within the supported surface by using `csdlc-edit apply` to encode
  required/deferred gate status in each lane's `proof_role` and
  `defer_reason`.
- The generated SRP default says `Exact implementation revision before
  publication`, which is inaccurate for this preparation-only session.
- The generated SOR default is generic pre-execution prose rather than a
  #5337-specific preparation outcome.

An attempted typed SRP `set_field(review_scope)` operation failed closed with
`invalid_transition: srp mutation is not allowed during bound`. Advancing into
the implementation phase solely to alter this prose would misstate the session,
so SRP/SOR were not hand-edited and remain additional publication blockers.

## Required Resolution

The typed v2 card-generation/edit route must consume the active `current.json`
registry (or the repository must intentionally reconcile its declared template
authority) and then regenerate and validate all six #5337 cards before this
preparation can be published as satisfying AC-1.

The typed lifecycle must also provide a preparation-safe way to make SRP/SOR
issue-specific before publication without falsely advancing product execution.
