# Structured Review Prompt

Template: 1.0.0

Issue: 5337

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

adl-characterization
.csdlc/issues/5337
.csdlc/prepared/issues/5337
.csdlc/evidence/5337

## Prompts

- Does the crate remain independent of incumbent ADL source and accept only a caller-supplied pinned v1 binary?
- Does the corpus cover every required positive, negative, ordering, determinism, mock-execution, and signing behavior?
- Can any normalizer rule erase array order, identifiers, error class/value, exit status, prompt order/content, or signature verdicts?
- Are all cases repeated at least three times with immutable raw evidence and exact binary provenance?
- Do equivalence, difference, stability, and coverage checks fail closed on unexplained or missing evidence?
- Can any command execute a network, credentialed, remote, AWS, or paid provider?
- Are tests PVF-classified, deterministic, complete, and run with external Cargo output?
- Are all findings resolved at the exact substantive revision with no deferred acceptance criteria?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Pre-tokenization stream bytes are not retained; captured-stream hashes rely on the trusted capture implementation and exact reviewed Git revision, while portable bytes and envelopes remain offline-recomputable.
- An inherited blank-line-at-EOF warning exists in upstream main at .csdlc/issues/5600/retained/diagram.mmd; it is outside #5337 and absent from the issue-range diff.

## Review Result

Revision: Some("git-blake3:c5b7fda0e618ca15d322f3a0052b70a3278ba012:cabd4a1dd3bb609efb1a1667e149be2cce26a549f610a9ef36489fe665e6f17d")

Reviewer: Some("task:019f4b3e-6c61-7653-957b-7a2a6042a80d")

Result: pass
