## Summary

Execute **WP-02** as a non-destructive repository copy into `agent-logic`.
Every repository under `danielbaustin` remains present and unchanged. This
issue does not authorize GitHub repository transfer, source deletion, source
rename, source archival, source visibility change, or source settings mutation.

## Exact Repository Denominator

Create five independent destination copies in this order:

1. `danielbaustin/cognitive-sdlc-paper` -> `agent-logic/cognitive-sdlc-paper` (private)
2. `danielbaustin/godel-hadamard-bayes-paper` -> `agent-logic/godel-hadamard-bayes-paper` (private)
3. `danielbaustin/general-intelligence-paper-private` -> `agent-logic/general-intelligence-paper-private` (private)
4. `danielbaustin/universal-tool-schema` -> `agent-logic/universal-tool-schema` (private)
5. `danielbaustin/agent-design-language` -> `agent-logic/agent-design-language` (public)

Immutable negative controls:

- `danielbaustin/asksifu` remains unchanged and receives no destination copy.
- `danielbaustin/Horust` remains unchanged and receives no destination copy.

## Required Outcome

Five verified destination mirrors with exact Git refs/history and Git LFS
objects, reconstructed destination-only configuration, explicit dispositions
for GitHub metadata that ordinary duplication cannot copy, and before/after
proof that all seven source repositories remain unchanged.

## Deliverables

- Organization readiness and exact seven-repository denominator receipt
- Fixed four-private/one-public destination visibility matrix
- Five redacted source-before, destination-after, and source-after manifests
- Exact Git ref/object and Git LFS parity proof for each destination
- Destination-only ruleset, Actions, Pages, package, OIDC, webhook, App,
  security, and consumer dispositions
- Explicit issue, pull-request, collaborator, and other non-copyable metadata
  dispositions without claiming false parity
- `asksifu` and `Horust` negative-control evidence
- Final copy report with zero unexplained source or destination drift

## Dependencies

- WP-01 issue #5817 and corrective WP-01B issue #5818 complete
- Copy-only migration plan is reviewed and current
- Destination organization owners, billing owner, recovery contact, and 2FA
  readiness are confirmed
- Five destination names are free
- Organization billing, private-repository, Actions, Pages, package, LFS, and
  security capabilities are ready

## Owned Paths

- `.adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md`
- `.adl/docs/TBD/V092_SPRINT_5858_FOUNDATION_SESSION_PROMPT.md`
- `.csdlc/evidence/5819`
- `.csdlc/issues/5819`
- `.csdlc/prepared/issues/5819`
- `docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md`
- `docs/milestones/v0.92/QUALITY_GATE_v0.92.md`
- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/SPRINT_v0.92.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml`
- `docs/milestones/v0.92/features/README.md`

## Read-Only Inputs

- The seven source repositories and all source-side settings are read-only.
- Existing `agent-logic` repositories outside the five named destinations are
  read-only integration inputs.
- Secret values and credential material are never read into retained evidence.

## Safety Invariants

- Every write names an `agent-logic/<repository>` destination explicitly.
- No write, transfer, settings mutation, or destructive operation targets a
  `danielbaustin` repository.
- A destination is created only after the operator starts that named copy
  window.
- One destination is completed and verified before the next begins.
- ADL copies last.
- `/private/tmp` is never used.
- Destination Actions are disabled before the first mirrored ref arrives and
  remain disabled until destination configuration is proven.

## Acceptance Criteria

- Organization readiness, destination names, exact five-copy allowlist, exact
  two-control denylist, and visibility matrix are verified.
- Each source has a redacted exact-ref manifest before copying.
- Exactly five destinations are created in order, with four private and ADL
  public.
- Each destination has exact Git ref/object parity and complete Git LFS parity.
- Every non-Git GitHub surface is reconstructed at the destination or has a
  reviewed truthful disposition; ordinary duplication is not claimed to copy
  issues, pull requests, or settings.
- A post-copy source manifest proves owner, visibility, default branch, refs,
  HEAD, and settings inventory remain unchanged for every copied source.
- `asksifu` and `Horust` remain unchanged and absent from `agent-logic`.
- Issue #5888 owns the four `agent-logic.ai` link updates and remains blocked
  until ADL's public destination passes verification; website cutover is not a
  WP-02 closeout requirement.
- Final evidence contains no secret values and no unexplained drift.

## Non-Goals

- Transferring, deleting, renaming, archiving, or changing any source repository
- Copying or modifying `asksifu` or `Horust`
- Claiming GitHub issues, pull requests, or settings were copied by a Git mirror
- WP-02A CI redesign
- Exporting secret values
- Downstream milestone implementation

## Recovery

Stop at the active destination boundary. Preserve every source unchanged.
Repair, quarantine, or, with explicit operator authorization, delete only the
incomplete destination. There is no transfer-back path because source ownership
never moves.

## Execution Boundary

This issue is copy-preflight only until #5818 is terminal and the operator
starts the first named destination window. No source mutation is authorized.

<!-- csdlc-github-operation:v092-copy-only-body-5819-20260805 -->
