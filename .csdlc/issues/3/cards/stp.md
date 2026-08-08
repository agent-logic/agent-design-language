# Structured Task Prompt

Template: 1.0.0

Issue: 3

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Close only the remaining production, regression-proof, schema, documentation, and canary-evidence gaps in issue #3.

## Deliverables

- Exact effective fetch and push remote verification
- Exhaustive unambiguous matching-PR reconciliation
- Focused split-authority regression suite
- Updated typed operator and GitHub-client contracts
- Live canonical-PR to preserved-issue canary evidence

## Acceptance

1. Typed publication represents the canonical code/PR repository separately from the preserved issue repository and issue number
2. Git fetch and effective push remotes, observed base/head repositories, pushed branch, and PR match the canonical code repository exactly
3. Split-authority PR bodies require a valid qualified closing keyword for the preserved tracker issue
4. Same-repository publication remains backward compatible and fail-closed
5. Cross-repository publication rejects unqualified linkage, tracker mismatch, fork/base/head drift, ambiguous matches, stale exact-head review, and push-URL substitution
6. Public schemas, focused tests, typed operator documentation, publication evidence, readiness, and finish validation cover both identities
7. Retained live evidence proves canonical Agent Logic PR #5 closed preserved legacy issue #5901 without a legacy code push

## Dependencies

- Merged canonical PR #5 / issue #5901 split-authority baseline
- Current canonical Agent Logic main

## Inputs

- AGENTS.md
- csdlc-v2/src/publication.rs
- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/tests/gate6.rs
- csdlc-v2/tests/gate_finish.rs
- .csdlc/issues/5901/index.json
- .csdlc/evidence/5901/split-authority-validation.json

## Non Goals

- Issue-tracker migration or renumbering
- Historical lifecycle rewrite
- Legacy repository code, refs, settings, or pull-request mutation
- AWS, CI policy, or unrelated workflow changes
