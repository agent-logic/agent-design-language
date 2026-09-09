# Issue #520: exact-candidate internal review

## Goal

Produce one findings-first internal review register for the final v0.92.1
candidate, with a deterministic denominator that cannot silently omit changed
code, tests, documentation, issue/PR truth, or release evidence.

## Design

#718/PR #809 and #758/PR #805 are the review gates. After both are merged and
their issues are closed by those PRs, the review fetches `origin/main` and
freezes that exact revision as the immutable candidate. Both gate merge commits
must be ancestors of the candidate. The review records the v0.92.1 base and
candidate as full SHAs, derives the complete changed-file inventory from that
range, and reconciles it with the fully paginated live v0.92.1 issue roster,
each issue's closing-PR references, and the milestone-assigned PR roster.

Every changed production file and test receives code/test review. Canonical
documentation, lifecycle evidence, demos, providers, cloud boundaries,
security, architecture, dependency manifests, and release claims are routed to
explicit mandatory specialist lanes. Sampling may prioritize reading order but
may not reduce the denominator. Historical #520 reports and the former #519
candidate are context only and cannot serve as current-candidate proof.

The canonical register retains every raw finding or an explicit, evidenced
no-finding result for each assigned surface. Green CI and successful commands
are evidence inputs, never substitutes for semantic review.

## Boundary

#520 reviews and reports. It does not repair product findings, conduct the
independent external review, approve release, merge, deploy, or spend money.
