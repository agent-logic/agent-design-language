# Issue #5384 Preparation Design

## Decision

Prepare WP-14A as an evidence-consumer gate without beginning acceptance,
deployment, handoff, product, Runtime, documentation, demo, or release work.
The preparation claim owns only the typed issue projection, its typed request
packet, and its lifecycle lock. Promotion requires a new typed claim after all
declared predecessors satisfy the terminal gate.

## Inputs And Authority

- Live issue `#5384` and its routing update are scope authority.
- `docs/milestones/v0.91.8/WBS_v0.91.8.md`,
  `WP_ISSUE_WAVE_v0.91.8.yaml`, and
  `features/PLATFORM_ACCEPTANCE_AND_DEPLOYMENT_v0.91.8.md` define the checked-in
  topology.
- Typed C-SDLC v2 projections and shared-Git closeout receipts provide
  lifecycle authority.
- Current `origin/main` ancestry provides integration authority.
- GitHub closed/merged state must be refreshed through an approved connector;
  cached prose is never sufficient.

## Preparation Boundary

Allowed writes are limited to:

- `.csdlc/issues/5384`
- `.csdlc/prepared/issues/5384`
- `.csdlc/locks/5384.lock`

No other path is authorized. In particular, this claim cannot change product,
Runtime, C-SDLC implementation, milestone documentation, tests, workflows,
deployment state, provider state, or external infrastructure.

## Dependency Gate

Every entry in `dependency-gate.json` must satisfy all of these facts against
one immutable SHA resolved from the same refreshed `origin/main`:

1. live GitHub issue state is closed and any implementation PR is merged;
2. canonical typed phase is exactly `closed_out`;
3. the shared-Git terminal receipt exists and agrees with the projection;
4. terminal disposition is `merged` with a non-empty observed SHA;
5. the observed SHA is an ancestor of current `origin/main`.

Any absent, stale, conflicting, or ambiguous fact keeps #5384 in preparation.
No blocked, planned, reviewed, published, closed-without-merge, or prose-only
state satisfies this deliberately strict promotion gate.

## Evidence Model

WP-14A will consume exact-revision evidence; it will not recreate predecessor
proof or absorb independently owned defects. The execution phase must assemble:

- three-product acceptance and stable-install provenance;
- fresh-consumer, operations, rollback, and lifecycle evidence;
- child disposition and v0.92 handoff ledgers;
- explicit non-claims for identity, consciousness, unsupported providers, and
  unreviewed v0.92 implementation.

## COTS And Reuse Decision

| Need | Decision | Reason |
| --- | --- | --- |
| Lifecycle state/cards | Reuse typed C-SDLC v2 binaries | Canonical authority already exists; bespoke lifecycle logic is forbidden. |
| Git ancestry | Reuse Git | Mature deterministic DAG query; no custom graph engine. |
| JSON validation | Reuse Ruby JSON stdlib and `jq` | Available COTS/stdlib surfaces; no dependency addition. |
| Live GitHub truth | Reuse approved GitHub connector at promotion | Avoid raw `gh`, tokens, and cached issue prose. |
| Diagram | Mermaid text | Reviewable, deterministic, and already supported by repository docs. |

No package, service, cloud resource, model provider, or paid COTS purchase is
needed for preparation.

## Budget

| Lane | Seconds | Tokens | Resource | Release gate |
| --- | ---: | ---: | --- | --- |
| typed-card-contracts | 300 | 2,000 | small | yes |
| dependency-terminal-gate | 300 | 2,000 | small | yes |
| preparation-scope | 120 | 1,000 | small | yes |
| diff-hygiene | 60 | 500 | small | yes |
| exact-preparation-review | 600 | 6,000 | medium | yes |
| Total | 1,380 | 11,500 | bounded local | yes |

These are planning ceilings, not evidence that execution occurred.

## PVF Contract

All five lanes are deterministic except the reviewer model invocation, which is
bounded by an exact diff and must leave a durable report. Each lane declares its
proof role, acceptance IDs, resource profile, time/token budget, parallel group,
and required/deferred status through `proof_role` plus `defer_reason` in the
native VPP schema. A null `defer_reason` means the lane is required. The scope
lane inventories tracked and untracked files; diff hygiene remains separate.

## Promotion

Promotion is a separate operator-authorized session. It must re-fetch
`origin/main`, refresh live issue/PR truth, run the dependency checker, verify
receipts and ancestry, release or transition this claim through typed v2, and
bind a new implementation claim with newly reviewed product paths. This
preparation packet is not implementation authority.
