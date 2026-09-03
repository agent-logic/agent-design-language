# Structured Output Record

Template: 1.0.0

Issue: 645

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the C-SDLC v2 publication guard so closing-mode publish normalization reads GitHub closingIssuesReferences, carries linked_issue/linkage_source in RemotePullRequest, and rejects body-only stacked closing PRs that lack the live GitHub closing relation.

## Artifacts

- csdlc-v2/src/bin/csdlc-publish.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/tests/gate6.rs
- csdlc-v2/tests/publication_ready.rs
- csdlc-v2/tests/publication_tail.rs
- .csdlc/prepared/issues/645/validate-stacked-closing-relation.sh
- .csdlc/issues/645

## Execution

- Extended RemotePullRequest with optional linked_issue and linkage_source fields for typed publish/readback agreement.
- Added closing-mode relation validation to publication remote identity checks; explicit PartOf checkpoint mode remains relation-free and non-closing.
- Updated csdlc-publish normalization to query GitHub closingIssuesReferences for closing-mode PRs and fail closed with stack/default/checkpoint guidance when absent or mismatched.
- Tightened existing-PR governed-mode matching so a pre-existing stacked PR with only a closing body keyword cannot be accepted.
- Added the #645 regression for PR #644 shape and updated publication fixtures to carry explicit relation truth.

## Validation

[]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
