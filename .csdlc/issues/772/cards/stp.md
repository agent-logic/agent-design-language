---
issue_card_schema: adl.issue.v1
wp: "TAIL-06.16"
slug: "gcp-b-audit-log-posture"
title: "[v0.92.1][TAIL-06.16][security] Prove GCP-B audit and log posture"
labels:
  - "track:roadmap"
issue_number: 772
generated_at: "2026-09-09T00:00:00-07:00"
card_status: "ready"
status: "draft"
action: "edit"
supersedes: []
duplicates: []
depends_on: []
milestone_sprint: "1.0.4"
required_outcome_type:
  - "security proof remediation with sanitized cloud readback evidence"
repo_inputs:
  - "https://github.com/agent-logic/agent-design-language/issues/772"
canonical_files: []
demo_required: true
demo_names: []
issue_graph_notes:
  - "#772 is R520-016 under #522 and depends on #769/R520-013 for retained publication redaction safety."
pr_start:
  enabled: true
  slug: "gcp-b-audit-log-posture"
---

Canonical Template Source: `docs/templates/prompts/1.0.4/stp.md`
Generated: 2026-09-09T00:00:00-07:00

# Structured Task Prompt

## Summary

Remediate the remaining GCP-B-ac-1 proof gap by proving audit configuration plus representative log delivery/readback for the accepted GCP-B environment.

## Goal

Produce candidate-bound, redacted GCP-B audit/log posture evidence that closes only the T520-TEST-004 / GCP-B-ac-1 remainder.

## Required Outcome

The exact GCP-B-ac-1 audit/log assertions are explicit, authorized read-only GCP evidence proves the intended project and provider identity, representative audit/log delivery is readable, and retained evidence is sanitized and candidate-bound.

## Deliverables

- Issue-owned read-only proof runner for GCP-B audit/log posture.
- Static and negative validator for candidate, project, provider, log-readback, and redaction invariants.
- Sanitized evidence packet for audit configuration and representative log delivery/readback.
- Narrow GCP-B proof documentation and exact-current mapping for GCP-B-ac-1.
- Lifecycle cards that preserve #769 as the redaction gate until satisfied.

## Acceptance Criteria

- Audit services, sinks/settings, log classes, provider identity, project/environment, and readback assertions are explicitly named.
- Authorized evidence proves the intended configuration exists in the intended GCP environment.
- Representative audit/log readback is present and readable through the approved path.
- Retained proof binds provider identity, environment identity, timestamp, candidate SHA, command/assertion class, and result without credentials or local key paths.
- Validator rejects stale candidate SHA, wrong project/provider identity, missing log readback, and unsafe retained content.
- The six proved and four amended #740 rows keep their existing closed dispositions.

## Repo Inputs

- #772 live issue body.
- #769 live issue body for R520-013 redaction gating.
- #520 source review and #522 parent remediation context.
- `docs/milestones/v0.92.1/evidence/cloud/gcp-b/bootstrap-identity-readiness.md`.
- `.csdlc/evidence/740/*.redacted.json` and `.csdlc/prepared/issues/740/run-gcp-b1-corrective-proof.sh`.
- `docs/milestones/v0.92.1/evidence/release/tail-01/reconciliation/current-exceptions.json`.

## Dependencies

- Parent remediation #522.
- Internal review source #520.
- #769/R520-013 must be complete, or #772 must carry equivalent reviewed issue-local redaction proof, before retained/publication-ready proof is claimed.
- Requires authorized GCP read access through the approved GCP-B credential route without exposing credentials.

## Target Files / Surfaces

- `.csdlc/prepared/issues/772/run-gcp-b-audit-log-posture.sh`
- `.csdlc/prepared/issues/772/validate-gcp-b-audit-log-posture.sh`
- `.csdlc/evidence/772/gcp-b-audit-log-posture.redacted.json`
- `docs/milestones/v0.92.1/evidence/cloud/gcp-b/audit-log-posture.md`
- narrow exact-current semantic mapping for `GCP-B-ac-1`

## Validation Plan

- Static validator over the #772 runner and sanitized proof schema.
- Authorized read-only GCP readback for accepted project/service-account identity, audit/log services, logging settings/sinks, and representative log entries.
- Negative fixture checks for stale candidate SHA, wrong project, wrong provider identity, missing readback, and unsafe retained text.
- Diff hygiene and redaction audit before review.

## Demo Expectations

Run the issue-owned proof command and show a redacted packet proving configuration and representative audit/log readback; do not retain raw log payloads.

## Non-goals

- No unrelated paid GCP proofs.
- No mutation of cloud resources unless a bounded live proof is separately authorized.
- No credentials, local key paths, tokens, or raw sensitive log payloads in retained evidence.
- No GCP architecture expansion beyond GCP-B-ac-1.
- No release/publication-ready claim while #769 remains unresolved unless equivalent redaction proof is reviewed.

## Issue-Graph Notes

- #772 is remediation candidate R520-016 under #522.
- #769 is R520-013 and gates retention/publication safety for new proof evidence.
- #740 remains the prior private posture/versioning/recovery proof; #772 owns only the missing audit/log posture remainder.

## Notes

The accepted GCP-B identity is recorded in `bootstrap-identity-readiness.md`; proof commands must use approved provider auth without printing or retaining key contents.

## Tooling Notes

Use native C-SDLC v3 in the bound FastWork worktree. Use typed card edits, issue-owned validators, fresh independent review, and fail closed on redaction or provider identity mismatch.
