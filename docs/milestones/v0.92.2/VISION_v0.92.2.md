# v0.92.2 Vision — CodeFriend Beta 1

Status: planned.

CodeFriend Beta 1 will make repository review a governed, evidence-bound product rather than a collection of prompts. An operator will be able to ingest a repository, run explainable review perspectives, inspect architecture and change risk, receive synthesized remediation and test guidance, compare a later run with an earlier one, and publish only the artifacts they explicitly approve.

## Intended Outcome

Beta 1 is usable end to end on ADL and on one bounded external open-source repository. Its outputs are portable, stable enough to compare, traceable to redacted evidence, clear about claims and non-claims, and available as Markdown, HTML, and PDF.

## Product Principles

1. Evidence precedes conclusions.
2. Architecture findings explain boundaries, coupling, drift, blast radius, and rationale.
3. Correctness, security, adversarial, and constitutional perspectives remain distinguishable before synthesis.
4. Humans control publication and mutation.
5. A second run is more valuable than an isolated snapshot.
6. Provider and Runtime integrations remain shared infrastructure, not tool-specific forks.

## Non-Goals

Beta 1 is not autonomous remediation, a general enterprise connector platform, a public multi-tenant service, an ATE implementation, or the Runtime v4 delivery milestone.
