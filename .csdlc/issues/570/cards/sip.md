# Structured Intent Prompt

Template: 1.0.0

Issue: 570

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make the C-SDLC v3 documentation and skill guidance ready for the clean v3 replacement path without treating v3 as current lifecycle authority before V3-F.

## Required Outcome

A bounded documentation and skill-guidance update makes the active repo docs, v3 package docs, v2 operator-skill docs, and installed Codex PR skills agree on the same transition truth: v2 remains live authority until explicit V3-F cutover; v3 is the planned clean replacement line; prepared v3 issue start should be three minutes or less once dependencies are satisfied.

## Scope

- AGENTS.md
- docs/onboarding.md
- docs/architecture/ADL_ARCHITECTURE.md
- docs/tooling/adl_pr_cycle_skill.md
- csdlc-v2/AGENTS.md
- csdlc-v2/operator/SKILLS.md
- csdlc-v2/operator/skills/**/SKILL.md
- csdlc-v3/README.md
- csdlc-v3/AGENTS.md
- Operator-local installed Codex PR skills under CODEX_SKILLS_ROOT or the default Codex skills root, checked only as local evidence and not committed as repository artifacts.
- .csdlc/issues/570/**
- .csdlc/prepared/issues/570/**
- .csdlc/evidence/570/**

## Authority

- C-SDLC v2 remains the sole live lifecycle authority until explicit V3-F/#505 cutover approval.
- C-SDLC v3 remains construction and cutover-readiness evidence only before #505.
- #570 may update docs and skill guidance, but it must not publish, finish, clean, or mutate lifecycle state through v3.
- Installed Codex PR skill updates outside the repository are local-only and must be recorded truthfully if changed.

## Assumptions

- none

## Operator Constraints

- Keep #570 as an explicit Sprint 6 gate.
- Do not treat v3 as current authority before #505.
- Do not hide documentation or skill cutover readiness inside #502.
- Preserve the three-minute prepared-issue start target without bypassing typed v2 authority.
- Do not delete or retire v2 in #570.
