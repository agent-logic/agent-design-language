# v0.91.7 Resilience Integration Mini-Sprint Review for `#4778`

Status: `complete_child_issues_closed`
Issue: `#4778`
Date: 2026-07-07

## Scope

This packet records the `#4778` resilience mini-sprint state. The sprint goal
is to move resilience from component substrate and proof packets into the
runtime, workflow, provider, AWS/local, and failure-injection paths used by
v0.91.7 execution.

This packet consumes child issue proof instead of rerunning every child proof.
All tracked child issues are now closed with retained proof or closeout truth.

## Child Issue State

| Issue | Scope | Current truth | Retained proof |
| --- | --- | --- | --- |
| `#4780` | PR and CI shepherding resilience | closed; PR `#5008` merged with `adl-ci` and `adl-coverage` green | `adl/src/cli/pr_cmd/github/transport.rs`; `adl/src/cli/pr_cmd/github/tests/validation.rs`; `docs/milestones/v0.91.5/CONTROL_PLANE_OBSERVABILITY_CONTRACT_3609.md` |
| `#4781` | Provider and model-call resilience | closed; PR `#5014` merged with `adl-ci` and `adl-coverage` green, `adl-slow-proof` skipped | `adl/src/provider_adapter.rs`; local focused provider tests, pre-PR review, and PR `#5014` checks |
| `#4782` | AWS SSM EC2 and remote-builder resilience | closed as a cross-cutting WP-06/WP-08 resilience dependency for AWS/remote operations | WP-06 remote-build packets, `docs/tooling/REMOTE_BUILD_HOW_TO.md`, and WP-08 AWS/signal consumption truth in `docs/milestones/v0.91.7/review/V0917_WP08_RUNTIME_AWS_SIGNAL_OPERATIONS_4635.md` |
| `#4783` | Scheduler watcher and AEE resilience middleware | closed with integrated runtime trace evidence | `docs/milestones/v0.91.7/review/runtime/RUNTIME_RESILIENCE_MIDDLEWARE_4783.md` |
| `#4784` | Integrated resilience failure injection | closed with retained failure-injection proof artifacts | `docs/milestones/v0.91.7/review/runtime/v0917_integrated_resilience_failure_injection_4784/README.md` |

## Implemented Resilience Surfaces

The landed child issues currently prove:

- PR/CI shepherding records durable `adl.pr_validation_attempt.v1` JSONL
  attempt evidence when `ADL_PR_VALIDATION_ATTEMPT_LOG` is set, stops on
  terminal red checks, preserves retryable transport behavior inside the
  existing octocrab retry policy, and records terminal/pending check snapshots
  without requiring raw GitHub CLI.
- Runtime scheduler/AEE integration emits `adl.runtime.resilience_trace.v1`
  decisions through the real executor path for admission, queued backpressure,
  success, degraded continue-on-error, terminal failure, timeout,
  cancellation, and called-workflow inner provider steps.
- Failure-injection proof covers retry, timeout, cancellation,
  circuit-terminal guard, rate/backpressure, bulkhead, degraded fallback, and
  terminal auth/quota/policy negative classifications across currently
  available integrated ADL paths.
- AWS/local operations consume WP-06 remote-builder proof and WP-08 SSM,
  storage, and signal proof surfaces with Agent Logic AWS account checks,
  explicit live/dry-run boundaries, retained redacted proof, and cleanup/cost
  documentation.

## Provider Gate

`#4781` was the final sprint gate. PR `#5014` implements provider/model call
circuit-breaker integration in `adl/src/provider_adapter.rs`, including:

- provider plus provider-model circuit keys;
- open-circuit short-circuiting to `ProviderInvocationFinalStatusV1::Blocked`;
- `circuit_breaker_decision` provider run events;
- preflight failures that return before circuit execution; and
- focused tests for repeated failures, half-open recovery, and preflight trace
  non-claims.

Repo-native validation reports PR `#5014` as merged at commit
`3c785df4cd1dea640476d4ac472e4ceece41b258`, with `adl-ci: SUCCESS`,
`adl-coverage: SUCCESS`, `adl-slow-proof: SKIPPED`, and no failed or pending
checks. Explicit `pr.sh closeout 4781` also passed STP/SIP/SOR validators and
confirmed the issue worktree was already absent.

## Validation Evidence

The child issues retain their local validation and PR evidence. The umbrella
worktree has so far run repo-native state checks:

```text
adl pr doctor 4778 --version v0.91.7 --json
adl pr watch 4778 --json
adl pr issue view 4780 --json
adl pr issue view 4781 --json
adl pr issue view 4782 --json
adl pr issue view 4783 --json
adl pr issue view 4784 --json
adl pr validation 5014 --json
adl pr watch 4781 --json
adl/tools/pr.sh closeout 4781 --version v0.91.7
```

Observed result: `#4780`, `#4781`, `#4782`, `#4783`, and `#4784` are closed.
PRs `#5008` and `#5014` are merged with required checks green.

## Non-Claims

- This packet does not claim full product resilience, durable hibernation,
  replay migration, or production runtime readiness beyond the listed proof
  surfaces.
- This packet does not claim paid AWS proof runs are automatic CI behavior.
- This packet does not replace child SOR/SRP closeout truth; it consumes the
  merged child outcomes for sprint-level review.
