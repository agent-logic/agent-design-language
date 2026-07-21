# Structured Planning Prompt

Template: 1.0.0

Issue: 5337

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement the standalone harness and full corpus, capture the pinned v1 behavior at least three times per case, verify narrow normalization and coverage fail closed, run all crate proof, fix exact-revision review findings, and publish only with every acceptance criterion complete.

## Plan

Revision 8

## Steps

[
  {
    "id": "S1",
    "action": "Implement the independent crate, typed manifest/schema model, runner, normalizer, comparator, report model, and CLI",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Author the complete versioned positive and negative fixture corpus and behavior coverage map",
    "acceptance_ids": [
      "AC-2",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Build the pinned v1 binary locally and capture at least three immutable observations for every case",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Verify normalization, equivalence, semantic differences, expected failures, coverage, and deterministic reports",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Run focused and full validation with external Cargo output and retain exact evidence",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S6",
    "action": "Run bounded exact-revision review, fix every actionable finding, and publish through typed lifecycle gates",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "in_progress"
  }
]

## Invariants

- no tracked work on main or sibling worktrees
- no dependency on or edits to incumbent adl or Runtime v2 source
- raw observations remain immutable and normalized observations remain derived
- array order, IDs, errors, exits, prompt content/order, and signature verdicts are semantic
- every case repeats at least three times
- network and credential variables are denied to child processes
- every acceptance criterion has required local proof and none is deferred
- all card changes use typed C-SDLC v2 operations

## Risks

- normalization could hide a semantic regression
- graph output alone may hide sequential ordering, requiring print-plan comparison
- environmental noise may cause false nondeterminism
- fixed signing evidence could accidentally retain private material
- large raw evidence could obscure corpus coverage
- v1 build provenance could drift from the pinned revision

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5337/design.md

Digest: 01e9dc48ac9023f6024077531c496eb99c9bde7bd68d582e6568ce51756b47bc

## Diagram

.csdlc/prepared/issues/5337/diagram.mmd

Digest: 808f6c6ff3a0b68845330c2642882fd660e97c17680a5f61ca146655abe54b57

## Stop Conditions

- the v1 binary cannot be proven to come from the pinned revision
- any case requires credentials, network, remote, or AWS execution
- a normalizer would need to erase semantic arrays, identifiers, errors, exits, or signature verdicts
- any required behavior lacks three retained observations or coverage mapping
- unexplained repeated-run divergence remains
- exact-revision review has unresolved actionable findings
- publication would require bypassing typed lifecycle truth

## Handoff

Proceed only after doctor readiness.
