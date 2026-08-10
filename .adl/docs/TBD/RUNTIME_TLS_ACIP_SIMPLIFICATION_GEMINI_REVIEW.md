# Gemini Review: Runtime TLS and ACIP Simplification

Status: Completed follow-on review

Related issue: #92

Related pull request: #98

Reviewed document: `.adl/docs/TBD/RUNTIME_TLS_ACIP_SIMPLIFICATION_REVIEW.md`

Source baseline: `e93603dc13ad7b21def61db9f09120c7d4524f35`

Provider-asserted model: `gemini-3.1-pro-preview`

## Review Boundary

Gemini was asked to review the duplication diagnosis, COTS recommendations,
consumer-preservation plan, security boundaries, staged removal, validation,
and final recommendation. It was instructed to distinguish defects in the
proposal from implementation defects reported by the proposal.

An initial response was incomplete because it reached the adapter output
boundary. It was not accepted as review authority. A direct bounded retry with
a larger output allowance completed normally and supplied the findings and
verdict recorded below.

## Findings

### G-1: Separate immediate security fixes from consolidation

Classification: Proposal correction

The document correctly identifies the Observatory read-token/write-authority
mismatch and the leaf-as-root trust-policy gap, but these are functional
security defects. They should receive bounded implementation and review rather
than waiting for the complete API consolidation.

Disposition: Incorporated. Stage 1 now says these fixes must not be hidden
inside or delayed by the larger refactor.

### G-2: Keep deployment COTS selection out of the immediate repair

Classification: Proposal correction

Caddy, AWS load balancing, and Envoy are plausible TLS-edge choices, but
choosing and deploying one is a deployment architecture decision. The internal
Rust duplication should be removed independently.

Disposition: Incorporated. The COTS section is explicitly an evaluation
appendix and not a prerequisite for consolidation.

### G-3: The incompatible endpoint implementations are a critical defect

Classification: Confirmed implementation defect

Gemini agreed that binary Protobuf handling in the production kernel and JSON
text handling in the facade cannot both define the same versioned
`/v1/acip/ws` endpoint.

Disposition: Retained as a PR #98 blocker pending implementation correction.

### G-4: The named production proof tests the wrong surface

Classification: Confirmed implementation defect

Gemini agreed that `wp14-native-acip.yml` exercises the facade test rather than
booting and probing the production `adl-runtime-kernel` binary.

Disposition: Retained as a PR #98 blocker pending proof correction.

### G-5: Parallel OpenAPI documents preserve drift

Classification: Confirmed implementation defect

Gemini agreed that independently maintained ACIP descriptions should be
replaced by one generated, derived, or strongly checked authority.

Disposition: Retained in the consolidation sequence.

## Duplication Verdict

Gemini confirmed that the duplicate API facade is not benign repetition. It
splits tests, protocol behavior, and review authority. The reviewer endorsed
deleting the duplicate router, listener wrappers, and WebSocket session rather
than introducing an abstraction that preserves both implementations.

## COTS Verdict

The COTS direction is sound but secondary. Standard edge termination can
remove commodity certificate and proxy responsibilities, while `tower-http`
can simplify ordinary Axum middleware. Neither decision should delay the
single-router correction.

## Consumer-Preservation Verdict

The proposed sequence appropriately protects real consumers of
`runtime_api_auth.rs`. Shared credential-store and admission primitives should
be extracted or retained before deleting the facade listener. The review does
not support deleting the entire authentication module wholesale.

## Final Decision

**Approved with modifications.**

The modifications are:

1. implement security defects as bounded fixes;
2. consolidate the Rust API independently of edge deployment selection;
3. treat COTS edge termination as a separate architecture decision.

The parent document incorporates these corrections. This provider review is
supplemental evidence and does not grant merge, publication, or lifecycle
authority.
