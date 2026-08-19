# Structured Task Prompt

Template: 1.0.0

Issue: 417

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue #417 typed C-SDLC implemented design-refresh recovery defect only.

## Deliverables

- Recovery-epoch-aware implemented design refresh eligibility.
- Exact sequence regression for recover_review through authored refresh.
- Negative stale/unrelated recovery provenance regression.
- Downstream authority-clear assertions.

## Acceptance

1. AC-1: An implemented record supports recover_review, supported planning and deliverable repairs, recover_design_review, then refresh_authored_design_after_recovery within one recovery epoch.
2. AC-2: The refreshed audit preserves the originating recover_review sequence and generation provenance rather than substituting the latest repair generation.
3. AC-3: Review assignment, review result, publication, and readiness authority remain cleared throughout recovery and after authored refresh.
4. AC-4: Stale, unrelated, absent, or superseded recovery epochs remain rejected fail-closed.
5. AC-5: Existing immediate post-recover_review and iterative implemented authored-refresh paths remain supported.
6. AC-6: Focused Rust regressions exercise the exact recovery ordering and negative provenance cases.

## Dependencies

- Issue #414 is frozen until this tooling issue is terminal, canonical, ancestral, and installed.

## Inputs

- GitHub issue #417
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate5.rs
- Issue #414 typed recovery failure evidence

## Non Goals

- Changing #414 product code or lifecycle state.
- Changing #268 or #269.
- AWS or runtime qualification work.
- Broad redesign of C-SDLC review recovery.
