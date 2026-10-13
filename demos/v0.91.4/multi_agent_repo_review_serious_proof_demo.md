# Multi-Agent Repo Review Serious Proof Demo

> Historical scenario reference. This document records the named milestone
> and its source-era commands, artifact paths, and implementation status.
> Those commands are not the execution contract for the decomposed repositories.
> Retaining this guide does not qualify the current CodeFriend product or authorize
> running providers, publishing customer reports, or retiring the original tests.
> See the [preserved source revision](https://github.com/agent-logic/agent-design-language/blob/485c11a10cb27cde1343bd5a0d165c550b739fe0/demos/v0.91.4/multi_agent_repo_review_serious_proof_demo.md)
> for the complete historical recipe. Current owner delivery and consumer proof
> are tracked separately by [ADL #1223](https://github.com/agent-logic/agent-design-language/issues/1223).

This `v0.91.4` demo is the reviewer-first companion to the older `v0.89`
multi-agent repo-review sample.

It is meant to feel like a serious bounded review packet rather than just a
coordination sketch.

## Command

The serious-review fixture producer, original test and validator now belong to CodeFriend at source revision `8236ee8e8ae7e31efdd391904d3f8d75b30693fb`. Set `CODEFRIEND_SERIOUS_PROOF_SOURCE_ROOT` to an absolute checkout of that revision. This is a generated offline review-packet fixture; it does not execute the represented reviewer skills or qualify live review, providers, publication or installed binaries. This ADL guide remains historical navigation, not a delivered CodeFriend guide.

```bash
(cd "${CODEFRIEND_SERIOUS_PROOF_SOURCE_ROOT:?set an absolute CodeFriend checkout}" && bash adl/tools/demo_v0914_multi_agent_repo_review_serious_proof.sh)
```

## What It Builds

The demo creates:

- one bounded review packet scope
- one visible heuristic contract mapped to specialist roles
- four specialist review artifacts
- one findings-first synthesis review
- one quality-gate pair for review quality and publication blocking
- one artifact README that gives the reviewer reading order

## Artifact Root

Default artifact root:

```text
artifacts/v0914/multi_agent_repo_review_serious_proof/
```

Primary proof surfaces:

- `run_manifest.json`
- `review_packet/repo_scope.md`
- `review_packet/heuristic_contract.json`
- `review_packet/evidence_index.json`
- `review_packet/specialist_assignments.json`
- `specialist_reviews/code.md`
- `specialist_reviews/security.md`
- `specialist_reviews/tests.md`
- `specialist_reviews/docs.md`
- `synthesis/final_findings_first_review.md`
- `quality_gate/review_quality_evaluation.md`
- `quality_gate/redaction_and_publication_gate.md`

## Important Boundary

This is a bounded proving fixture.

It does **not** claim:

- live provider execution quality
- merge approval authority
- customer-ready publication
- autonomous remediation

The point is to show a serious review packet shape with visible heuristics,
explicit non-findings, role-specific caveats, and findings-first synthesis.
