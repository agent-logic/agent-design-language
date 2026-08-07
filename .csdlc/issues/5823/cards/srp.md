# Structured Review Prompt

Template: 1.0.0

Issue: 5823

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/5823
.csdlc/prepared/issues/5823/validate-platform-matrix.rb
adl/tools/aws_spot_artifact_finalize.py
adl/tools/run_aws_spot_remote_validation_lane.sh
adl/tools/run_nessus_remote_validation.sh
adl/tools/test_run_aws_spot_remote_validation_lane.sh
adl/tools/test_run_nessus_remote_validation.sh
tools/aws_remote_validation/src/aws_remote_validation.rs
tools/aws_remote_validation/src/bin/adl_aws_remote_validation.rs
tools/aws_remote_validation/tests/portable_adapter.rs
tools/remote_validation/Cargo.lock
tools/remote_validation/Cargo.toml
tools/remote_validation/src/bin/adl-remote-validation.rs
tools/remote_validation/src/lib.rs
tools/remote_validation/tests/contract.rs

## Prompts

- Do all adapters preserve one exact-revision request/result and artifact contract?
- Can network, timeout, cancellation, malformed output, or cleanup failure produce false success?
- Does local no-network fallback remain equivalent and available?
- Are Linux, macOS, Windows, observability, redaction, credential, and AWS-account boundaries truthful?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Live AWS and Nessus behavior remains dependent on external provider timing and availability.
- Windows proof remains fixture-qualified rather than live-native.
- Native Linux execution is pinned to 2c76b1f8; later fail-closed redaction repairs are covered by focused deterministic contract and wrapper proof.

## Review Result

Revision: Some("git-blake3:6ce1e9051ffb5124f670eda5e9bd7477855d72c9:80c27d2cbd7a4a7a78a1f9070d6d09fb85125001146db14d10a6afb1212cc39a")

Reviewer: Some("subagent:019fde22-3b39-7572-8a05-847988d33b3d")

Result: pass
