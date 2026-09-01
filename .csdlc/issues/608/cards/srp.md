# Structured Review Prompt

Template: 1.0.0

Issue: 608

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/provider/http_family.rs
adl/src/provider/http_family/tests.rs
.csdlc/prepared/issues/608
.csdlc/evidence/608
.csdlc/issues/608

## Prompts

- Does global location use the first-party global Vertex endpoint without requiring a custom override?
- Are regional endpoint behavior and trust policy preserved?
- Are thinking controls config-backed, mutually constrained, and tested?
- Does live proof avoid credential exposure and exclude Polis integration?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Hosted CI remains the final integration gate before merge.
- Live Vertex proof used the approved company service-account key path in the implementation session; committed evidence redacts local key path, token material, and service-account identity.
- #592 Polis integration remains a follow-on and is intentionally not implemented by #608.

## Review Result

Revision: Some("git-blake3:01c1e3abe635850261db1b3062535230e986a202:ed6b585597baf5bcc7eec8ed5c1bc47d0832d840c5c95b7d514f93a4162116f4")

Reviewer: Some("fresh-session:46bb64c7-f7cb-4e58-9591-d0e4ad8af0ed")

Result: pass
