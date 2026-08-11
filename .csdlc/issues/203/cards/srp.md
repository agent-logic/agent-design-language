# Structured Review Prompt

Template: 1.0.0

Issue: 203

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

Exact existing-store adapter and gate integration, certificate/lease/fencing low-level changes, deterministic/local time split, focused proof/evidence, typed issue truth, and absence of serving/migration/operational scope.

## Prompts

- Can any normal-build caller open an ungated store or reach a raw certificate, ledger, or fencing authorization/mutation path?
- Does every concrete operation consume and store-verify the exact signed artifact bound by the private #201 token without synthesizing authority?
- Are canonical lease state and receipts replica-deterministic while local safety anchors only delay restoration?
- Do Fence/Revoke, Activate, and OwnerCommit preserve exact floor, ledger, safety, possession, and barrier ordering across every crash window?
- Do retained handles, permits, receipts, retries, rollback, bounds, or unsafe paths ever expose partial authority?
- Does the exact proof bind all forty-four cases without claiming #205/#204 or operational serving?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: None

Reviewer: None

Result: pre_review
