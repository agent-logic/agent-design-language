# v0.91.7 WP-19 External Review Handoff (#4646)

Status: external_review_complete

Issue: #4646

Last verified: 2026-07-19

## Decision

The internal-review remediation gate is complete. The operator authorized
WP-19 to proceed on a frozen exact revision. Open #5572 / PR #5574 and #5575
are v0.91.8 follow-ons excluded from this review corpus. They must not mutate
or silently supplement the dispatched snapshot. Closeout audit #5573 remains
open and underway in another session; merged PR #5578 already retains its
427-issue register, and WP-19 does not own or wait on its remaining closeout.

Issue #5571 is a `version:v0.91.7` publication-boundary audit. It remains a
release-tail residual but is not a WP-19 send prerequisite because the bounded
public allowlist excludes the raw WP-18 packet, live-state, and validation
trees that #5571 audits.

## Completed Predecessor And Remediation Gates

| Gate | Current truth |
| --- | --- |
| WP-18 #4645 / PR #5543 | Issue closed; PR merged at `f393671dce71d5e1a1a94d2444f2d5b451b81581`. |
| Runtime hardening #5408 / PR #5419 | Issue closed; PR merged at `6fcd3accafc15e3b6cc8064d836293b4495983de`; required CI and hosted coverage passed. |
| Terminal SOR repair #5527 | Closed with retained terminal reconciliation evidence. |
| Release-truth repair #5544 | Closed. |
| Provider and Runtime v3 hardening #5545 / PR #5557 | Issue closed; PR merged at `a5ba8c6bc486f249a29ecdd376ed05a6399aaf60`; required CI and hosted coverage passed. |
| Coverage, supply-chain, and AWS-boundary proof #5546 | Closed. |
| C-SDLC identity and ownership residual disposition #5547 | Closed; behavioral module splits remain explicitly deferred to v0.91.8. |
| WP-21A #5489 | Closed with the v0.91.8 planning and third-party-review handoff package retained. |

WP-20 #4647 remains open intentionally. It owns synthesis and remediation of
any findings returned by this external review; its open state is not a
pre-review blocker.

## Send Gate

Every row must be satisfied from live repository and GitHub truth immediately
before sending:

| Gate | Required state |
| --- | --- |
| Stable corpus | Use only the exact frozen revision named in the dispatch receipt; exclude v0.91.8 #5572 / PR #5574, #5575, and any later source changes. |
| Exact revision | Record repository, branch or PR, and exact target commit SHA in the dispatch receipt. Any later source change stales the review. |
| Packet digest | Compute and record the authoritative corpus digest in the dispatch receipt. |
| Predecessor truth | #4645, #5408, #5489, #5527, and #5544-#5547 remain closed; PRs #5419, #5543, and #5557 remain merged. |
| Validation | Run `git diff --check`, parse the v0.91.7 issue-wave YAML, and validate every manifest path exists. |
| Publication safety | Exclude secrets, credentials, private prompt output, untracked artifacts, and machine-local scratch evidence. #5571 remains a disclosed v0.91.7 residual; no broader publication approval is inferred. |

If a row fails, return `blocked` or `deferred`; do not ask the reviewer to
infer readiness from stale issue state.

## Target Revision

The exact target identity is issued in
`external_review_4646/DISPATCH_RECEIPT.md`. The receipt is committed after,
and excluded from, the immutable target revision and corpus digest. This keeps
review identity non-self-referential.

## Included Scope

Review the exact target revision for:

1. v0.91.7 milestone, feature, issue-wave, WBS, review-register, and handoff truth;
2. the WP-18 internal review packet and all twelve finding dispositions;
3. landed remediation for #5408 and #5544 through #5547;
4. Runtime v3 and provider API hardening from PR #5557;
5. coverage, supply-chain, C-SDLC identity, ownership, and publication-boundary residuals;
6. the retained #4906 `blocked_with_evidence` coherence-gate rows and their
   release impact without treating closed issue state as resolution;
7. the v0.91.8 bridge and its precedence before v0.92 consumption.

## Publication-Safe Evidence Manifest

`external_review_4646/REVIEW_CORPUS.v1.txt` is the sole authoritative path
manifest. Use it unchanged for publication auditing, path-existence validation,
reviewer scope, and digest computation. Do not supplement it with paths inferred
from prose in this handoff.

The internal review's `packet/`, `live-state/`, and `validation/` directories
are explicitly excluded from the external handoff. Their retained manifest is
`local_only`, forbids publication, and includes machine-local build paths. The
allowlist above exposes the synthesized findings and dispositions without
requiring those raw evidence directories. See
`external_review_4646/PUBLICATION_SAFE_MANIFEST.md` for the bounded audit.

## Digest Procedure

Run from the exact target revision. The first command fails if any manifest
entry is absent. The second hashes tracked object identity for exactly the same
manifest entries:

```sh
while IFS= read -r path; do test -e "$path" || exit 1; done \
  < docs/milestones/v0.91.7/review/external_review_4646/REVIEW_CORPUS.v1.txt
xargs git ls-tree -r HEAD -- \
  < docs/milestones/v0.91.7/review/external_review_4646/REVIEW_CORPUS.v1.txt \
  | LC_ALL=C sort > /tmp/v0917-wp19-review-paths.txt
shasum -a 256 /tmp/v0917-wp19-review-paths.txt
```

Normal `git ls-tree` output includes each tracked blob object id, so this digest
changes when either allowlisted content or paths change. Record the immutable
target SHA and resulting digest in `DISPATCH_RECEIPT.md`; never add that receipt
to `REVIEW_CORPUS.v1.txt`. If the target revision changes, issue a new receipt
before relying on review results.

## Reviewer Authority

The external reviewer may inspect the listed repository evidence, run
read-only validation, and return severity-ranked findings with file and line
evidence. The reviewer must not edit files, mutate GitHub state, deploy,
release, use AWS, or infer v0.92 activation readiness.

Return `P0` through `P3` findings with summary, evidence, impact, violated
invariant, recommended bounded remediation, and residual risk. Return
`no_findings` only when no actionable finding remains; retain non-claims and
residual risks either way.

## Finding Return Path

WP-19 retains the external finding register. WP-20 #4647 deduplicates and
groups accepted findings before implementation. Do not create one issue per
finding automatically.

## Non-Claims And Residuals

- External review completed against the exact revision in the dispatch receipt;
  its two procedural findings were fixed before closeout.
- This handoff does not approve v0.91.7 release readiness or v0.92 activation.
- #5571 is an open v0.91.7 publication-boundary audit; no secret exposure has
  been demonstrated, and no publication-safety approval is inferred here.
- #5572 / PR #5574 and #5575 are open v0.91.8 follow-ons excluded from the
  frozen review corpus; no result from them is claimed or required here.
- #5573 remains open and underway in another session. Its merged 427-issue
  register remains retained; WP-19 does not rerun, expand, or close that work.
- #4906 is closed as an issue but retains unresolved `blocked_with_evidence`
  rows; external review must preserve that release-readiness boundary.
- The ownership-first module splits recorded by #5547 are v0.91.8 work, not
  completed v0.91.7 refactoring.
- No AWS or paid remote validation is required for this review.
