# Structured Review Prompt

Template: 1.0.0

Issue: 560

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/560/cards/stp.md
.csdlc/issues/560/cards/spp.md
.csdlc/issues/560/cards/srp.md
.csdlc/issues/560/cards/stp.values.json
.csdlc/issues/560/cards/spp.values.json
.csdlc/issues/560/cards/srp.values.json
adl/.config/nextest.toml
adl/src/adl_gws_context_mirror.rs
adl/src/bin/demo_adl_gws_context_mirror.rs
docs/planning/ADL_FEATURE_LIST.md
.csdlc/prepared/issues/560/validate-focused-proof.sh
.csdlc/evidence/560/focused-runtime-v2-unified-kernel-coverage.log

## Prompts

- Verify the exact seven-test runtime_v2 unified-kernel ci-coverage override remains module-bounded and preserves semantic assertions.
- Verify the context-mirror fixture supplies canonical inputs and the previously failing binary test passes.
- Verify milestone detection accepts exactly one explicit active-status marker and rejects future, completed, or conflicting markers.
- Verify focused evidence, operative STP/SPP truth, diff hygiene, and hosted-green gating are current before publication.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted workspace coverage remains the final pre-merge integration gate.

## Review Result

Revision: Some("git-blake3:0508f56547e088972199b4924040da4214bee458:68da112de14b5c7bffbc79a1d10b04d67e5b87cf33ac11b86dcf7f352eee1596")

Reviewer: Some("fresh-session:review-319-exact-final")

Result: pass
