# CodeFriend persistent AWS environments

Reviewed planning proposal for three persistent environments, two AZs each, application/worker ASGs and DynamoDB application storage. Compute and account count remain open. No deployment approval.

- [Plan](PLAN.md)
- [Architecture PNG](AWS_ARCHITECTURE.png) / [SVG](AWS_ARCHITECTURE.svg)
- [Cost model](COST_MODEL.json) and [official icon provenance](ICON_PROVENANCE.json)
- [Claude final review](reviews/claude-review.md) and [Gemini final review](reviews/gemini-review.md)
- [Finding dispositions](reviews/FINDING_DISPOSITIONS.json) and [review hashes/model/usage](reviews/REVIEW_MANIFEST.json)

Both final vendor reviews returned COMPLETE REVIEW. Claude's nonblocking chronology note is resolved here: the plan retains the original review instructions as history; later completion text and these actual review artifacts record execution. Exact reviewed plan bytes are preserved.

Independent review found stale database-choice and broker-deadline wording; both were corrected before final vendor reviews. Diagram rendering was visually checked; cost arithmetic reproduced. No cloud, IAM, CI, database migration or deployment effects occurred.

This PR covers only the AWS portion of #1422. GCP planning and both GCP provider reviews remain outstanding, so this PR must not close #1422. Native preparation/recovery/binding refused with RecoveryRequired; the operator authorized gh publication. This is a reviewable Git artifact, not a successful native lifecycle admission/finish claim.
