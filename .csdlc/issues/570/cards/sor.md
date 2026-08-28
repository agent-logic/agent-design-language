# Structured Output Record

Template: 1.0.0

Issue: 570

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Align C-SDLC v3 documentation and skill guidance for cutover readiness while preserving C-SDLC v2 as the live lifecycle authority until V3-F/#505.

## Artifacts

- AGENTS.md
- docs/onboarding.md
- csdlc-v3/README.md
- csdlc-v2/AGENTS.md
- csdlc-v2/operator/SKILLS.md
- csdlc-v2/operator/skills/csdlc-v2-bind/SKILL.md
- csdlc-v2/operator/skills/csdlc-v2-card-editor/SKILL.md
- csdlc-v2/operator/skills/csdlc-v2-clean/SKILL.md
- csdlc-v2/operator/skills/csdlc-v2-doctor/SKILL.md
- csdlc-v2/operator/skills/csdlc-v2-finish/SKILL.md
- csdlc-v2/operator/skills/csdlc-v2-github/SKILL.md
- csdlc-v2/operator/skills/csdlc-v2-init/SKILL.md
- csdlc-v2/operator/skills/csdlc-v2-publish/SKILL.md
- csdlc-v2/operator/skills/csdlc-v2-review/SKILL.md
- csdlc-v2/operator/skills/csdlc-v2-shepherd/SKILL.md
- csdlc-v2/operator/skills/csdlc-v2-validate/SKILL.md
- .csdlc/prepared/issues/570/validate-authority-boundary.sh
- .csdlc/prepared/issues/570/validate-docs-routes.sh
- .csdlc/prepared/issues/570/validate-skill-guidance.sh
- /Users/daniel/.codex/skills/pr-janitor/SKILL.md

## Execution

- Updated root AGENTS.md and docs/onboarding.md to make the until-V3-F/#505 v2 authority boundary explicit and add the three-minute prepared-v3 issue start target.
- Expanded csdlc-v3/README.md with current V3-A/B/C/D state, the #571 corrective gate, the clean replacement target, pre-#505 non-goals, and focused proof commands.
- Updated csdlc-v2/AGENTS.md, csdlc-v2/operator/SKILLS.md, and every checked-in v2 operator skill with until-cutover v2 authority and v3 construction non-authority guidance.
- Added and hardened #570 validator lanes for stale-route, skill-guidance, and authority-boundary scans.
- Checked installed operator guidance and applied matching local-only wording to /Users/daniel/.codex/skills/pr-janitor/SKILL.md; this path is outside the repository and is not part of the commit.

## Validation

[
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/570/validate-authority-boundary.sh"
    ],
    "purpose": "Run the issue-owned authority-boundary scan.",
    "outcome": "passed",
    "evidence_ref": "authority-boundary-scan.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run git diff --check.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/570/validate-docs-routes.sh"
    ],
    "purpose": "Run the issue-owned docs route scan.",
    "outcome": "passed",
    "evidence_ref": "docs-stale-route-scan.log"
  },
  {
    "command": [
      "bash",
      ".csdlc/prepared/issues/570/validate-skill-guidance.sh"
    ],
    "purpose": "Run the issue-owned skill guidance scan.",
    "outcome": "passed",
    "evidence_ref": "skill-guidance-scan.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
