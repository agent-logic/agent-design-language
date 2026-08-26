# Structured Planning Prompt

Template: 1.0.0

Issue: 313

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Verify terminal entry gates, freeze one clean exact SHA, build the review packet, run nine independent specialist lanes, synthesize findings without remediation, validate identity/digests/redaction, and obtain independent meta-review before handoff.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Reconcile canonical dependency, issue, PR, lifecycle, ancestry, worktree, and candidate-SHA truth.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Build the exact-SHA packet manifest and explicit included, excluded, unknown, private, generated, and redacted scope inventory.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run all nine independent specialist lanes against the identical packet and exact SHA.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Create the provenance-preserving findings register and findings-first synthesis without remediation.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run packet schema, identity, digest, link, redaction, path-hygiene, and negative validation.",
    "acceptance_ids": [
      "AC-7",
      "AC-9"
    ],
    "status": "pending"
  },
  {
    "id": "S6",
    "action": "Obtain independent meta-review, correct actionable packet-quality findings, and retain the bounded WP-26 handoff disposition.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- Every lane reviews the same repository, manifest, and exact SHA
- A source or manifest change makes all dependent review evidence stale
- Missing evidence, lane identity, or dependency truth blocks review
- Synthesis cannot erase finding provenance, duplicates, or disagreement
- WP-25 never converts review evidence into remediation or release authority

## Risks

- Typed initialization could regress the already-correct #313 WP-24A deferral unless live issue truth is preserved
- Large scope can produce superficial lanes unless evidence denominators are explicit
- Self-referential packet commits can stale exact-head claims
- Historical evidence may contain host-local paths or private data requiring careful boundary treatment

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/313/design.md

Digest: 02b1d0666e26fc4ab6dc8fe91007659b2acc4a0116dff20ac8c5f78ac3fe845a

## Diagram

.csdlc/prepared/issues/313/diagram.mmd

Digest: 94c6240fafa592052a8d45e548757a7452fe6b966faea144948828c7134e5b7c

## Stop Conditions

- Any active v0.92 dependency is nonterminal, noncanonical, nonancestral, or has active worktree topology at execution time
- The candidate checkout is dirty or the target SHA changes after packet generation
- Any specialist lane is missing, incomplete, stale, or lacks reviewer identity
- Manifest digest, evidence link, finding schema, redaction, or private-path validation fails
- Independent meta-review reports an unresolved actionable quality gap

## Handoff

Proceed only after doctor readiness.
