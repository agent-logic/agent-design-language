# Structured Review Prompt

Template: 1.0.0

Issue: 604

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/604/full-cycle-defects-tail.md
.csdlc/prepared/issues/604/validate-implementation.sh
csdlc-v3/src/commands/remote/tests.rs
csdlc-v3/tests/real_issue_canary.rs
docs/csdlc-v3/full-replacement-denominator.json

## Prompts

- Does csdlc-publish ready verify exact live PR identity before and after mutation?
- Does reconcile-ready recover only from independently observed remote truth?
- Do stale generation/digest, wrong PR/head/repository, closed PR, and non-draft pre-state fail before lifecycle mutation?
- Are the publication skill and operator inventory aligned with the implemented command surface?
- Does the PR body include Closes #604 only after implementation, validation, and review truth are current?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- C-SDLC v3 remains incomplete and non-authoritative until #505/#596 implement and prove the full 21-entrypoint replacement denominator.
- The #604 v3 canaries prove real-record ingestion and non-authoritative model behavior only; they do not implement the missing v3 command families.

## Review Result

Revision: Some("git-blake3:6e40e130c27e483a074f27909e0642b37de1e1e6:344e236b483d51482560302ae8146117ec35f6a8a2458d354b2cb98ceabf2abb")

Reviewer: Some("/root/review_604_v3_full_delta")

Result: pass
