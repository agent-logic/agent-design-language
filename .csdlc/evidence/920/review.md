# v0.92.2 external-review handoff

Status: **preparation only; external review has not started**.

This packet prepares the TAIL-05 review surface. It does not authorize external contact, disclose private evidence, establish reviewer independence, accept #919, or approve release. Before sending this request, refresh `review-manifest.json` from the accepted #917, #918, and #919 outputs and run the focused validator. The validator must reject this current preparation state as a completed review.

## Required activation gates

All of the following must be true before review begins:

1. #919 has an accepted internal-review output bound to an exact candidate and findings digest.
2. #917 and #918 provide accepted handoff, candidate, and artifact-manifest identities.
3. The candidate revision and complete artifact-manifest SHA-256 agree across the handoff, internal review, and this manifest.
4. The reviewer is independent of implementation and internal-review authorship, with a retained basis for that conclusion.
5. External contact and the exact disclosure scope have explicit authorization.
6. Every required source is accessible to the reviewer without exposing private or machine-local material outside the approved scope.

If any gate is absent or changes, stop and refresh the packet. A sent request is not a completed review.

## Review scope

Review the exact candidate and artifact manifest recorded in `review-manifest.json`. Assess whether the v0.92.2 Beta 1 claims are supported by the provided source and retained evidence, with particular attention to:

- candidate and manifest identity consistency;
- quality-gate and qualification claims;
- CodeFriend Beta 1 product behavior and supported non-claims;
- security, privacy, provenance, and credential boundaries;
- documentation, diagrams, tests, deployment claims, and release-truth consistency;
- internal-review findings, their evidence, and any unaddressed gaps;
- release blockers defined in the milestone quality gate and release plan.

Do not broaden this into a future-roadmap redesign. Do not infer successful Runtime, provider, cloud, deployment, or publication behavior from plans, deterministic validators, or green CI alone.

## Required source set

The activated packet must provide accessible exact-revision links or retained artifacts for:

- the accepted #917 documentation handoff and its manifest;
- the accepted #918 publication artifact manifest;
- the accepted #919 internal review and findings register;
- `docs/milestones/v0.92.2/RELEASE_PLAN_v0.92.2.md`;
- `docs/milestones/v0.92.2/QUALITY_GATE_v0.92.2.md`;
- `docs/milestones/v0.92.2/MILESTONE_CHECKLIST_v0.92.2.md`;
- the exact candidate diff and all evidence referenced by material claims.

## Requested response

Return one context-free report containing:

1. reviewer identity and independence basis;
2. exact reviewed candidate revision and artifact-manifest SHA-256;
3. methods, sources accessed, checks or reproductions actually performed, and resource/provider use;
4. actionable P0-P3 findings first, each with stable source evidence and impact;
5. verified non-findings;
6. limitations, inaccessible evidence, omitted scope, and unresolved uncertainty;
7. one verdict: `pass`, `fail`, or `not_proven`.

An adverse or `not_proven` review is a valid retained assessment. It does not disappear and must not be rewritten after receipt. Route every accepted finding to #921 with its original severity and evidence.

## Findings intake rules

Copy the received assessment into immutable issue-local evidence, then populate `findings.json` without changing the reviewer’s meaning. The reviewed revision and manifest digest in the findings file must equal the manifest. Preserve negative results and limitations. Any substantive candidate or claim change invalidates the current assessment and requires a fresh internal and external review under the release plan.

## Current unresolved inputs

- #917 is observed only at draft revision `c10757270098aea35e36453c91054ea8b4f947db`; accepted identity remains pending.
- #918 has no accepted candidate or artifact-manifest identity in this packet.
- #919 has no accepted internal-review revision or findings digest in this packet.
- No external reviewer or contact/disclosure authorization is recorded.

The current truthful verdict is `not_proven`.
