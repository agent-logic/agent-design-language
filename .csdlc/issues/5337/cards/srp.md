# Structured Review Prompt

Template: 1.0.0

Issue: 5337

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

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

## Review Result

Revision: Some("git-blake3:9042a20823f6b160377021542de19293cb724fc1:d1d5cf629d5c6c016b273d743eac3f9c5ce07401841cbd8d45931c543ea633ac")

Reviewer: Some("task:019f4b3e-6c61-7653-957b-7a2a6042a80d")

Result: pass
