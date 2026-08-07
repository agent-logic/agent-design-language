# Structured Review Prompt

Template: 1.0.0

Issue: 5823

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/prepared/issues/5823/validate-platform-matrix.rb
adl/tools/run_aws_spot_remote_validation_lane.sh
adl/tools/run_nessus_remote_validation.sh
adl/tools/test_run_aws_spot_remote_validation_lane.sh
adl/tools/test_run_nessus_remote_validation.sh
tools/aws_remote_validation/src/aws_remote_validation.rs
tools/aws_remote_validation/src/bin/adl_aws_remote_validation.rs
tools/aws_remote_validation/tests/portable_adapter.rs

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

## Review Result

Revision: Some("git-blake3:47c6e446efc17200015d4482b384bfe0b708499d:d07f8ad3e1e4ce1d8904ab3a239bf0148753b1919bc4ad9754dd64859cea8cab")

Reviewer: Some("subagent:wp06-final-review")

Result: pass
