# Validation Planning Prompt

Template: 1.0.0

Issue: 4762

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Run only preparation validation for #4762: diff hygiene, six-card surface integrity, bounded preparation review, and typed doctor with the expected expired-claim blocker recorded as execution-time work.

## Lane Inputs

Design: .csdlc/prepared/issues/4762/design.md

Diagram: .csdlc/prepared/issues/4762/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prep-diff-hygiene",
    "proof_role": "Confirm #4762 issue-local preparation artifacts have no whitespace/conflict-marker issues.",
    "acceptance_ids": [
      "AC7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check",
      "--",
      ".csdlc/issues/4762",
      ".csdlc/prepared/issues/4762",
      ".csdlc/evidence/4762"
    ],
    "parallel_group": "prep-local",
    "defer_reason": null
  },
  {
    "lane": "prep-card-surface",
    "proof_role": "Confirm all six rendered cards and values files exist for #4762.",
    "acceptance_ids": [
      "AC1",
      "AC4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "test",
      "-f",
      ".csdlc/issues/4762/cards/sip.md",
      "-a",
      "-f",
      ".csdlc/issues/4762/cards/stp.md",
      "-a",
      "-f",
      ".csdlc/issues/4762/cards/spp.md",
      "-a",
      "-f",
      ".csdlc/issues/4762/cards/vpp.md",
      "-a",
      "-f",
      ".csdlc/issues/4762/cards/srp.md",
      "-a",
      "-f",
      ".csdlc/issues/4762/cards/sor.md",
      "-a",
      "-f",
      ".csdlc/issues/4762/cards/sip.values.json",
      "-a",
      "-f",
      ".csdlc/issues/4762/cards/stp.values.json",
      "-a",
      "-f",
      ".csdlc/issues/4762/cards/spp.values.json",
      "-a",
      "-f",
      ".csdlc/issues/4762/cards/vpp.values.json",
      "-a",
      "-f",
      ".csdlc/issues/4762/cards/srp.values.json",
      "-a",
      "-f",
      ".csdlc/issues/4762/cards/sor.values.json"
    ],
    "parallel_group": "prep-local",
    "defer_reason": null
  },
  {
    "lane": "prep-doctor",
    "proof_role": "Run typed doctor and record only the expected claim_not_live gate as deferred to execution-time acquisition.",
    "acceptance_ids": [
      "AC5",
      "AC7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "csdlc-doctor",
      "--repo",
      "/Volumes/FastWork/adl-wp-4762",
      "--issue",
      "4762"
    ],
    "parallel_group": "prep-local",
    "defer_reason": "Expected preparation-only blocker: existing claim is expired and must be acquired by the later execution session."
  },
  {
    "lane": "prep-gpt-5.5-review",
    "proof_role": "Bounded preparation review over cards, design, diagram, paths, budgets, PVF lanes, and non-claims.",
    "acceptance_ids": [
      "AC6"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 1200,
    "budget_tokens": 6000,
    "argv": [
      "openai",
      "responses.create",
      "--model",
      "gpt-5.5",
      "--bounded-preparation-review",
      "#4762"
    ],
    "parallel_group": "review",
    "defer_reason": null
  }
]

## Parallelization

`prep-diff-hygiene` and `prep-card-surface` may run together. `prep-doctor` and `prep-gpt-5.5-review` should be recorded independently because one is typed local state and one is provider review evidence.

## Budgets

Seconds: 1200

Tokens: 8000

## Commands

- `git diff --check -- .csdlc/issues/4762 .csdlc/prepared/issues/4762 .csdlc/evidence/4762`
- `find .csdlc/issues/4762/cards -maxdepth 1 -type f`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo /Volumes/FastWork/adl-wp-4762 --issue 4762`
- Bounded OpenAI Responses request using model `gpt-5.5` and the approved credential source, writing retained output under `.csdlc/evidence/4762/gpt-5.5-review/`

## Failure Semantics

Fail closed on diff hygiene, missing card/value files, review-identified preparation blockers, or any doctor finding other than the expected `claim_not_live`. Do not reacquire the claim in preparation. Do not convert unavailable provider review into a fake pass.

## Handoff

Later execution must refresh this VPP if it adds source code, validators, COTS dependencies, runtime changes, cloud services, publication, or closeout work.
