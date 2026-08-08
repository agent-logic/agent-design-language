# Issue 17 Design: Fail-Closed Execution Readiness

## Problem

`csdlc-doctor` currently accepts card consistency as execution readiness even
when the cards identify a different GitHub repository, a planned Rust module
cannot be routed within the owned paths, required validator targets do not
exist, or validation selects only unrelated tests. Issue 5795 demonstrates all
four false-ready conditions.

## Design

1. Resolve the configured `origin` GitHub repository without network access.
   When it is recognizable, compare it with the issue record repository and
   emit a deterministic repository-identity finding on mismatch.
2. Strengthen owned-path validation for new Rust modules under `src/`. A new
   module must include an owned existing module-routing surface such as the
   crate root or parent module. Existing files and independent Rust test or
   binary targets retain their current behavior.
3. Resolve explicit validator targets from validation-lane arguments. Missing
   shell scripts and Cargo integration-test targets block readiness unless the
   exact path is owned, listed as a deliverable, explicitly deferred, and
   governed by a fail-closed failure policy.
4. Require at least one lane to select an issue-owned validator target. Hygiene
   commands and unrelated existing tests may supplement proof but cannot be
   the issue-specific denominator.
5. Add focused binary-path fixtures reproducing issue 5795's false-ready shape
   and each corrected variant.

## Boundary

This issue changes C-SDLC doctor and card-readiness validation only. It does not
implement issue 5795, change Runtime behavior, contact GitHub, or infer build
success without executing the declared validation later in the lifecycle.

## Proof

The focused `gate2` integration suite creates temporary Git repositories and
typed issue records, then proves deterministic findings for repository drift,
unroutable Rust modules, undeclared missing validators, and zero issue-specific
test denominators. Existing valid create, doctor, and bind behavior remains
green.
