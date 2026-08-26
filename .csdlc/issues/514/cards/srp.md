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

- OpenAI Responses API review resp_0dde11635cd2b4fc006a8f586514a487d08a40eec3abb60f4d passed the #514 substantive provider-profile diff before the current-main ancestry refresh.
- Current review revision is the clean #514 head after merging origin/main for the shared CI timeout/coverage repair; provider-profile implementation scope remains bounded.
- Full workspace integration remains PR CI authority after republish.

## Review Result

Revision: Some("git-blake3:47e6b73c0c3d310eb520b73422bde2ae4f6108b4:790ddfc34212dce3b3366fae8c7fa21226cc30b22c0e444f402764d45fb4b694")

Reviewer: Some("openai-responses:gpt-5.6-sol")

Result: pass
