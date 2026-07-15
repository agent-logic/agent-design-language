# Tools Workflow Reliability Tail Review

Issue: #5036
Review issue: #5403
Status: changes required
Remediation: #5407; shared records issue #5406

## Findings

### P1: Build-action logging closed after implementing only one producer

`docs/tooling/BUILD_ACTION_LOGS.md:3` and line 42 identify
`validation_manager.py --run` as the integrated producer and leave CI
integration to future consumers. Repository references locate packet production
only at `adl/tools/validation_manager.py:1404`.

#5032 also required `pr finish`, owner lanes, remote builders, CI ingestion,
watcher/shepherd reporting, and fail-closed closeout behavior. Those acceptance
surfaces are not implemented.

Impact: build and validation actions outside validation-manager execution can
still disappear without durable action evidence despite the child being closed.

Disposition: open. Route a #5032 completion issue covering every named producer
and consumer before claiming mandatory build-action logging.

### P1: The retained CLI taxonomy directs operators to sunset v1 commands

`docs/tooling/ADL_PLATFORM_CLI_BINARY_TAXONOMY.md:28` and line 36 recommend
`adl/tools/pr.sh` and the removed compatibility binary. Current Gate 10D2
authority at `AGENTS.md:5` and line 43 says those wrappers are removed and the
typed v2 binaries are the sole operational authority.

Impact: current operator-facing documentation directs users to unsupported,
deleted lifecycle commands.

Disposition: open. Route a documentation/authority repair tied to #4995 and
the final C-SDLC v2 sunset contract.

### P2: The umbrella lacks an internally current retained closeout synthesis

`docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md:98` records
that the execution packet still contains pending children, the named closeout
artifact is absent, and integrated #4938 proof is not retained.

Impact: #5036 closure is supported by live issue and PR state rather than a
complete durable lifecycle and closeout packet.

Disposition: partly fixed by this review. Route typed-v2 closeout normalization
through the shared records-retention remediation.

### P2: The claimed material CI speedup lacks comparative hosted-run evidence

`docs/milestones/v0.91.7/review/build_throughput/CI_CONTRACT_SPLIT_5037.md:57`
says wall-clock improvement must be confirmed from GitHub-hosted runs, but line
61 retains only near-zero local policy-script timings.

Impact: green checks establish correctness, not the claimed material reduction
in CI duration.

Disposition: open. Route a #5037 evidence follow-up retaining comparable
before/after hosted timings or narrow the claim to policy-split correctness.

## Child Coverage

Reviewed #5034, #5032, #5037, #5031, #5028, #5012, #5002, #4999, #4995,
#4987, and #4938. All are live-closed through merged PRs with successful
required checks. #5037 was omitted from the operator-selected execution list
but later added to the declared umbrella wave and is included in this review.

Previously discovered and fixed defects, including #5037's two pre-PR P1
findings, are not counted among this review's four findings. All four current
findings are review-discovered.

## Validation And Limits

No tests or mutating commands were run during the read-only specialist pass.
Historical local SRP/SOR cards are absent after v1 sunset; PR descriptions were
used as lifecycle summaries but not treated as durable card truth.

## Review Result

Changes required. The two P1 findings affect currently supported operator and
logging behavior; the two P2 findings limit closeout and performance claims.
