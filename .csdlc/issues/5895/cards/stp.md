# Structured Task Prompt

Template: 1.0.0

Issue: 5895

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Verify and remove only active csdlc-migrate installer authority, then prove the installed claim-free route.

## Deliverables

- Authoritative occurrence classification
- Corrected active binary inventory if needed
- Installed-generation provenance evidence
- Representative installed claim-free lifecycle canary evidence
- csdlc-v2/tests/gate10a.rs

## Acceptance

1. AC-1: No authoritative installer, coexistence, selector, or current operator surface requires csdlc-migrate
2. AC-2: Clean current-main build and stable v2 installation succeed
3. AC-3: csdlc-install resolve selects v2 and installed provenance matches the exact source revision
4. AC-4: A focused negative guard rejects csdlc-migrate reappearance
5. AC-5: An installed claim-free create/validate/bind canary passes
6. AC-6: Historical evidence is unchanged and no compatibility route is added

## Dependencies

- #5861 claim-free creation and binding
- #5896 bound-topology migration
- Execute before #5883 to avoid shared installer/coexistence conflicts

## Inputs

- danielbaustin/agent-design-language#5895
- .csdlc/evidence/5891/active-issue-dispositions.tsv
- csdlc-v2/operator/coexistence.json
- csdlc-v2/operator/generation-selector.json
- adl/tools/install_owner_binaries.sh

## Non Goals

- Reintroduce csdlc-migrate
- Redesign issue creation or binding
- Change card semantics
- Broad tooling refactor
- Edit #5844 or historical Gate 10 evidence
