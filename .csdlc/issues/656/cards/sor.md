# Structured Output Record

Template: 1.0.0

Issue: 656

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented immutable matched Runtime v3 generations with one receipt, atomic current and previous references, rollback, platform and schema checks, and CSM preflight before start or reload mutation.

## Artifacts

- adl/tools/install_runtime_v3_generation.sh
- adl/tools/runtime_v3_generation.py
- adl/src/cli/csm_runtime_v3_cmd.rs
- adl/tools/test_runtime_v3_generation_install.sh
- adl/tests/csm_runtime_v3_generation.rs

## Execution

- Added a single Runtime v3 generation command for install, verify, and rollback.
- Bound CSM, Guardian, and kernel hashes plus source revision, platform, build profile, and Runtime-init schema in one receipt.
- Required installed CSM, kernel, and launchd or systemd Guardian paths to resolve through the same current generation.
- Moved current and candidate generation verification ahead of interrupted-transaction reconciliation and service stop paths.

## Validation

[]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
