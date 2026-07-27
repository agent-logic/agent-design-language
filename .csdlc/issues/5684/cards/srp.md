# Structured Review Prompt

Template: 1.0.0

Issue: 5684

Repository: danielbaustin/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/5684/diff-check.log
.csdlc/evidence/5684/gate10a-install-bootstrap-tests.log
.csdlc/evidence/5684/github-action-split-tests.log
.csdlc/evidence/5684/post-opus-csdlc-v2-clippy.log
.csdlc/evidence/5684/post-opus-csdlc-v2-fmt-check.log
.csdlc/evidence/5684/post-opus-csdlc-v2-full-tests.log
.csdlc/evidence/5684/post-opus-gate10a-tests.log
.csdlc/evidence/5684/post-opus-github-action-tests.log
.csdlc/evidence/5684/post-opus-resilience-tests.log
.csdlc/evidence/5684/resilience-tests.log
.csdlc/evidence/5684/runtime-resilience-check.log
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
csdlc-v2/src/operator.rs
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

[
  {
    "id": "OPUS-5684-01",
    "severity": "p1",
    "summary": "csdlc-github-pr run manually mapped PR-state request fields and could drift from csdlc-github compatibility facade.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652640f460c152b59e2a7bcfe52fd81d594f5dc0:803f00cb6cc8d7a368d7940e9c66169619df24bb21efabc318d5d9e66cfe9498",
    "route": null
  },
  {
    "id": "OPUS-5684-02",
    "severity": "p2",
    "summary": "Split binaries emit structured error JSON on stdout; accepted because this intentionally preserves the existing csdlc-github machine-output convention.",
    "actionable": true,
    "in_scope": true,
    "disposition": "accepted_risk",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "OPUS-5684-03",
    "severity": "p3",
    "summary": "Issue-create readback retry retried every reconciliation failure instead of only transient marker-lag cases.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652640f460c152b59e2a7bcfe52fd81d594f5dc0:803f00cb6cc8d7a368d7940e9c66169619df24bb21efabc318d5d9e66cfe9498",
    "route": null
  },
  {
    "id": "OPUS-5684-04",
    "severity": "p3",
    "summary": "adl-resilience exponential-backoff exponent cap was an undocumented magic constant without direct cap behavior proof.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652640f460c152b59e2a7bcfe52fd81d594f5dc0:803f00cb6cc8d7a368d7940e9c66169619df24bb21efabc318d5d9e66cfe9498",
    "route": null
  },
  {
    "id": "HEGEL-5684-01",
    "severity": "p1",
    "summary": "Owner-binary install dirty-source guard covered csdlc-v2 but not shared adl-resilience source, allowing dirty dependency code to stamp installed binaries.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652640f460c152b59e2a7bcfe52fd81d594f5dc0:803f00cb6cc8d7a368d7940e9c66169619df24bb21efabc318d5d9e66cfe9498",
    "route": null
  },
  {
    "id": "PASTEUR-5688-01",
    "severity": "p1",
    "summary": "PR #5688 failed csdlc-v2-standalone format because gate10a.rs needed rustfmt after the added operator-source assertion.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652640f460c152b59e2a7bcfe52fd81d594f5dc0:803f00cb6cc8d7a368d7940e9c66169619df24bb21efabc318d5d9e66cfe9498",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- Direct Anthropic claude-opus-5 and OpenRouter anthropic/claude-opus-5 both failed closed with HTTP 200 but no usable final review text; the actionable Opus-family review evidence is OpenRouter anthropic/claude-opus-4.8 plus Hegel exact delta review.
- Split-binary structured error JSON remains on stdout by design to preserve existing csdlc-github machine-output convention; human diagnostics stay on stderr where the current facade already does so.
- The retry classifier stops non-retryable reconciliation failures correctly; the generic retry trace still reports the existing terminal budget reason because adl-resilience has no separate not_retryable terminal enum yet.

## Review Result

Revision: Some("git-blake3:652640f460c152b59e2a7bcfe52fd81d594f5dc0:803f00cb6cc8d7a368d7940e9c66169619df24bb21efabc318d5d9e66cfe9498")

Reviewer: Some("subagent:019fa15b-8bd-7840-997c-9bff10247f65+opus-family")

Result: pass
