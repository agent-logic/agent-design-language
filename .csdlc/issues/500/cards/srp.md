# Structured Review Prompt

Template: 1.0.0

Issue: 500

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

docs/csdlc-v3/CONTRACT.md
docs/csdlc-v3/predecessor-coverage.json
docs/csdlc-v3/proportional-lifecycle.json
csdlc-v3/Cargo.toml
csdlc-v3/Cargo.lock
csdlc-v3/src/lib.rs
.csdlc/prepared/issues/500
.csdlc/evidence/500
.csdlc/issues/500

## Prompts

- Does every retained requirement from #161 through #163 have exactly one traceable disposition?
- Can any contract clause be read as granting v3 operational authority before V3-F?
- Are construction and rollback decisions grounded in #162 measurements and #163 operator approval?
- Does the minimal crate surface stay non-authoritative and inside V3-A?
- Does the proportional-lifecycle matrix classify every checkpoint, projection, review, and transition as retained, collapsed, derived, or removed, with a concrete hazard named for every retained gate?
- Does the default path actually eliminate duplicate authority, repeated generation/digest choreography, duplicate readiness review, and umbrella re-review of child proof while preserving one design gate, focused validation, one independent implementation review, and truthful closeout?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Focused V3-A static contract proof does not implement V3-B/V3-C runtime behavior or authority cutover.

## Review Result

Revision: Some("git-blake3:c5182e209c7c0bc645440c1659b4f42c885595d3:2c83492ae76405f8b698f816f106bba02ab01dd266f83caa0186d07ad1b0aff8")

Reviewer: Some("fresh-session:e77d61f5-ab00-4989-8596-ad1b2327e172")

Result: pass
