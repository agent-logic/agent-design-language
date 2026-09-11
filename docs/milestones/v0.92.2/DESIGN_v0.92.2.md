# v0.92.2 Design — CodeFriend Beta 1

Status: planning candidate.

## Product Flow

`operator setup -> repository adapter -> evidence core -> analysis perspectives -> synthesis -> remediation/test plan -> governed publication -> longitudinal comparison`

## Boundaries

- **Product shell:** setup, onboarding, run configuration, status, artifact browsing, and publication controls.
- **Adapter v2:** normalizes local, GitHub, and CI inputs into a portable repository packet without embedding host paths.
- **Evidence core:** assigns stable artifact identity, captures provenance, applies redaction and retention policy, and separates evidence from inference.
- **Architecture cognition:** analyzes dependencies, boundaries, coupling and connascence, drift, blast radius, architectural quanta, and ADR/rationale signals.
- **Governance:** expresses bounded fitness functions and CI gates without hiding policy inside tests.
- **Review engine:** runs correctness, security, adversarial, and constitutional perspectives, then deduplicates and synthesizes findings.
- **Action planning:** creates remediation and test plans without mutating the repository.
- **Memory:** compares current and previous runs using stable identity and explicit schema/version compatibility.
- **Publication:** renders Markdown, HTML, and PDF behind explicit human approval, with manifests, privacy/legal checks, claims, non-claims, and release notes.

## Authority Model

CodeFriend consumes the shared provider and Runtime contracts. It does not create private provider implementations for individual tools. The evidence packet is the source for review claims; generated reports are projections. Human approval remains the publication boundary. CI integration may block on declared fitness functions but cannot silently widen review scope.

## Integration Strategy

Independent tasks follow the exact dependencies in the [issue wave](WP_ISSUE_WAVE_v0.92.2.yaml). Local ingestion precedes the GitHub and CI routes; structure analysis precedes drift, impact and rationale analysis; local fitness execution precedes CI gating. Isolated review precedes synthesis, then separate remediation and test-plan generation. Approval control precedes Markdown rendering; HTML and PDF consume that shared approved contract. Provider configuration precedes RT-PROVIDER/#855 lifecycle execution. These are complete behaviors under the [atomic task contracts](ATOMIC_TASK_CONTRACTS_v0.92.2.md).

CF-INTEGRATE first connects merged, reviewed, complete consumers into one installed candidate. CF-PROOF then independently executes that product on ADL and the pinned external Rust repository, including second-run comparison and all three exports. TAIL-01 requires both completed results. A schema, scaffold, unused library or prewritten packet cannot substitute for those executed journeys.

## Failure Posture

Malformed inputs, provider failures, redaction failures, unsupported artifact versions, partial perspective results, and renderer failures must produce explicit incomplete/non-proving states. The last valid retained evidence remains readable; no failed run may be presented as a successful review.

## Adopted interfaces

[Adopted design contracts](ADOPTED_DESIGN_CONTRACTS_v0.92.2.md) owns the tracked Beta boundary, shared finding/run contract, review isolation and implementation decision gates. CF-EVIDENCE must deliver that versioned contract and executable consumer fixtures before review, longitudinal memory or publication implementations start. The ignored TBD corpus supplies historical rationale, not a required runtime or clean-checkout planning input.
