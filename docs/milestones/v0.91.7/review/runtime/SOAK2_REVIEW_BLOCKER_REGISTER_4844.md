# Soak 2 Review And Blocker Register (#4844)

This review consumes the current WP-07 Soak 2 evidence available on July 4,
2026. It is findings-first and intentionally conservative: no matrix row is
classified as `integrated_proven` because #4682 has not yet completed a final
integrated Soak 2 run against consumable upstream runtime and resilience PRs.

Authoritative machine-readable companion:

- `docs/milestones/v0.91.7/review/runtime/soak2_review_blocker_register_4844.json`

## Findings

### F-4844-01: Soak 2 cannot prove v0.92 runtime coherence yet

Severity: blocker

Evidence:

- #4682 PR #4873 records `blocked_before_full_soak` at
  `d12db8b82be6827fb800942e641ec955c0a9e463`.
- #4681 PR #4868 is non-draft at
  `0b8821ec1de112d3b84fa64e87d2a6fb9fb63a02`, with `adl-coverage` green and
  `adl-ci` still pending.
- #4783 PR #4869 is non-draft at
  `cc87f735758ff78bc5a3360fe19d45c4b025552e`, with `adl-ci` green and
  `adl-coverage` failed.

Disposition: WP-07 and #4634 must remain open. #4682 must rerun or refresh
after prerequisites are consumable.

### F-4844-02: Logging/OTel is issue-proven, not final Soak-integrated

Severity: blocker

Evidence:

- #4718 is merged and its proof packet records parse-safe stdout, stderr
  `adl_event` observability, redaction hygiene, and an OTel-compatible mapping
  boundary.
- #4682 records `logging_observability` as `prerequisite_proven`, not
  `integrated_proven`.

Disposition: do not claim full WP-07 logging/OTel runtime integration until
#4682 consumes #4718 in the final integrated Soak 2 run.

### F-4844-03: Row-level matrix truth must be preserved

Severity: high

Evidence:

- #4843 defines 15 Soak 2 matrix rows.
- #4682 currently records 6 aggregate row results because the run stopped before
  upstream prerequisites were consumable.

Disposition: this #4844 register expands every #4843 row into a blocked,
deferred, or future-Soak disposition without inventing integrated evidence.

## Row Dispositions

| Matrix row | Disposition | Blocking owner | v0.92 impact |
| --- | --- | --- | --- |
| `tokio_runtime_substrate` | blocked | #4681 | Blocks runtime-coherence claims until the assembled runtime path runs with retained evidence. |
| `agent_lifecycle` | blocked | #4681 | Blocks startup, wake, stop, copied-state, and continuity claims. |
| `aee_path` | blocked | #4682 | Blocks temporary-agent execution claims in the integrated runtime path. |
| `acip_a2a_path` | blocked | #4658 | Blocks ACIP/A2A activation claims unless explicitly non-claimed by the operator. |
| `provider_model_substrate` | blocked | #4682 | Blocks provider-backed runtime claims depending on live or cheapest-validated routing. |
| `scheduler` | blocked | #4682 | Blocks premium-capacity and cheapest-validated-outcome runtime claims in live packets. |
| `resilience` | blocked | #4783 | Blocks resilience claims for the activation runtime path. |
| `logging_observability` | blocked | #4682 | Blocks sprint-level logging/OTel readiness beyond the #4718 proof boundary. |
| `runtime_aws_signal_bridge` | blocked | #4684 | Blocks runtime AWS/signal bridge claims unless explicitly non-claimed. |
| `observatory_unity` | blocked | #4682 | Blocks live Observatory consumption claims for Soak 2. |
| `obsmem_memory_handoff` | blocked | #4682 | Blocks long-running context memory continuity claims depending on retained handoff evidence. |
| `identity_continuity` | blocked | #4681 | Blocks continuity and birthday identity claims. |
| `capability_envelope` | blocked | #4656 | Blocks capability-envelope claims and witness/receipt readiness. |
| `security_cav_boundary` | blocked | #4656 | Blocks security/CAV activation readiness. |
| `curiosity_constructability_optional` | deferred | none | Does not block v0.92 unless promoted into the activation path. |

## Blocker Register

| Blocker | Owner | Evidence | Required action |
| --- | --- | --- | --- |
| Canonical runtime path not consumable | #4681 | PR #4868 has `adl-ci` pending. | Wait for checks, merge or explicitly route blocker, then rerun #4682. |
| Scheduler watcher/AEE resilience middleware not consumable | #4783 | PR #4869 has `adl-coverage` failed. | Route through janitor, restore checks, merge or explicitly route blocker, then rerun #4682 resilience rows. |
| Matrix, failure-injection proof, and diet map are PR-bound | #4843/#4784/#4683 | PRs #4870/#4871/#4872 are ready-green but not on `main`. | Land or explicitly sequence before final #4682 consumption. |
| Logging/OTel proof not consumed by final integrated run | #4682 | #4718 is merged, but #4682 has not run final Soak 2. | Consume #4718 during final #4682 run before claiming sprint-level integration. |
| WP-12 and AWS/signal rows remain blocked before Soak 2 | #4656/#4658/#4684 family | #4843 matrix marks these rows blocked before Soak 2. | Keep blocked unless owner issues prove or operator explicitly approves non-claims. |

## Non-Claims

- This review does not claim v0.92 runtime coherence.
- This review does not claim any row is `integrated_proven`.
- This review does not claim production OpenTelemetry collector, OTLP exporter,
  hosted telemetry service, or Unity editor execution.
- This review does not replace final #4682 Soak 2 runtime evidence.

## Required Next Actions

1. Keep #4634 open.
2. Continue watching #4868 until pending checks resolve and route #4869 through janitor for failed `adl-coverage`.
3. After #4681/#4783 and sequencing PRs are consumable, rerun #4682 against the
   #4843 matrix.
4. Refresh this register after a real #4682 integrated run or after explicit
   operator blocker approval.
