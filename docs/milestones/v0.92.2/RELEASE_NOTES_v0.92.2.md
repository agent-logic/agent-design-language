# v0.92.2 Release Notes — Draft

Status: planned draft; rewrite from actual landed evidence before release.

## Intended Headline

CodeFriend Beta 1 provides a governed repository-review product with portable ingestion, evidence-backed architecture and multi-perspective review, executable fitness functions, longitudinal comparison, and human-controlled Markdown, HTML, and PDF publication.

## Intended Capabilities

- guided setup, onboarding, and operator controls
- local, GitHub, and CI repository ingestion
- stable evidence identity, provenance, redaction, and retention
- explainable architecture and change-risk analysis
- correctness, security, adversarial, and constitutional review perspectives
- synthesis, remediation, and test planning
- architecture fitness functions and CI gates
- second-run comparison
- manifests, claims/non-claims, privacy/legal checks, and governed publication
- documentation, examples, fixtures, ADL self-review, and external OSS proof

## Not Included

The release is not planned to include Jira, Linear, Slack, broad Workspace integration, autonomous source mutation, public customer-scale or multi-tenant hosting, security tournaments, ATE, OCI model packaging, optional general modernization, or Runtime v4. MLX/Metal support is bounded to the admitted provider-platform track and does not imply broad platform coverage. The admitted Observatory hosting work is one static edge sidecar based on #679/PR #685; it does not expand the product into a hosted customer platform. Required milestone ADRs are generated and reconciled through ARCH-ADR, with acceptance remaining explicit.

## Evidence Note

No capability above should remain in final release notes unless the release packet cites its merged implementation and current proving evidence.

## Evidence granularity

Final notes must map each claim to the separate implementation owners in the [atomic task contracts](ATOMIC_TASK_CONTRACTS_v0.92.2.md): three ingestion routes; structure, drift, impact and rationale analysis; local and CI fitness execution; review, synthesis and the two action planners; publication approval and three renderers; provider configuration and lifecycle. Runtime repair/qualification and local/remote C-SDLC refactors retain separate evidence. All seven planning tasks remain required, but their completed documents must not be presented as implemented product capabilities. The eleven tightened completion criteria apply before any corresponding capability is claimed.
