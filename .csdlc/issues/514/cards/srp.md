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

## Prompts

- Does the implementation stay inside the declared unit boundary?
- Does every acceptance criterion have proving evidence?
- Are operator-only actions and private material kept outside Git?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- OpenAI Responses API review resp_0dde11635cd2b4fc006a8f586514a487d08a40eec3abb60f4d passed the #514 substantive provider-profile diff before the #560 shared coverage repair merge.
- Current review revision is the clean #514 head after merging origin/main at #560 merge 6a2a6f1d0b595797022eb291528a3c4c8c5541e9; provider-profile implementation scope remains bounded.
- Full workspace integration remains PR CI authority after republish.

## Review Result

Revision: Some("git-blake3:521422349892e58f828044a5238c2ea687d9b28a:7afe37b998c0f11dcae9e7d481f12c3f36fc93a532c88937c0e18f03b8c06613")

Reviewer: Some("openai-responses:gpt-5.6-sol")

Result: pass
