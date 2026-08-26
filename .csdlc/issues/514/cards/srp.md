# Structured Review Prompt

Template: 1.0.0

Issue: 514

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

adl/src/provider/mod.rs
adl/src/provider/profiles.rs
docs/provider/inference-profiles.md
docs/milestones/v0.92.1/evidence/provider/prov-a/README.md
.csdlc/prepared/issues/514/validate-profile-schema.rb
.csdlc/prepared/issues/514/validate-ollama-materialization.rb
.csdlc/prepared/issues/514/validate-invalid-profile.rb
.csdlc/prepared/issues/514/validate-last-known-good.rb
.csdlc/prepared/issues/514/validate-redaction.rb
.csdlc/evidence/514/profile-schema.log
.csdlc/evidence/514/ollama-materialization.log
.csdlc/evidence/514/invalid-profile.log
.csdlc/evidence/514/last-known-good.log
.csdlc/evidence/514/redaction.log

## Prompts

- Does the implementation stay inside the declared unit boundary?
- Does every acceptance criterion have proving evidence?
- Are operator-only actions and private material kept outside Git?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- OpenAI Responses API review artifact: response id resp_0dde11635cd2b4fc006a8f586514a487d08a40eec3abb60f4d, model gpt-5.6-sol, verdict PASS, publication_safe true.
- Validation is focused on the issue-local provider tests and validators; a full workspace test suite is not reported.
- Last-known-good retention is an in-memory document transaction contract; persistence and concurrent activation coordination remain caller responsibilities.

## Review Result

Revision: Some("git-blake3:9dacacbbf44c7ba9354bf1c1831f045928188b50:3220f5e1d194dbf98ccc653ef9f3489f0d20d5d4ab9d98717995ecbb71170eaf")

Reviewer: Some("openai-responses:gpt-5.6-sol")

Result: pass
