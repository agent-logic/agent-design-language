# v0.91.8 ADR Plan

## Status

Planned. WP-02 determines whether these become new ADRs or updates to existing records.

| Candidate | Decision boundary | Owner WP |
|---|---|---|
| ADL clean-room product boundary | language/compiler/engine/CLI versus external products | WP-02 |
| Versioned ExecutionPlan contract | compiler output stability and compatibility | WP-05 |
| Provider and tool port authority | captured nondeterminism and governed actions | WP-06/WP-09 |
| ADL generation selector | opt-in, default, rollback, compatibility expiry | WP-12/WP-13 |
| Legacy deletion policy | denominator, retained manifest, 80/90 thresholds | WP-13 |

## Rules

- Architecture decisions must cite exact implementation and proof revisions.
- Planning acceptance does not equal final ADR acceptance.
- No ADR may transfer Runtime v3 or C-SDLC v2 authority back into ADL core.
