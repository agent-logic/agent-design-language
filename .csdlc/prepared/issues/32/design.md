# Issue #32 design: branch-independent larger-runner eligibility preflight

## Problem

GitHub can report an organization-hosted larger runner as Ready while a job is
permanently ineligible to acquire it. Runner group 3 was already scoped to
`agent-logic/agent-design-language`, but its redundant workflow restriction
contained branch-qualified entries, including a deleted experiment branch.
Normal pull-request branches therefore queued without a runner assignment even
though the runner reported available capacity.

The current repository has no focused native diagnostic that distinguishes
runner capacity from runner-group policy. Operators must correlate multiple
GitHub settings manually, and a policy-ineligible job looks like ordinary
capacity pressure.

## Design

Add a focused `runner-preflight` operation to the existing independent Rust
`csdlc-github` binary. It reuses the package's Octocrab, serde, clap, token
resolution, typed-error, and JSON-output conventions without adding another
installed binary. It accepts one typed JSON request naming the repository,
organization, runner group, expected hosted runner label, and workflow path.

The preflight reads only these GitHub surfaces:

1. the organization's hosted larger-runner inventory;
2. the named runner group's visibility and workflow-restriction state;
3. the group's selected-repository inventory; and
4. any selected workflow references retained by the group.

It emits a redacted typed packet with separate `capacity`, `policy`, and
`dispatchability` classifications. An optional Actions job id provides the
bounded canary observation:

| Capacity | Policy | Dispatchability | Overall classification |
|---|---|---|---|
| ready | eligible | proven | eligible |
| ready | eligible | unproven | configuration_eligible_dispatch_unproven |
| ready | eligible | timed_out | dispatch_unavailable |
| unavailable | eligible | any | capacity_unavailable |
| any | ineligible | any | policy_ineligible |

`policy=eligible` requires the target repository to be explicitly selected and
`restricted_to_workflows=false`. This is deliberately branch-independent: the
repository boundary remains, while no branch-qualified workflow allowlist can
silently exclude a pull request. When restrictions are enabled, the packet
reports the selected workflow entries and checks their repository, workflow
path, and Git ref so stale/deleted entries are visible. It never prints the
token or request headers.

`Ready` is configuration evidence, not dispatch proof. When a canary job is
provided, the preflight reads its runner name, runner group, labels, timestamps,
run id, workflow path, pull requests, and head SHA. A job assigned to the
expected label and group proves dispatch only when its run, workflow, PR, and
head exactly match the typed request. A still-unassigned job older than the
request's bounded queue threshold reports `dispatch_unavailable`; a completed
skipped or cancelled job inside that bound is terminal-unassigned rather than
being mislabeled a timeout. With no canary, an otherwise healthy configuration is
explicitly `configuration_eligible_dispatch_unproven`, never `eligible`.

The command is diagnostic and fail-closed. GitHub authorization or schema
failures return a typed error instead of claiming eligibility. Every packet
other than canary-proven `eligible` is valid diagnostic JSON and exits nonzero
so CI/operator preflight can stop before launching an indefinitely queued job.

## Invariants

1. Branch-independent eligibility requires workflow restriction to be off.
2. Repository scope must explicitly include `agent-logic/agent-design-language`.
3. Capacity and policy are never collapsed into one ambiguous `queued` state.
4. Selected workflow references are reported without credentials and stale
   references are identified.
5. The preflight is read-only and performs no organization-setting mutation.
6. No AWS surface is used.
7. `Ready` plus policy eligibility is not reported as dispatchable without an
   assigned live canary.

## Validation

- Rust unit tests cover eligible, configuration-eligible/unproven,
  dispatch-unavailable, capacity-unavailable, and policy-ineligible
  classification.
- Rust parser tests cover selected workflow refs, foreign repositories,
  mismatched workflow paths, malformed refs, and stale-ref reporting.
- Public-schema tests prove the typed request/result contract is discoverable.
- A live read-only preflight checks group 3 and the hosted runner after local
  validation.
- The reviewable PR is the canary: the configured larger-runner job must receive
  a runner name and reach a terminal result without branch allowlist changes.

## Rollback

Remove the command, its module/export, and focused tests. The command is read-only
and owns no persistent state, so rollback requires no migration or GitHub setting
change.
