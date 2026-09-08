# Issue 525 design — independent successor review

## Result

One findings-first review report bound to the exact merged planning revision
produced through #523 and #524. This issue is review-only and does not remediate
its own findings.

## Review denominator

The reviewer records the immutable revision, clean checkout state, every
canonical file under `docs/milestones/v0.92.2/**`, and relevant feature-routing
surfaces. Semantic review covers completeness, dependency executability,
single-result issue units, deferrals, closeout order, operator gates, and
unsupported claims. Validator output is supporting evidence, not approval.

## Execution shape

1. Freeze exact revision and path denominator after #524 merges.
2. Run independent semantic and focused mechanical review.
3. Record every finding with severity, evidence, owner, and disposition.
4. Publish one immutable review report; any planning change requires re-review.
5. If actionable findings exist, route them back to #523 or #524, merge the
   bounded repair, and repeat #525 against the new exact revision. #526 cannot
   start until a #525 report records zero unresolved release-blocking findings.

## Guardrails

- Reviewer does not edit reviewed planning.
- No v0.92.2 issue creation or current release approval.
- Stop if the revision changes or #524 is not merged and reviewed.
- A changes-required report is a valid #525 result, but it is not release
  authority and must route through repair and fresh review before #526.
