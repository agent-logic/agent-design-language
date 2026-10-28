# WP-13 Boundary And Implementation Sprint Review

Issue: #4640
Review issue: #5403
Status: changes required
Remediation: #5405; shared records issue #5406

## Findings

### P1: Guild `integrated_proven` status lacks an implemented guild substrate

`adl/src/runtime_v2/guild_foundation_boundary.rs:43` constructs six string
identifiers such as `guild_identity_record` and `member_role_registry`, and its
validator around line 217 checks that exact string set. It defines no identity
record, registry, membership event, moderation hook, or witness-evidence
producer/consumer behavior. The focused test at
`adl/src/runtime_v2/tests/guild_foundation_boundary.rs:16` proves containment of
one identifier rather than production or consumption of a guild record.

The parent closeout nevertheless labels #4755 `integrated_proven` at
`docs/milestones/v0.91.7/review/wp13_closeout_4640.md:27`, and the v0.92 handoff
repeats that classification at
`docs/milestones/v0.91.7/V092_HANDOFF_v0.91.7.md:69`.

Impact: downstream work can treat guild identity and witness routing as
available when only vocabulary and a non-claim gate exist.

Disposition: open. Route a #4755 remediation issue and downgrade current truth
to `boundary_proven` until one producer/consumer path proves the named records
and hooks.

### P2: Godel documentation calls non-invoked launch-plan records executable

`docs/milestones/v0.91.7/review/wp13_godel_constructability_boundary_4753.md:5`
says the agents are executable as admitted provider requests. Source creates
every request as `admitted_provider_request_not_invoked` at
`adl/src/runtime_v2/godel_agent_runtime.rs:418` and validates the non-invocation
contract around line 917.

Impact: `executable` can be read as dispatch or runtime-execution proof, while
the implementation proves admission-plan completeness only. The parent
closeout's narrower launch-admission wording is accurate.

Disposition: open. Correct #4753 documentation to use `admission readiness`
and retain the explicit `not_invoked` boundary.

### P2: Economics validation accepts duplicate policy entries

The allowed-consumption, postponed-surface, and promotion-gate validators use
`BTreeSet` equality without checking the original collection length at
`adl/src/runtime_v2/economics_civilization_boundary.rs:224`, line 242, and line
319. Canonicalization at line 182 sorts but preserves duplicate entries.

Impact: malformed policy packets containing duplicate semantic rows pass
validation and produce non-canonical evidence.

Disposition: open. Route a focused #4754 regression fix using the
duplicate-rejection pattern already present in the guild validator.

### P2: Merged source cannot audit the lifecycle records required by closeout

The parent packet says it is not a replacement for child SORs at
`docs/milestones/v0.91.7/review/wp13_closeout_4640.md:71`. Those records live
under ignored `.adl` state (`.gitignore:3`), while
`adl/tools/check_no_tracked_adl_issue_record_residue.sh:13` explicitly forbids
tracking the old issue-record format. None of the seven issue bundles is
available in the reviewed clean worktree.

Impact: exact SRP findings, SOR commands, deferred proof, and lifecycle
normalization cannot be reconstructed from merged source. PR summaries are not
durable card truth.

Disposition: open. Route records-hygiene work that retains typed-v2 ledger or
immutable closeout references without restoring sunset v1 `.adl` records.

## Child Coverage

| Child | PR | Reviewed result |
| ---: | ---: | --- |
| #4752 | #5165 | Affect safe-test composition, resident metadata, runtime fallback, tests, and claim guards; no new finding |
| #4753 | #5171 | Ten-agent admission plan and constructability boundary; provider invocation remains unproven; P2 wording finding |
| #4754 | #5185 | Context-only economics boundary and tests; P2 duplicate-validation finding |
| #4755 | #5189 | Declarative guild boundary and tests; P1 integration overclaim |
| #4756 | #5193 | CodeFriend plan, obligation contract, and JSON parity test; product, adapter, and external execution remain non-claims |
| #4757 | #5197 | Publication scope/non-claim packet and milestone wiring; no publication artifact is claimed |
| #4640 | #5199 | Parent reconciliation packet; all child issues and PRs are closed and merged |

## Previously Found During Pre-PR Work

- #4752 recorded weak safe-test scenario, allowed-claim, and runtime-input
  validation; those findings were fixed before merge.
- #4755 recorded duplicate-drift risk and added regression tests before merge.
- The economics duplicate finding above is review-discovered and distinct from
  the fixed guild duplicate defect.

## Validation And Limits

All four findings above are review-discovered; no test-discovered defect is
counted above.

- Architecture, code, safety/claims, tests, docs, dependencies, lifecycle,
  closeout, and retained-evidence lanes were reviewed.
- WP-13 added no direct manifest or lockfile dependency changes.
- No tests were rerun during the read-only specialist pass.
- Slow-proof jobs were skipped on the reviewed PRs.
- No end-to-end provider invocation, guild workflow, economics activation,
  CodeFriend external-repo execution, or external publication is proven.

## Review Result

Changes required. WP-13 must not be consumed as review-clean while the guild
integration overclaim remains. The three P2 findings require remediation or
explicit bounded acceptance before release consumption.
