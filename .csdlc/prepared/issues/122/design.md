# Issue #122 Design: Deferred Public Observatory Exposure

## Deferred Outcome And Authority

Issue #122 is deferred beyond v0.92. It does not gate #83 or #111-#117 and
does not authorize implementation, publication, or AWS activity during this
preparation. Execution starts only after the distributed Runtime is terminal
and an operator separately authorizes bounded AWS work.

After those gates close, #122 may expose the completed Observatory at
`observatory.dev.agent-logic.ai` and its matching distributed Runtime gateway
at `runtime.dev.agent-logic.ai`. AWS work must use the approved Agent Logic
business profile. No EC2, Spot, or CodeBuild is permitted.

## Public Boundary

The public Observatory is static content delivered through S3 and CloudFront,
with Route53 and ACM providing canonical DNS and ordinary browser trust. The
Runtime gateway exposes only the reviewed distributed Runtime API and WSS
surface. Exact hostname, certificate, CORS, CSP, WSS origin, authentication,
rate-limit, redaction, health, revision, ownership, and rollback contracts fail
closed.

Public reachability never grants write authority. Signed Layer 8 operations
retain their Runtime authorization, replay, refusal, and audit contracts. No
private agent state, credentials, raw provider payloads, internal topology, or
unreviewed diagnostics may cross the public boundary.

## Issue-Owned Targets

Deferred issue-owned targets, created only in a separately authorized execution
session:

- `infra/aws/public-observatory/**`
- `adl/tools/validate_public_observatory_exposure.sh`
- `adl/tools/validate_public_runtime_gateway.sh`
- `docs/milestones/post-v0.92/features/PUBLIC_OBSERVATORY_EXPOSURE.md`
- `.csdlc/issues/122/**`
- `.csdlc/prepared/issues/122/**`
- `.csdlc/evidence/122/**`

Issue #122 may consume terminal distributed Runtime and #83/#110-#117
contracts, but may not mutate their issue-owned implementation or evidence.
Any shared path discovered later requires typed replanning before execution.

## Serial Gates

Do not bind for implementation or use AWS until the distributed Runtime is
terminal through merged, ancestral, independently reviewed proof; #83 has local
implementation and independent validation; and an operator separately
authorizes AWS execution for #122. The approved business profile must then
resolve to the Agent Logic business account before any account state is used.

The authorization remains bounded to Route53, ACM, S3, CloudFront, and an
approved non-EC2 Runtime ingress. EC2, Spot, and CodeBuild remain forbidden.
#83 and #111-#117 continue independently and are not gated by #122.

## Validation And Review

Preparation validation proves only card structure and truthful deferral. Product
and AWS lanes remain deferred because their issue-owned targets do not yet exist
and the serial gates are open. Later execution must first prove local policy and
configuration, then exact DNS, certificate, HTTPS, WSS, revision parity, origin
policy, redaction, rate limits, ownership, rollback, and cleanup against the
operator-authorized business account.

An exact-head security and operations review must have no unresolved actionable
findings before public exposure. Evidence must omit account identifiers,
credentials, private state, and unnecessary infrastructure identifiers.

## Failure Policy And Non-Goals

Fail closed on a nonterminal distributed Runtime, missing operator
authorization, wrong AWS profile or account, forbidden compute, revision
mismatch, invalid certificate or hostname, permissive CORS/WSS origins, weak
authentication, unbounded traffic, redaction failure, ownership ambiguity,
incomplete rollback, or unresolved review findings.

This preparation does not implement product or infrastructure, use AWS, deploy,
publish, push, open a PR, merge, close #122, mutate #83/#110-#117, or gate their
local implementation, review, or publication.
