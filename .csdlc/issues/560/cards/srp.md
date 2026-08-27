# Structured Review Prompt

Template: 1.0.0

Issue: 560

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/560/cards/stp.md
.csdlc/issues/560/cards/spp.md
.csdlc/issues/560/cards/stp.values.json
.csdlc/issues/560/cards/spp.values.json
adl/.config/nextest.toml
adl/src/adl_gws_context_mirror.rs
adl/src/bin/demo_adl_gws_context_mirror.rs
docs/planning/ADL_FEATURE_LIST.md
.csdlc/prepared/issues/560/validate-focused-proof.sh
.csdlc/evidence/560/focused-runtime-v2-unified-kernel-coverage.log

## Prompts

- Verify the change is an exact ci-coverage timeout/profile adjustment for only the three observed runtime_v2 tests.
- Verify Runtime v2 semantics and assertions are unchanged.
- Verify hosted coverage remains the final shared-gate proof.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted workspace coverage remains the final pre-merge integration gate.

## Review Result

Revision: Some("git-blake3:4a6e49e0e0119254a117197e44e5ef930eb08ee0:92891ec76abfc8fd55ec7576ea822de9166ab1005bc3d601bfc52a59563efd75")

Reviewer: Some("fresh-session:review-319-exact-r3")

Result: pass
