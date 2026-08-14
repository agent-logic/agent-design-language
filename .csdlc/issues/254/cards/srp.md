# Structured Review Prompt

Template: 1.0.0

Issue: 254

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/254/audit.jsonl
.csdlc/issues/254/cards/sip.md
.csdlc/issues/254/cards/sip.values.json
.csdlc/issues/254/cards/sor.md
.csdlc/issues/254/cards/sor.values.json
.csdlc/issues/254/cards/spp.md
.csdlc/issues/254/cards/spp.values.json
.csdlc/issues/254/cards/srp.md
.csdlc/issues/254/cards/srp.values.json
.csdlc/issues/254/cards/stp.md
.csdlc/issues/254/cards/stp.values.json
.csdlc/issues/254/cards/vpp.md
.csdlc/issues/254/cards/vpp.values.json
.csdlc/issues/254/index.json
.csdlc/locks/254.lock
.csdlc/prepared/issues/254/design.md
.csdlc/prepared/issues/254/diagram.mmd
.github/workflows/ci.yaml
adl/tools/test_ci_path_policy.sh
adl/tools/test_ci_runtime_contracts.sh
adl/tools/validate_ci_workflow_policy.rb

## Prompts

- Verify the aggregate job cannot recompile workspace coverage.
- Verify required adl-coverage still fails closed on missing producer evidence.
- Verify runner allocation reserves Azure heavy capacity only for Rust producers.
- Verify local contract tests cover the new topology.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Local exact-head review and focused contract validation only; GitHub CI remains the remote integration proof after publication.

## Review Result

Revision: Some("git-blake3:402d605a546a93135dd6a9584b6d1ff801240cce:c250b68ebb8abbda80bd63e09cb9d31b8efdde984eab08845903b303c725c92b")

Reviewer: Some("codex:/root/fix_issue_254_ci_topology/review_issue_254_final_current")

Result: pass
