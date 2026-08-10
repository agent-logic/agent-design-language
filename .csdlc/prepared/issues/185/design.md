# DRT-05 Design

Issue: #185

## Objective

Qualify distributed identity, authority, TLS, capability, stale-fence, provider-failure, malformed-message, and pre-auth disclosure boundaries.

## Scope

Node, agent, Shepherd, operator, and Observatory key separation; trust domains; public TLS and private mTLS; permits and capabilities; stale lease/fence; cross-polis replay; provider stalls; malformed traffic; REST/WSS pre-auth behavior.

## Dependencies

- DRT-03: issue #183
- DRT-04: issue #184

## Architecture Decisions

- No issue-specific source decision; all milestone decisions still apply.

## Deliverables

- Producer-derived security and failure matrix with exact envelope and authority inputs.
- Independent verifier for every positive and negative outcome without hard-coded counts.

## Owned Paths

- `adl-runtime/tests/v0921_distributed_security.rs`
- `adl/tools/v0921/drt-05/**`
- `.csdlc/issues/185/**`
- `.csdlc/prepared/issues/185/**`
- `.csdlc/evidence/185/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

1. Voting, agent, Shepherd, operator, and Observatory identities use separated keys and roles; Shepherd cannot vote.
2. Production TLS chains to an approved trust anchor and no self-signed certificate appears on a production path.
3. Forged, stale, wrong-domain, missing-capability, cross-polis, malformed, and pre-auth disclosure attempts are denied with typed receipts.
4. Provider timeout, stall, malformed output, and partial failure preserve state and authority invariants.

## PVF Lanes

- `drt-05-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/185/validate-outcome.rb`.
- `drt-05-production-proof`: Execute the exact production-path qualification or deterministic conformance command for this Runtime slice. Command: `bash adl/tools/v0921/drt-05/validate.sh`.
- `drt-05-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Key-role inventory, trust-chain validation, permit/capability mutation matrix, stale fence/lease, cross-polis, malformed, pre-auth REST/WSS, provider-failure, and independent receipt recomputation.

## Authority Boundary

- Issue DRT-05 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Writing custom TLS primitives
- Treating transport encryption as authorization
- Generating outcome totals independently of producer results

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Any role shares an unauthorized key
- A production path accepts self-signed TLS
- A denied operation mutates state
- Receipt totals are not producer-derived

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml#drt-05`
