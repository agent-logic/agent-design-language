# Issue 312 — Documentation review and external-review readiness design

## Outcome boundary

#312 is the v0.92 documentation review pass. It reconciles canonical current
documentation to merged evidence, produces the exact external-review corpus and
third-party handoff, and stops before external review, release approval,
publication, deployment, or product repair.

The predecessor documentation input is the merged WP-22 authority. #311's
blocked result remains historical provenance; corrective #467 is the current
quality-gate authority and its passing result is incorporated into the candidate.
Terminal reconciliation, closeout receipts, and worktree cleanup are
asynchronous bookkeeping and never gate #312.

## Canonical denominator

The standard document set is not maintained by an ad hoc list. It is the exact
case-insensitive, sorted, unique union of:

1. every tracked file whose basename is `README.md`, including root,
   `docs/README.md`, and `adl/README.md`;
2. `CHANGELOG.md`, `AGENTS.md`, `REVIEW.md`,
   `docs/planning/ADL_FEATURE_LIST.md`, and `csdlc-v2/AGENTS.md`;
3. every tracked regular file under `docs/milestones/v0.92/`, including every
   feature document, the canonical inventory, review index, and third-party
   review handoff; and
4. every tracked `SKILL.md` under `csdlc-v2/operator/skills/`, plus the current
   ADR navigation indexes at `docs/adr/README.md`,
   `docs/architecture/adr/README.md`, and
   `docs/architecture/adr/V092_ADR_INDEX_143.md`.

The issue-owned validator regenerates this set from Git. The external-review
inventory has exactly one row for every current release-truth document; the
independent `.csdlc/evidence/312/readme-paths.txt` manifest covers the complete
repository-wide README denominator without duplicating historical README rows
into the review-input inventory. The validator compares both derived sets with
Git and rejects missing, duplicate,
extra, stale, ambiguous, or machine-local entries. JSON and YAML syntax plus
Markdown local links are validated for current v0.92 and current navigation
surfaces. Historical README files are denominator members but are not all
content-rewrite targets for #312.

## Candidate reconciliation

The inventory is a bounded documentation-corpus manifest, not a replacement
for the product quality gate or a per-document GitHub producer ledger. Each row
records the review owner, review-input status, evidence source, required action,
and exact file digest. The validator derives the denominator and changed-path
set from Git and rejects missing, duplicate, extra, stale, or out-of-scope
content. Before publication, the candidate is rebased or refreshed when a
merged producer changed an owned document; the inventory is then regenerated
from the resulting exact candidate. Product completion and release claims stay
governed by the canonical #467 packet: 30 accepted rows, 3 explicitly
scoped-out rows, 0 blocked, and downstream unlock true. The superseded #311
packet is retained as historical provenance. Typed administrative closeout
state is ignored.

## External-review packet

`docs/milestones/v0.92/review/THIRD_PARTY_REVIEW_HANDOFF_v0.92.md` is the
reviewer entrypoint. It contains:

- repository, PR, base, exact head, and reproducible corpus digest;
- send gates and a stale-revision rule;
- the canonical source/evidence manifest and navigation map;
- review scope, exclusions, reviewer authority, and findings format;
- exact read-only validation commands;
- blocker, residual-risk, and non-claim register;
- redaction and portable-path requirements; and
- a feedback route into the later review/remediation issues without granting
  release authority.

`docs/milestones/v0.92/CANONICAL_DOC_INVENTORY_v0.92.md` and
`docs/milestones/v0.92/review/README.md` make the handoff reachable from the
standard milestone entrypoints. The handoff is ready to send only when its
exact revision and corpus digest are populated and all send gates pass.

## Validation architecture

Four bounded lanes run independently: documentation/release truth, adversarial
negative cases, structure/link/handoff validation, and `git diff --check`.
The structure lane parses the full denominator, verifies the external-review
manifest, rejects every tracked `.adl` path or dependency, and compares the
candidate diff to the exact declared path set. The diff lane claims only patch
hygiene. Fixture, stale, synthetic, self-attested, cross-repository,
machine-local, out-of-scope, or `.adl`-dependent evidence cannot grant a claim.

## Non-claims and stop conditions

The packet does not convert #467's passing evidence gate into release approval
or claim that an external reviewer approved v0.92. It explicitly preserves
provider, platform, birthday, privacy, governance, legal, personhood,
consciousness, and future-version non-claims. Missing product evidence remains
a blocker or non-claim and is routed to its product owner. #312 stops if the
merged #467 result is absent or misstated, the denominator is incomplete,
candidate overlap is unresolved, exact scope or no-`.adl` guards fail, or
documentation truth would require product implementation.
