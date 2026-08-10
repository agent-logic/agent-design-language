# Structured Task Prompt

Template: 1.0.0

Issue: 137

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Create, prove, independently review, publish, shepherd, and merge only the WP-04 native distributed workflow and issue #137 lifecycle/proof surfaces.

## Deliverables

- .github/workflows/wp04-native-distributed.yml
- .csdlc/prepared/issues/137/validate-workflow.rb

## Acceptance

1. AC-1: Manual dispatch requires an exact 40-character lowercase hexadecimal commit SHA and checks out exactly that revision.
2. AC-2: A bounded matrix runs Linux, macOS, and Windows with Rust and cargo-nextest installed and the existing producer invoked through Bash.
3. AC-3: Each platform uploads a distinct fail-closed receipt fragment and Ubuntu downloads all fragments for validation with the existing aggregate validator.
4. AC-4: Actions are commit-pinned, permissions are read-only, timeouts are bounded, and no #5878 producer, validator, manifest, or evidence file changes.
5. AC-5: Focused workflow contract validation, repository path-policy validation, diff hygiene, exact-head independent review, and full hosted CI pass.

## Dependencies

- agent-logic/agent-design-language#5878
- Existing #5878 producer and aggregate validator at origin/main

## Inputs

- AGENTS.md
- .github/workflows/ci.yaml
- adl/tools/validate_v092_distributed_guardian.sh
- adl/tools/validate_v092_distributed_native_receipts.rb
- adl/tools/test_ci_path_policy.sh
- git commit d401c2d1627ac9596e3b8bd1636987beed02274e:.github/workflows/wp04-native-distributed.yml

## Non Goals

- Changing #5878 product code, producer, validator, manifest, or evidence
- Claiming native proof before hosted jobs execute
- Adding secrets, write permissions, schedules, or unbounded jobs
- Post-merge closeout or cleanup bookkeeping
