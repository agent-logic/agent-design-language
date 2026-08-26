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

- Validation is focused on the provider-profile unit; broader workspace integration remains dependent on PR CI.
- OpenAI Responses API review artifact: response id resp_0a1dc925edf7ce12006a8f55eea81c87d0aaf8c23c83a661aa, model gpt-5.6-sol, publication_safe true.

## Review Result

Revision: Some("git-blake3:29bcb1b40ebab52dd086c04d8eba3de67d1aed0d:09774b0ee82929a0015fef2849116fed4074fc6477afc3aafcd49f1024c9097c")

Reviewer: Some("openai-responses:gpt-5.6-sol")

Result: pass
