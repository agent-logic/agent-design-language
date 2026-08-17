# Structured Task Prompt

Template: 1.0.0

Issue: 283

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Create and validate issue-local reconciliation evidence for ADR 0065 from existing terminal artifacts only.

## Deliverables

- Issue-local ADR 0065 reconciliation packet under .csdlc/evidence/283
- Artifact/hash inventory for #209 replacement proof and #5832 historical proof
- Promotion-readiness classification for #207/#288 handoff
- Validated C-SDLC card truth for #283

## Acceptance

1. AC-1: live #209 / PR #215 state is recorded with exact head, merge commit, and closure linkage or classified as missing
2. AC-2: derived terminal cache for #209 is verified and digest-bound or classified as stale/incomplete
3. AC-3: #209 local and native validation manifests are verified as non-empty, exact-revision-bound, and artifact-bound
4. AC-4: #5832 evidence is inventoried and explicitly classified as historical/superseded rather than sole terminal promotion proof
5. AC-5: #283 records a clear ADR 0065 readiness result and residual gaps for #207/#288 without editing shared ADR docs or accepting the ADR

## Dependencies

- #207 parent ADR proof-gate coordination
- #209 / PR #215 terminal ACIP repair evidence
- #5832 historical ACIP native receipts
- #288 final serialization after #283-#287

## Inputs

- agent-logic/agent-design-language#207
- agent-logic/agent-design-language#283
- agent-logic/agent-design-language#209
- agent-logic/agent-design-language#215
- .git/csdlc-v2/derived-terminal/209.json
- .csdlc/evidence/209/local-validation-manifest.json
- .csdlc/evidence/209/native-validation-manifest.json
- .csdlc/evidence/5832/acip-native-receipts.json
- docs/milestones/v0.92/ADR_PLAN_v0.92.md
- docs/architecture/adr/V092_ADR_INDEX_143.md

## Non Goals

- ACIP implementation or proof repair
- Editing product code, shared ADR docs, ADR index, plan, manifest, or #288 review packet
- Moving ADR 0065 to Accepted
- Replacing existing owner acceptance criteria
- Creating a new proof in place of existing terminal owner evidence
