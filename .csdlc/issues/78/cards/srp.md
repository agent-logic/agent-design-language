# Structured Review Prompt

Template: 1.0.0

Issue: 78

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/78
.csdlc/issues/78
.csdlc/prepared/issues/78
csdlc-v2/src/cards.rs
csdlc-v2/src/store.rs
csdlc-v2/tests/gate5.rs

## Prompts

- Can any issue without typed review recovery invoke the operation?
- Can the operation modify any field other than STP deliverables?
- Does audit evidence retain both exact previous and replacement values?
- Do negative tests cover every phase, wrong card, stale CAS, malformed input, and projection drift?
- Does the design avoid weakening review and publication authority?

## Findings

[
  {
    "id": "F-78-1",
    "severity": "p2",
    "summary": "SPP steps S1-S3 remained pending after implementation and validation completed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:06d98991d9a92b2165ac383d3b42b5d40caab47e:4f336c134eeaa5fa0ed5bbe865474621a242ee4556b7872c943e325053f06c1d",
    "route": null
  },
  {
    "id": "F-78-2",
    "severity": "p2",
    "summary": "Focused proof covered stale generation but not stale digest and unchanged durable state.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:06d98991d9a92b2165ac383d3b42b5d40caab47e:4f336c134eeaa5fa0ed5bbe865474621a242ee4556b7872c943e325053f06c1d",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The operation is intentionally narrow; broader lifecycle correction completeness belongs to the separately planned C-SDLC v3 robustness work.

## Review Result

Revision: Some("git-blake3:06d98991d9a92b2165ac383d3b42b5d40caab47e:4f336c134eeaa5fa0ed5bbe865474621a242ee4556b7872c943e325053f06c1d")

Reviewer: Some("codex-subagent:Volta")

Result: pass
