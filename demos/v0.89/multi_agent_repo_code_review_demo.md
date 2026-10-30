# Multi-Agent Repo Code Review Demo

> Historical scenario reference. This document records the named milestone
> and its source-era commands, artifact paths, and implementation status.
> Those commands are not the execution contract for the decomposed repositories.
> Retaining this guide does not qualify the current CodeFriend product or authorize
> running providers or publishing customer reports. The original ADL source test
> and validator are preserved in Git and #1223 custody; their retirement is not
> a claim that a new owner test passed.
> See the [preserved source revision](https://github.com/agent-logic/agent-design-language/blob/485c11a10cb27cde1343bd5a0d165c550b739fe0/demos/v0.89/multi_agent_repo_code_review_demo.md)
> for the complete historical recipe. Current owner delivery and consumer proof
> are tracked separately by [ADL #1223](https://github.com/agent-logic/agent-design-language/issues/1223).

This `v0.89` demo shows ADL hosting a calm, specialist full-repo review over
one bounded packet instead of pretending one reviewer can cover every concern
perfectly at once.

The review roles are:

- code reviewer
- security reviewer
- test reviewer
- docs reviewer
- synthesis reviewer

## Historical source command (retired by #1223)

```bash
bash adl/tools/demo_v089_multi_agent_repo_code_review.sh
```

## What It Builds

The demo creates:

- one bounded review packet
- four specialist reviewer artifacts
- one cross-review note surface
- one final synthesized review
- one demo manifest and reviewer README

## Artifact Root

Default artifact root:

```text
artifacts/v089/multi_agent_repo_code_review/
```

Primary proof surfaces:

- `review_packet/review_packet_manifest.json`
- `reviewers/code_review.md`
- `reviewers/security_review.md`
- `reviewers/test_review.md`
- `reviewers/docs_review.md`
- `reviewers/cross_review_notes.md`
- `synthesis/final_synthesis_review.md`
- `demo_manifest.json`

## See Also

The serious-review fixture producer, original test and validator now belong to CodeFriend at source revision `8236ee8e8ae7e31efdd391904d3f8d75b30693fb`. Set `CODEFRIEND_SERIOUS_PROOF_SOURCE_ROOT` to an absolute checkout of that revision. This is a generated offline review-packet fixture; it does not execute the represented reviewer skills or qualify live review, providers, publication or installed binaries. This ADL guide remains historical navigation, not a delivered CodeFriend guide.

If you want the stronger reviewer-first proof companion with visible heuristics,
explicit non-findings, role-specific caveats, and stricter publication
boundaries, use:

```bash
(cd "${CODEFRIEND_SERIOUS_PROOF_SOURCE_ROOT:?set an absolute CodeFriend checkout}" && bash adl/tools/demo_v0914_multi_agent_repo_review_serious_proof.sh)
```

See:

- `demos/v0.91.4/multi_agent_repo_review_serious_proof_demo.md`

## Important Boundary

This is a bounded review demo.

It does **not** claim:

- autonomous merge approval
- repository-wide autofix
- unbounded browsing outside the selected packet

The point is to show specialization, explicit review roles, and a final
findings-first synthesis artifact that a human reviewer can read quickly.
