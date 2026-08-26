# Structured Task Prompt

Template: 1.0.0

Issue: 268

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement the fixed six-hour suite and issue-owned launch/proof wrapper, run one authorized Spot attempt, review exact evidence, publish, merge, finish, and clean up.

## Deliverables

- .csdlc/evidence/268
- .csdlc/issues/268
- .csdlc/prepared/issues/268
- adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
- adl/tools/validate_v092_runtime_guardian_lifecycle.sh
- adl/tools/run_aws_spot_remote_validation_lane.sh
- adl/tools/run_issue268_six_hour_spot_qualification.sh
- adl/tools/test_run_issue268_six_hour_spot_qualification.sh

## Acceptance

1. AC-1: Exact base contains the merged #266, #267, #373, and #374 production qualification inputs.
2. AC-2: The suite fixes a minimum 21,600-second monotonic exposure, enforces a numeric 600-second timeout and cap on final-cycle overshoot, records measured elapsed and overshoot, and rejects caller duration weakening.
3. AC-3: Launch fails before mutation without exact Agent Logic identity, immutable revision/image, Spot-only posture, deadline, kill switch, and USD 20-or-lower ceiling.
4. AC-4: One attempt sustains at least 50 authenticated HTTPS and WSS connections and records every declared fault/recovery receipt without threshold weakening.
5. AC-5: Success, failure, interruption, timeout, and cancellation retain causal evidence and terminate only exact run-tag-owned resources.
6. AC-6: Independent post-cleanup readback proves zero task-owned instances and evidence is redacted and digest-bound.
7. AC-7: Focused contracts, strict Clippy, exact scope, paid receipt validation, exact-head review, hosted CI, finish/cache/ancestry all pass.

## Dependencies

- Closed #266 with squash merge 86a18c8f5
- Closed #267 with squash merge ea8b76fcd
- Closed #373 with squash merge 03e23c6a6
- Closed #374 with squash merge 87b100dfb
- Operator authorization: #268 only, USD 20 total ceiling

## Inputs

- adl-runtime/src/runtime_v3_soak.rs
- adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
- adl/tools/run_aws_spot_remote_validation_lane.sh
- adl/tools/run_runtime_v3_guardian_soak.sh

## Non Goals

- Issue #269 or any 24-hour run
- GPU instances
- On-Demand fallback
- Release-policy change
- Cloud Polis deployment
