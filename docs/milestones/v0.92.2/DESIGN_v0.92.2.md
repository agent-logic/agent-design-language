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

Independent tracks may proceed in parallel after the milestone-opening package. The integration track begins only from merged, reviewed outputs. Proof work may develop fixtures in parallel, but Beta 1 acceptance requires one integrated ADL self-review and one bounded external open-source repository run.

## Failure Posture

Malformed inputs, provider failures, redaction failures, unsupported artifact versions, partial perspective results, and renderer failures must produce explicit incomplete/non-proving states. The last valid retained evidence remains readable; no failed run may be presented as a successful review.
