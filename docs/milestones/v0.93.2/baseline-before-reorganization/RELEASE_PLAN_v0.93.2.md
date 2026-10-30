# v0.93.2 Release Plan

## Metadata

Planning template set: 1.1.0. Target: v0.93.2. Planning issue: #922. Accountable role: milestone owner; named execution owners remain unassigned.

## Status

Milestone opening authorized on 2026-10-19; [WP-01 #1296](https://github.com/agent-logic/agent-design-language/issues/1296) is in progress in Planning #12. See [opening record](OPENING_1296.md). Downstream execution, provider spend, deployment and release are not authorized by this opening.

## How To Use

Execute only after product qualification; this is not release authority.

## 0. Release-Tail Convergence

TAIL-01 quality decision → TAIL-02 docs/review handoff → TAIL-03 publication finalization → TAIL-04 internal review → TAIL-05 external review → TAIL-06 remediation/disposition → TAIL-07 successor planning → TAIL-08 successor closeout planning → TAIL-09 planning review → TAIL-10 release ceremony/close.

## 1. Release Readiness

Require accepted installed lockset, all 67 result dispositions, independent qualification and explicit handoff of any unresolved v0.93.1 #875 disposition.

## 2. Branch And Tag Preparation

Pin reviewed source heads and release versions per repository. No tag or branch is created by this document; verify compatible consumer upgrade and rollback before authorization.

## 3. GitHub Release Steps

Prepare private/public artifact manifests, provenance, notes and review evidence. Release creation and publication require explicit operator authority through current repository workflow.

## 4. Verification

Read back published identities and validate installation, compatibility, recovery and privacy at the actual destinations.

## 5. Communication

State delivered behavior, unsupported scenarios and remaining risks. v0.93.1 targets ready-to-deploy qualification; public launch remains separately authorized through CodeFriend #92.

## Exit Criteria

Authorized release readback and successor handoff are accepted; failed or deferred results remain visible.

## v0.93.2 scope integration

The milestone retains the existing platform tracks and adds CodeFriend plus the approved full GHB program. CodeFriend public launch is routed through existing CodeFriend #92 and still requires exact deployment authorization. GHB requires Gate A, Gate B, and WP12 before milestone qualification; planning approval does not authorize experiments, provider spend, deployment, or issue creation. Runtime-v4 activation/generation and citizen-continuity/CM mechanisms are shared dependencies for GHB-WP07/WP08, not duplicate engines, schedulers, or transaction owners. ADL #1226 remains a deferred assessment.
