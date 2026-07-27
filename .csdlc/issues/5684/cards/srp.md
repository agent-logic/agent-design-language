# Structured Review Prompt

Template: 1.0.0

Issue: 5684

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/5684/audit.jsonl
.csdlc/issues/5684/cards/sip.md
.csdlc/issues/5684/cards/sip.values.json
.csdlc/issues/5684/cards/sor.md
.csdlc/issues/5684/cards/sor.values.json
.csdlc/issues/5684/cards/spp.md
.csdlc/issues/5684/cards/spp.values.json
.csdlc/issues/5684/cards/srp.md
.csdlc/issues/5684/cards/srp.values.json
.csdlc/issues/5684/cards/stp.md
.csdlc/issues/5684/cards/stp.values.json
.csdlc/issues/5684/cards/vpp.md
.csdlc/issues/5684/cards/vpp.values.json
.csdlc/issues/5684/design.md
.csdlc/issues/5684/diagram.mmd
.csdlc/issues/5684/index.json
adl-resilience/Cargo.lock
adl-resilience/Cargo.toml
adl-resilience/src/lib.rs
adl-runtime/Cargo.lock
adl-runtime/Cargo.toml
adl-runtime/src/guardian.rs
adl-runtime/src/lib.rs
adl-runtime/src/supervision.rs
csdlc-v2/AGENTS.md
csdlc-v2/Cargo.lock
csdlc-v2/Cargo.toml
csdlc-v2/operator/coexistence.json
csdlc-v2/operator/skills.json
csdlc-v2/operator/skills/csdlc-v2-github/SKILL.md
csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md
csdlc-v2/src/bin/csdlc-github-issue.rs
csdlc-v2/src/bin/csdlc-github-pr.rs
csdlc-v2/src/github.rs
csdlc-v2/tests/gate10a.rs
csdlc-v2/tests/gate_github_actions.rs
docs/default_workflow.md
docs/templates/prompts/1.0.3/schemas/sip.structure.json
docs/templates/prompts/1.0.3/schemas/sor.structure.json
docs/templates/prompts/1.0.3/schemas/spp.structure.json
docs/templates/prompts/1.0.3/schemas/srp.structure.json
docs/templates/prompts/1.0.3/schemas/stp.structure.json
docs/templates/prompts/1.0.3/schemas/vpp.structure.json
docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md
docs/tooling/OWNER_BINARY_INSTALLATION.md
docs/tooling/README.md
docs/tooling/structured-prompt-validator-binary-resolution.md

## Prompts

- Review whether split binaries materially reduce the GitHub command surface and whether install/coexistence enforcement covers the new binaries.

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Historical audit entries retain the earlier 9006007f install-proof observation; current SOR values/rendered card and superseding audit sequence 8 record refreshed install proof at a0a270adc67678af9d4f5cb4712e1b2d3d8264aa.
- Read-only reviewer did not rerun Cargo tests or install commands; local implementation owner reran focused validation and install/verify proof on FastWork.

## Review Result

Revision: Some("git-blake3:f97c1234b3ccd8b2e8abf376c4e0eeb11f3da205:0ccaf66709c0ba2c3f8ad79b7c1e6acf5462de342bf7e71795c68a0c88369bc6")

Reviewer: Some("subagent:019fa13d-ca2e-7f13-b556-dbe8e168d611")

Result: pass
