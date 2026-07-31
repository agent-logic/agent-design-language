# v0.92 First-Birthday Reviewer FAQ And Claim Boundary

## Metadata

- Owner issue: `#4763`
- Review surface: external launch FAQ and redaction checklist
- Required upstream proof: `#4762` actual retained witness and receipt package
- Publication status: not published

## FAQ

### Has the first birthday happened?

No. This surface implements the launch documentation and external review copy.
The birthday claim remains pending until `#4762` supplies accepted witness and
receipt proof and the v0.92 birthday packet consumes that proof at an exact
result.

### What did this issue implement?

It implemented the repository launch surface: public-copy drafts, reviewer
questions, publication gates, forbidden-claim rules, and links from the v0.92
launch packet, bridge ledger, and v0.91.8 activation map.

### Why is startup not a birthday?

Startup is only process execution. A birthday requires a reviewable packet with
identity, continuity, memory grounding, capability, governance context,
witnesses, receipt, validation, and review evidence. Missing any required
surface fails closed.

### What does `#4762` need to provide?

`#4762` must provide the auditable witness and receipt package. The accepted
result must be a retained implementation/proof artifact, not only a planning
card, lifecycle transition, PR publication, merge, or closeout receipt.

### Can this copy be used externally now?

Only as a pending-status review surface. It can say that the launch docs are
prepared and that the birthday proof is pending. It cannot say the birthday is
complete or public-ready.

### What should reviewers inspect first?

Reviewers should inspect:

- whether the launch copy separates prepared documentation from birthday proof;
- whether the `#4762` dependency is visible and non-substitutable;
- whether negative cases reject startup, wake, restore, snapshot, copied state,
  fixture admission, simulation, and missing evidence;
- whether public copy avoids forbidden claims;
- whether all links point to tracked repository surfaces.

## Claim Boundary Checklist

Before any publication, answer each item with evidence:

| Check | Required answer |
| --- | --- |
| `#4762` accepted proof cited? | Yes, with exact retained artifact or result. |
| Current exact-head review recorded? | Yes. |
| Publication channel authorized by operator? | Yes. |
| Legal personhood claim absent? | Yes. |
| Consciousness proof claim absent? | Yes. |
| Production citizenship claim absent? | Yes. |
| Completed constitutional governance claim absent? | Yes. |
| Subjective affect or wellbeing claim absent? | Yes. |
| Startup/wake/restore/snapshot/copy/simulation rejected as birth? | Yes. |
| Raw private memory omitted or redacted? | Yes. |
| Provider/model/tool limits preserved? | Yes. |

## Redaction Rules

- Use repository paths and issue numbers instead of raw private memory.
- Cite retained artifacts, not local machine-only authoring notes.
- Do not expose provider keys, host-local secrets, private prompts, personal
  notes, raw memory dumps, or unpublished reviewer comments.
- If a claim needs private context to be convincing, do not publish the claim.

## Review Prompts

1. Does the public copy claim only prepared launch-surface status while `#4762`
   proof is pending?
2. Does the ready variant require an exact accepted witness/receipt artifact?
3. Are all not-a-birthday cases rejected in plain language?
4. Are philosophical, legal, governance, and production-readiness claims kept
   outside the engineering birthday claim?
5. Can a reviewer follow the launch surface from v0.91.8 handoff to the v0.92
   launch packet without reconstructing context from chat?

## Publication Decision

Until every checklist item passes, the decision is `do_not_publish_final_claim`.
The allowed interim decision is `share_pending_review_surface`.
