# Structured Review Prompt

Template: 1.0.0

Issue: 4647

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/4647
.csdlc/issues/5467
.csdlc/issues/5542
.csdlc/locks/4647.lock
.csdlc/prepared/issues/4647
adl/Cargo.lock
adl/Cargo.toml
adl/src/cli/provider_cmd.rs
adl/src/csm_networking.rs
adl/src/csm_runtime_api.rs
adl/src/provider/http_family.rs
adl/src/provider/http_family/config.rs
adl/src/provider/http_family/tests.rs
adl/src/provider/local.rs
adl/tests/cli_smoke/agent.rs
adl/tools/check_coverage_impact.sh
adl/tools/run_authoritative_coverage_lane.sh
adl/tools/test_check_coverage_impact.sh
adl/tools/test_ci_runtime_contracts.sh
adl/tools/test_run_authoritative_coverage_lane.sh
docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml
docs/milestones/v0.91.7/review/ADL_v0.91.7_THIRD_PARTY_REVIEW_HANDOFF.md
docs/milestones/v0.91.7/review/V0917_EXTERNAL_REVIEW_VERIFICATION_2026-07-19.md
docs/milestones/v0.91.7/review/V0917_SPRINT_REVIEW_REGISTER.md
docs/milestones/v0.91.7/review/V0917_WP20_REMEDIATION_PREFLIGHT_4647.md
docs/milestones/v0.91.7/review/external_review_4646/FINDINGS_REGISTER.md
docs/milestones/v0.91.7/review/external_review_4646/PUBLICATION_SAFE_MANIFEST.md
docs/milestones/v0.91.7/review/wp20_remediation_4647
docs/milestones/v0.91.7/review/wp20_remediation_5544/RELEASE_TRUTH_GATE_STATUS_5544.md
docs/milestones/v0.91.8/CANONICAL_DOC_INVENTORY_v0.91.8.md
docs/milestones/v0.91.8/review/THIRD_PARTY_REVIEW_HANDOFF_v0.91.8.md
docs/milestones/v0.91.8/setup/5383/DIAGRAM.mmd

## Prompts

- Does the issue stay within its WP scope?
- Are claims supported by retained or fresh evidence?
- Are skipped and unproven surfaces explicit?
- Are sibling WP and release/activation non-claims preserved?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Exact-head bounded review by subagent 019f7df0-b305-7403-84db-b4a82cf26d1d confirmed HEAD 3f250321dba93a94419b2880baa1905ede9204c5 is CLEAN/fixed-confirmed for the post-publication coverage-impact mapping fix.
- Focused local proof after the fix: bash adl/tools/test_check_coverage_impact.sh passed; PR-style --print-risk-nextest-expression includes provider, cli::provider_cmd::tests, and csm_networking selectors.
- GitHub run 29718251274 was canceled immediately after the early adl-coverage-hosted failure to avoid continuing failed work; no AWS live operation was run.

## Review Result

Revision: Some("git-blake3:3f250321dba93a94419b2880baa1905ede9204c5:2fdde5255aece2074c416d20d473ec64ab47af2c9627461f88eb7e04fd8e0192")

Reviewer: Some("subagent:019f7df0-b305-7403-84db-b4a82cf26d1d")

Result: pass
