---
issue_card_schema: adl.issue.v1
wp: "[v0.92.2][CF-ADAPTER-CI] Ingest CI repository inputs into a repository packet"
slug: "880-ci-ingestion"
title: "[v0.92.2][CF-ADAPTER-CI] Ingest CI repository inputs into a repository packet"
labels:
  - "track:roadmap"
issue_number: 880
generated_at: "2026-09-11T23:55:27.199943+00:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "v0.92.2"
required_outcome_type:
  - "production-behavior"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/880"
canonical_files: []
demo_required: yes
demo_names: []
issue_graph_notes:
  - "Use only numeric prerequisites from source issue; no sprint-wide or closeout dependency."
pr_start:
  enabled: true
  slug: "880-ci-ingestion"
---

Canonical Template Source: `docs/templates/prompts/1.0.5/stp.md`
Generated: 2026-09-11T23:55:27.199943+00:00

# Structured Task Prompt

## Summary

[v0.92.2][CF-ADAPTER-CI] Ingest CI repository inputs into a repository packet

## Goal

The CI ingestion entrypoint consumes declared checkout/revision/scope inputs and emits a usable common packet with truthful partial-input state. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); shared command/path decisions belong to WP-01/#864. This task provides CI input acquisition, not architecture fitness gating (CF-GOV-CI).

## Required Outcome

The CI ingestion entrypoint consumes declared checkout/revision/scope inputs and emits a usable common packet with truthful partial-input state. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); shared command/path decisions belong to WP-01/#864. This task provides CI input acquisition, not architecture fitness gating (CF-GOV-CI).

## Deliverables

The CI ingestion entrypoint consumes declared checkout/revision/scope inputs and emits a usable common packet with truthful partial-input state. Depends only on CF-ADAPTER's merged contract (canonical number supplied at batch creation); shared command/path decisions belong to WP-01/#864. This task provides CI input acquisition, not architecture fitness gating (CF-GOV-CI). Include production proof, failure handling and operator documentation required by the source issue.

## Acceptance Criteria

1. In an isolated CI-style checkout, invoke the production route with explicit revision and scope, read back its packet through the production reader, and compare to local acquisition of the same content. Environment metadata is provenance, not authority to substitute a different revision or scope. 2. Execute a real required CI smoke using the installed candidate on a bounded fixture checkout. Preserve exact candidate/source revisions and artifact digest; local emulation is local proof, not proof that CI ran. The job uses least necessary read-only repository permission and no provider secrets. 3. Reject missing, malformed, mismatched or unresolvable revision; handle shallow/partial checkout or missing files with explicit omissions/incomplete state. Test input bounds, path escape and credential-shaped environment values. Never serialize broad environment dumps, checkout absolute paths or runner credentials. 4. Ensure deterministic common packet semantics across relocated runner directories and equivalence with local source; retain run-specific provenance separately. Failed upload/transport or missing artifact cannot be represented as successful evidence delivery. Repository instructions remain inert data. 5. Publish concise declared-input/output and failure instructions. Complete the installed acquisition path and focused CI proof, not a workflow stub or schema. No review success or fitness-function result is inferred from ingestion success.

## Repo Inputs

Full canonical issue https://github.com/agent-logic/agent-design-language/issues/880; docs/milestones/v0.92.2/CREATION_SELECTIONS_v0.92.2.md; WP_EXECUTION_SPECIFICATIONS_v0.92.2.yaml and ATOMIC_TASK_CONTRACTS_v0.92.2.json.

## Dependencies

Accepted merged output required from #878. No sprint-wide barrier or asynchronous closeout dependency.

## Target Files / Surfaces

Reuse the predecessor's selected installed command and `adl/src/codefriend/ingestion/` contract. Own proposed `adl/src/codefriend/ingestion/ci.rs`, narrow CLI registration, focused `adl/tests/codefriend_ci_ingestion.rs` and one named `.github/workflows/` smoke job chosen in the issue plan. The workflow must call the installed product path; it is not a competing ingestion implementation. Read the portable-adapter feature and adopted contracts before execution.

## Validation Plan

1. In an isolated CI-style checkout, invoke the production route with explicit revision and scope, read back its packet through the production reader, and compare to local acquisition of the same content. Environment metadata is provenance, not authority to substitute a different revision or scope. 2. Execute a real required CI smoke using the installed candidate on a bounded fixture checkout. Preserve exact candidate/source revisions and artifact digest; local emulation is local proof, not proof that CI ran. The job uses least necessary read-only repository permission and no provider secrets. 3. Reject missing, malformed, mismatched or unresolvable revision; handle shallow/partial checkout or missing files with explicit omissions/incomplete state. Test input bounds, path escape and credential-shaped environment values. Never serialize broad environment dumps, checkout absolute paths or runner credentials. 4. Ensure deterministic common packet semantics across relocated runner directories and equivalence with local source; retain run-specific provenance separately. Failed upload/transport or missing artifact cannot be represented as successful evidence delivery. Repository instructions remain inert data. 5. Publish concise declared-input/output and failure instructions. Complete the installed acquisition path and focused CI proof, not a workflow stub or schema. No review success or fitness-function result is inferred from ingestion success. PVF: deterministic local contract/installed integration followed by required CI smoke; role: declared revision, packet parity and partial/credential negatives; resources: bounded runner CPU/disk, isolated fixture repository, no metered inference; gate: required Beta CI input route. Retain exact command and nonzero outcome in CI independently from local proof. Stop on missing input, failed required proof, contract conflict, credential capture, host-bound packet or unexecuted CI gate. Exclude CF-GOV-CI, GitHub API adapter, review/features, broad connectors and schema/scaffold-only closure.

## Demo Expectations

Execute and retain the complete acceptance/proving cases in the source issue; no schema-only or fixture-only substitution.

## Non-goals

PVF: deterministic local contract/installed integration followed by required CI smoke; role: declared revision, packet parity and partial/credential negatives; resources: bounded runner CPU/disk, isolated fixture repository, no metered inference; gate: required Beta CI input route. Retain exact command and nonzero outcome in CI independently from local proof. Stop on missing input, failed required proof, contract conflict, credential capture, host-bound packet or unexecuted CI gate. Exclude CF-GOV-CI, GitHub API adapter, review/features, broad connectors and schema/scaffold-only closure.

## Issue-Graph Notes

Use only numeric prerequisites from source issue; no sprint-wide or closeout dependency.

## Notes

Prepared only. Child implementation and its review have not run.

## Tooling Notes

Native v3 bind/edit/validate/review/publish; stable installed owners; primary main inspection-only.
